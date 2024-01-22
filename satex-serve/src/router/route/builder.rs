use tower::Layer;

use satex_core::Error;
use satex_layer::NamedRouteServiceLayer;
use satex_matcher::NamedRouteMatcher;
use satex_service::NamedRouteService;

use crate::router::route::Route;

pub struct Builder<'a> {
    id: String,
    matchers: Vec<NamedRouteMatcher>,
    layers: Vec<&'a NamedRouteServiceLayer>,
    service: NamedRouteService,
}

impl<'a> Builder<'a> {
    pub fn new(id: impl Into<String>, service: NamedRouteService) -> Self {
        Self {
            id: id.into(),
            matchers: vec![],
            layers: vec![],
            service,
        }
    }

    pub fn matchers(mut self, matchers: Vec<NamedRouteMatcher>) -> Self {
        self.matchers.extend(matchers);
        self
    }

    pub fn layers(mut self, layers: &'a [NamedRouteServiceLayer]) -> Self {
        self.layers.extend(layers);
        self
    }

    pub fn build(self) -> Result<Route, Error> {
        let Builder {
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
