use std::str::FromStr;
use std::task::{Context, Poll};

use futures_util::future::BoxFuture;
use hyper::header::HOST;
use hyper::{Request, Response, Uri};
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use tower::Service;
use tracing::debug;

pub use make::MakeProxyService;
use satex_core::endpoint::Endpoint;
use satex_core::essential::Essential;
use satex_core::http::Body;
use satex_core::{satex_error, Error};
use satex_discovery::{NamedServerDiscovery, ServerDiscovery};

mod make;

const DEFAULT_SCHEMA: &str = "http";

const REMOVE_HEADER_NAMES: [&str; 9] = [
    "connection",
    "keep-alive",
    "transfer-encoding",
    "te",
    "trailer",
    "proxy-authorization",
    "proxy-authenticate",
    "x-application-context",
    "upgrade",
];

type HttpClient = Client<HttpsConnector<HttpConnector>, Body>;

#[derive(Clone)]
pub struct ProxyService {
    uri: String,
}

impl ProxyService {
    pub fn new(uri: impl Into<String>) -> Self {
        Self { uri: uri.into() }
    }
}

impl Service<Request<Body>> for ProxyService {
    type Response = Response<Body>;
    type Error = Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let uri = self.uri.clone();
        Box::pin(async move { proxy(uri, req).await })
    }
}

async fn proxy(uri: String, mut req: Request<Body>) -> Result<Response<Body>, Error> {
    let prefix = uri.strip_suffix('/').unwrap_or(&uri);
    let path = req
        .uri()
        .path_and_query()
        .map(|x| x.as_str())
        .unwrap_or(req.uri().path())
        .to_string();

    // reconstruct uri
    let mut uri = format!("{}{}", prefix, path)
        .parse::<Uri>()
        .map_err(|e| satex_error!(e))?;
    let host = uri
        .host()
        .ok_or_else(|| satex_error!("Proxy service uri miss `host`!"))?;

    // get server instance from discovery
    let discovery = req
        .extensions_mut()
        .remove::<NamedServerDiscovery>()
        .ok_or_else(|| satex_error!("Cannot get `ServerDiscovery` extension!"))?;

    // replace proxy uri host and port
    let essential = req
        .extensions_mut()
        .remove::<Essential>()
        .ok_or_else(|| satex_error!("Cannot get `Essential` extension!"))?;

    if let Some(endpoint) = discovery.resolve(&essential, host).await? {
        let host = match endpoint {
            Endpoint::Ip(addr) => format!("{}:{}", addr.ip(), addr.port()),
            Endpoint::Raw(raw) => raw.to_string(),
        };
        uri = Uri::from_str(&format!(
            "{}://{}{}",
            uri.scheme_str().unwrap_or(DEFAULT_SCHEMA),
            host,
            path
        ))
        .map_err(|e| satex_error!(e))?;
    }
    *req.uri_mut() = uri;

    // remove headers
    let headers = req.headers_mut();
    for name in REMOVE_HEADER_NAMES {
        headers.remove(name);
    }
    if !essential.keep_host_header().map_or_else(|| false, |x| x) {
        headers.remove(HOST);
    }

    let client = req
        .extensions_mut()
        .remove::<HttpClient>()
        .ok_or_else(|| satex_error!("Cannot get `HttpClient` extension!"))?;

    debug!("Proxy service request: {:?}", req);
    let res = client.request(req).await.map_err(|e| satex_error!(e))?;
    debug!("Proxy service request: {:?}", res);
    Ok(res.map(Body::new))
}
