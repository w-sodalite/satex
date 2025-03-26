mod layer;
mod make;

use futures::{
    future::{Either, LocalBoxFuture},
    TryFutureExt,
};
use http::{Request, Uri};
use satex_core::{BoxError, Error};
use std::future::ready;
use std::{future::Ready, str::FromStr};
use tower::Service;
use tracing::debug;

pub use layer::*;
pub use make::*;

#[derive(Debug, Clone)]
pub struct StripPrefix<S> {
    inner: S,
    level: usize,
}

impl<S, ReqBody> Service<Request<ReqBody>> for StripPrefix<S>
where
    S: Service<Request<ReqBody>>,
    S::Error: Into<BoxError>,
    S::Future: 'static,
{
    type Response = S::Response;

    type Error = Error;

    type Future = Either<
        Ready<Result<Self::Response, Self::Error>>,
        LocalBoxFuture<'static, Result<Self::Response, Self::Error>>,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|e| Error::new(e.into()))
    }

    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        if self.level > 0 {
            let (mut parts, body) = request.into_parts();
            let uri = parts.uri;
            match strip_prefix(uri, self.level) {
                Ok(uri) => {
                    parts.uri = uri;
                    let request = Request::from_parts(parts, body);
                    Either::Right(Box::pin(
                        self.inner.call(request).map_err(|e| Error::new(e.into())),
                    ))
                }
                Err(e) => Either::Left(ready(Err(e))),
            }
        } else {
            Either::Right(Box::pin(
                self.inner.call(request).map_err(|e| Error::new(e.into())),
            ))
        }
    }
}

fn strip_prefix(uri: Uri, level: usize) -> Result<Uri, Error> {
    const SEP: &str = "/";
    let query = uri
        .query()
        .map(|query| format!("?{}", query))
        .unwrap_or_default();

    let path = uri
        .path()
        .split(SEP)
        .filter(|x| !x.is_empty())
        .skip(level)
        .collect::<Vec<_>>()
        .join(SEP);

    let path_and_query = if path.is_empty() {
        format!("{}{}", SEP, query)
    } else {
        format!("{}{}{}", SEP, path, query)
    };

    match Uri::from_str(&path_and_query) {
        Ok(new_uri) => {
            debug!(
                "Strip path prefix with level ({}): {} => {}",
                level, uri, new_uri
            );
            Ok(new_uri)
        }
        Err(e) => Err(Error::new(e)),
    }
}
