use satex_core::config::args::Args;
use satex_core::pattern::Pattern;
use satex_core::Error;

use crate::make::make_matcher;
use crate::MakeRouteMatcher;

use super::CookieMatcher;

make_matcher! {
    Cookie,
    TailSequence,
    name: String,
    values: Vec<Pattern>
}

fn make(args: Args<'_>) -> Result<CookieMatcher, Error> {
    let config = Config::try_from(args)?;
    Ok(CookieMatcher::new(config.name, config.values))
}

#[cfg(test)]
mod test {
    use hyper::http::HeaderValue;

    use satex_core::config::args::{Args, Shortcut};

    use crate::make::assert_matcher;
    use crate::make::cookie::make::MakeCookieMatcher;
    use crate::MakeRouteMatcher;
    use crate::RouteMatcher;

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("k1,Exact,v1"));
        assert_matcher!(
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
