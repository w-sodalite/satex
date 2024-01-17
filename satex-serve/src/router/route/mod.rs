use std::fmt::{Debug, Formatter};
use std::task::{Context, Poll};

use bytes::Bytes;
use futures::future::BoxFuture;
use hyper::{Request, Response};
use tower::Service;
use tracing::debug;

use satex_core::essential::Essential;
use satex_core::http::Body;
use satex_core::BoxError;
use satex_core::Error;
use satex_matcher::{NamedRouteMatcher, RouteMatcher};
use satex_service::NamedRouteService;

pub mod make;

#[derive(Clone)]
pub struct Route {
    id: String,
    matchers: Vec<NamedRouteMatcher>,
    service: NamedRouteService,
}

impl Debug for Route {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Route").field("id", &self.id).finish()
    }
}

impl Route {
    pub fn new(
        id: impl Into<String>,
        matchers: Vec<NamedRouteMatcher>,
        service: NamedRouteService,
    ) -> Self {
        Self {
            id: id.into(),
            matchers,
            service,
        }
    }

    pub(crate) fn is_match(&self, essential: &mut Essential) -> Result<bool, Error> {
        let mut match_msg = String::new();
        let mut iter = self.matchers.iter();
        let result = loop {
            if let Some(matcher) = iter.next() {
                match_msg.push_str(&format!("{:?}", matcher));
                match_msg.push_str(" -> ");
                match matcher.is_match(essential) {
                    Ok(true) => {
                        continue;
                    }
                    Ok(false) => {
                        match_msg.push_str("@Miss!");
                        break Ok(false);
                    }
                    Err(e) => {
                        match_msg.push_str(&format!("@Err: {}", e));
                        break Err(e);
                    }
                }
            } else {
                match_msg.push_str("@Matched!");
                break Ok(true);
            }
        };
        debug!("{:?} match stream: {}", self, match_msg);
        result
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl<ReqBody> Service<Request<ReqBody>> for Route
where
    ReqBody: hyper::body::Body<Data = Bytes> + Send + 'static,
    ReqBody::Error: Into<BoxError>,
{
    type Response = Response<Body>;
    type Error = Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        <NamedRouteService as Service<Request<ReqBody>>>::poll_ready(&mut self.service, cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        self.service.call(req.map(Body::new))
    }
}
