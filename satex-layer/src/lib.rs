use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use bytes::Bytes;
use hyper::{Request, Response};
use tower::layer::layer_fn;
use tower::{Layer, Service};

pub use make::default::*;
use satex_core::http::Body;
use satex_core::{export_make, BoxError};
use satex_service::NamedRouteService;

mod make;
mod registry;
export_make!(MakeRouteServiceLayer);

#[derive(Clone)]
pub struct NamedRouteServiceLayer {
    name: &'static str,
    inner: Arc<dyn Layer<NamedRouteService, Service = NamedRouteService> + Send + Sync + 'static>,
}

impl Debug for NamedRouteServiceLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RouteServiceLayer")
            .field("name", &self.name)
            .finish()
    }
}

impl NamedRouteServiceLayer {
    pub fn new<L, ResBody, E>(name: &'static str, layer: L) -> Self
    where
        L: Layer<NamedRouteService> + Send + Sync + 'static,
        L::Service: Service<Request<Body>, Response = Response<ResBody>, Error = E>
            + Clone
            + Send
            + 'static,
        <<L as Layer<NamedRouteService>>::Service as Service<Request<Body>>>::Future:
            Send + 'static,
        ResBody: hyper::body::Body<Data = Bytes> + Send + 'static,
        ResBody::Error: Into<BoxError> + Send + 'static,
        E: Into<BoxError> + Send + 'static,
    {
        Self {
            name,
            inner: Arc::new(layer_fn(move |inner| {
                NamedRouteService::new(name, layer.layer(inner))
            })),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl Layer<NamedRouteService> for NamedRouteServiceLayer {
    type Service = NamedRouteService;

    fn layer(&self, inner: NamedRouteService) -> Self::Service {
        self.inner.layer(inner)
    }
}
