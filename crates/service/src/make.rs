use crate::RouteService;
use bytes::Bytes;
use http::{Request, Response};
use satex_core::body::Body;
use satex_core::component::Args;
use satex_core::make::Make;
use satex_core::util::try_downcast;
use satex_core::{BoxError, Error};
use std::sync::Arc;
use tower::Service;

pub trait MakeRouteService: Make {
    type Service: Service<Request<Body>>;

    fn make(&self, args: Args) -> Result<Self::Service, Error>;
}

#[derive(Clone)]
pub struct ArcMakeRouteService(Arc<dyn MakeRouteService<Service=RouteService> + Send + Sync>);

impl ArcMakeRouteService {
    pub fn new<M, S, E, ResBody>(make: M) -> Self
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
        try_downcast::<ArcMakeRouteService, _>(make)
            .unwrap_or_else(|make| ArcMakeRouteService(Arc::new(MapMake(make))))
    }
}

impl Make for ArcMakeRouteService {
    fn name(&self) -> &'static str {
        self.0.name()
    }
}

impl MakeRouteService for ArcMakeRouteService {
    type Service = RouteService;

    fn make(&self, args: Args) -> Result<Self::Service, Error> {
        self.0.make(args)
    }
}

struct MapMake<M>(M);

impl<M, S, E, ResBody> Make for MapMake<M>
where
    M: MakeRouteService<Service=S>,
    S: Service<Request<Body>, Response=Response<ResBody>, Error=E>,
{
    fn name(&self) -> &'static str {
        self.0.name()
    }
}

impl<M, S, E, ResBody> MakeRouteService for MapMake<M>
where
    M: 'static + MakeRouteService<Service=S>,
    S: 'static
    + Clone
    + Send
    + Service<Request<Body>, Response=Response<ResBody>, Error=E>
    + Sync,
    ResBody: 'static + Send + http_body::Body<Data=Bytes>,
    ResBody::Error: Into<BoxError>,
    E: Into<BoxError>,
{
    type Service = RouteService;

    fn make(&self, args: Args) -> Result<Self::Service, Error> {
        self.0.make(args).map(RouteService::new)
    }
}
