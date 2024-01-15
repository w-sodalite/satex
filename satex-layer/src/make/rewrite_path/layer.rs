use tower::Layer;

use super::RewritePath;

#[derive(Clone)]
pub struct RewritePathLayer {
    path: String,
}

impl RewritePathLayer {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl<S> Layer<S> for RewritePathLayer {
    type Service = RewritePath<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RewritePath::new(self.path.clone(), inner)
    }
}
