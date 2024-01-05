use tower::Layer;

use crate::make::path_strip::PathStrip;

#[derive(Default, Debug, Clone, Copy)]
pub struct PathStripLayer {
    level: usize,
}

impl PathStripLayer {
    pub fn new(level: usize) -> Self {
        Self { level }
    }
}

impl<S> Layer<S> for PathStripLayer {
    type Service = PathStrip<S>;

    fn layer(&self, inner: S) -> Self::Service {
        PathStrip::new(inner, self.level)
    }
}
