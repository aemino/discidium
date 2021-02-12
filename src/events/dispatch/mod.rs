pub mod channel;
pub mod guild;
pub mod message;

use futures_async_stream::try_stream;
use message::MessageCreate;
use serde::{Deserialize, Serialize};

use crate::{
    events::{Event, StoreUpdate},
    models::UnavailableGuild,
    store::Store,
};

use self::guild::GuildCreate;

// A temporary workaround for https://github.com/serde-rs/serde/issues/1714
mod workaround {
    use serde::{de::IgnoredAny, Deserializer};

    pub fn deserialize_unknown_event<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(IgnoredAny)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "t", content = "d", rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum DispatchEvent {
    Ready(Ready),

    GuildCreate(GuildCreate),

    MessageCreate(MessageCreate),

    #[serde(other, deserialize_with = "workaround::deserialize_unknown_event")]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Ready {
    #[serde(rename = "v")]
    gateway_version: u8,

    session_id: String,
    guilds: Vec<UnavailableGuild>,
}

impl<S> StoreUpdate<S> for Ready
where
    S: Store<UnavailableGuild>,
{
    #[try_stream(boxed, ok = Event, error = anyhow::Error)]
    async fn update<'a>(&'a mut self, store: &'a S) {
        store.insert(&self.guilds).await;
    }
}
