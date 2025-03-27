use crate::Error;
use regex::Regex;
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Expression {
    Equals(Value),
    NotEquals(Value),
    StartsWith(Value),
    NotStartsWith(Value),
    EndsWith(Value),
    NotEndsWith(Value),
    Contains(Value),
    NotContains(Value),
    Exists,
    NotExists,
    Regex(Regex),
}

impl Expression {
    pub fn equals(value: impl AsRef<str>, sensitive: bool) -> Self {
        Expression::Equals(Value::new(value, sensitive))
    }

    pub fn not_equals(value: impl AsRef<str>, sensitive: bool) -> Self {
        Expression::NotEquals(Value::new(value, sensitive))
    }

    pub fn starts_with(value: impl AsRef<str>, sensitive: bool) -> Self {
        Expression::StartsWith(Value::new(value, sensitive))
    }

    pub fn not_starts_with(value: impl AsRef<str>, sensitive: bool) -> Self {
        Expression::NotStartsWith(Value::new(value, sensitive))
    }

    pub fn ends_with(value: impl AsRef<str>, sensitive: bool) -> Self {
        Expression::EndsWith(Value::new(value, sensitive))
    }

    pub fn not_ends_with(value: impl AsRef<str>, sensitive: bool) -> Self {
        Expression::NotEndsWith(Value::new(value, sensitive))
    }

    pub fn contains(value: impl AsRef<str>, sensitive: bool) -> Self {
        Expression::Contains(Value::new(value, sensitive))
    }

    pub fn not_contains(value: impl AsRef<str>, sensitive: bool) -> Self {
        Expression::NotContains(Value::new(value, sensitive))
    }

    pub fn regex(value: impl AsRef<str>) -> Result<Self, Error> {
        Regex::from_str(value.as_ref())
            .map(Expression::Regex)
            .map_err(Error::new)
    }

    pub fn exists() -> Self {
        Self::Exists
    }

    pub fn not_exists() -> Self {
        Self::NotExists
    }

    pub fn matches(&self, text: Option<&str>) -> bool {
        match self {
            Expression::Equals(value) => value.equals(text),
            Expression::NotEquals(value) => value.not_equals(text),
            Expression::StartsWith(value) => value.starts_with(text),
            Expression::NotStartsWith(value) => value.not_starts_with(text),
            Expression::EndsWith(value) => value.ends_with(text),
            Expression::NotEndsWith(value) => value.not_ends_with(text),
            Expression::Contains(value) => value.contains(text),
            Expression::NotContains(value) => value.not_contains(text),
            Expression::Exists => text.is_some(),
            Expression::NotExists => text.is_none(),
            Expression::Regex(regex) => match text {
                Some(text) => regex.is_match(text),
                None => false,
            },
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Equals(value) => {
                f.write_str("Equals")?;
                <Value as Display>::fmt(value, f)
            }
            Expression::NotEquals(value) => {
                f.write_str("NotEquals")?;
                <Value as Display>::fmt(value, f)
            }
            Expression::StartsWith(value) => {
                f.write_str("StartsWith")?;
                <Value as Display>::fmt(value, f)
            }
            Expression::NotStartsWith(value) => {
                f.write_str("NotStartWith")?;
                <Value as Display>::fmt(value, f)
            }
            Expression::EndsWith(value) => {
                f.write_str("EndsWith")?;
                <Value as Display>::fmt(value, f)
            }
            Expression::NotEndsWith(value) => {
                f.write_str("NotEndsWith")?;
                <Value as Display>::fmt(value, f)
            }
            Expression::Contains(value) => {
                f.write_str("Contains")?;
                <Value as Display>::fmt(value, f)
            }
            Expression::NotContains(value) => {
                f.write_str("NotContains")?;
                <Value as Display>::fmt(value, f)
            }
            Expression::Exists => f.write_str("Exists"),
            Expression::NotExists => f.write_str("NotExists"),
            Expression::Regex(regex) => f.write_str(regex.as_str()),
        }
    }
}

impl FromStr for Expression {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();
        match value {
            "Exists" => Ok(Expression::Exists),
            "NotExists" => Ok(Expression::NotExists),
            _ => match value.find('(').zip(value.rfind(')')) {
                Some((open, close)) => {
                    let r#type = value[..open].trim();
                    let value = &value[open + 1..close];
                    match r#type {
                        "Equals" => Ok(Expression::equals(value, true)),
                        "?Equals" => Ok(Expression::equals(value, false)),
                        "NotEquals" => Ok(Expression::not_equals(value, true)),
                        "?NotEquals" => Ok(Expression::not_equals(value, false)),
                        "StartsWith" => Ok(Expression::starts_with(value, true)),
                        "?StartsWith" => Ok(Expression::starts_with(value, false)),
                        "NotStartsWith" => Ok(Expression::not_starts_with(value, true)),
                        "?NotStartsWith" => Ok(Expression::not_starts_with(value, false)),
                        "EndsWith" => Ok(Expression::ends_with(value, true)),
                        "?EndsWith" => Ok(Expression::ends_with(value, false)),
                        "NotEndsWith" => Ok(Expression::not_ends_with(value, true)),
                        "?NotEndsWith" => Ok(Expression::not_ends_with(value, false)),
                        "Contains" => Ok(Expression::contains(value, true)),
                        "?Contains" => Ok(Expression::contains(value, false)),
                        "NotContains" => Ok(Expression::not_contains(value, true)),
                        "?NotContains" => Ok(Expression::not_contains(value, false)),
                        "Regex" => Expression::regex(value),
                        _ => Err(Error::new(format!("invalid expression type: {}", r#type))),
                    }
                }
                None => Err(Error::new(format!("invalid expression: {}", value))),
            },
        }
    }
}

impl<'de> Deserialize<'de> for Expression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        Expression::from_str(&text).map_err(D::Error::custom)
    }
}

impl Serialize for Expression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[derive(Debug, Clone)]
pub struct Value {
    raw: Arc<str>,
    sensitive: bool,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (value, enabled) = match self.sensitive {
            true => (self.raw.as_ref(), true),
            false => (self.raw.as_ref(), false),
        };
        f.write_char('(')?;
        f.write_str(value)?;
        f.write_char(',')?;
        f.write_str(if enabled { "true" } else { "false" })?;
        f.write_char(')')
    }
}

impl Value {
    pub fn new(value: impl AsRef<str>, sensitive: bool) -> Self {
        let raw = if sensitive {
            Arc::from(value.as_ref())
        } else {
            Arc::from(value.as_ref().to_lowercase())
        };
        Self { raw, sensitive }
    }

    pub fn equals(&self, value: Option<&str>) -> bool {
        match value {
            Some(value) => {
                if self.sensitive {
                    self.raw.as_ref() == value
                } else {
                    self.raw.as_ref().eq_ignore_ascii_case(value)
                }
            }
            None => false,
        }
    }

    pub fn not_equals(&self, value: Option<&str>) -> bool {
        !self.equals(value)
    }

    pub fn starts_with(&self, value: Option<&str>) -> bool {
        match value {
            Some(value) => {
                if self.sensitive {
                    value.starts_with(self.raw.as_ref())
                } else {
                    value.to_lowercase().starts_with(self.raw.as_ref())
                }
            }
            None => false,
        }
    }

    pub fn not_starts_with(&self, value: Option<&str>) -> bool {
        !self.starts_with(value)
    }

    pub fn ends_with(&self, value: Option<&str>) -> bool {
        match value {
            Some(value) => {
                if self.sensitive {
                    value.ends_with(self.raw.as_ref())
                } else {
                    value.to_lowercase().ends_with(self.raw.as_ref())
                }
            }
            None => false,
        }
    }

    pub fn not_ends_with(&self, value: Option<&str>) -> bool {
        !self.ends_with(value)
    }

    pub fn contains(&self, value: Option<&str>) -> bool {
        match value {
            Some(value) => {
                if self.sensitive {
                    value.contains(self.raw.as_ref())
                } else {
                    value.to_lowercase().contains(self.raw.as_ref())
                }
            }
            None => false,
        }
    }

    pub fn not_contains(&self, value: Option<&str>) -> bool {
        !self.contains(value)
    }
}
