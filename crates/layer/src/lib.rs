use bytes::Bytes;
use http::{Request, Response};
use satex_core::body::Body;
use satex_core::util::try_downcast;
use satex_core::BoxError;
use satex_service::RouteService;
use std::sync::Arc;
use tower::layer::layer_fn;
use tower::{Layer, Service};

pub mod concurrency_limit;
pub mod cors;
pub mod make;
pub mod remove_header;
pub mod set_header;
pub mod set_method;
pub mod set_prefix;
pub mod set_status_code;
pub mod strip_prefix;
pub mod timeout;
pub mod trace;
mod util;

#[derive(Clone)]
pub struct ArcRouteLayer(Arc<dyn Layer<RouteService, Service = RouteService> + Send + Sync>);

impl ArcRouteLayer {
    pub fn new<S, L, E, ResBody>(layer: L) -> Self
    where
        S: Service<Request<Body>, Response=Response<ResBody>, Error=E>
            + Clone
            + Send
            + Sync
            + 'static,
        E: Into<BoxError>,
        L: Layer<RouteService, Service = S> + Send + Sync + 'static,
        ResBody: http_body::Body<Data = Bytes> + Send + 'static,
        ResBody::Error: Into<BoxError>,
    {
        try_downcast::<ArcRouteLayer, _>(layer).unwrap_or_else(|layer| {
            Self(Arc::new(layer_fn(move |service| {
                RouteService::new(layer.layer(service))
            })))
        })
    }
}

impl Layer<RouteService> for ArcRouteLayer {
    type Service = RouteService;

    fn layer(&self, service: RouteService) -> Self::Service {
        self.0.layer(service)
    }
}
