use futures_async_stream::try_stream;
use serde::{Deserialize, Serialize};

use crate::{
    events::{Event, StoreUpdate},
    models::message::Message,
    store::Store,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct MessageCreate {
    #[serde(flatten)]
    message: Message,
}

impl<S> StoreUpdate<S> for MessageCreate
where
    S: Store<Message>,
{
    #[try_stream(boxed, ok = Event, error = anyhow::Error)]
    async fn update<'a>(&'a mut self, store: &'a S) {
        store.insert_one(&self.message).await;

        let message = self.message.clone();
        yield Event::MessageSent { message };
    }
}
