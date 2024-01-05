mod layer;
mod make;

use hyper::{Request, Uri};
use tower::Service;

pub use make::MakeSetPathLayer;

#[derive(Clone)]
pub struct SetPath<S> {
    path: String,
    inner: S,
}

impl<S> SetPath<S> {
    pub fn new(path: String, inner: S) -> Self {
        Self { path, inner }
    }
}

impl<S, ReqBody> Service<Request<ReqBody>> for SetPath<S>
where
    S: Service<Request<ReqBody>>,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let uri = req.uri();
        let mut builder = Uri::builder();
        if let Some(schema) = uri.scheme_str() {
            builder = builder.scheme(schema);
        }
        if let Some(authority) = uri.authority() {
            builder = builder.authority(authority.as_str());
        }
        let path_and_query = uri
            .query()
            .map(|query| format!("{}?{}", self.path, query))
            .unwrap_or(self.path.to_string());
        let uri = builder
            .path_and_query(path_and_query)
            .build()
            .expect("build uri error!");
        *req.uri_mut() = uri;
        self.inner.call(req)
    }
}
