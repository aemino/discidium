#![feature(type_alias_impl_trait)]
#![deny(clippy::all)]

pub mod util;
pub mod client;
pub mod events;
pub mod gateway;
pub mod http;
pub mod models;

pub mod prelude {
    pub use crate::client::*;
    pub use crate::events::EventDelegate;
}
