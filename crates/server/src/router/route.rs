use async_trait::async_trait;
use bytes::Bytes;
use futures::future::LocalBoxFuture;
use http::request::Parts;
use http::{Request, Response, StatusCode};
use satex_core::body::Body;
use satex_core::{BoxError, Error};
use satex_layer::ArcRouteLayer;
use satex_matcher::{ArcRouteMatcher, RouteMatcher};
use satex_service::RouteService;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{service_fn, Layer, Service};

#[derive(Clone)]
pub struct Route {
    id: Arc<str>,
    matchers: Arc<[ArcRouteMatcher]>,
    service: RouteService,
}

impl Route {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }
}

pub struct Builder {
    id: String,
    matchers: Vec<ArcRouteMatcher>,
    layers: Vec<ArcRouteLayer>,
    service: RouteService,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            id: String::default(),
            matchers: Vec::default(),
            layers: Vec::default(),
            service: RouteService::new(service_fn(service_unavailable)),
        }
    }
}

impl Builder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn service<S, E, ResBody>(mut self, service: S) -> Self
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
        self.service = RouteService::new(service);
        self
    }

    pub fn layer<S, L, E, ResBody>(mut self, layer: L) -> Self
    where
        S: Service<Request<Body>, Response=Response<ResBody>, Error=E>
            + Clone
            + Send
            + Sync
            + 'static,
        E: Into<BoxError>,
        L: Layer<RouteService, Service = S> + Send + Sync + 'static,
        ResBody: http_body::Body<Data = Bytes> + Send + 'static,
        ResBody::Error: Into<BoxError>,
    {
        self.layers.push(ArcRouteLayer::new(layer));
        self
    }

    pub fn matcher<M>(mut self, matcher: M) -> Self
    where
        M: RouteMatcher + Send + Sync + 'static,
    {
        self.matchers.push(ArcRouteMatcher::new(matcher));
        self
    }

    pub fn build(self) -> Route {
        let service = self
            .layers
            .iter()
            .fold(RouteService::new(self.service), |service, layer| {
                layer.layer(service)
            });
        Route {
            id: Arc::from(self.id),
            matchers: Arc::from(self.matchers),
            service,
        }
    }
}

async fn service_unavailable(_: Request<Body>) -> Result<Response<Body>, Error> {
    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::SERVICE_UNAVAILABLE;
    Ok(response)
}

#[async_trait]
impl RouteMatcher for Route {
    async fn matches(&self, parts: &mut Parts) -> satex_core::Result<bool> {
        for matcher in self.matchers.iter() {
            match matcher.matches(parts).await {
                Ok(true) => continue,
                Ok(false) => return Ok(false),
                Err(e) => return Err(e),
            }
        }
        Ok(true)
    }
}

impl<ReqBody> Service<Request<ReqBody>> for Route
where
    ReqBody: http_body::Body<Data = Bytes> + Send + 'static,
    ReqBody::Error: Into<BoxError>,
{
    type Response = Response<Body>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        <RouteService as Service<Request<ReqBody>>>::poll_ready(&mut self.service, cx)
    }

    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        self.service.call(request)
    }
}
