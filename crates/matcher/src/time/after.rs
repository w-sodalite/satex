use crate::make::MakeRouteMatcher;
use crate::RouteMatcher;
use async_trait::async_trait;
use chrono::{Local, NaiveDateTime};
use http::request::Parts;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::make;

pub struct AfterRouteMatcher {
    date_time: NaiveDateTime,
}

impl AfterRouteMatcher {
    pub fn new(date_time: NaiveDateTime) -> Self {
        Self { date_time }
    }
}

#[async_trait]
impl RouteMatcher for AfterRouteMatcher {
    async fn matches(&self, _: &mut Parts) -> Result<bool, Error> {
        let now = Local::now().naive_local();
        Ok(now >= self.date_time)
    }
}

#[make(kind = After)]
pub struct MakeAfterRouteMatcher {
    date_time: NaiveDateTime,
}

impl MakeRouteMatcher for MakeAfterRouteMatcher {
    type Matcher = AfterRouteMatcher;

    fn make(&self, args: Args) -> Result<Self::Matcher, Error> {
        Config::with_args(args).map(|config| AfterRouteMatcher::new(config.date_time))
    }
}
