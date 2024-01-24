use satex_core::config::args::Args;
use satex_core::pattern::Pattern;
use satex_core::Error;

use crate::make::make_matcher;
use crate::make::query::QueryMatcher;
use crate::MakeRouteMatcher;

make_matcher! {
    Query,
    TailSequence,
    name: String,
    values: Vec<Pattern>,
}

fn make(args: Args) -> Result<QueryMatcher, Error> {
    let config = Config::try_from(args)?;
    Ok(QueryMatcher::new(config.name, config.values))
}

#[cfg(test)]
mod test {
    use hyper::Uri;

    use satex_core::config::args::{Args, Shortcut};

    use crate::make::assert_matcher;
    use crate::{MakeRouteMatcher, RouteMatcher};

    use super::MakeQueryMatcher;

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("k1,StartsWith,v1"));
        assert_matcher!(
            MakeQueryMatcher,
            args,
            [
                Ok(true) => |e| { e.uri = Uri::from_static("https://satex.dev/index.html?k1=v1") },
                Ok(true) => |e| { e.uri = Uri::from_static("https://satex.dev/index.html?k1=v1111") },
                Ok(false) => |e| { e.uri = Uri::from_static("https://satex.dev/index.html?k1=v2") },
            ]
        );
    }
}
