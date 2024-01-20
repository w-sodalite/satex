use aho_corasick::AhoCorasick;
use hyper::{Request, Uri};
use tower::Service;
use tracing::debug;

pub use make::MakeRewritePathLayer;
use satex_core::essential::{Essential, PathVariables};

mod layer;
mod make;

#[derive(Clone)]
pub struct RewritePath<S> {
    path: String,
    inner: S,
}

impl<S> RewritePath<S> {
    pub fn new(path: String, inner: S) -> Self {
        Self { path, inner }
    }
}

fn make_variable_template(key: &str) -> String {
    let mut variable = String::with_capacity(key.len() + 4);
    variable.push('{');
    variable.push('{');
    variable.push_str(key);
    variable.push('}');
    variable.push('}');
    variable
}

impl<S, ReqBody> Service<Request<ReqBody>> for RewritePath<S>
where
    S: Service<Request<ReqBody>>,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let variables = req
            .extensions()
            .get::<Essential>()
            .and_then(|essential| essential.extensions.get::<PathVariables>());
        let path = match variables {
            None => self.path.clone(),
            Some(variables) => {
                let keys = variables
                    .0
                    .keys()
                    .map(|key| make_variable_template(key))
                    .collect::<Vec<_>>();
                let values = variables.0.values().collect::<Vec<_>>();
                let corasick = AhoCorasick::new(keys).expect("Invalid path variable!");
                corasick.replace_all(&self.path, values.as_slice())
            }
        };
        debug!("Rewrite path: {} => {}", req.uri().path(), path);
        let uri = req.uri();
        let mut builder = Uri::builder();
        if let Some(schema) = uri.scheme_str() {
            builder = builder.scheme(schema);
        }
        if let Some(authority) = uri.authority() {
            builder = builder.authority(authority.as_str());
        }
        let path_and_query = uri
            .query()
            .map(|query| format!("{}?{}", path, query))
            .unwrap_or(path);
        let uri = builder
            .path_and_query(path_and_query)
            .build()
            .expect("build uri error!");
        *req.uri_mut() = uri;
        self.inner.call(req)
    }
}
