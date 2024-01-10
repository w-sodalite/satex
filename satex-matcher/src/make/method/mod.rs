use std::collections::HashSet;

use hyper::Method;

pub use make::MakeMethodMatcher;
use satex_core::essential::Essential;
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
    fn is_match(&self, essential: &mut Essential) -> Result<bool, Error> {
        Ok(self.methods.contains(&essential.method))
    }
}
