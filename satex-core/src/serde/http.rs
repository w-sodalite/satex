use hyper::header::{HeaderName, HeaderValue};
use hyper::{HeaderMap, Method, StatusCode};

use crate::serde_with;

serde_with!(
    StatusCode,
    "http_serde::status_code",
    [Debug, Clone, PartialEq, Eq, Hash]
);

serde_with!(
    Method,
    "http_serde::method",
    [Debug, Clone, PartialEq, Eq, Hash]
);

serde_with!(HeaderMap, "http_serde::header_map", [Debug, Clone]);

serde_with!(
    HeaderName,
    "header_name",
    [Debug, Clone, Eq, PartialEq, Hash]
);

serde_with!(HeaderValue, "header_value", [Debug, Clone, Hash]);

pub mod header_name {
    use std::str::FromStr;

    use hyper::http::HeaderName;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(
        header_name: &HeaderName,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(header_name.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HeaderName, D::Error>
    where
        D: Deserializer<'de>,
    {
        let header_name = String::deserialize(deserializer)?;
        HeaderName::from_str(&header_name).map_err(serde::de::Error::custom)
    }
}

pub mod header_value {
    use hyper::http::HeaderValue;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(
        header_value: &HeaderValue,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(header_value.as_bytes())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HeaderValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        let header_value = String::deserialize(deserializer)?;
        HeaderValue::from_str(&header_value).map_err(serde::de::Error::custom)
    }
}
