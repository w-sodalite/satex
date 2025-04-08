#![allow(unused)]

use crate::config::Config;
use crate::registry::Registry;
use http::Extensions;
use satex_core::Error;
use satex_core::util::With;
use satex_layer::make::MakeRouteLayer;
use satex_load_balancer::resolver::{
    ArcLoadBalancerResolver, ArcMakeLoadBalancerResolver, CompositeLoadBalancerResolver,
    MakeLoadBalancerResolver,
};
use satex_matcher::make::MakeRouteMatcher;
use satex_server::router::{Route, Router};
use satex_service::make::MakeRouteService;
use tower::{Layer, Service};

#[derive(Clone, Default)]
pub struct MakeRouter {
    registry: Registry,
}

impl MakeRouter {
    pub fn new(registry: Registry) -> Self {
        Self { registry }
    }

    pub fn make(&self, config: &Config) -> Result<Router, Error> {
        let resolver = self.make_resolver(config)?;
        let mut extensions = Extensions::default();
        extensions.insert(resolver);

        config
            .router
            .routes
            .iter()
            .try_fold(vec![], |routes, route| {
                self.make_route(route, &config.router.global, &extensions)
                    .map(|route| routes.with(|routes| routes.push(route)))
            })
            .map(Router::new)
    }

    fn make_resolver(&self, config: &Config) -> Result<ArcLoadBalancerResolver, Error> {
        config
            .discoveries
            .iter()
            .try_fold(
                CompositeLoadBalancerResolver::default(),
                |mut composite, component| match self.registry.get_resolver(component.kind()) {
                    Some(make) => make
                        .make(component.args())
                        .map(|resolver| composite.push(resolver)),
                    None => Err(Error::new(format!(
                        "Miss load balancer resolver: {}",
                        component.kind()
                    ))),
                },
            )
            .map(ArcLoadBalancerResolver::new)
    }

    fn make_route(
        &self,
        route: &crate::config::router::Route,
        global: &crate::config::router::Global,
        extensions: &Extensions,
    ) -> Result<Route, Error> {
        let matchers = global
            .matchers
            .iter()
            .chain(route.matchers.iter())
            .try_fold(vec![], |matchers, component| {
                match self.registry.get_matcher(component.kind()) {
                    Some(make) => make
                        .make(component.args())
                        .map(|matcher| matchers.with(|matchers| matchers.push(matcher))),
                    None => Err(Error::new(format!(
                        "Miss route matcher: {}",
                        component.kind()
                    ))),
                }
            })?;

        let layers = global.layers.iter().chain(route.layers.iter()).try_fold(
            vec![],
            |layers, component| match self.registry.get_layer(component.kind()) {
                Some(make) => make
                    .make(component.args())
                    .map(|matcher| layers.with(|layers| layers.push(matcher))),
                None => Err(Error::new(format!(
                    "Miss route layer: {}",
                    component.kind()
                ))),
            },
        )?;

        let service = match &route.service {
            Some(component) => {
                let make = self.registry.get_service(component.kind()).ok_or_else(|| {
                    Error::new(format!("Miss route service: {}", component.kind()))
                })?;
                let service = make.make(component.args(), extensions)?;
                Some(service)
            }
            None => None,
        };

        let mut builder = Route::builder().id(&route.id);
        builder = matchers
            .into_iter()
            .fold(builder, |builder, matcher| builder.matcher(matcher));
        builder = layers
            .into_iter()
            .rfold(builder, |builder, layer| builder.layer(layer));
        if let Some(service) = service {
            builder = builder.service(service);
        }

        Ok(builder.build())
    }
}
