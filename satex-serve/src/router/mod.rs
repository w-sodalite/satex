use std::future::{ready, Ready};
use std::task::{Context, Poll};

use futures_util::future::{BoxFuture, Either};
use hyper::{Request, Response, StatusCode};
use tower::{Service, ServiceExt};
use tracing::{info, warn};

use route::Route;
use satex_core::essential::Essential;
use satex_core::http::{make_response, Body};
use satex_core::Error;

pub mod make;
pub mod route;

#[derive(Default, Clone)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new(routes: Vec<Route>) -> Self {
        Self { routes }
    }

    pub fn append(&mut self, route: Route) -> &mut Self {
        self.routes.push(route);
        self
    }

    pub fn prepend(&mut self, route: Route) -> &mut Self {
        self.routes.insert(0, route);
        self
    }
}

pub type RouterFuture<T, E> = Either<Ready<Result<T, E>>, BoxFuture<'static, Result<T, E>>>;

impl Service<Request<Body>> for Router {
    type Response = Response<Body>;
    type Error = Error;
    type Future = RouterFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // router should not poll ready, please use route poll_ready
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        let mut iter = self.routes.iter();
        let essential = request.extensions_mut().get_mut::<Essential>().unwrap();
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
                info!("Matched route: {:?}", route);
                let route = route.clone();
                Either::Right(Box::pin(async move {
                    match <Route as ServiceExt<Request<Body>>>::ready_oneshot(route).await {
                        Ok(mut route) => match route.call(request).await {
                            Ok(response) => Ok(response),
                            Err(e) => Ok(make_response(
                                format!("{e}"),
                                StatusCode::INTERNAL_SERVER_ERROR,
                            )),
                        },
                        Err(e) => Ok(make_response(
                            format!("{e}"),
                            StatusCode::INTERNAL_SERVER_ERROR,
                        )),
                    }
                }))
            }
            Ok(None) => {
                warn!("No matched route!");
                Either::Left(ready(Ok(make_response(
                    StatusCode::NOT_FOUND.to_string(),
                    StatusCode::NOT_FOUND,
                ))))
            }
            Err(e) => {
                warn!("Find matched route appear error: {}", e);
                Either::Left(ready(Ok(make_response(
                    format!("{e}"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))))
            }
        }
    }
}
