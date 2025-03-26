use crate::registry::push;
use satex_matcher::cookie::MakeCookieRouteMatcher;
use satex_matcher::header::MakeHeaderRouteMatcher;
use satex_matcher::host::MakeHostRouteMatcher;
use satex_matcher::make::{ArcMakeRouteMatcher, MakeRouteMatcher};
use satex_matcher::method::MakeMethodRouteMatcher;
use satex_matcher::path::MakePathRouteMatcher;
use satex_matcher::query::MakeQueryRouteMatcher;
use satex_matcher::remote_addr::MakeRemoteAddrRouteMatcher;
use satex_matcher::time::{
    MakeAfterRouteMatcher, MakeBeforeRouteMatcher, MakeBetweenRouteMatcher,
};
use satex_matcher::RouteMatcher;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MakeRouteMatcherRegistry(HashMap<&'static str, ArcMakeRouteMatcher>);

impl MakeRouteMatcherRegistry {
    pub fn without_default() -> Self {
        Self(HashMap::new())
    }

    pub fn with_default() -> Self {
        let mut registry = Self::without_default();
        push! {
            registry,
            MakePathRouteMatcher,
            MakeMethodRouteMatcher,
            MakeQueryRouteMatcher,
            MakeCookieRouteMatcher,
            MakeHeaderRouteMatcher,
            MakeBeforeRouteMatcher,
            MakeAfterRouteMatcher,
            MakeBetweenRouteMatcher,
            MakeHostRouteMatcher,
            MakeRemoteAddrRouteMatcher
        }
        registry
    }

    pub fn push<M>(&mut self, make: M)
    where
        M: MakeRouteMatcher + Send + Sync + 'static,
        M::Matcher: RouteMatcher + Send + Sync + 'static,
    {
        self.0.insert(make.name(), ArcMakeRouteMatcher::new(make));
    }

    pub fn get(&self, name: &str) -> Option<ArcMakeRouteMatcher> {
        self.0.get(name).cloned()
    }
}
