use crate::make::MakeRouteMatcher;
use crate::RouteMatcher;
use async_trait::async_trait;
use http::request::Parts;
use matchit::Router;
use satex_core::component::{Args, Configurable};
use satex_core::extension::insert_url_params;
use satex_core::util::With;
use satex_core::Error;
use satex_macro::make;

#[make(kind = Path)]
struct MakePathRouteMatcher {
    patterns: Vec<String>,
}

impl MakeRouteMatcher for MakePathRouteMatcher {
    type Matcher = PathRouteMatcher;

    fn make(&self, args: Args) -> Result<Self::Matcher, Error> {
        Config::with_args(args)
            .map_err(Error::new)
            .and_then(|config| PathRouteMatcher::new(config.patterns))
    }
}

pub struct PathRouteMatcher {
    router: Router<()>,
}

impl PathRouteMatcher {
    pub fn new<T: AsRef<str>, I: IntoIterator<Item = T>>(iter: I) -> Result<Self, Error> {
        iter.into_iter()
            .try_fold(Router::new(), |router, pattern| {
                router.try_with(|router| router.insert(pattern.as_ref(), ()))
            })
            .map_err(Error::new)
            .map(|router| PathRouteMatcher { router })
    }
}

#[async_trait]
impl RouteMatcher for PathRouteMatcher {
    async fn matches(&self, parts: &mut Parts) -> Result<bool, Error> {
        let path = parts.uri.path();
        match self.router.at(path) {
            Ok(m) => {
                insert_url_params(&mut parts.extensions, m.params);
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }
}
