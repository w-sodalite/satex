use std::{str::FromStr, sync::Arc};

use crate::make::MakeRouteLayer;
use crate::remove_header::Removable;
use http::HeaderName;
use satex_core::component::Args;
use satex_core::{component::Configurable, Error};
use satex_macro::make;
use tower::{Layer, Service};

#[make(kind = RemoveRequestHeader)]
pub struct MakeRemoveRequestHeaderRouteLayer {
    name: String,
}

impl MakeRouteLayer for MakeRemoveRequestHeaderRouteLayer {
    type Layer = RemoveRequestHeaderLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        Config::with_args(args).and_then(|config| {
            HeaderName::from_str(&config.name)
                .map_err(Error::new)
                .map(RemoveRequestHeaderLayer::new)
        })
    }
}

#[derive(Debug, Clone)]
pub struct RemoveRequestHeaderLayer {
    name: Arc<HeaderName>,
}

impl RemoveRequestHeaderLayer {
    pub fn new(name: HeaderName) -> Self {
        Self {
            name: Arc::new(name),
        }
    }
}

impl<S> Layer<S> for RemoveRequestHeaderLayer {
    type Service = RemoveRequestHeader<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RemoveRequestHeader {
            name: self.name.clone(),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RemoveRequestHeader<S> {
    inner: S,
    name: Arc<HeaderName>,
}

impl<S, Req> Service<Req> for RemoveRequestHeader<S>
where
    S: Service<Req>,
    Req: Removable,
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

    fn call(&mut self, mut req: Req) -> Self::Future {
        req.remove(self.name.as_ref());
        self.inner.call(req)
    }
}
