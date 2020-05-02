use std::ops::Deref;

use super::Client;

pub struct Context<'c, T = ()> {
    client: &'c Client,
    inner: T,
}

impl<'c, T> Context<'c, T> {
    pub fn new(client: &'c Client, inner: T) -> Self {
        Context { client, inner }
    }

    pub fn client(&self) -> &'c Client {
        self.client
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<'c, T> Deref for Context<'c, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
