use std::task::{Context, Poll};

use hyper::header::{HeaderName, HeaderValue};
use hyper::Request;
use serde::Deserialize;
use tower::Service;

pub use make::MakeXForwardLayer;
use satex_core::apply::Apply;
use satex_core::essential::Essential;
use satex_core::satex_error;

mod layer;
mod make;

static X_FORWARDED_FOR: HeaderName = HeaderName::from_static("x-forwarded-for");

const FORWARD_NODE_SEP: &str = " , ";

#[derive(Debug, Copy, Clone, Default, Deserialize)]
pub enum Mode {
    #[default]
    Append,
    Override,
}

#[derive(Debug, Clone)]
pub struct XForward<S> {
    inner: S,
    mode: Mode,
}

impl<S> XForward<S> {
    pub fn new(inner: S, mode: Mode) -> Self {
        Self { inner, mode }
    }
}

impl<S, ReqBody> Service<Request<ReqBody>> for XForward<S>
where
    S: Service<Request<ReqBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        if let Some(addr) = req
            .extensions()
            .get::<Essential>()
            .map(|essential| essential.client_addr)
        {
            let headers = req.headers_mut();
            let value = match (headers.remove(&X_FORWARDED_FOR), self.mode) {
                (Some(value), Mode::Append) => String::from_utf8(value.as_bytes().to_vec())
                    .map_err(|e| satex_error!(e))
                    .map(|value| {
                        value.apply(|value| {
                            value.insert_str(
                                0,
                                &addr
                                    .ip()
                                    .to_string()
                                    .apply(|value| value.push_str(FORWARD_NODE_SEP)),
                            )
                        })
                    })
                    .and_then(|value| HeaderValue::try_from(value).map_err(|e| satex_error!(e))),
                _ => HeaderValue::try_from(addr.ip().to_string()).map_err(|e| satex_error!(e)),
            }
            .expect("Illegal header value!");
            headers.insert(&X_FORWARDED_FOR, value);
        }
        self.inner.call(req)
    }
}
