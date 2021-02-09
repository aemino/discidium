use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::dispatch::guild::*;

// A temporary workaround for https://github.com/serde-rs/serde/issues/1714
mod workaround {
    use serde::{Deserializer, de::IgnoredAny};

    pub fn deserialize_unknown_event<'de, D>(deserializer: D) -> Result<(), D::Error>
        where D: Deserializer<'de>,
    {
        deserializer.deserialize_any(IgnoredAny)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "op")]
#[non_exhaustive]
pub enum Payload {
    #[serde(rename = "0")]
    Dispatch(Dispatch),

    #[serde(rename = "1")]
    Heartbeat {
        #[serde(rename = "d")]
        data: Heartbeat,
    },

    #[serde(rename = "2")]
    Identify {
        #[serde(rename = "d")]
        data: Identify,
    },

    #[serde(rename = "10")]
    Hello {
        #[serde(rename = "d")]
        data: Hello,
    },

    #[serde(rename = "11")]
    HeartbeatAck,

    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dispatch {
    #[serde(rename = "s")]
    pub seqnum: u64,

    #[serde(flatten)]
    pub event: DispatchEvent,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "t", content = "d", rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum DispatchEvent {
    Ready {},

    GuildCreate(GuildCreate),

    #[serde(other, deserialize_with = "workaround::deserialize_unknown_event")]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Heartbeat(pub Option<u64>);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Identify {
    pub token: String,
    pub properties: IdentifyProperties,

    #[serde(rename = "compress")]
    #[serde(default)]
    pub use_payload_compression: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct IdentifyProperties {
    #[serde(rename = "$os")]
    pub os: String,

    #[serde(rename = "$browser")]
    pub browser: String,

    #[serde(rename = "$device")]
    pub device: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Hello {
    heartbeat_interval: u64,
}

impl Hello {
    pub fn heartbeat_interval(&self) -> Duration {
        Duration::from_millis(self.heartbeat_interval)
    }
}
