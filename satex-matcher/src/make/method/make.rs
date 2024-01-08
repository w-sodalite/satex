use std::collections::HashSet;

use hyper::Method;

use satex_core::config::args::Args;
use satex_core::serde::http::SerdeMethod;
use satex_core::Error;

use crate::make::method::MethodMatcher;
use crate::{MakeRouteMatcher, __matcher};

__matcher! {
    Method,
    List,
    methods: HashSet<SerdeMethod>,
}

fn make(args: Args) -> Result<MethodMatcher, Error> {
    let config = Config::try_from(args)?;
    Ok(MethodMatcher::new(
        config.methods.into_iter().map(Method::from).collect(),
    ))
}

#[cfg(test)]
mod test {
    use hyper::{Method, Request};
    use satex_core::{
        config::args::{Args, Shortcut},
        http::Body,
    };

    use crate::{MakeRouteMatcher, RouteMatcher};

    use super::MakeMethodMatcher;

    fn new_request(method: Method) -> Request<Body> {
        Request::builder()
            .method(method)
            .body(Body::empty())
            .unwrap()
    }

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("GET,POST"));
        let make = MakeMethodMatcher::default();
        let matcher = make.make(args).unwrap();
        assert!(matcher.is_match(&new_request(Method::GET)).unwrap());
        assert!(matcher.is_match(&new_request(Method::POST)).unwrap());
        assert!(!matcher.is_match(&new_request(Method::PUT)).unwrap());
    }
}
