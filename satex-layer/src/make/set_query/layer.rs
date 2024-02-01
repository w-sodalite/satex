use tower::Layer;

use crate::make::set_mode::SetMode;
use crate::make::set_query::SetQuery;

pub struct SetQueryLayer {
    name: String,
    value: String,
    mode: SetMode,
}

impl SetQueryLayer {
    pub fn new(name: String, value: String, mode: SetMode) -> Self {
        Self { name, value, mode }
    }
}

impl<S> Layer<S> for SetQueryLayer {
    type Service = SetQuery<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SetQuery::new(
            inner,
            self.name.to_string(),
            self.value.to_string(),
            self.mode,
        )
    }
}
