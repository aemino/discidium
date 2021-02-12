mod delegate;
pub mod dispatch;
pub mod payload;

pub use self::delegate::PayloadDelegate;
pub use self::payload::Payload;

use std::pin::Pin;

use futures::{stream, Stream};
use futures_async_stream::try_stream;

use crate::{models::{Guild, UnavailableGuild, message::Message}, store::Store, util::{AsyncSink, AsyncStream}};

use self::dispatch::DispatchEvent;

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

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event {
    GuildAvailable { guild: Guild },
    GuildJoined { guild: Guild },
    MessageSent { message: Message }
}

pub(crate) trait StoreUpdate<S> {
    #[try_stream(boxed, ok = Event, error = anyhow::Error)]
    async fn update<'a>(&'a mut self, store: &'a S);
}

impl<S> StoreUpdate<S> for Payload
where
    S: Store<UnavailableGuild> + Store<Guild> + Store<Message>,
{
    fn update<'a>(
        &'a mut self,
        store: &'a S,
    ) -> Pin<Box<dyn Stream<Item = Result<Event, anyhow::Error>> + Send + '_>> {
        match self {
            Payload::Dispatch(dispatch) => match &mut dispatch.event {
                DispatchEvent::Ready(event) => event.update(store),
                DispatchEvent::GuildCreate(event) => event.update(store),
                DispatchEvent::MessageCreate(event) => event.update(store),

                _ => Box::pin(stream::empty()),
            },
            _ => Box::pin(stream::empty()),
        }
    }
}
