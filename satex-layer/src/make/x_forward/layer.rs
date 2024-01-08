use tower::Layer;

use crate::make::x_forward::{Mode, XForward};

pub struct XForwardLayer {
    mode: Mode,
}

impl XForwardLayer {
    pub fn new(mode: Mode) -> Self {
        Self { mode }
    }
}

impl<S> Layer<S> for XForwardLayer {
    type Service = XForward<S>;

    fn layer(&self, inner: S) -> Self::Service {
        XForward::new(inner, self.mode)
    }
}
