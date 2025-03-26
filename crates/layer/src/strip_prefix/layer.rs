use crate::strip_prefix::StripPrefix;
use tower::Layer;

#[derive(Debug, Clone)]
pub struct StripPrefixRouteLayer {
    level: usize,
}

impl StripPrefixRouteLayer {
    pub fn new(level: usize) -> Self {
        Self { level }
    }
}

impl<S> Layer<S> for StripPrefixRouteLayer {
    type Service = StripPrefix<S>;

    fn layer(&self, inner: S) -> Self::Service {
        StripPrefix {
            inner,
            level: self.level,
        }
    }
}
