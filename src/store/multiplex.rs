use async_trait::async_trait;
use futures_lite::{stream, StreamExt};

use crate::models::Resource;

use super::Store;

pub(crate) struct MultiplexedStore<R: Resource>(pub(crate) Vec<Box<dyn Store<R>>>);

impl<R: Resource> Default for MultiplexedStore<R> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<R: Resource> Store<R> for MultiplexedStore<R>
where
    R: 'static + Send + Sync,
{
    // TODO: These insert and remove impls are wrong; these operations should be
    // performed on each store and followed by a resolution algorithm that takes
    // into account `received_at` timestamps.

    async fn get(&self, ids: &[R::Id]) -> Vec<R> {
        stream::iter(&self.0)
            .then(|store| store.get(ids))
            .find(|resources| !resources.is_empty())
            .await
            .unwrap_or_default()
    }

    async fn insert(&self, resources: &[R]) -> Vec<R> {
        stream::iter(&self.0)
            .then(|store| store.insert(resources))
            .find_map(|resources| (!resources.is_empty()).then_some(resources))
            .await
            .unwrap_or_default()
    }

    async fn remove(&self, ids: &[R::Id]) -> Vec<R> {
        stream::iter(&self.0)
            .then(|store| store.remove(ids))
            .find_map(|resources| (!resources.is_empty()).then_some(resources))
            .await
            .unwrap_or_default()
    }

    async fn get_one(&self, id: &R::Id) -> Option<R> {
        stream::iter(&self.0)
            .then(|store| store.get_one(id))
            .find_map(|resource| resource)
            .await
    }

    async fn insert_one(&self, resource: &R) -> Option<R> {
        stream::iter(&self.0)
            .then(|store| store.insert_one(resource))
            .find_map(|resource| resource)
            .await
    }

    async fn remove_one(&self, id: &R::Id) -> Option<R> {
        stream::iter(&self.0)
            .then(|store| store.remove_one(id))
            .find_map(|resource| resource)
            .await
    }
}
