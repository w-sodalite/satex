use tracing::Level;

use crate::serde_with;

serde_with!(Level, "level", [Copy, Clone, Debug, PartialEq, Eq, Hash]);

pub mod level {
    use std::str::FromStr;

    use serde::{Deserialize, Deserializer, Serializer};
    use tracing::Level;

    pub fn serialize<S>(level: &Level, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(level.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Level, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Level::from_str(&value).map_err(serde::de::Error::custom)
    }
}
