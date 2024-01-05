use std::fmt::{Debug, Display, Formatter};

#[doc(hidden)]
pub use anyhow::anyhow;

pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct Error(anyhow::Error);

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Error(value)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl std::error::Error for Error {}

#[macro_export]
macro_rules! satex_error {
    ($($tt:tt)*) => {
        $crate::Error::from($crate::anyhow!($($tt)*))
    };
}
