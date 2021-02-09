use async_trait::async_trait;
use dashmap::DashMap;

use crate::models::Resource;

use super::Store;

pub struct MemoryStore<R: Resource>(DashMap<R::Id, R>);

impl<R: Resource> MemoryStore<R> {
    pub fn new() -> Self {
        Self(DashMap::new())
    }

    fn _get(&self, id: &R::Id) -> Option<R> {
        self.0.get(id).as_deref().cloned()
    }

    fn _insert(&self, resource: &R) -> Option<R> {
        self.0.insert(resource.id(), resource.clone())
    }

    fn _remove(&self, id: &R::Id) -> Option<R> {
        self.0.remove(id).map(|(_, r)| r)
    }
}

#[async_trait]
impl<R> Store<R> for MemoryStore<R>
where
    R: 'static + Resource + Send + Sync,
{
    async fn get(&self, ids: &[R::Id]) -> Vec<R> {
        ids.iter().filter_map(|id| self._get(id)).collect()
    }

    async fn insert(&self, resources: &[R]) -> Vec<R> {
        resources
            .iter()
            .filter_map(|resource| self._insert(resource))
            .collect()
    }

    async fn remove(&self, ids: &[R::Id]) -> Vec<R> {
        ids.iter().flat_map(|id| self._remove(id)).collect()
    }

    async fn get_one(&self, id: R::Id) -> Option<R> {
        self._get(&id)
    }

    async fn insert_one(&self, resource: &R) -> Option<R> {
        self._insert(resource)
    }

    async fn remove_one(&self, id: R::Id) -> Option<R> {
        self._remove(&id)
    }
}
