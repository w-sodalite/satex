use crate::make::MakeRouteMatcher;
use crate::RouteMatcher;
use async_trait::async_trait;
use chrono::{Local, NaiveDateTime};
use http::request::Parts;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::make;

pub struct BeforeRouteMatcher {
    date_time: NaiveDateTime,
}

impl BeforeRouteMatcher {
    pub fn new(date_time: NaiveDateTime) -> Self {
        Self { date_time }
    }
}

#[async_trait]
impl RouteMatcher for BeforeRouteMatcher {
    async fn matches(&self, _: &mut Parts) -> Result<bool, Error> {
        let now = Local::now().naive_local();
        Ok(now <= self.date_time)
    }
}

#[make(kind = Before)]
pub struct MakeBeforeRouteMatcher {
    date_time: NaiveDateTime,
}

impl MakeRouteMatcher for MakeBeforeRouteMatcher {
    type Matcher = BeforeRouteMatcher;

    fn make(&self, args: Args) -> Result<Self::Matcher, Error> {
        Config::with_args(args).map(|config| BeforeRouteMatcher::new(config.date_time))
    }
}
