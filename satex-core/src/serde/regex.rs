use regex::Regex;

pub use private::{deserialize, serialize};

use crate::serde_with;

serde_with!(Regex, "private", [Clone]);

mod private {
    use std::str::FromStr;

    use regex::Regex;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(regex: &Regex, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(regex.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Regex, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Regex::from_str(&value).map_err(serde::de::Error::custom)
    }
}
