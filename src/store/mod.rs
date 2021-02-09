#[cfg(feature = "memory-store")]
pub mod memory;

use async_trait::async_trait;

use crate::models::Resource;

#[async_trait]
pub trait Store<R: Resource>: 'static + Send + Sync
where
    R: 'static + Send + Sync,
{
    async fn get(&self, ids: &[R::Id]) -> Vec<R>;
    async fn insert(&self, resources: &[R]) -> Vec<R>;
    async fn remove(&self, ids: &[R::Id]) -> Vec<R>;

    async fn get_one(&self, id: R::Id) -> Option<R> {
        self.get(&[id]).await.into_iter().next()
    }

    async fn insert_one(&self, resource: &R) -> Option<R> {
        self.insert(&[resource.clone()]).await.into_iter().next()
    }

    async fn remove_one(&self, id: R::Id) -> Option<R> {
        self.remove(&[id]).await.into_iter().next()
    }
}
