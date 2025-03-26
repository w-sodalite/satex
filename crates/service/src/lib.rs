pub mod echo;
pub mod make;
pub mod proxy;
pub mod serve_dir;
pub mod status_code;

use bytes::Bytes;
use futures::future::LocalBoxFuture;
use http::{Request, Response};
use satex_core::body::Body;
use satex_core::util::{try_downcast, SyncBoxCloneService};
use satex_core::{BoxError, Error};
use std::task::{Context, Poll};
use tower::{Service, ServiceExt};

#[derive(Clone)]
pub struct RouteService(SyncBoxCloneService<Request<Body>, Response<Body>, Error>);

impl RouteService {
    pub fn new<S, E, ResBody>(service: S) -> Self
    where
        S: Service<Request<Body>, Response=Response<ResBody>, Error=E>
            + Clone
            + Send
            + Sync
            + 'static,
        E: Into<BoxError>,
        ResBody: http_body::Body<Data = Bytes> + Send + 'static,
        ResBody::Error: Into<BoxError>,
    {
        try_downcast::<RouteService, _>(service).unwrap_or_else(|service| {
            Self(SyncBoxCloneService::new(
                service
                    .map_response(|response| response.map(Body::new))
                    .map_err(|e| Error::new(e.into())),
            ))
        })
    }
}

impl<ReqBody> Service<Request<ReqBody>> for RouteService
where
    ReqBody: http_body::Body<Data = Bytes> + Send + 'static,
    ReqBody::Error: Into<BoxError>,
{
    type Response = Response<Body>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(ctx)
    }

    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        self.0.call(request.map(Body::new))
    }
}
