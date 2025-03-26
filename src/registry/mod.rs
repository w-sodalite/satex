pub mod layer;
pub mod matcher;
pub mod service;

use crate::registry::layer::MakeRouteLayerRegistry;
use crate::registry::matcher::MakeRouteMatcherRegistry;
use crate::registry::service::MakeRouteServiceRegistry;
use bytes::Bytes;
use http::{Request, Response};
use satex_core::body::Body;
use satex_core::BoxError;
use satex_layer::make::{ArcMakeRouteLayer, MakeRouteLayer};
use satex_matcher::make::{ArcMakeRouteMatcher, MakeRouteMatcher};
use satex_matcher::RouteMatcher;
use satex_service::make::{ArcMakeRouteService, MakeRouteService};
use satex_service::RouteService;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct Registry {
    matchers: MakeRouteMatcherRegistry,
    layers: MakeRouteLayerRegistry,
    services: MakeRouteServiceRegistry,
}

impl Registry {
    #[inline]
    pub fn without_default_matchers(&mut self) -> &mut Self {
        self.matchers = MakeRouteMatcherRegistry::without_default();
        self
    }

    #[inline]
    pub fn without_default_layers(&mut self) -> &mut Self {
        self.layers = MakeRouteLayerRegistry::without_default();
        self
    }

    #[inline]
    pub fn without_default_services(&mut self) -> &mut Self {
        self.services = MakeRouteServiceRegistry::without_default();
        self
    }

    #[inline]
    pub fn without_additions(&mut self) -> &mut Self {
        self.without_default_matchers()
            .without_default_layers()
            .without_default_services()
    }

    #[inline]
    pub fn with_matcher<M>(&mut self, make: M) -> &mut Self
    where
        M: MakeRouteMatcher + Send + Sync + 'static,
        M::Matcher: RouteMatcher + Send + Sync + 'static,
    {
        self.matchers.push(make);
        self
    }

    #[inline]
    pub fn with_layer<M, L, S, E, ResBody>(&mut self, make: M) -> &mut Self
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
        self.layers.push(make);
        self
    }

    #[inline]
    pub fn with_service<M, S, E, ResBody>(&mut self, make: M) -> &mut Self
    where
        M: MakeRouteService<Service=S> + Send + Sync + 'static,
        S: Service<Request<Body>, Response=Response<ResBody>, Error=E>
        + Clone
        + Send
        + Sync
        + 'static,
        E: Into<BoxError>,
        ResBody: http_body::Body<Data=Bytes> + Send + 'static,
        ResBody::Error: Into<BoxError>,
    {
        self.services.push(make);
        self
    }

    pub fn get_matcher(&self, kind: &str) -> Option<ArcMakeRouteMatcher> {
        self.matchers.get(kind)
    }

    pub fn get_layer(&self, kind: &str) -> Option<ArcMakeRouteLayer> {
        self.layers.get(kind)
    }

    pub fn get_service(&self, kind: &str) -> Option<ArcMakeRouteService> {
        self.services.get(kind)
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            matchers: MakeRouteMatcherRegistry::with_default(),
            layers: MakeRouteLayerRegistry::with_default(),
            services: MakeRouteServiceRegistry::with_default(),
        }
    }
}

macro_rules! push {
    ($registry:expr, $($matcher:expr),+ $(,)?) => {
        $(
            $registry.push($matcher);
        )+
    };
}

pub(crate) use push;
