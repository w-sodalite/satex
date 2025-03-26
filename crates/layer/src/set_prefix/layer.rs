use crate::set_prefix::SetPrefix;
use std::sync::Arc;
use tower::Layer;

#[derive(Debug, Clone)]
pub struct SetPrefixLayer {
    prefix: Arc<str>,
}

impl SetPrefixLayer {
    pub fn new(prefix: String) -> Self {
        Self {
            prefix: Arc::from(prefix),
        }
    }
}

impl<S> Layer<S> for SetPrefixLayer {
    type Service = SetPrefix<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SetPrefix::new(inner, self.prefix.clone())
    }
}
