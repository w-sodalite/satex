use satex_core::config::ServeConfig;
use satex_core::Error;
use satex_layer::{MakeDefaultRouteServiceLayerRegistry, MakeRouteServiceLayerRegistry};
use satex_matcher::MakeRouteMatcherRegistry;
use satex_service::MakeRouteServiceRegistry;

use crate::router::route::make::MakeRoute;
use crate::router::Router;

pub enum MakeRouter {}

impl MakeRouter {
    pub fn make(config: &ServeConfig) -> Result<Router, Error> {
        // 路由
        let mut router = Router::default();

        // 默认Layer集合
        let default_layers = MakeDefaultRouteServiceLayerRegistry::make_all(config)?;

        // 全局Layer集合
        let global_layers =
            MakeRouteServiceLayerRegistry::make_many(config.router().global().layers())?;

        // 全局Matcher集合
        let global_matchers =
            MakeRouteMatcherRegistry::make_many(config.router().global().matchers())?;

        // 创建所有的路由
        for route in config.router().routes() {
            let id = route.id();
            let route_service = MakeRouteServiceRegistry::make_single(route.service())?;
            let route_layers = MakeRouteServiceLayerRegistry::make_many(route.layers())?;
            let route_matchers = MakeRouteMatcherRegistry::make_many(route.matchers())?;
            match MakeRoute::new(id, route_service)
                .add_layers(global_layers.clone())
                .add_layers(default_layers.clone())
                .add_layers(route_layers)
                .add_matchers(global_matchers.clone())
                .add_matchers(route_matchers)
                .make()
            {
                Ok(route) => {
                    router.push(route);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(router)
    }
}
