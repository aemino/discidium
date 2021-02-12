use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::dispatch::DispatchEvent;

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
