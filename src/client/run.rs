use anyhow::Result;
use futures::{stream::FuturesUnordered, TryStreamExt};
use tokio::task;
use type_map::TypeMap;

use crate::{
    events::{EventDelegate, PayloadDuplex},
    models::Resource,
    store::Store,
};

pub struct RunOptions<'run, D> {
    pub(crate) delegate: &'run D,
    pub(crate) payload_duplexes: Vec<Box<dyn PayloadDuplex>>,
    pub(crate) stores: TypeMap,
}

pub type StoreCollection<R> = Vec<Box<dyn Store<R>>>;

impl<'run, D: EventDelegate> RunOptions<'run, D> {
    pub fn with_delegate(delegate: &'run D) -> Self {
        Self {
            delegate,
            payload_duplexes: Default::default(),
            stores: Default::default(),
        }
    }

    pub fn add_payload_duplexes(
        mut self,
        duplexes: impl IntoIterator<Item = Box<dyn PayloadDuplex>>,
    ) -> Self {
        self.payload_duplexes.extend(duplexes);
        self
    }

    pub fn register_store<R: 'static + Resource + Send + Sync>(
        mut self,
        store: impl Store<R>,
    ) -> Self {
        self.stores
            .entry::<StoreCollection<R>>()
            .or_insert_with(Default::default)
            .push(Box::new(store));

        self
    }

    pub fn register_stores<R: 'static + Resource>(
        mut self,
        stores: impl IntoIterator<Item = Box<dyn Store<R>>>,
    ) -> Self {
        self.stores
            .entry::<StoreCollection<R>>()
            .or_insert_with(Default::default)
            .extend(stores);

        self
    }

    pub async fn run(self) -> Result<()> {
        // TODO: Proper, non-anyhow error handling

        self.payload_duplexes
            .into_iter()
            .map(|mut duplex| {
                task::spawn(async move {
                    while let Some(payload) = duplex.next().await.transpose()? {
                        println!("payload :: {:?}", payload);
                    }

                    Ok::<(), anyhow::Error>(())
                })
            })
            .collect::<FuturesUnordered<_>>()
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }
}
