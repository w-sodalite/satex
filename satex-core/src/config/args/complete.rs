use serde::de::DeserializeOwned;
use serde_yaml::Value;

use crate::{satex_error, Error};

#[derive(Clone)]
pub struct Complete<'a>(pub &'a Value);

impl<'a> Complete<'a> {
    pub fn new(v: &'a Value) -> Self {
        Self(v)
    }
}

impl<'a> Complete<'a> {
    pub fn deserialize<T: DeserializeOwned>(&self) -> Result<T, Error> {
        serde_yaml::from_value(self.0.clone()).map_err(|e| satex_error!(e))
    }
}
