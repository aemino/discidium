use crate::util::{AsyncSink, AsyncStream};

mod delegate;
pub mod dispatch;
pub mod payload;

pub use delegate::PayloadDelegate;
pub use payload::Payload;

pub trait PayloadDuplex:
    AsyncStream<Item = Payload, Error = anyhow::Error>
    + AsyncSink<Item = Payload, Error = anyhow::Error>
    + Send
    + Sync
{
}

impl<T> PayloadDuplex for T where
    T: AsyncStream<Item = Payload, Error = anyhow::Error>
        + AsyncSink<Item = Payload, Error = anyhow::Error>
        + Send
        + Sync
{
}
