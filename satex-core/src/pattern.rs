use std::collections::{HashMap, VecDeque};
use std::fmt::Formatter;
use std::sync::Arc;

use regex::Regex;
use serde::de::{MapAccess, Visitor};
use serde::{Deserializer, Serialize, Serializer};
use serde_this_or_that::Deserialize;

use crate::{satex_error, Error};

#[derive(Debug, Clone)]
pub enum Mode {
    Exact(Arc<str>, Arc<str>, bool),
    StartsWith(Arc<str>, Arc<str>, bool),
    EndsWith(Arc<str>, Arc<str>, bool),
    Contains(Arc<str>, Arc<str>, bool),
    NotContains(Arc<str>, Arc<str>, bool),
    Exists,
    NotExists,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Simple(Mode),
    Regex(Regex),
}

macro_rules! construct {
    ($(($name:ident,$variant:ident)),* $(,)?) => {
        $(
            pub fn $name(value: &str, sensitive: bool) -> Self {
                let lowercase = value.to_ascii_lowercase();
                Self::Simple(Mode::$variant(Arc::from(value), Arc::from(lowercase), sensitive))
            }
        )*
    };
}

impl Pattern {
    construct! {
        (exact,Exact),
        (starts_with,StartsWith),
        (ends_with,EndsWith),
        (contains,Contains),
        (not_contains,NotContains)
    }

    pub fn exists() -> Self {
        Self::Simple(Mode::Exists)
    }

    pub fn not_exists() -> Self {
        Self::Simple(Mode::NotExists)
    }

    pub fn regex(regex: &str) -> Result<Self, Error> {
        Regex::new(regex)
            .map(|regex| Self::Regex(regex))
            .map_err(|e| satex_error!(e))
    }

    pub fn is_match(&self, input: Option<&str>) -> bool {
        match input {
            Some(input) => match self {
                Pattern::Simple(mode) => match mode {
                    Mode::Exact(value, lowercase, sensitive) => {
                        if *sensitive {
                            input.eq(value.as_ref())
                        } else {
                            input.to_ascii_lowercase().eq(lowercase.as_ref())
                        }
                    }
                    Mode::StartsWith(value, lowercase, sensitive) => {
                        if *sensitive {
                            input.starts_with(value.as_ref())
                        } else {
                            input.to_ascii_lowercase().starts_with(lowercase.as_ref())
                        }
                    }
                    Mode::EndsWith(value, lowercase, sensitive) => {
                        if *sensitive {
                            input.ends_with(value.as_ref())
                        } else {
                            input.to_ascii_lowercase().ends_with(lowercase.as_ref())
                        }
                    }
                    Mode::Contains(value, lowercase, sensitive) => {
                        if *sensitive {
                            input.contains(value.as_ref())
                        } else {
                            input.to_ascii_lowercase().contains(lowercase.as_ref())
                        }
                    }
                    Mode::NotContains(value, lowercase, sensitive) => {
                        !if *sensitive {
                            input.contains(value.as_ref())
                        } else {
                            input.to_ascii_lowercase().contains(lowercase.as_ref())
                        }
                    }
                    Mode::Exists => true,
                    Mode::NotExists => false,
                },
                Pattern::Regex(regex) => regex.is_match(input.as_ref()),
            },
            None => matches!(self, Pattern::Simple(Mode::NotExists)),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum StrOrBool<'a> {
    Str(&'a str),
    Bool(bool),
}

impl Serialize for Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        impl<'a> From<bool> for StrOrBool<'a> {
            fn from(value: bool) -> Self {
                StrOrBool::Bool(value)
            }
        }
        impl<'a> From<&'a str> for StrOrBool<'a> {
            fn from(value: &'a str) -> Self {
                StrOrBool::Str(value)
            }
        }

        fn set_value<'a>(
            data: &mut HashMap<&'a str, StrOrBool<'a>>,
            mode: &'a str,
            value: Option<&'a str>,
            sensitive: Option<bool>,
        ) {
            data.insert("mode", StrOrBool::from(mode));
            if let Some(value) = value {
                data.insert("value", StrOrBool::from(value.as_ref()));
            }
            if let Some(sensitive) = sensitive {
                data.insert("sensitive", StrOrBool::from(sensitive));
            }
        }

        let mut data = HashMap::new();
        match self {
            Pattern::Simple(mode) => match mode {
                Mode::Exact(value, _, sensitive) => {
                    set_value(&mut data, "Exact", Some(value.as_ref()), Some(*sensitive));
                }
                Mode::StartsWith(value, _, sensitive) => {
                    set_value(
                        &mut data,
                        "StartsWith",
                        Some(value.as_ref()),
                        Some(*sensitive),
                    );
                }
                Mode::EndsWith(value, _, sensitive) => {
                    set_value(
                        &mut data,
                        "EndsWith",
                        Some(value.as_ref()),
                        Some(*sensitive),
                    );
                }
                Mode::Contains(value, _, sensitive) => {
                    set_value(
                        &mut data,
                        "Contains",
                        Some(value.as_ref()),
                        Some(*sensitive),
                    );
                }
                Mode::NotContains(value, _, sensitive) => {
                    set_value(
                        &mut data,
                        "NotContains",
                        Some(value.as_ref()),
                        Some(*sensitive),
                    );
                }
                Mode::Exists => {
                    set_value(&mut data, "Exists", None, None);
                }
                Mode::NotExists => {
                    set_value(&mut data, "NotExists", None, None);
                }
            },
            Pattern::Regex(regex) => set_value(&mut data, "Regex", Some(regex.as_str()), None),
        };
        data.serialize(serializer)
    }
}

struct PatternVisitor;

fn try_from<E: serde::de::Error>(
    mode: Option<&str>,
    value: Option<&str>,
    sensitive: Option<bool>,
) -> Result<Pattern, E> {
    match mode {
        Some(mode) => match mode {
            "Exists" => Ok(Pattern::exists()),
            "NotExists" => Ok(Pattern::not_exists()),
            _ => match value {
                Some(value) => match mode {
                    "Regex" => Pattern::regex(value).map_err(|e| serde::de::Error::custom(e)),
                    "Exact" => Ok(Pattern::exact(value, sensitive.unwrap_or_default())),
                    "StartsWith" => Ok(Pattern::starts_with(value, sensitive.unwrap_or_default())),
                    "EndsWith" => Ok(Pattern::ends_with(value, sensitive.unwrap_or_default())),
                    "Contains" => Ok(Pattern::contains(value, sensitive.unwrap_or_default())),
                    "NotContains" => {
                        Ok(Pattern::not_contains(value, sensitive.unwrap_or_default()))
                    }
                    _ => Err(serde::de::Error::unknown_variant(
                        mode,
                        &[
                            "Exists",
                            "NotExists",
                            "Regex",
                            "Exact",
                            "StartsWith",
                            "EndsWith",
                            "Contains",
                            "NotContains",
                        ],
                    )),
                },
                None => Err(serde::de::Error::missing_field("value")),
            },
        },
        None => Err(serde::de::Error::missing_field("mode")),
    }
}

impl<'de> Visitor<'de> for PatternVisitor {
    type Value = Pattern;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Pattern")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut items = v
            .split(',')
            .map(|item| item.trim())
            .collect::<VecDeque<_>>();
        let mode = items.pop_front();
        let value = items.pop_front();
        let sensitive = items
            .pop_front()
            .map(|v| v.parse::<bool>().unwrap_or_default());
        try_from(mode, value, sensitive)
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut mode = None;
        let mut value = None;
        let mut sensitive = None;
        while let Some(key) = access.next_key::<&str>()? {
            match key {
                "mode" => {
                    mode = Some(access.next_value::<&str>()?);
                }
                "value" => {
                    value = Some(access.next_value::<&str>()?);
                }
                "sensitive" => {
                    sensitive = Some(access.next_value::<bool>()?);
                }
                _ => continue,
            }
        }
        try_from(mode, value, sensitive)
    }
}

impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PatternVisitor)
    }
}

#[cfg(test)]
mod test {
    use crate::pattern::Pattern;

    #[test]
    fn test() {
        let pattern = serde_yaml::from_str::<Pattern>("Exact,ABC,false").unwrap();
        println!("{:?}", pattern);
    }
}
