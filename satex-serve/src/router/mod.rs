use std::future::{ready, Ready};

use bytes::Bytes;
use futures::future::{BoxFuture, Either};
use hyper::{Request, Response, StatusCode};
use tower::{Service, ServiceExt};
use tracing::{debug, warn};

use route::Route;
use satex_core::config::ServeConfig;
use satex_core::essential::Essential;
use satex_core::http::{make_response, Body};
use satex_core::{BoxError, Error};
use satex_layer::{MakeDefaultRouteServiceLayerRegistry, MakeRouteServiceLayerRegistry};
use satex_matcher::MakeRouteMatcherRegistry;
use satex_service::MakeRouteServiceRegistry;

pub mod route;

#[derive(Default, Clone)]
pub struct Router {
    ///
    /// 路由表
    ///
    routes: Vec<Route>,
}

impl Router {
    ///
    ///
    /// 根据输入的路透表创建一个新的路由
    ///
    /// # Arguments
    ///
    /// * `routes`: 路由表
    ///
    /// returns: Router
    ///
    pub fn new(routes: Vec<Route>) -> Self {
        Self { routes }
    }

    ///
    /// 添加新的路由到路由表中
    ///
    /// # Arguments
    ///
    /// * `route`: 添加的路由
    ///
    /// returns: &mut Router
    ///
    pub fn push(&mut self, route: Route) -> &mut Self {
        self.routes.push(route);
        self
    }
}

impl<'a> TryFrom<&'a ServeConfig> for Router {
    type Error = Error;

    fn try_from(config: &'a ServeConfig) -> Result<Self, Self::Error> {
        let mut router = Router::default();

        // 全局Layer集合
        let global_layers =
            MakeRouteServiceLayerRegistry::make_many(config.router().global().layers())?;

        // 默认Layer集合
        let default_layers = MakeDefaultRouteServiceLayerRegistry::make_all(config)?;

        // 全局Matcher集合
        let global_matchers =
            MakeRouteMatcherRegistry::make_many(config.router().global().matchers())?;

        // 创建所有的路由
        for route in config.router().routes() {
            let id = route.id();
            let route_service = MakeRouteServiceRegistry::make_single(route.service())?;
            let route_layers = MakeRouteServiceLayerRegistry::make_many(route.layers())?;
            let route_matchers = MakeRouteMatcherRegistry::make_many(route.matchers())?;
            match Route::builder(id, route_service)
                .layers(&global_layers)
                .layers(&default_layers)
                .layers(&route_layers)
                .matchers(global_matchers.clone())
                .matchers(route_matchers)
                .build()
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

impl<ReqBody> hyper::service::Service<Request<ReqBody>> for Router
where
    ReqBody: hyper::body::Body<Data = Bytes> + Send + 'static,
    ReqBody::Error: Into<BoxError>,
{
    type Response = Response<Body>;
    type Error = Error;
    type Future = RouterFuture<Self::Response, Self::Error>;

    fn call(&self, mut request: Request<ReqBody>) -> Self::Future {
        let mut iter = self.routes.iter();
        let essential = request.extensions_mut().get_mut::<Essential>().unwrap();
        let essential_display = essential.display();
        let route = loop {
            if let Some(route) = iter.next() {
                match route.is_match(essential) {
                    Ok(false) => continue,
                    Ok(true) => break Ok(Some(route)),
                    Err(e) => break Err(e),
                }
            } else {
                break Ok(None);
            }
        };
        match route {
            Ok(Some(route)) => {
                // 更新路由ID
                essential.route_id = Some(route.id().to_string());

                // 执行路由
                debug!("Matched {:?} for: {:?}", route, essential_display);
                let mut route = route.clone();
                Either::Right(Box::pin(async move {
                    match <Route as ServiceExt<Request<Body>>>::ready(&mut route).await {
                        Ok(route) => match route.call(request).await {
                            Ok(response) => Ok(response),
                            Err(e) => {
                                warn!(
                                    "{:?} call error for: {:?} => {}",
                                    route, essential_display, e
                                );
                                Ok(make_response(
                                    format!("{e}"),
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                ))
                            }
                        },
                        Err(e) => {
                            warn!(
                                "{:?} poll ready error for: {:?} => {}",
                                route, essential_display, e
                            );
                            Ok(make_response(
                                format!("{e}"),
                                StatusCode::INTERNAL_SERVER_ERROR,
                            ))
                        }
                    }
                }))
            }
            Ok(None) => {
                debug!("Not found matched route for: {:?}", essential_display);
                Either::Left(ready(Ok(make_response(
                    StatusCode::NOT_FOUND.to_string(),
                    StatusCode::NOT_FOUND,
                ))))
            }
            Err(e) => {
                warn!(
                    "Find matched route error for: {} => {}",
                    essential_display, e
                );
                Either::Left(ready(Ok(make_response(
                    format!("{e}"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))))
            }
        }
    }
}

pub type RouterFuture<T, E> = Either<Ready<Result<T, E>>, BoxFuture<'static, Result<T, E>>>;
