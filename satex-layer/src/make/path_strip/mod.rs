use std::task::{Context, Poll};

use hyper::{Request, Uri};
use tower::Service;
use tracing::debug;

pub use make::MakePathStripLayer;

mod layer;
mod make;

#[derive(Clone)]
pub struct PathStrip<S> {
    inner: S,
    level: usize,
}

impl<S> PathStrip<S> {
    pub fn new(inner: S, level: usize) -> Self {
        Self { inner, level }
    }
}

impl<S, ReqBody> Service<Request<ReqBody>> for PathStrip<S>
where
    S: Service<Request<ReqBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let level = self.level;
        if level > 0 {
            let source = req.uri();
            let mut path = source
                .path()
                .split('/')
                .filter(|segment| !segment.is_empty())
                .skip(level)
                .collect::<Vec<_>>()
                .join("/");
            path.insert(0, '/');
            let path_and_query = req
                .uri()
                .query()
                .map(|query| format!("{}?{}", path, query))
                .unwrap_or(path);
            let mut builder = Uri::builder().path_and_query(path_and_query);
            if let Some(schema) = source.scheme_str() {
                builder = builder.scheme(schema);
            }
            if let Some(authority) = source.authority() {
                builder = builder.authority(authority.as_str());
            }
            let uri = builder.build().expect("build uri error!");
            debug!("Strip path: {} => {}", source, uri);
            *req.uri_mut() = uri;
        }
        self.inner.call(req)
    }
}
