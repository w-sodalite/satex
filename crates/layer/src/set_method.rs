use crate::make::MakeRouteLayer;
use http::{Method, Request};
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::make;
use std::str::FromStr;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[make(kind = SetMethod)]
struct MakeSetMethodRouteLayer {
    method: String,
}

impl MakeRouteLayer for MakeSetMethodRouteLayer {
    type Layer = SetMethodRouteLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        let config = Config::with_args(args)?;
        let method = Method::from_str(&config.method).map_err(Error::new)?;
        Ok(SetMethodRouteLayer { method })
    }
}

#[derive(Debug, Clone)]
pub struct SetMethodRouteLayer {
    method: Method,
}

impl<S> Layer<S> for SetMethodRouteLayer {
    type Service = SetMethod<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SetMethod {
            method: self.method.clone(),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SetMethod<S> {
    method: Method,
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for SetMethod<S>
where
    S: Service<Request<ReqBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<ReqBody>) -> Self::Future {
        *request.method_mut() = self.method.clone();
        self.inner.call(request)
    }
}
