use crate::make::MakeRouteService;
use http::{Request, Response};
use satex_core::component::Args;
use satex_core::Error;
use satex_macro::make;
use std::future::{ready, Ready};
use std::task::{Context, Poll};
use tower::Service;

#[make(kind = Echo)]
pub struct MakeEchoRouteService;

impl MakeRouteService for MakeEchoRouteService {
    type Service = EchoRouteService;

    fn make(&self, _: Args) -> Result<Self::Service, Error> {
        Ok(EchoRouteService)
    }
}

#[derive(Debug, Clone)]
pub struct EchoRouteService;

impl<Body> Service<Request<Body>> for EchoRouteService {
    type Response = Response<Body>;
    type Error = Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let (_, body) = request.into_parts();
        ready(Ok(Response::new(body)))
    }
}
