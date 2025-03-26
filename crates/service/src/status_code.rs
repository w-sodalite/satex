use crate::make::MakeRouteService;
use http::{Response, StatusCode};
use satex_core::body::Body;
use satex_core::component::{Args, Configurable};
use satex_core::util::ResponseExt;
use satex_core::Error;
use satex_macro::{Configurable, Make};
use serde::Deserialize;
use std::convert::Infallible;
use std::future::{ready, Ready};
use std::task::{Context, Poll};
use tower::Service;

#[derive(Debug, Clone, Copy, Configurable, Deserialize)]
#[configurable(companion = "MakeStatusCodeRouteService")]
struct Config {
    status_code: u16,
}

#[derive(Debug, Clone, Copy, Make)]
#[make(name = "StatusCode")]
pub struct MakeStatusCodeRouteService;

impl MakeRouteService for MakeStatusCodeRouteService {
    type Service = StatusCodeRouteService;

    fn make(&self, args: Args) -> Result<Self::Service, Error> {
        Config::with_args(args)
            .and_then(|config| StatusCode::from_u16(config.status_code).map_err(Error::new))
            .map(StatusCodeRouteService)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StatusCodeRouteService(StatusCode);

impl<Req> Service<Req> for StatusCodeRouteService {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: Req) -> Self::Future {
        ready(Ok(Response::new(Body::empty()).with_status(self.0)))
    }
}
