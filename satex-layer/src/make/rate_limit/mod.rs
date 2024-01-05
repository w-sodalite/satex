use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{ready, Context, Poll};

use bytes::Bytes;
use futures::future::{BoxFuture, Either};
use hyper::{Request, Response, StatusCode};
use leaky_bucket::{AcquireOwned, RateLimiter};
use serde::{Deserialize, Serialize};
use tower::Service;

pub use make::MakeRateLimitLayer;
use satex_core::http::make_response;
use satex_core::http::Body;
use satex_core::BoxError;
use satex_core::{satex_error, Error};

mod layer;
mod make;

enum State {
    Initial,
    Acquire(Pin<Box<AcquireOwned>>),
    Ready,
    Overflow,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Policy {
    Await,
    Break,
}

pub struct RateLimit<S> {
    inner: S,
    limiter: Option<Arc<RateLimiter>>,
    state: State,
    policy: Policy,
}

impl<S> RateLimit<S> {
    pub fn new(inner: S, limiter: Arc<RateLimiter>, policy: Policy) -> Self {
        Self {
            inner,
            limiter: Some(limiter),
            state: State::Initial,
            policy,
        }
    }
}

impl<S> Clone for RateLimit<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            limiter: self.limiter.clone(),
            state: State::Initial,
            policy: self.policy,
        }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for RateLimit<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    S::Error: Into<BoxError>,
    S::Future: Send + 'static,
    ResBody: hyper::body::Body<Data = Bytes> + Send + 'static,
    ResBody::Error: Into<BoxError>,
{
    type Response = Response<Body>;
    type Error = Error;
    type Future = Either<
        Ready<Result<Self::Response, Self::Error>>,
        BoxFuture<'static, Result<Self::Response, Self::Error>>,
    >;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(loop {
            match &mut self.state {
                State::Initial => {
                    let limiter = self
                        .limiter
                        .take()
                        .expect("RateLimit poll ready only call once!");
                    self.state = State::Acquire(Box::pin(limiter.acquire_owned(1)));
                }
                State::Acquire(acquire) => match self.policy {
                    Policy::Await => {
                        ready!(acquire.as_mut().poll(cx));
                        self.state = State::Ready;
                    }
                    Policy::Break => match acquire.as_mut().poll(cx) {
                        Poll::Ready(_) => {
                            self.state = State::Ready;
                        }
                        Poll::Pending => {
                            self.state = State::Overflow;
                        }
                    },
                },
                State::Ready => {
                    break ready!(self.inner.poll_ready(cx)).map_err(|e| satex_error!(e.into()));
                }
                State::Overflow => break Ok(()),
            }
        })
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        match self.state {
            State::Ready => {
                let future = self.inner.call(req);
                Either::Right(Box::pin(async move {
                    match future.await {
                        Ok(response) => Ok(response.map(Body::new)),
                        Err(e) => Err(satex_error!(e.into())),
                    }
                }))
            }
            State::Overflow => Either::Left(ready(Ok(make_response(
                Body::empty(),
                StatusCode::TOO_MANY_REQUESTS,
            )))),
            _ => Either::Left(ready(Ok(make_response(
                Body::empty(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )))),
        }
    }
}
