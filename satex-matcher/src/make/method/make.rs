use std::collections::HashSet;

use hyper::Method;

use satex_core::config::args::Args;
use satex_core::serde::http::SerdeMethod;
use satex_core::Error;

use crate::make::make_matcher;
use crate::make::method::MethodMatcher;
use crate::MakeRouteMatcher;

make_matcher! {
    Method,
    Sequence,
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
    use hyper::Method;

    use satex_core::config::args::{Args, Shortcut};

    use crate::make::assert_matcher;
    use crate::{MakeRouteMatcher, RouteMatcher};

    use super::MakeMethodMatcher;

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("GET,POST"));
        assert_matcher!(
            MakeMethodMatcher,
            args,
            [
                Ok(true) => |e| { e.method = Method::GET },
                Ok(true) => |e| { e.method = Method::POST },
                Ok(false) => |e| { e.method = Method::DELETE },
                Ok(false) => |e| { e.method = Method::PUT },
            ]
        );
    }
}
