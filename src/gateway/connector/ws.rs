use anyhow::Result;
use async_trait::async_trait;
use async_tungstenite::{
    tokio::connect_async,
    tungstenite::{Error as WsError, Message as WsMessage},
};
use futures::{Sink, Stream, StreamExt};
use serde_urlencoded;
use url::Url;

use super::{GatewayConnectionParams, GatewayConnector, GatewayResult, GatewayStream};
use crate::models::Gateway;

pub struct WsGatewayConnector;

#[async_trait]
impl GatewayConnector for WsGatewayConnector {
    type Input = impl Stream<Item = GatewayResult>;
    type Output = impl Sink<WsMessage, Error = WsError>;

    async fn connect(
        &self,
        gateway: Gateway,
        conn_params: GatewayConnectionParams,
    ) -> Result<GatewayStream<Self::Input, Self::Output>> {
        let query_params = serde_urlencoded::to_string(conn_params)?;
        let mut url = Url::parse(gateway.url())?;
        url.set_query(Some(query_params.as_str()));

        let (ws_stream, _) = connect_async(url.as_str()).await?;

        let (sink, stream) = ws_stream.split();

        Ok(GatewayStream::new(stream, sink))
    }
}
