use std::fmt::Formatter;
use std::marker::PhantomData;

use serde::de::{Error, IntoDeserializer, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, ExposeHeaders, Vary};

use satex_core::serde::http::{SerdeHeaderName, SerdeHeaderValue, SerdeMethod};

const WILDCARD: &str = "*";

pub enum Complex<T> {
    Any,
    Single(T),
    Many(Vec<T>),
}

impl<T> Serialize for Complex<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Complex::Any => serializer.serialize_str(WILDCARD),
            Complex::Single(value) => value.serialize(serializer),
            Complex::Many(values) => values.serialize(serializer),
        }
    }
}

struct ComplexVisitor<T> {
    mark: PhantomData<T>,
}

impl<T> Default for ComplexVisitor<T> {
    fn default() -> Self {
        ComplexVisitor { mark: PhantomData }
    }
}

impl<'de, T> Visitor<'de> for ComplexVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = Complex<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Expecting `str` or `seq`!")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match v {
            WILDCARD => Ok(Complex::Any),
            value => value.into_deserializer().deserialize_str(self),
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = vec![];
        while let Some(value) = seq.next_element::<T>()? {
            values.push(value);
        }
        Ok(Complex::Many(values))
    }
}

impl<'de, T> Deserialize<'de> for Complex<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ComplexVisitor::default())
    }
}

impl From<Complex<SerdeMethod>> for AllowMethods {
    fn from(complex: Complex<SerdeMethod>) -> Self {
        match complex {
            Complex::Any => AllowMethods::any(),
            Complex::Single(method) => AllowMethods::exact(method.into()),
            Complex::Many(methods) => AllowMethods::from(
                methods
                    .into_iter()
                    .map(|method| method.into())
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl From<Complex<SerdeHeaderName>> for AllowHeaders {
    fn from(complex: Complex<SerdeHeaderName>) -> Self {
        match complex {
            Complex::Any => AllowHeaders::any(),
            Complex::Single(header) => AllowHeaders::from([header.into()]),
            Complex::Many(headers) => AllowHeaders::from(
                headers
                    .into_iter()
                    .map(|header| header.into())
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl From<Complex<SerdeHeaderValue>> for AllowOrigin {
    fn from(complex: Complex<SerdeHeaderValue>) -> Self {
        match complex {
            Complex::Any => AllowOrigin::any(),
            Complex::Single(value) => AllowOrigin::exact(value.into()),
            Complex::Many(values) => AllowOrigin::from(
                values
                    .into_iter()
                    .map(|value| value.into())
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl From<Complex<SerdeHeaderName>> for ExposeHeaders {
    fn from(complex: Complex<SerdeHeaderName>) -> Self {
        match complex {
            Complex::Any => ExposeHeaders::any(),
            Complex::Single(header) => ExposeHeaders::from([header.into()]),
            Complex::Many(headers) => ExposeHeaders::from(
                headers
                    .into_iter()
                    .map(|header| header.into())
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl From<Complex<SerdeHeaderName>> for Vary {
    fn from(complex: Complex<SerdeHeaderName>) -> Self {
        match complex {
            Complex::Any => Vary::default(),
            Complex::Single(value) => Vary::from([value.into()]),
            Complex::Many(values) => Vary::from(
                values
                    .into_iter()
                    .map(|value| value.into())
                    .collect::<Vec<_>>(),
            ),
        }
    }
}
