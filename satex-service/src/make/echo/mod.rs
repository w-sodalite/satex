use std::future::{ready, Ready};
use std::task::{Context, Poll};

use bytes::Bytes;
use hyper::{Request, Response};
use tower::Service;

pub use make::MakeEchoService;
use satex_core::http::Body;
use satex_core::BoxError;
use satex_core::Error;

mod make;

#[derive(Clone)]
pub struct EchoService;

impl<ReqBody> Service<Request<ReqBody>> for EchoService
where
    ReqBody: hyper::body::Body<Data = Bytes> + Send + 'static,
    ReqBody::Error: Into<BoxError>,
{
    type Response = Response<Body>;
    type Error = Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let body = req.into_body();
        ready(Ok(Response::new(Body::new(body))))
    }
}
