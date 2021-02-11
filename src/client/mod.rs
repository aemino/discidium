use anyhow::Result;
use log::debug;

use crate::{
    events::PayloadDuplex,
    gateway::Shard,
    http::Http,
    models::{Gateway, Guild},
    store::memory::MemoryStore,
};

mod context;
mod run;

pub use self::{context::Context, run::Runner};

pub struct Client {
    token: String,
    http: Http,
}

impl Client {
    pub fn new(token: impl AsRef<str>) -> Result<Self> {
        let token = token.as_ref().into();
        let http = Http::new(&token)?;

        Ok(Self { token, http })
    }

    pub fn http(&self) -> &Http {
        &self.http
    }

    pub fn wrap<T>(&self, inner: T) -> Context<T> {
        Context::new(self, inner)
    }

    pub fn context(&self) -> Context {
        Context::new(self, ())
    }

    pub async fn default_runner(&self) -> Result<Runner> {
        let gateway = Gateway::get(&self.context()).await?;

        debug!("[Client] Using gateway {:?}", gateway);

        let shards: Vec<Box<dyn PayloadDuplex>> =
            vec![Box::new(Shard::default_with(gateway, self.token.clone()))];

        let mut runner = Runner::new();

        runner
            .add_payload_duplexes(shards)
            .register_store(MemoryStore::<Guild>::new());

        Ok(runner)
    }
}
