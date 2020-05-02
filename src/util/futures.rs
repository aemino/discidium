use async_trait::async_trait;

#[async_trait]
#[must_use = "streams do nothing unless polled"]
pub trait AsyncStream {
    type Item;
    type Error;

    async fn next(&mut self) -> Option<Result<Self::Item, Self::Error>>;
}

#[async_trait]
#[must_use = "sinks do nothing unless pushed to"]
pub trait AsyncSink {
    type Item;
    type Error;

    async fn push(&mut self, item: Self::Item) -> Result<(), Self::Error>;
    async fn close(&mut self) -> Result<(), Self::Error>;
}
