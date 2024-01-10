use std::fmt::{Debug, Formatter};
use std::task::{Context, Poll};

use bytes::Bytes;
use futures_util::future::BoxFuture;
use hyper::{Request, Response};
use tower::util::{BoxCloneService, MapErrLayer, MapResponseLayer};
use tower::{Service, ServiceBuilder};

pub use make::proxy::KeepHostHeaderState;
use satex_core::http::Body;
use satex_core::{export_make, BoxError};
use satex_core::{satex_error, Error};

export_make!(MakeRouteService);
mod make;
mod registry;

#[derive(Clone)]
pub struct NamedRouteService {
    name: &'static str,
    inner: BoxCloneService<Request<Body>, Response<Body>, Error>,
}

impl Debug for NamedRouteService {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RouteService")
            .field("name", &self.name)
            .finish()
    }
}

impl NamedRouteService {
    pub fn new<S, ResBody, E>(name: &'static str, service: S) -> Self
    where
        S: Service<Request<Body>, Response = Response<ResBody>, Error = E> + Clone + Send + 'static,
        S::Future: Send + 'static,
        ResBody: hyper::body::Body<Data = Bytes> + Send + 'static,
        ResBody::Error: Into<BoxError> + Send + 'static,
        E: Into<BoxError> + Send + 'static,
    {
        Self {
            name,
            inner: BoxCloneService::new(
                ServiceBuilder::default()
                    .layer(MapResponseLayer::new(|res: Response<_>| res.map(Body::new)))
                    .layer(MapErrLayer::new(|e: E| satex_error!(e.into())))
                    .service(service),
            ),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl<ReqBody> Service<Request<ReqBody>> for NamedRouteService
where
    ReqBody: hyper::body::Body<Data = Bytes> + Send + 'static,
    ReqBody::Error: Into<BoxError>,
{
    type Response = Response<Body>;
    type Error = Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        self.inner.call(req.map(Body::new))
    }
}
