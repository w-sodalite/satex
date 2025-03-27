use crate::make::MakeRouteService;
use http::{Request, Response};
use satex_core::body::Body;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::make;
use std::future::{ready, Ready};
use std::task::{Context, Poll};
use tower::Service;

#[make(kind = Echo)]
pub struct MakeEchoRouteService {
    #[serde(default)]
    text: String,
}

impl MakeRouteService for MakeEchoRouteService {
    type Service = EchoRouteService;

    fn make(&self, args: Args) -> Result<Self::Service, Error> {
        Config::with_args(args).map(|config| EchoRouteService::new(config.text))
    }
}

#[derive(Debug, Clone, Default)]
pub struct EchoRouteService {
    text: String,
}

impl EchoRouteService {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl<ReqBody> Service<Request<ReqBody>> for EchoRouteService {
    type Response = Response<Body>;
    type Error = Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: Request<ReqBody>) -> Self::Future {
        ready(Ok(Response::new(Body::from(self.text.clone()))))
    }
}
