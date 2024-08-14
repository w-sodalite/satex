use std::str::FromStr;
use std::task::{Context, Poll};

use hyper::http::uri::PathAndQuery;
use hyper::{Request, Uri};
use qstring::QString;
use tower::Service;

use satex_core::{satex_error, Error};

use crate::make::set_mode::SetMode;

mod layer;
mod make;

pub struct SetQuery<S> {
    inner: S,
    name: String,
    value: String,
    mode: SetMode,
}

impl<S> SetQuery<S> {
    pub fn new(inner: S, name: String, value: String, mode: SetMode) -> Self {
        Self {
            inner,
            name,
            value,
            mode,
        }
    }
}

impl<S, ReqBody> Service<Request<ReqBody>> for SetQuery<S>
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
        set_uri_query(req.uri_mut(), &self.name, &self.value, self.mode)
            .expect("Set uri query failed!");
        self.inner.call(req)
    }
}

fn set_uri_query(uri: &mut Uri, name: &str, value: &str, mode: SetMode) -> Result<(), Error> {
    let query = uri.query().unwrap_or("");
    let mut params = QString::from(query);
    match mode {
        SetMode::Append => match params.get(name) {
            None => {
                params.add_pair((name, value));
            }
            Some(x) => {
                params.add_pair((name, format!("{},{}", x, value)));
            }
        },
        SetMode::IfNotPresent => {
            if params.get(name).is_none() {
                params.add_pair((name, value));
            }
        }
        SetMode::Override => {
            if params.get(name).is_some() {
                let mut pairs = params
                    .to_pairs()
                    .into_iter()
                    .filter(|(k, _)| *k == name)
                    .collect::<Vec<_>>();
                pairs.push((name, value));
                params = QString::new(pairs);
            } else {
                params.add_pair((name, value));
            }
        }
    }
    let path_and_query = PathAndQuery::from_str(&format!("{}?{}", uri.path(), params))
        .map_err(|e| satex_error!(e))?;
    let mut builder = Uri::builder().path_and_query(path_and_query);
    if let Some(schema) = uri.scheme_str() {
        builder = builder.scheme(schema);
    }
    if let Some(authority) = uri.authority() {
        builder = builder.scheme(authority.as_str());
    }
    *uri = builder.build().map_err(|e| satex_error!(e))?;
    Ok(())
}
