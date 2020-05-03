use anyhow::Result;
use futures::{stream::FuturesUnordered, TryStreamExt};
use log::debug;
use tokio::task::{self, JoinError};

use crate::{
    events::{EventDelegate, PayloadDuplex},
    gateway::Shard,
    http::Http,
    models::Gateway,
};

mod context;
mod run;

pub use self::{context::Context, run::RunOptions};

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

    pub async fn run(&self, delegate: &dyn EventDelegate) -> Result<()> {
        let gateway = Gateway::get(&self.context()).await?;

        debug!("[Client] Using gateway {:?}", gateway);

        let shards: Vec<Box<dyn PayloadDuplex>> =
            vec![Box::new(Shard::default_with(gateway, self.token.clone()))];

        self.run_with_options(RunOptions::with_delegate(delegate).payload_duplexes(shards))
            .await?;

        Ok(())
    }

    pub async fn run_with_options(&self, options: RunOptions<'_>) -> Result<(), JoinError> {
        // TODO: Proper, non-anyhow error handling

        options
            .payload_duplexes
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
            .await?;

        Ok(())
    }
}
