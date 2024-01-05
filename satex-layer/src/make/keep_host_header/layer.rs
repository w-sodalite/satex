use tower::Layer;

use crate::make::keep_host_header::KeepHostHeader;

#[derive(Clone, Debug, Default)]
pub struct KeepHostHeaderLayer;

impl<S> Layer<S> for KeepHostHeaderLayer {
    type Service = KeepHostHeader<S>;

    fn layer(&self, inner: S) -> Self::Service {
        KeepHostHeader::new(inner)
    }
}
