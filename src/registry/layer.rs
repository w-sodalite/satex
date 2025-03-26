use crate::registry::push;
use bytes::Bytes;
use http::{Request, Response};
use satex_core::body::Body;
use satex_core::BoxError;
use satex_layer::concurrency_limit::MakeConcurrencyLimitRouteLayer;
use satex_layer::cors::MakeCorsRouteLayer;
use satex_layer::make::{ArcMakeRouteLayer, MakeRouteLayer};
use satex_layer::remove_header::MakeRemoveHeaderRouteLayer;
use satex_layer::set_header::{MakeSetRequestHeaderRouteLayer, MakeSetResponseHeaderRouteLayer};
use satex_layer::set_method::MakeSetMethodRouteLayer;
use satex_layer::set_prefix::MakeSetPrefixRouteLayer;
use satex_layer::set_status_code::MakeSetResponseStatusCodeRouteLayer;
use satex_layer::strip_prefix::MakeStripPrefixRouteLayer;
use satex_layer::timeout::MakeTimeoutRouteLayer;
use satex_layer::trace::MakeTraceRouteLayer;
use satex_service::RouteService;
use std::collections::HashMap;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct MakeRouteLayerRegistry(HashMap<&'static str, ArcMakeRouteLayer>);

impl MakeRouteLayerRegistry {
    pub fn without_default() -> Self {
        Self(HashMap::new())
    }

    pub fn with_default() -> Self {
        let mut registry = Self::without_default();
        push! {
            registry,
            MakeSetResponseStatusCodeRouteLayer,
            MakeSetMethodRouteLayer,
            MakeSetRequestHeaderRouteLayer,
            MakeSetResponseHeaderRouteLayer,
            MakeRemoveHeaderRouteLayer,
            MakeStripPrefixRouteLayer,
            MakeTraceRouteLayer,
            MakeTimeoutRouteLayer,
            MakeConcurrencyLimitRouteLayer,
            MakeSetPrefixRouteLayer,
            MakeCorsRouteLayer
        }
        registry
    }

    pub fn push<M, L, S, E, ResBody>(&mut self, make: M)
    where
        M: MakeRouteLayer<Layer=L> + Send + Sync + 'static,
        L: Layer<RouteService, Service=S> + Send + Sync + 'static,
        S: Service<Request<Body>, Response=Response<ResBody>, Error=E>
        + Clone
        + Send
        + Sync
        + 'static,
        E: Into<BoxError>,
        ResBody: http_body::Body<Data=Bytes> + Send + 'static,
        ResBody::Error: Into<BoxError>,
    {
        self.0.insert(make.name(), ArcMakeRouteLayer::new(make));
    }

    pub fn get(&self, name: &str) -> Option<ArcMakeRouteLayer> {
        self.0.get(name).cloned()
    }
}
