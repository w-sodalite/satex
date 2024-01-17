use std::str::FromStr;
use std::task::{Context, Poll};

use futures::future::BoxFuture;
use hyper::header::HOST;
use hyper::{Request, Response, Uri};
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use tower::Service;

pub use keep_host_header::KeepHostHeaderState;
pub use make::MakeProxyService;
use satex_core::endpoint::Endpoint;
use satex_core::essential::Essential;
use satex_core::http::Body;
use satex_core::{satex_error, Error};
use satex_discovery::{NamedServerDiscovery, ServerDiscovery};

mod keep_host_header;
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

async fn proxy(uri: String, mut request: Request<Body>) -> Result<Response<Body>, Error> {
    let prefix = uri.strip_suffix('/').unwrap_or(&uri);
    let path = request
        .uri()
        .path_and_query()
        .map(|x| x.as_str())
        .unwrap_or(request.uri().path())
        .to_string();

    // reconstruct uri
    let mut uri = format!("{}{}", prefix, path)
        .parse::<Uri>()
        .map_err(|e| satex_error!(e))?;
    let host = uri
        .host()
        .ok_or_else(|| satex_error!("Proxy service uri miss `host`!"))?;

    // get server instance from discovery
    let discovery = request
        .extensions_mut()
        .remove::<NamedServerDiscovery>()
        .unwrap();

    // replace proxy uri host and port
    let essential = request.extensions_mut().remove::<Essential>().unwrap();

    if let Some(endpoint) = discovery.resolve(&essential, host).await? {
        let host = match endpoint {
            Endpoint::Ip(addr) => format!("{}:{}", addr.ip(), addr.port()),
            Endpoint::Domain(raw) => raw.to_string(),
        };
        uri = Uri::from_str(&format!(
            "{}://{}{}",
            uri.scheme_str().unwrap_or(DEFAULT_SCHEMA),
            host,
            path
        ))
        .map_err(|e| satex_error!(e))?;
    }
    *request.uri_mut() = uri;

    // 处理请求头
    let keep_host_header = request
        .extensions_mut()
        .remove::<KeepHostHeaderState>()
        .is_some();
    let headers = request.headers_mut();
    if !keep_host_header {
        headers.remove(HOST);
    }
    for name in REMOVE_HEADER_NAMES {
        headers.remove(name);
    }
    let client = request.extensions_mut().remove::<HttpClient>().unwrap();
    let response = client.request(request).await.map_err(|e| satex_error!(e))?;
    Ok(response.map(Body::new))
}
