use anyhow::Result;

use crate::{
    events::{EventDelegate, PayloadChannel},
    http::Http,
};

pub mod context;

pub use self::context::Context;

pub struct Client {
    token: String,
    http: Http,
    event_delegate: Box<dyn EventDelegate>,
}

impl Client {
    pub fn new<Ev: EventDelegate + 'static>(
        token: impl AsRef<str>,
        event_delegate: Ev,
    ) -> Result<Self> {
        let token = token.as_ref().into();

        let http = Http::new(&token)?;

        let event_delegate = Box::new(event_delegate);

        Ok(Self {
            token,
            http,
            event_delegate,
        })
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
}
