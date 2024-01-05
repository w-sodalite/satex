use std::collections::HashSet;

use hyper::{Method, Request};

pub use make::MakeMethodMatcher;
use satex_core::http::Body;
use satex_core::Error;

use crate::RouteMatcher;

mod make;

#[derive(Debug, Clone)]
pub struct MethodMatcher {
    methods: HashSet<Method>,
}

impl MethodMatcher {
    pub fn new(methods: HashSet<Method>) -> Self {
        Self { methods }
    }
}

impl RouteMatcher for MethodMatcher {
    fn is_match(&self, request: &Request<Body>) -> Result<bool, Error> {
        Ok(self.methods.contains(request.method()))
    }
}
