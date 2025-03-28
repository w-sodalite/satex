use crate::make::MakeRouteLayer;
use crate::remove_header::Removable;
use http::HeaderName;
use pin_project_lite::pin_project;
use satex_core::component::Args;
use satex_core::{component::Configurable, Error};
use satex_macro::make;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
use std::{str::FromStr, sync::Arc};
use tower::{Layer, Service};

#[make(kind = RemoveResponseHeader)]
pub struct MakeRemoveResponseHeaderRouteLayer {
    name: String,
}

impl MakeRouteLayer for MakeRemoveResponseHeaderRouteLayer {
    type Layer = RemoveResponseHeaderLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        Config::with_args(args).and_then(|config| {
            HeaderName::from_str(&config.name)
                .map_err(Error::new)
                .map(RemoveResponseHeaderLayer::new)
        })
    }
}

#[derive(Debug, Clone)]
pub struct RemoveResponseHeaderLayer {
    name: Arc<HeaderName>,
}

impl RemoveResponseHeaderLayer {
    pub fn new(name: HeaderName) -> Self {
        Self {
            name: Arc::new(name),
        }
    }
}

impl<S> Layer<S> for RemoveResponseHeaderLayer {
    type Service = RemoveResponseHeader<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RemoveResponseHeader {
            name: self.name.clone(),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RemoveResponseHeader<S> {
    inner: S,
    name: Arc<HeaderName>,
}

impl<S, Req, Res> Service<Req> for RemoveResponseHeader<S>
where
    S: Service<Req, Response=Res>,
    Res: Removable,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Req) -> Self::Future {
        let name = self.name.clone();
        ResponseFuture::new(name, self.inner.call(request))
    }
}

pin_project! {
    #[project = ResponseFutureProj]
    #[project_replace = MapProjReplace]
    pub enum ResponseFuture<F> {
        Incomplete {
            name: Arc<HeaderName>,
            #[pin]
            future: F,
        },
        Complete,
    }
}

impl<F> ResponseFuture<F> {
    pub fn new(name: Arc<HeaderName>, future: F) -> Self {
        Self::Incomplete { name, future }
    }
}

impl<F, Res, E> Future for ResponseFuture<F>
where
    F: Future<Output=Result<Res, E>>,
    Res: Removable,
{
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.as_mut().project() {
            ResponseFutureProj::Incomplete { future, .. } => match ready!(future.poll(cx)) {
                Ok(mut res) => match self.project_replace(ResponseFuture::Complete) {
                    MapProjReplace::Incomplete { name, .. } => {
                        res.remove(name.as_ref());
                        Poll::Ready(Ok(res))
                    }
                    MapProjReplace::Complete => unreachable!(),
                },
                Err(e) => Poll::Ready(Err(e)),
            },
            ResponseFutureProj::Complete { .. } => {
                panic!("polled after completion")
            }
        }
    }
}
