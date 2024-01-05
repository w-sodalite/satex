use tower::Layer;

use satex_core::Error;
use satex_layer::NamedRouteServiceLayer;
use satex_matcher::NamedRouteMatcher;
use satex_service::NamedRouteService;

use crate::router::route::Route;

pub struct MakeRoute<'a> {
    id: &'a str,
    matchers: Vec<NamedRouteMatcher>,
    layers: Vec<NamedRouteServiceLayer>,
    service: NamedRouteService,
}

impl<'a> MakeRoute<'a> {
    pub fn new(id: &'a str, service: NamedRouteService) -> Self {
        Self {
            id,
            matchers: vec![],
            layers: vec![],
            service,
        }
    }

    pub fn add_matchers(mut self, matchers: Vec<NamedRouteMatcher>) -> Self {
        self.matchers.extend(matchers);
        self
    }

    pub fn add_layers(mut self, layers: Vec<NamedRouteServiceLayer>) -> Self {
        self.layers.extend(layers);
        self
    }

    pub fn make(self) -> Result<Route, Error> {
        let MakeRoute {
            matchers,
            layers,
            mut service,
            ..
        } = self;
        service = layers
            .iter()
            .rfold(service, |service, layer| layer.layer(service));
        Ok(Route::new(self.id, matchers, service))
    }
}
