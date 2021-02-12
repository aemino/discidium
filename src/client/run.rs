use async_trait::async_trait;
use futures_async_stream::try_stream;
use streamunordered::{StreamUnordered, StreamYield};
use type_map::concurrent::TypeMap;

use crate::{
    events::{Event, PayloadDuplex, StoreUpdate},
    models::Resource,
    store::{multiplex::MultiplexedStore, Store},
};

#[derive(Default)]
pub(crate) struct StoreCollection {
    stores: TypeMap,
}

impl StoreCollection {
    fn store<R: 'static + Resource + Send + Sync>(&self) -> &MultiplexedStore<R> {
        self.stores.get::<MultiplexedStore<R>>().unwrap()
    }

    fn store_mut<R: 'static + Resource + Send + Sync>(&mut self) -> &mut MultiplexedStore<R> {
        self.stores
            .entry::<MultiplexedStore<R>>()
            .or_insert_with(Default::default)
    }
}

#[async_trait]
impl<R: Resource> Store<R> for StoreCollection
where
    R: 'static + Send + Sync,
{
    async fn get(&self, ids: &[R::Id]) -> Vec<R> {
        self.store().get(ids).await
    }

    async fn insert(&self, resources: &[R]) -> Vec<R> {
        self.store().insert(resources).await
    }

    async fn remove(&self, ids: &[R::Id]) -> Vec<R> {
        self.store().remove(ids).await
    }

    async fn get_one(&self, id: &R::Id) -> Option<R> {
        self.store().get_one(id).await
    }

    async fn insert_one(&self, resource: &R) -> Option<R> {
        self.store().insert_one(resource).await
    }

    async fn remove_one(&self, id: &R::Id) -> Option<R> {
        self.store().remove_one(id).await
    }
}

pub struct Runner {
    pub(crate) payload_duplexes: Vec<Box<dyn PayloadDuplex>>,
    pub(crate) stores: StoreCollection,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            payload_duplexes: Default::default(),
            stores: Default::default(),
        }
    }

    pub fn add_payload_duplexes(
        &mut self,
        duplexes: impl IntoIterator<Item = Box<dyn PayloadDuplex>>,
    ) -> &mut Self {
        self.payload_duplexes.extend(duplexes);
        self
    }

    pub fn register_store<R: 'static + Resource + Send + Sync>(
        &mut self,
        store: impl Store<R>,
    ) -> &mut Self {
        self.stores.store_mut().0.push(Box::new(store));

        self
    }

    pub fn register_stores<R: 'static + Resource + Send + Sync>(
        &mut self,
        stores: impl IntoIterator<Item = Box<dyn Store<R>>>,
    ) -> &mut Self {
        self.stores.store_mut().0.extend(stores);

        self
    }

    #[try_stream(ok = Event, error = anyhow::Error)]
    pub async fn run(&mut self) {
        // TODO: Once PayloadDuplex impls the "real" Stream/Sink traits, this can be
        // simplified further.
        let payload_stream = self
            .payload_duplexes
            .iter_mut()
            .map(|duplex| {
                #[stream]
                async move {
                    while let Some(payload) = duplex.next().await {
                        yield payload;
                    }
                }
            })
            .collect::<StreamUnordered<_>>();

        #[for_await]
        for (result, _token) in payload_stream {
            match result {
                StreamYield::Item(payload) => {
                    let mut payload = payload?;

                    #[for_await]
                    for event in payload.update(&self.stores) {
                        yield event?;
                    }
                }
                StreamYield::Finished(stream) => stream.keep(),
            }
        }
    }
}
