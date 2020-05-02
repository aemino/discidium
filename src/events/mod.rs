use crate::util::{AsyncSink, AsyncStream};

mod delegate;
pub mod dispatch;
pub mod payload;

pub use delegate::PayloadDelegate;
pub use payload::Payload;

pub trait EventDelegate {
    fn ready(&self) {}
}

pub trait PayloadChannel: AsyncStream<Item = Payload> + AsyncSink<Item = Payload> {}

impl<T: AsyncStream<Item = Payload> + AsyncSink<Item = Payload>> PayloadChannel for T {}
