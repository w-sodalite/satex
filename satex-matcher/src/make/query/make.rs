use satex_core::config::args::Args;
use satex_core::serde::regex::SerdeRegex;
use satex_core::Error;

use crate::make::query::QueryMatcher;
use crate::{MakeRouteMatcher, __make_matcher};

__make_matcher! {
    Query,
    name: String,
    value: SerdeRegex,
}

fn make(args: Args) -> Result<QueryMatcher, Error> {
    let config = Config::try_from(args)?;
    Ok(QueryMatcher::new(config.name, config.value.into()))
}

#[cfg(test)]
mod test {
    use hyper::Uri;

    use satex_core::config::args::{Args, Shortcut};

    use crate::{MakeRouteMatcher, RouteMatcher, __assert_matcher};

    use super::MakeQueryMatcher;

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("k1,v1"));
        __assert_matcher!(
            MakeQueryMatcher,
            args,
            [
                Ok(true) => |e| { e.uri = Uri::from_static("https://satex.dev/index.html?k1=v1") },
                Ok(false) => |e| { e.uri = Uri::from_static("https://satex.dev/index.html?k1=v2") },
            ]
        );
    }
}
