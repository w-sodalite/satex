use http::Response;
use satex_core::body::Body;
use std::fmt::{Debug, Display, Formatter};
use std::future::{ready, Ready};
use std::task::{Context, Poll};
use tower::Service;

pub struct Identify<F> {
    f: F,
}

impl<F> Identify<F> {
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F, Request> Service<Request> for Identify<F>
where
    F: FnMut(Request) -> bool,
{
    type Response = Response<Body>;
    type Error = Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request) -> Self::Future {
        if (self.f)(request) {
            ready(Ok(Response::new(Body::empty())))
        } else {
            ready(Err(Error))
        }
    }
}

pub struct Error;

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Mistake")
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Mistake")
    }
}

impl std::error::Error for Error {}
