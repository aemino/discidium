use serde::{Deserialize, Serialize};

mod connector;
mod shard;

pub use connector::*;
pub use shard::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum PayloadEncoding {
    #[serde(rename = "json")]
    Json,

    #[serde(rename = "etf")]
    Etf,
}

impl Default for PayloadEncoding {
    fn default() -> Self {
        Self::Json
    }
}

#[derive(Clone, Debug)]
pub enum GatewayCompression {
    None,
    Payload(PayloadCompression),
    Transport(TransportCompression),
}

impl Default for GatewayCompression {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PayloadCompression {
    Zlib,
}

impl Default for PayloadCompression {
    fn default() -> Self {
        Self::Zlib
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum TransportCompression {
    #[serde(rename = "zlib-stream")]
    ZlibStream,
}

impl Default for TransportCompression {
    fn default() -> Self {
        Self::ZlibStream
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct GatewayConnectionParams {
    #[serde(rename = "v")]
    version: u8,

    encoding: PayloadEncoding,

    #[serde(rename = "compress")]
    #[serde(default)]
    compression: Option<TransportCompression>,
}
