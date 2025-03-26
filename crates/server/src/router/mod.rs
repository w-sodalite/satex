mod route;

use futures::{Stream, StreamExt};
pub use route::Builder as RouteBuilder;
pub use route::Route;

use bytes::Bytes;
use futures::future::{BoxFuture, LocalBoxFuture};
use http::request::Parts;
use http::{Request, Response, StatusCode};
use hyper::service::Service as HyperService;
use satex_core::body::Body;
use satex_core::extension::{RawUri, RouteId};
use satex_core::util::ResponseExt;
use satex_core::{BoxError, Error};
use satex_matcher::RouteMatcher;
use std::future::{poll_fn, ready, Ready};
use std::pin::pin;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::Service;
use tracing::{debug, info};

#[derive(Clone, Default)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new(routes: Vec<Route>) -> Self {
        Self { routes }
    }

    pub fn into_static_service(self) -> MakeRouterService {
        MakeRouterService::new(InternalRouter::Static(Arc::from(self.routes)))
    }

    pub fn into_dynamic_service<S, F>(self, events: S, f: F) -> MakeRouterService
    where
        S: Stream<Item=Event> + Send + 'static,
        F: FnOnce(BoxFuture<'static, ()>),
    {
        let routes = Arc::new(RwLock::new(self.routes));
        let make_service = MakeRouterService::new(InternalRouter::Dynamic(routes.clone()));

        // 更新路由任务
        let future = async move {
            let mut events = pin!(events);
            while let Some(event) = events.next().await {
                match event {
                    Event::Set(router) => {
                        info!("refresh routes: {}", router.routes.len());
                        *routes.write().await = router.routes;
                    }
                    Event::Clear => {
                        routes.write().await.clear();
                    }
                }
            }
        };
        f(Box::pin(future));

        make_service
    }
}

#[derive(Clone)]
pub enum Event {
    Set(Router),
    Clear,
}

#[doc(hidden)]
#[derive(Clone)]
pub enum InternalRouter {
    Static(Arc<[Route]>),
    Dynamic(Arc<RwLock<Vec<Route>>>),
}

impl<ReqBody> HyperService<Request<ReqBody>> for InternalRouter
where
    ReqBody: http_body::Body<Data=Bytes> + Send + 'static,
    ReqBody::Error: Into<BoxError>,
{
    type Response = Response<Body>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, request: Request<ReqBody>) -> Self::Future {
        let router = self.clone();
        Box::pin(async move {
            let (mut parts, body) = request.into_parts();

            // 如果是动态路由, 找到匹配的路由后就会释放读锁
            let route = match &router {
                InternalRouter::Static(routes) => find_route(routes, &mut parts).await,
                InternalRouter::Dynamic(routes) => {
                    find_route(routes.read().await.as_slice(), &mut parts).await
                }
            };

            match route {
                Ok(Some(mut route)) => {
                    match poll_fn(|ctx| {
                        <Route as Service<Request<ReqBody>>>::poll_ready(&mut route, ctx)
                    })
                        .await
                    {
                        Ok(_) => {
                            // raw uri
                            parts.extensions.insert(RawUri::new(parts.uri.clone()));
                            // route id
                            parts.extensions.insert(RouteId::new(route.id()));
                            // poll ready
                            poll_fn(|ctx| {
                                <Route as Service<Request<ReqBody>>>::poll_ready(&mut route, ctx)
                            })
                                .await
                                .map_err(Error::new)?;

                            // call route
                            route.call(Request::from_parts(parts, body)).await
                        }
                        Err(e) => Err(Error::new(e)),
                    }
                }
                Ok(None) => Ok(Response::new(Body::empty()).with_status(StatusCode::NOT_FOUND)),
                Err(e) => Err(e),
            }
        })
    }
}

#[inline(always)]
async fn find_route(routes: &[Route], parts: &mut Parts) -> Result<Option<Route>, Error> {
    for route in routes {
        match route.matches(parts).await {
            Err(e) => {
                return Err(e);
            }
            Ok(false) => {
                debug!("Not matched route: {}", route.id());
            }
            Ok(true) => {
                debug!("Matched route: {}", route.id());
                return Ok(Some(route.clone()));
            }
        }
    }
    debug!("Not find any matched route!");
    Ok(None)
}

#[derive(Clone)]
pub struct MakeRouterService(InternalRouter);

impl MakeRouterService {
    fn new(router: InternalRouter) -> Self {
        Self(router)
    }
}

impl HyperService<()> for MakeRouterService {
    type Response = InternalRouter;
    type Error = ();
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, _: ()) -> Self::Future {
        ready(Ok(self.0.clone()))
    }
}
