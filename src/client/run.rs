use futures_async_stream::try_stream;
use streamunordered::{StreamUnordered, StreamYield};
use type_map::concurrent::TypeMap;

use crate::{
    events::{Payload, PayloadDuplex},
    models::Resource,
    store::Store,
};

pub struct Runner {
    pub(crate) payload_duplexes: Vec<Box<dyn PayloadDuplex>>,
    pub(crate) stores: TypeMap,
}

pub type StoreCollection<R> = Vec<Box<dyn Store<R>>>;

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
        self.stores
            .entry::<StoreCollection<R>>()
            .or_insert_with(Default::default)
            .push(Box::new(store));

        self
    }

    pub fn register_stores<R: 'static + Resource>(
        &mut self,
        stores: impl IntoIterator<Item = Box<dyn Store<R>>>,
    ) -> &mut Self {
        self.stores
            .entry::<StoreCollection<R>>()
            .or_insert_with(Default::default)
            .extend(stores);

        self
    }

    #[try_stream(ok = Payload, error = anyhow::Error)]
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
                StreamYield::Item(payload) => yield payload?,
                StreamYield::Finished(stream) => stream.keep(),
            }
        }
    }
}
