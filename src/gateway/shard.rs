use std::{
    borrow::Cow,
    env::consts,
    time::{Duration, Instant},
};

use anyhow::Result;
use async_trait::async_trait;
use async_tungstenite::tungstenite::{protocol::frame::coding::CloseCode, Message as WsMessage};
use futures::{SinkExt, StreamExt};
use log::{debug, trace};
use serde_json;
use tokio::time::{delay_for, timeout, timeout_at};

use super::{
    GatewayCompression, GatewayConnectionParams, GatewayConnector, GatewayStream,
    PayloadCompression, PayloadEncoding, WsGatewayConnector,
};
use crate::{
    events::{payload::*, PayloadDelegate},
    models::Gateway,
    util::{AsyncSink, AsyncStream, NoneError},
};
use regex::Regex;

#[derive(Clone, Debug)]
#[non_exhaustive]
enum GatewayCloseCode {
    UnknownError,
    UnknownOpcode,
    DecodeError,
    NotAuthenticated,
    AuthenticationFailed,
    AlreadyAuthenticated,
    InvalidSeqnum,
    Ratelimited,
    SessionTimeout,
    InvalidShard,
    ShardingRequired,
    InvalidVersion,
    InvalidIntents,
    UnauthorizedIntents,
    Unknown,
}

impl From<u16> for GatewayCloseCode {
    fn from(code: u16) -> Self {
        match code {
            4000 => GatewayCloseCode::UnknownError,
            4001 => GatewayCloseCode::UnknownOpcode,
            4002 => GatewayCloseCode::DecodeError,
            4003 => GatewayCloseCode::NotAuthenticated,
            4004 => GatewayCloseCode::AuthenticationFailed,
            4005 => GatewayCloseCode::AlreadyAuthenticated,
            4007 => GatewayCloseCode::InvalidSeqnum,
            4008 => GatewayCloseCode::Ratelimited,
            4009 => GatewayCloseCode::SessionTimeout,
            4010 => GatewayCloseCode::InvalidShard,
            4011 => GatewayCloseCode::ShardingRequired,
            4012 => GatewayCloseCode::InvalidVersion,
            4013 => GatewayCloseCode::InvalidIntents,
            4014 => GatewayCloseCode::UnauthorizedIntents,

            // Fallback case
            #[allow(overlapping_patterns)]
            4000..=4999 => GatewayCloseCode::Unknown,

            _ => panic!("Invalid gateway close code {:?}", code),
        }
    }
}

pub enum ConnectionState<C: GatewayConnector> {
    Disconnected,
    Connecting,
    Connected(GatewayState<C>),
}

impl<C: GatewayConnector> Default for ConnectionState<C> {
    fn default() -> Self {
        Self::Disconnected
    }
}

pub struct GatewayState<C: GatewayConnector> {
    connection: GatewayStream<C::Input, C::Output>,
    last_seqnum: Option<u64>,
    heartbeat_interval: Option<Duration>,
    last_heartbeat: Instant,
    last_heartbeat_ack: Instant,
}

impl<C: GatewayConnector> GatewayState<C> {
    fn new_from_connection(connection: GatewayStream<C::Input, C::Output>) -> Self {
        Self {
            connection,
            last_seqnum: Default::default(),
            heartbeat_interval: Default::default(),
            last_heartbeat: Instant::now(),
            last_heartbeat_ack: Instant::now(),
        }
    }
}

enum ConnectError<E> {
    ShouldReconnect,
    ShouldAbort(E),
}

pub struct Shard<C: GatewayConnector + Send> {
    gateway: Gateway,
    token: String,
    encoding: PayloadEncoding,
    compression: GatewayCompression,

    connector: C,
    state: ConnectionState<C>,
}

impl Shard<WsGatewayConnector> {
    pub fn default_with(gateway: Gateway, token: String) -> Self {
        Self::new(
            gateway,
            token,
            Default::default(),
            Default::default(),
            WsGatewayConnector,
        )
    }
}

impl<C: GatewayConnector + Send + Sync> Shard<C> {
    const GATEWAY_VERSION: u8 = 6;

    pub fn new(
        gateway: Gateway,
        token: String,
        encoding: PayloadEncoding,
        compression: GatewayCompression,
        connector: C,
    ) -> Self {
        Self {
            gateway,
            token,
            encoding,
            compression,
            connector,
            state: Default::default(),
        }
    }

    pub fn conn_params(&self) -> GatewayConnectionParams {
        GatewayConnectionParams {
            version: Self::GATEWAY_VERSION,
            encoding: self.encoding.clone(),
            compression: match &self.compression {
                GatewayCompression::Transport(ty) => Some(ty.clone()),
                _ => None,
            },
        }
    }

    // FIXME: This is a hack. This should be removed as soon as `serde` has support
    // for integer tag renaming.
    #[inline]
    fn deserialize_workaround_json(string: &str) -> Cow<str> {
        Regex::new(r#""op":\s*(\d+)"#)
            .unwrap()
            .replace(string, "\"op\":\"$1\"")
    }

    // FIXME: This is a hack. This should be removed as soon as `serde` has support
    // for integer tag renaming.
    #[inline]
    fn serialize_workaround_json(string: &str) -> Cow<str> {
        Regex::new(r#""op":\s*"(\d+)""#)
            .unwrap()
            .replace(string, "\"op\":$1")
    }

    #[inline]
    fn decode_bytes(&self, bytes: &[u8]) -> Result<Payload> {
        match self.encoding {
            PayloadEncoding::Json => Ok(serde_json::from_slice(
                Self::deserialize_workaround_json(std::str::from_utf8(bytes)?).as_bytes(),
            )?),
            PayloadEncoding::Etf => unimplemented!(),
        }
    }

    #[inline]
    async fn _decompress_bytes(&self, _bytes: &[u8]) -> Vec<u8> {
        todo!()
    }

    #[inline]
    async fn deserialize_message(&self, message: WsMessage) -> Result<Payload> {
        trace!("[Shard] Deserializing message {:?}", message);

        match message {
            WsMessage::Binary(_bytes) => unimplemented!(),
            WsMessage::Text(string) => self.decode_bytes(string.as_bytes()),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn serialize_payload(&self, payload: Payload) -> Result<WsMessage> {
        trace!("[Shard] Serializing payload {:?}", payload);

        match self.encoding {
            PayloadEncoding::Json => Ok(WsMessage::Text(
                Self::serialize_workaround_json(serde_json::to_string(&payload)?.as_str())
                    .into_owned(),
            )),
            PayloadEncoding::Etf => unimplemented!(),
        }
    }

    /// Computes a delay for reconnecting based on the number of previous
    /// `conn_attempts` using exponential backoff.
    #[inline]
    fn compute_connect_delay(conn_attempts: u32) -> Duration {
        // Delay follows the form 5 * 2^n where n is the number of previous attempts.
        // n is capped at 6 to prevent the delay from becoming unreasonably long.
        Duration::from_secs(5 * 2u64.pow(conn_attempts.min(6)))
    }

    async fn connect(
        &mut self,
    ) -> Result<GatewayStream<C::Input, C::Output>, ConnectError<anyhow::Error>> {
        let timeout_res = timeout(
            Duration::from_secs(5),
            self.connector
                .connect(self.gateway.clone(), self.conn_params()),
        )
        .await;

        match timeout_res {
            Ok(conn_res) => match conn_res {
                Ok(conn) => Ok(conn),
                Err(conn_err) => Err(ConnectError::ShouldAbort(conn_err)),
            },
            // Timeout error
            Err(_) => Err(ConnectError::ShouldReconnect),
        }
    }

    async fn ensure_connected(&mut self) -> Result<&mut GatewayState<C>> {
        if let ConnectionState::Disconnected = self.state {
            let mut conn_attempts = 0u32;

            let connection = loop {
                match self.connect().await {
                    Ok(conn) => break conn,
                    Err(conn_err) => match conn_err {
                        ConnectError::ShouldReconnect => {
                            delay_for(Self::compute_connect_delay(conn_attempts)).await
                        }
                        ConnectError::ShouldAbort(err) => return Err(err),
                    },
                }

                conn_attempts += 1;
            };

            let state = GatewayState::new_from_connection(connection);

            self.state = ConnectionState::Connected(state);
        }

        if let ConnectionState::Connected(state) = &self.state {
            if Self::should_heartbeat(state) {
                self.heartbeat().await?;
            }
        }

        if let ConnectionState::Connected(state) = &mut self.state {
            return Ok(state);
        }

        panic!("Expected to be connected");
    }

    async fn fetch_next(
        &mut self,
    ) -> Result<<Self as AsyncStream>::Item, <Self as AsyncStream>::Error> {
        let state = self.ensure_connected().await?;

        let next_heartbeat_time = Self::next_heartbeat_time(state);

        // If a heartbeat interval is set, only poll until the next time at
        // which a heartbeat should be sent. If the timeout is hit, the
        // connection-polling future will be cancelled and the resulting timeout error
        // will be mapped into a NoneError, allowing connection-maintaining activities
        // to be resumed before polling again.
        let message = if let Some(next_heartbeat_time) = next_heartbeat_time {
            timeout_at(next_heartbeat_time.into(), state.connection.next())
                .await
                // Map the timeout error into a NoneError
                .map_err(|_| NoneError)?
        } else {
            state.connection.next().await
        }
        .ok_or(NoneError)??;

        match message {
            WsMessage::Close(frame_opt) => match frame_opt {
                Some(frame) => match frame.code {
                    CloseCode::Library(code) => match code.into() {
                        // Close codes which can be recovered from
                        // TODO: Handle these more specifically (e.g. resume/reconnect)
                        GatewayCloseCode::UnknownError
                        | GatewayCloseCode::InvalidSeqnum
                        | GatewayCloseCode::Ratelimited
                        | GatewayCloseCode::SessionTimeout => return Err(NoneError.into()),

                        // Close codes which represent a fault on the side of the library and are
                        // likely unrecoverable
                        GatewayCloseCode::UnknownOpcode
                        | GatewayCloseCode::DecodeError
                        | GatewayCloseCode::NotAuthenticated
                        | GatewayCloseCode::AlreadyAuthenticated
                        | GatewayCloseCode::InvalidShard
                        | GatewayCloseCode::InvalidVersion
                        | GatewayCloseCode::InvalidIntents => todo!(), // TODO: return error

                        // Close codes which represent a fault on the side of the end-user and are
                        // unrecoverable
                        GatewayCloseCode::AuthenticationFailed
                        | GatewayCloseCode::ShardingRequired
                        | GatewayCloseCode::UnauthorizedIntents => todo!(), // TODO: return error

                        GatewayCloseCode::Unknown => {
                            debug!("[Shard] Unknown gateway close code {:?}", code);
                            return Err(NoneError.into());
                        }
                    },
                    // TODO: For now, it seems like all/most standard close codes should be
                    // recoverable and thus the shard should reconnect. This should be reevaluated
                    // later.
                    _ => return Err(NoneError.into()),
                },
                None => return Err(NoneError.into()),
            },
            _ => {}
        }

        let payload = self.deserialize_message(message).await?;

        trace!("[Shard] Received payload {:?}", payload);

        self.delegate_payload(&payload).await?;

        Ok(payload)
    }
}

// Gateway functionality
impl<C: GatewayConnector + Send + Sync> Shard<C> {
    #[inline]
    fn next_heartbeat_time(state: &GatewayState<C>) -> Option<Instant> {
        if let Some(heartbeat_interval) = state.heartbeat_interval {
            Some(state.last_heartbeat + heartbeat_interval)
        } else {
            None
        }
    }

    #[inline]
    fn should_heartbeat(state: &GatewayState<C>) -> bool {
        if let Some(next_heartbeat_time) = Self::next_heartbeat_time(state) {
            Instant::now() >= next_heartbeat_time
        } else {
            false
        }
    }

    async fn heartbeat(&mut self) -> Result<()> {
        let payload = match &self.state {
            ConnectionState::Connected(state) => Some(Payload::Heartbeat {
                data: Heartbeat(state.last_seqnum),
            }),
            _ => None,
        };

        if let Some(payload) = payload {
            debug!("[Shard] Sending heartbeat");

            if let ConnectionState::Connected(state) = &mut self.state {
                state.last_heartbeat = Instant::now();
            }

            self.push(payload).await?;
        }

        Ok(())
    }

    async fn identify(&mut self) -> Result<()> {
        debug!("[Shard] Identifying");

        self.push(Payload::Identify {
            data: Identify {
                token: self.token.clone(),
                properties: IdentifyProperties {
                    os: consts::OS.to_string(),
                    browser: env!("CARGO_PKG_NAME").to_string(),
                    device: env!("CARGO_PKG_NAME").to_string(),
                },
                use_payload_compression: Some(
                    if let GatewayCompression::Payload(PayloadCompression::Zlib) = self.compression
                    {
                        true
                    } else {
                        false
                    },
                ),
            },
        })
        .await
    }
}

#[async_trait]
impl<C: GatewayConnector + Send + Sync> PayloadDelegate for Shard<C> {
    async fn dispatch(&mut self, data: &Dispatch) -> Result<()> {
        debug!("[Shard] Received dispatch {{ seqnum: {:?} }}", data.seqnum);

        if let ConnectionState::Connected(state) = &mut self.state {
            state.last_seqnum = Some(data.seqnum);
        }

        Ok(())
    }

    async fn heartbeat_req(&mut self, _data: &Heartbeat) -> Result<()> {
        self.heartbeat().await
    }

    async fn hello(&mut self, data: &Hello) -> Result<()> {
        debug!("[Shard] Received hello");

        if let ConnectionState::Connected(state) = &mut self.state {
            debug!(
                "[Shard] Setting heartbeat interval to {:?}",
                data.heartbeat_interval()
            );
            state.heartbeat_interval = Some(data.heartbeat_interval());
        }

        // TODO: Handle resuming

        self.identify().await?;
        self.heartbeat().await?;

        Ok(())
    }

    async fn heartbeat_ack(&mut self) -> Result<()> {
        if let ConnectionState::Connected(state) = &mut self.state {
            debug!(
                "[Shard] Heartbeat acknowledged with latency of {:?}",
                state.last_heartbeat.elapsed()
            );

            state.last_heartbeat_ack = Instant::now();
        }

        Ok(())
    }
}

#[async_trait]
impl<C: GatewayConnector + Send + Sync> AsyncStream for Shard<C> {
    type Item = Payload;
    type Error = anyhow::Error;

    async fn next(&mut self) -> Option<Result<Self::Item, Self::Error>> {
        loop {
            // TODO: This needs proper reconnect/shutdown logic. Retrying indefinitely will
            // not solve all problems.
            match self.fetch_next().await {
                Err(err) => match err.downcast_ref::<NoneError>() {
                    Some(NoneError) => continue,
                    None => return Some(Err(err)),
                },
                Ok(res) => return Some(Ok(res)),
            }
        }
    }
}

#[async_trait]
impl<C: GatewayConnector + Send + Sync> AsyncSink for Shard<C> {
    type Item = Payload;
    type Error = anyhow::Error;

    async fn push(&mut self, payload: Self::Item) -> Result<(), Self::Error> {
        let message = self.serialize_payload(payload)?;

        trace!("[Shard] Sending message {:?}", message);

        let state = self.ensure_connected().await?;

        state.connection.send(message).await?;

        Ok(())
    }

    async fn close(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}
