use anyhow::Result;
use hyper::{
    client::HttpConnector,
    header::{self, HeaderValue},
    Client, Request,
};
use hyper_tls::HttpsConnector;
use log::debug;
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json;
use url::Url;

mod request;
mod route;

pub use request::RequestBuilder;
pub use route::Route;

pub struct Api;

impl Api {
    pub fn route() -> Route {
        Route::from_static("/api")
    }
}

#[non_exhaustive]
pub enum RequestContent<'p, P = ()> {
    Empty,
    Payload(&'p P),
}

impl<'p> From<()> for RequestContent<'p> {
    fn from(_: ()) -> Self {
        RequestContent::Empty
    }
}

impl<'p, P> From<&'p P> for RequestContent<'p, P> {
    fn from(p: &'p P) -> Self {
        RequestContent::Payload(p)
    }
}

pub struct Http {
    token_header: HeaderValue,
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Http {
    const USER_AGENT_HEADER: &'static str = concat!(
        "DiscordBot (",
        env!("CARGO_PKG_HOMEPAGE"),
        ", ",
        env!("CARGO_PKG_VERSION"),
        ")",
    );

    const BASE_HOST: &'static str = "https://discord.com/";

    pub fn new(token: impl AsRef<str>) -> Result<Self> {
        let mut token_header = HeaderValue::from_str(token.as_ref())?;
        token_header.set_sensitive(true);

        let connector = HttpsConnector::new();
        let client = Client::builder().build(connector);

        Ok(Self {
            token_header,
            client,
        })
    }

    pub async fn send<'p, D, S>(
        &self,
        in_req: RequestBuilder,
        content: impl Into<RequestContent<'p, S>>,
    ) -> Result<D>
    where
        D: DeserializeOwned,
        S: Serialize + 'p,
    {
        let url = Url::parse(Self::BASE_HOST)?.join(in_req.route().to_string().as_str())?;
        let mut req = Request::builder().uri(url.as_str()).method(in_req.method());

        let body = if let RequestContent::Payload(data) = content.into() {
            serde_json::to_vec(data)?
        } else {
            Vec::new()
        };

        {
            let headers = req.headers_mut().unwrap();

            let extra_headers = in_req.headers().iter();

            let num_headers = 4 + extra_headers.size_hint().0 + if body.is_empty() { 0 } else { 1 };

            headers.reserve(num_headers);

            headers.insert(
                header::USER_AGENT,
                HeaderValue::from_static(Self::USER_AGENT_HEADER),
            );
            headers.insert(header::AUTHORIZATION, self.token_header.clone());
            headers.insert(header::CONTENT_LENGTH, body.len().into());
            headers.insert(
                "X-Ratelimit-Precision",
                HeaderValue::from_static("millisecond"),
            );

            if !body.is_empty() {
                headers.insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );
            }

            for (key, value) in extra_headers {
                headers.append(key, value.clone());
            }
        }

        let req = req.body(body.into())?;

        debug!("[Http] Sending request to {:?} -> {:?}", url, req);

        let res = self.client.request(req).await?;
        let res_data = hyper::body::to_bytes(res.into_body()).await?;

        debug!("[Http] Received response for request to {:?} -> {:?}", url, res_data);

        let res_content = serde_json::from_slice(&res_data)?;

        Ok(res_content)
    }
}
