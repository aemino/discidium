use std::pin::Pin;

use anyhow::Result;
use async_trait::async_trait;
use async_tungstenite::tungstenite::{Error as WsError, Message as WsMessage};
use futures::{
    sink::Sink,
    stream::Stream,
    task::{Context, Poll},
};

use super::*;

mod ws;

use crate::models::Gateway;
pub use ws::WsGatewayConnector;

type GatewayResult = Result<WsMessage, WsError>;

// TODO: This entire abstraction is somewhat useless in its current form. It
// would be best to figure out how to make it more generic or else scrap it.
#[async_trait]
pub trait GatewayConnector {
    type Input: Stream<Item = GatewayResult> + Send + Sync + Unpin;
    type Output: Sink<WsMessage, Error = WsError> + Send + Sync + Unpin;

    async fn connect(
        &self,
        gateway: Gateway,
        conn_params: GatewayConnectionParams,
    ) -> Result<GatewayStream<Self::Input, Self::Output>>;
}

// TODO: This type shouldn't be necessary anymore
pub struct GatewayStream<
    I: Stream<Item = GatewayResult> + Send + Sync + Unpin,
    O: Sink<WsMessage, Error = WsError> + Send + Sync + Unpin,
> {
    stream: I,
    sink: O,
}

impl<I, O> GatewayStream<I, O>
where
    I: Stream<Item = GatewayResult> + Send + Sync + Unpin,
    O: Sink<WsMessage, Error = WsError> + Send + Sync + Unpin,
{
    pub fn new(stream: I, sink: O) -> Self {
        Self { stream, sink }
    }
}

impl<I, O> Stream for GatewayStream<I, O>
where
    I: Stream<Item = GatewayResult> + Send + Sync + Unpin,
    O: Sink<WsMessage, Error = WsError> + Send + Sync + Unpin,
{
    type Item = I::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.stream).poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}

impl<I, O> Sink<WsMessage> for GatewayStream<I, O>
where
    I: Stream<Item = GatewayResult> + Send + Sync + Unpin,
    O: Sink<WsMessage, Error = WsError> + Send + Sync + Unpin,
{
    type Error = WsError;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.sink).poll_ready(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: WsMessage) -> Result<(), Self::Error> {
        Pin::new(&mut self.sink).start_send(item)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.sink).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.sink).poll_close(cx)
    }
}
