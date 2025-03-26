use crate::proxy::client::Client;
use futures::future::LocalBoxFuture;
use http::{HeaderName, Request, Response, Uri};
use satex_core::body::Body;
use satex_core::Error;
use std::future::ready;
use std::task::{Context, Poll};
use tower::Service;
use tracing::debug;

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

#[derive(Debug, Clone)]
pub struct ProxyRouteService {
    uri: Uri,
    client: Client,
}

impl ProxyRouteService {
    pub fn new(uri: Uri, client: Client) -> Self {
        Self { uri, client }
    }
}

impl Service<Request<Body>> for ProxyRouteService {
    type Response = Response<Body>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.client.poll_ready(cx).map_err(Error::new)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        // 替换请求uri中的host和port,只保留path和query
        let uri = request.uri();
        match uri.path_and_query() {
            Some(path_and_query) => {
                let mut parts = self.uri.clone().into_parts();
                parts.path_and_query = Some(path_and_query.clone());
                match Uri::from_parts(parts) {
                    Ok(uri) => {
                        *request.uri_mut() = uri;
                    }
                    Err(e) => {
                        return Box::pin(ready(Err(Error::new(e))));
                    }
                }
            }
            None => {
                *request.uri_mut() = self.uri.clone();
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
