use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use hyper::Request;

use satex_core::http::Body;
use satex_core::{export_make, Error};

mod make;
mod registry;
export_make!(MakeRouteMatcher);

pub trait RouteMatcher {
    fn is_match(&self, request: &Request<Body>) -> Result<bool, Error>;
}

#[derive(Clone)]
pub struct NamedRouteMatcher {
    name: &'static str,
    inner: Arc<dyn RouteMatcher + Sync + Send + 'static>,
}

impl Debug for NamedRouteMatcher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RouteMatcher")
            .field("name", &self.name)
            .finish()
    }
}

impl NamedRouteMatcher {
    pub fn new<M: RouteMatcher + Send + Sync + 'static>(name: &'static str, matcher: M) -> Self {
        Self {
            name,
            inner: Arc::new(matcher),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl RouteMatcher for NamedRouteMatcher {
    fn is_match(&self, request: &Request<Body>) -> Result<bool, Error> {
        self.inner.is_match(request)
    }
}

#[derive(Clone)]
pub(crate) struct MatchFn<F> {
    f: F,
}

impl<F> RouteMatcher for MatchFn<F>
where
    F: Fn(&Request<Body>) -> Result<bool, Error>,
{
    fn is_match(&self, request: &Request<Body>) -> Result<bool, Error> {
        (self.f)(request)
    }
}
