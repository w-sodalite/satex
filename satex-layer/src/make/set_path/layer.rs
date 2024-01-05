use tower::Layer;

use super::SetPath;

#[derive(Clone)]
pub struct SetPathLayer {
    path: String,
}

impl SetPathLayer {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl<S> Layer<S> for SetPathLayer {
    type Service = SetPath<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SetPath::new(self.path.clone(), inner)
    }
}
