use std::time::Duration;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::prelude::*;

/// Data necessary for connecting to the gateway.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Gateway {
    url: String,
}

impl Gateway {
    pub fn route() -> Route {
        Api::route().join("/gateway")
    }

    pub async fn get(ctx: &Context<'_>) -> Result<Self> {
        Self::route().get().send(ctx, ()).await
    }

    /// The URL that can be used to connect to the gateway.
    pub fn url(&self) -> &str {
        &self.url
    }
}

/// Data necessary for connecting to the gateway.
///
/// This variant is only available to bot users and contains extra information related to sharding.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct BotGateway {
    url: String,
    shards: u64,
    session_start_limit: SessionStartLimit,
}

impl BotGateway {
    pub fn route() -> Route {
        Gateway::route().join("/bot")
    }

    /// The URL that can be used to connect to the gateway.
    pub fn url(&self) -> &String {
        &self.url
    }

    /// The recommended number of shards to start.
    pub fn recommended_shard_count(&self) -> u64 {
        self.shards
    }

    /// Data regarding session start limits.
    pub fn session_start_limit(&self) -> &SessionStartLimit {
        &self.session_start_limit
    }
}

/// Data regarding session start limits.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct SessionStartLimit {
    total: u64,
    remaining: u64,
    reset_after: u64,
}

impl SessionStartLimit {
    /// The total number of allowed session starts.
    pub fn total(&self) -> &u64 {
        &self.total
    }

    /// The remaining number of allowed session starts.
    pub fn remaining(&self) -> &u64 {
        &self.remaining
    }

    /// The amount of time until the session start limit resets.
    pub fn resets_after(&self) -> Duration {
        Duration::from_millis(self.reset_after)
    }
}
