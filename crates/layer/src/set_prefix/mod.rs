mod layer;
mod make;
pub use layer::*;
pub use make::*;

use futures::future::LocalBoxFuture;
use futures::{FutureExt, TryFutureExt};
use http::{Request, Uri};
use satex_core::{BoxError, Error};
use std::future::ready;
use std::str::FromStr;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::Service;

#[derive(Debug, Clone)]
pub struct SetPrefix<S> {
    inner: S,
    prefix: Arc<str>,
}

impl<S> SetPrefix<S> {
    pub fn new(inner: S, prefix: Arc<str>) -> Self {
        Self { prefix, inner }
    }
}

impl<S, ReqBody> Service<Request<ReqBody>> for SetPrefix<S>
where
    S: Service<Request<ReqBody>>,
    S::Error: Into<BoxError>,
    S::Future: 'static,
    S::Response: Send + 'static,
{
    type Response = S::Response;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|e| Error::new(e.into()))
    }

    fn call(&mut self, mut request: Request<ReqBody>) -> Self::Future {
        let uri = request.uri();
        let path_and_query = match uri.path_and_query() {
            Some(path_and_query) => {
                let path_and_query = path_and_query.as_str();
                if path_and_query == "/" {
                    self.prefix.to_string()
                } else {
                    format!("{}{}", self.prefix.as_ref(), path_and_query)
                }
            }
            None => self.prefix.to_string(),
        };

        match Uri::from_str(&path_and_query) {
            Ok(uri) => {
                *request.uri_mut() = uri;
                self.inner
                    .call(request)
                    .map_err(|e| Error::new(e.into()))
                    .boxed_local()
            }
            Err(e) => ready(Err(Error::new(e))).boxed(),
        }
    }
}
