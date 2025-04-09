use serde::de::DeserializeOwned;
use serde_yaml::Value;

use crate::Error;

pub enum Args<'a> {
    Full(&'a Value),
    Shortcut(Option<&'a str>),
}

impl Default for Args<'_> {
    fn default() -> Self {
        Args::Shortcut(None)
    }
}

impl<'a> Args<'a> {
    pub fn shortcut(value: &'a str) -> Self {
        Args::Shortcut(Some(value))
    }

    pub fn full(value: &'a Value) -> Self {
        Args::Full(value)
    }

    pub fn deserialize<T: DeserializeOwned>(&self) -> Result<T, Error> {
        match self {
            Args::Full(value) => serde_yaml::from_value((*value).clone()),
            Args::Shortcut(Some(value)) => serde_yaml::from_str(value),
            Args::Shortcut(None) => serde_yaml::from_value(Value::Null),
        }
        .map_err(Error::new)
    }
}
