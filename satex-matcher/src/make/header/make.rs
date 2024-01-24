use satex_core::config::args::Args;
use satex_core::pattern::Pattern;
use satex_core::serde::http::SerdeHeaderName;
use satex_core::Error;

use crate::make::header::HeaderMatcher;
use crate::make::make_matcher;
use crate::MakeRouteMatcher;

make_matcher! {
    Header,
    TailSequence,
    name: SerdeHeaderName,
    values: Vec<Pattern>
}

fn make(args: Args<'_>) -> Result<HeaderMatcher, Error> {
    let config = Config::try_from(args)?;
    Ok(HeaderMatcher::new(config.name.into(), config.values))
}

#[cfg(test)]
mod test {
    use hyper::http::HeaderValue;

    use satex_core::config::args::{Args, Shortcut};

    use crate::make::assert_matcher;
    use crate::make::header::MakeHeaderMatcher;
    use crate::{MakeRouteMatcher, RouteMatcher};

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("k1,Exact,v1"));
        assert_matcher!(
            MakeHeaderMatcher,
            args,
            [
                Ok(true) => |e| { e.headers.insert("k1", HeaderValue::from_static("v1")) },
                Ok(false) => |e| { e.headers.insert("k1", HeaderValue::from_static("v2")) }
            ]
        );
    }
}
