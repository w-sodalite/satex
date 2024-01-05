use std::task::{Context, Poll};

use hyper::Request;
use tower::Service;

pub use make::MakeKeepHostHeaderLayer;
use satex_core::essential::Essential;

mod layer;
mod make;

#[derive(Clone)]
pub struct KeepHostHeader<S> {
    inner: S,
}

impl<S> KeepHostHeader<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, ReqBody> Service<Request<ReqBody>> for KeepHostHeader<S>
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
        if let Some(essential) = req.extensions_mut().get_mut::<Essential>() {
            essential.set_keep_host_header(Some(true));
        }
        self.inner.call(req)
    }
}
