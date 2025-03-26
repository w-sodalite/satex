use std::fmt::{Debug, Display, Formatter};

///
/// satex result
///
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct Error(Box<dyn Display + Send + Sync>);

impl Error {
    pub fn new<E: Display + Send + Sync + 'static>(error: E) -> Self {
        Self(Box::new(error))
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {}
