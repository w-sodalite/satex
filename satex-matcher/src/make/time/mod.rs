use chrono::{Local, NaiveDateTime};
use serde::Deserialize;

pub use make::MakeTimeMatcher;
use satex_core::essential::Essential;
use satex_core::Error;

use crate::RouteMatcher;

mod make;

#[derive(Deserialize)]
pub enum Mode {
    Before,
    After,
}

pub struct TimeMatcher {
    mode: Mode,
    time: NaiveDateTime,
}

impl TimeMatcher {
    pub fn new(mode: Mode, time: NaiveDateTime) -> Self {
        Self { mode, time }
    }
}

impl RouteMatcher for TimeMatcher {
    fn is_match(&self, _: &mut Essential) -> Result<bool, Error> {
        let now = Local::now().naive_local();
        match self.mode {
            Mode::Before => Ok(self.time.ge(&now)),
            Mode::After => Ok(self.time.le(&now)),
        }
    }
}
