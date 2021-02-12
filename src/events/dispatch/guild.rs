use futures_async_stream::try_stream;
use serde::{Deserialize, Serialize};

use crate::{
    events::{Event, StoreUpdate},
    models::{Guild, ResourceId, UnavailableGuild},
    store::Store,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct GuildCreate {
    #[serde(flatten)]
    pub guild: Guild,
}

impl<S> StoreUpdate<S> for GuildCreate
where
    S: Store<UnavailableGuild> + Store<Guild>,
{
    #[try_stream(boxed, ok = Event, error = anyhow::Error)]
    async fn update<'a>(&'a mut self, store: &'a S) {
        store.insert_one(&self.guild).await;

        let guild = self.guild.clone();

        if let Some(_) = Store::<UnavailableGuild>::remove_one(store, self.guild.id()).await {
            yield Event::GuildAvailable { guild }
        } else {
            yield Event::GuildJoined { guild }
        }
    }
}
