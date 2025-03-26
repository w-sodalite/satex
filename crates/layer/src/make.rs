use crate::ArcRouteLayer;
use bytes::Bytes;
use http::{Request, Response};
use satex_core::body::Body;
use satex_core::component::Args;
use satex_core::make::Make;
use satex_core::util::try_downcast;
use satex_core::{BoxError, Error};
use satex_service::RouteService;
use std::sync::Arc;
use tower::{Layer, Service};

pub trait MakeRouteLayer: Make {
    type Layer: Layer<RouteService>;

    fn make(&self, args: Args) -> Result<Self::Layer, Error>;
}

#[derive(Clone)]
pub struct ArcMakeRouteLayer(Arc<dyn MakeRouteLayer<Layer=ArcRouteLayer> + Send + Sync>);

impl ArcMakeRouteLayer {
    pub fn new<M, L, S, E, ResBody>(make: M) -> Self
    where
        M: MakeRouteLayer<Layer=L> + Send + Sync + 'static,
        L: Layer<RouteService, Service=S> + Send + Sync + 'static,
        S: Service<Request<Body>, Response=Response<ResBody>, Error=E>
            + Clone
            + Send
            + Sync
            + 'static,
        E: Into<BoxError>,
        ResBody: http_body::Body<Data = Bytes> + Send + 'static,
        ResBody::Error: Into<BoxError>,
    {
        try_downcast::<ArcMakeRouteLayer, _>(make)
            .unwrap_or_else(|make| Self(Arc::new(MapMake(make))))
    }
}

struct MapMake<M>(M);

impl<M, L, S, E, ResBody> Make for MapMake<M>
where
    M: MakeRouteLayer<Layer=L>,
    S: Service<Request<Body>, Response=Response<ResBody>, Error=E>,
    L: Layer<RouteService, Service=S>,
{
    fn name(&self) -> &'static str {
        self.0.name()
    }
}

impl<M, L, S, E, ResBody> MakeRouteLayer for MapMake<M>
where
    M: MakeRouteLayer<Layer=L>,
    L: Layer<RouteService, Service=S> + Send + Sync + 'static,
    S: Service<Request<Body>, Response=Response<ResBody>, Error=E>
        + Clone
        + Send
        + Sync
        + 'static,
    E: Into<BoxError>,
    ResBody: http_body::Body<Data = Bytes> + Send + 'static,
    ResBody::Error: Into<BoxError>,
{
    type Layer = ArcRouteLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        self.0.make(args).map(ArcRouteLayer::new)
    }
}

impl Make for ArcMakeRouteLayer {
    fn name(&self) -> &'static str {
        self.0.name()
    }
}

impl MakeRouteLayer for ArcMakeRouteLayer {
    type Layer = ArcRouteLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        self.0.make(args)
    }
}
