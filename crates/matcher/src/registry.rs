use crate::make::{ArcMakeRouteMatcher, MakeRouteMatcher};
use crate::RouteMatcher;
use std::collections::HashMap;

#[derive(Default)]
pub struct MatcherRegistry {
    matchers: HashMap<&'static str, ArcMakeRouteMatcher>,
}

impl MatcherRegistry {
    pub fn add<M>(&mut self, make: M)
    where
        M: MakeRouteMatcher + Send + Sync + 'static,
        M::Matcher: RouteMatcher + Send + Sync + 'static,
    {
        let name = make.name();
        self.matchers.insert(name, ArcMakeRouteMatcher::new(make));
    }

    pub fn get(&self, name: &str) -> Option<&ArcMakeRouteMatcher> {
        self.matchers.get(name)
    }

    pub fn remove(&mut self, name: &str) {
        self.matchers.remove(name);
    }
}
