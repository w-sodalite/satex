use crate::registry::push;
use bytes::Bytes;
use http::{Request, Response};
use satex_core::body::Body;
use satex_core::BoxError;
use satex_service::echo::MakeEchoRouteService;
use satex_service::make::{ArcMakeRouteService, MakeRouteService};
use satex_service::proxy::MakeProxyRouteService;
use satex_service::serve_dir::MakeServeDirRouteService;
use satex_service::status_code::MakeStatusCodeRouteService;
use std::collections::HashMap;
use tower::Service;

#[derive(Clone)]
pub struct MakeRouteServiceRegistry(HashMap<&'static str, ArcMakeRouteService>);

impl MakeRouteServiceRegistry {
    pub fn without_default() -> Self {
        Self(HashMap::new())
    }

    pub fn with_default() -> Self {
        let mut registry = Self::without_default();
        push! {
            registry,
            MakeEchoRouteService,
            MakeStatusCodeRouteService,
            MakeServeDirRouteService,
            MakeProxyRouteService
        }
        registry
    }

    pub fn push<M, S, E, ResBody>(&mut self, make: M)
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
        self.0.insert(make.name(), ArcMakeRouteService::new(make));
    }

    pub fn get(&self, name: &str) -> Option<ArcMakeRouteService> {
        self.0.get(name).cloned()
    }
}
