pub mod cookie;
pub mod header;
pub mod host;
pub mod make;
pub mod method;
pub mod path;
pub mod query;
pub mod remote_addr;
pub mod time;

use async_trait::async_trait;
use http::request::Parts;
use satex_core::util::try_downcast;
use satex_core::Error;
use std::sync::Arc;

#[async_trait]
pub trait RouteMatcher {
    async fn matches(&self, parts: &mut Parts) -> Result<bool, Error>;
}

#[derive(Clone)]
pub struct ArcRouteMatcher(Arc<dyn RouteMatcher + Send + Sync>);

impl ArcRouteMatcher {
    pub fn new<M>(matcher: M) -> Self
    where
        M: RouteMatcher + Send + Sync + 'static,
    {
        try_downcast::<ArcRouteMatcher, _>(matcher)
            .unwrap_or_else(|matcher| Self(Arc::new(matcher)))
    }
}

#[async_trait]
impl RouteMatcher for ArcRouteMatcher {
    async fn matches(&self, parts: &mut Parts) -> Result<bool, Error> {
        self.0.matches(parts).await
    }
}
