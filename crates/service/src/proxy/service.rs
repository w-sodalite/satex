use crate::proxy::client::Client;
use futures::future::LocalBoxFuture;
use http::{HeaderName, Request, Response, Uri};
use satex_core::Error;
use satex_core::body::Body;
use satex_core::digest::Digester;
use satex_load_balancer::LoadBalancer;
use std::future::ready;
use std::net::SocketAddr;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::Service;
use tracing::debug;
use url::Url;

const REMOVE_HEADERS: [HeaderName; 9] = [
    HeaderName::from_static("connection"),
    HeaderName::from_static("keep-alive"),
    HeaderName::from_static("transfer-encoding"),
    HeaderName::from_static("te"),
    HeaderName::from_static("trailer"),
    HeaderName::from_static("proxy-authorization"),
    HeaderName::from_static("proxy-authenticate"),
    HeaderName::from_static("x-application-context"),
    HeaderName::from_static("upgrade"),
];

#[derive(Clone)]
pub struct ProxyRouteService<D> {
    url: Url,
    client: Client,
    digester: Arc<D>,
    load_balancer: Option<Arc<LoadBalancer>>,
}

impl<D> ProxyRouteService<D> {
    pub fn new(
        url: Url,
        client: Client,
        digester: D,
        load_balancer: Option<Arc<LoadBalancer>>,
    ) -> Self {
        Self {
            url,
            client,
            digester: Arc::new(digester),
            load_balancer,
        }
    }
}

impl<D> Service<Request<Body>> for ProxyRouteService<D>
where
    D: Digester<Request<Body>>,
{
    type Response = Response<Body>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.client.poll_ready(cx).map_err(Error::new)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        let addr = self.load_balancer.as_deref().and_then(|load_balancer| {
            let key = self.digester.digest(&request);
            load_balancer.select(&key).map(|backend| backend.addr)
        });

        // 重新构造请求的uri
        let uri = request.uri();
        let path = uri.path();
        let query = uri.query();
        match reconstruct(self.url.clone(), addr, path, query) {
            Ok(uri) => {
                *request.uri_mut() = uri;
            }
            Err(e) => {
                return Box::pin(ready(Err(Error::new(e))));
            }
        }

        // 删除不应该转到后端的请求头
        let headers = request.headers_mut();
        REMOVE_HEADERS.iter().for_each(|header| {
            headers.remove(header);
        });

        debug!("proxy send request:\n{:#?}", request);

        // 发送请求到后端
        let future = self.client.request(request);
        Box::pin(async move {
            future
                .await
                .map_err(Error::new)
                .map(|response| response.map(Body::new))
        })
    }
}

fn reconstruct(
    mut url: Url,
    addr: Option<SocketAddr>,
    path: &str,
    query: Option<&str>,
) -> Result<Uri, Error> {
    if let Some(addr) = addr {
        url.set_ip_host(addr.ip())
            .map_err(|_| Error::new("set ip host error!"))?;
        url.set_port(Some(addr.port()))
            .map_err(|_| Error::new("set port error!"))?;
    }
    url.set_path(path);
    url.set_query(query);
    Uri::from_maybe_shared(String::from(url)).map_err(|_| Error::new("reconstruct url error!"))
}
