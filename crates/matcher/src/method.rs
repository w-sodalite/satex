use crate::RouteMatcher;
use async_trait::async_trait;
use http::request::Parts;
use http::Method;
use std::collections::HashSet;

use crate::make::MakeRouteMatcher;
use satex_core::component::{Args, Configurable};
use satex_core::util::With;
use satex_core::Error;
use satex_macro::make;
use std::str::FromStr;

#[make(kind = Method, shortcut_mode = Sequence)]
struct MakeMethodRouteMatcher {
    methods: Vec<String>,
}

impl MakeRouteMatcher for MakeMethodRouteMatcher {
    type Matcher = MethodRouteMatcher;

    fn make(&self, args: Args) -> Result<Self::Matcher, Error> {
        Config::with_args(args)
            .map_err(Error::new)
            .and_then(|config| {
                config
                    .methods
                    .into_iter()
                    .try_fold(vec![], |methods, method| {
                        methods.try_with(|methods| {
                            Method::from_str(&method).map(|method| methods.push(method))
                        })
                    })
                    .map_err(Error::new)
                    .map(MethodRouteMatcher::new)
            })
    }
}

#[derive(Debug, Clone)]
pub struct MethodRouteMatcher {
    methods: HashSet<Method>,
}

impl MethodRouteMatcher {
    pub fn new<I: IntoIterator<Item=Method>>(iter: I) -> Self {
        Self {
            methods: HashSet::from_iter(iter),
        }
    }
}

#[async_trait]
impl RouteMatcher for MethodRouteMatcher {
    async fn matches(&self, parts: &mut Parts) -> satex_core::Result<bool> {
        Ok(self.methods.contains(&parts.method))
    }
}
