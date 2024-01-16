use std::sync::Arc;

use bytes::Bytes;
use hyper::{Request, Response};
use tower::{Layer, Service};

use satex_core::config::ServeConfig;
use satex_core::http::Body;
use satex_core::{make, BoxError};
use satex_service::NamedRouteService;

use crate::NamedRouteServiceLayer;

make! {
    @compcat,
    MakeDefaultRouteServiceLayer,
    Layer,
    (Layer<NamedRouteService>),
    (&ServeConfig),
    NamedRouteServiceLayer
}

impl ArcMakeDefaultRouteServiceLayer {
    pub fn new<M, L, ResBody, E>(make: M) -> Self
    where
        M: MakeDefaultRouteServiceLayer<Layer = L> + Send + Sync + 'static,
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
        let name = make.name();
        Self(Arc::new(MakeFn::new(make, |inner| {
            NamedRouteServiceLayer::new(name, inner)
        })))
    }
}
