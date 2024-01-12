use satex_core::config::args::Args;
use satex_core::pattern::Pattern;
use satex_core::Error;

use crate::{MakeRouteMatcher, __make_matcher};

use super::CookieMatcher;

__make_matcher! {
    Cookie,
    CollectTail,
    name: String,
    patterns: Vec<Pattern>
}

fn make(args: Args<'_>) -> Result<CookieMatcher, Error> {
    let config = Config::try_from(args)?;
    Ok(CookieMatcher::new(config.name, config.patterns))
}

#[cfg(test)]
mod test {
    use hyper::http::HeaderValue;

    use satex_core::config::args::{Args, Shortcut};

    use crate::make::cookie::make::MakeCookieMatcher;
    use crate::{MakeRouteMatcher, RouteMatcher, __assert_matcher};

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("k1,v1"));
        __assert_matcher!(
            MakeCookieMatcher,
            args,
            [
                Ok(true) => |e| {
                    e.headers
                        .insert("cookie", HeaderValue::from_static("k1=v1"))
                },
                Ok(false) => |e| {
                    e.headers
                        .insert("cookie", HeaderValue::from_static("k1=v2"))
                }
            ]
        );
    }
}
