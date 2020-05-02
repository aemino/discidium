use anyhow::Result;

use hyper::{
    header::{HeaderValue, IntoHeaderName},
    HeaderMap, Method,
};

use serde::{de::DeserializeOwned, ser::Serialize};

use super::{RequestContent, Route};
use crate::client::Context;

pub struct RequestBuilder {
    pub(crate) route: Route,
    pub(crate) method: Method,
    pub(crate) headers: HeaderMap,
}

impl RequestBuilder {
    pub fn new(route: Route, method: Method) -> Self {
        Self {
            route,
            method,
            headers: HeaderMap::default(),
        }
    }

    pub(crate) fn route(&self) -> &Route {
        &self.route
    }

    pub(crate) fn method(&self) -> &Method {
        &self.method
    }

    pub(crate) fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn header<K: IntoHeaderName>(&mut self, key: K, val: HeaderValue) -> &mut Self {
        self.headers.insert(key, val);

        self
    }

    pub async fn send<'p, T, D, S>(
        self,
        ctx: &Context<'_, T>,
        content: impl Into<RequestContent<'p, S>>,
    ) -> Result<D>
    where
        D: DeserializeOwned,
        S: Serialize + 'p,
    {
        ctx.client().http().send(self, content).await
    }
}
