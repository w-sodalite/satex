pub use make::MakeQueryMatcher;
use satex_core::essential::Essential;
use satex_core::pattern::Pattern;
use satex_core::Error;
use satex_core::Error;

use crate::RouteMatcher;

mod make;

#[derive(Clone)]
pub struct QueryMatcher {
    name: String,
    patterns: Vec<Pattern>,
}

impl QueryMatcher {
    pub fn new(name: String, patterns: Vec<Pattern>) -> Self {
        Self { name, patterns }
    }
}

impl RouteMatcher for QueryMatcher {
    fn is_match(&self, essential: &mut Essential) -> Result<bool, Error> {
        Ok(match essential.uri.query() {
            Some(query) => {
                let query = qstring::QString::from(query);
                self.patterns
                    .iter()
                    .any(|pattern| pattern.is_match(query.get(&self.name)))
            }
            None => self.patterns.iter().any(|pattern| pattern.is_match(None)),
        })
    }
}
