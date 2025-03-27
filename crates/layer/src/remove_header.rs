use std::{str::FromStr, sync::Arc};

use crate::make::MakeRouteLayer;
use http::{HeaderName, Request};
use satex_core::component::Args;
use satex_core::{component::Configurable, Error};
use satex_macro::make;
use tower::{Layer, Service};

#[make(kind = RemoveHeader)]
pub struct MakeRemoveHeaderRouteLayer {
    name: String,
}

impl MakeRouteLayer for MakeRemoveHeaderRouteLayer {
    type Layer = RemoveHeaderRouteLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        Config::with_args(args).and_then(|config| {
            HeaderName::from_str(&config.name)
                .map_err(Error::new)
                .map(|name| RemoveHeaderRouteLayer {
                    name: Arc::new(name),
                })
        })
    }
}

#[derive(Debug, Clone)]
pub struct RemoveHeaderRouteLayer {
    name: Arc<HeaderName>,
}

impl RemoveHeaderRouteLayer {
    pub fn new(name: &str) -> Self {
        Self {
            name: Arc::new(HeaderName::from_str(name).unwrap()),
        }
    }
}

impl<S> Layer<S> for RemoveHeaderRouteLayer {
    type Service = RemoveHeader<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RemoveHeader {
            name: self.name.clone(),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RemoveHeader<S> {
    inner: S,
    name: Arc<HeaderName>,
}

impl<S, ReqBody> Service<Request<ReqBody>> for RemoveHeader<S>
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

    fn call(&mut self, mut request: Request<ReqBody>) -> Self::Future {
        let headers = request.headers_mut();
        headers.remove(self.name.as_ref());
        self.inner.call(request)
    }
}
