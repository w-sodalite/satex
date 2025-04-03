use crate::proxy::client::Client;
use futures::future::LocalBoxFuture;
use http::{HeaderName, Request, Response, Uri};
use satex_core::body::Body;
use satex_core::digest::Digester;
use satex_core::Error;
use satex_load_balancing::LoadBalancer;
use std::future::ready;
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
        let host = match &self.load_balancer {
            Some(load_balancer) => {
                let key = self.digester.digest(&request);
                match load_balancer.select(&key) {
                    Some(backend) => backend.addr.to_string(),
                    None => {
                        return Box::pin(ready(Err(Error::new("no backend found!"))));
                    }
                }
            }
            None => match self.url.host_str() {
                Some(host) => host.to_string(),
                None => {
                    return Box::pin(ready(Err(Error::new(format!(
                        "not found host for url: {}",
                        self.url
                    )))));
                }
            },
        };

        // 重新构造请求的uri
        let uri = request.uri();
        let path = uri.path();
        let query = uri.query();
        match reconstruct(self.url.clone(), &host, path, query) {
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

        debug!("Proxy send request:\n{:#?}", request);

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

fn reconstruct(mut url: Url, host: &str, path: &str, query: Option<&str>) -> Result<Uri, Error> {
    url.set_host(Some(host))
        .map_err(|_| Error::new("set host error!"))?;
    url.set_path(path);
    url.set_query(query);
    Uri::from_maybe_shared(String::from(url)).map_err(|_| Error::new("reconstruct url error!"))
}
