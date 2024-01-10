use regex::Regex;

pub use make::MakePathMatcher;
use satex_core::essential::Essential;
use satex_core::Error;

use crate::RouteMatcher;

mod make;

pub enum Pattern {
    StartsWith(String),
    Regex(Regex),
}

pub struct PathMatcher {
    patterns: Vec<Pattern>,
}

impl PathMatcher {
    pub fn new(patterns: Vec<Pattern>) -> Self {
        Self { patterns }
    }
}

impl RouteMatcher for PathMatcher {
    fn is_match(&self, essential: &mut Essential) -> Result<bool, Error> {
        let path = essential.uri.path();
        for pattern in self.patterns.iter() {
            match pattern {
                Pattern::StartsWith(pattern) => {
                    if path.starts_with(pattern) {
                        return Ok(true);
                    }
                }
                Pattern::Regex(regex) => {
                    if regex.is_match(path) {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }
}
