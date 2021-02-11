#![feature(bool_to_option)]
#![feature(type_alias_impl_trait)]
#![feature(generators)]
#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#![deny(clippy::all)]

pub mod client;
pub mod events;
pub mod gateway;
pub mod http;
pub mod models;
pub mod store;
pub mod util;

pub mod prelude {
    pub use crate::client::*;
}
