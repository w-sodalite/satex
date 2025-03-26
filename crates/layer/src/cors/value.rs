use http::{HeaderName, HeaderValue, Method};
use serde::de::{Error, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::str::FromStr;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, ExposeHeaders, Vary};

const WILDCARD: &str = "*";

#[derive(Default)]
pub(super) enum Value<T> {
    #[default]
    Empty,
    Any,
    Exact(T),
    List(Vec<T>),
}

macro_rules! impl_from {
    ($($source:ty => $target:ident),* $(,)?) => {
        $(
            impl From<Value<$source>> for $target {
                fn from(value: Value<$source>) -> Self {
                    match value {
                        Value::Empty => $target::default(),
                        Value::Any => $target::any(),
                        Value::Exact(value) => $target::list(vec![value]),
                        Value::List(values) => $target::list(values),
                    }
                }
            }
        )*
    };

    (@exact, $($source:ty => $target:ident),* $(,)?) => {
        $(
            impl From<Value<$source>> for $target {
                fn from(value: Value<$source>) -> Self {
                    match value {
                        Value::Empty => $target::default(),
                        Value::Any => $target::any(),
                        Value::Exact(value) => $target::exact(value),
                        Value::List(values) => $target::list(values),
                    }
                }
            }
        )*
    };
}

impl_from! {
    HeaderName => AllowHeaders,
    HeaderName => ExposeHeaders,
}

impl_from! {
    @exact,
    Method => AllowMethods,
    HeaderValue => AllowOrigin,
}

impl From<Value<HeaderName>> for Vary {
    fn from(value: Value<HeaderName>) -> Self {
        match value {
            Value::List(headers) => Vary::list(headers),
            _ => Vary::default(),
        }
    }
}

struct ValueVisitor<T> {
    _p: PhantomData<T>,
}

impl<T> ValueVisitor<T> {
    pub fn new() -> Self {
        Self { _p: PhantomData }
    }
}

impl<'de, T> Visitor<'de> for ValueVisitor<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error,
{
    type Value = Value<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("expecting `str` or `list`")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if s == WILDCARD {
            Ok(Value::Any)
        } else {
            s.parse::<T>()
                .map(|item| Value::Exact(item))
                .map_err(E::custom)
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = vec![];
        while let Some(item) = seq.next_element::<String>()? {
            items.push(item.parse::<T>().map_err(Error::custom)?);
        }
        Ok(Value::List(items))
    }
}

impl<'de, T> Deserialize<'de> for Value<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor::new())
    }
}
