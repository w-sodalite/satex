use crate::make::MakeRouteLayer;
use satex_core::component::Args;
use satex_core::Error;
use satex_macro::make;
use tower_http::trace::{HttpMakeClassifier, TraceLayer};

#[make(kind = Trace)]
pub struct MakeTraceRouteLayer;

impl MakeRouteLayer for MakeTraceRouteLayer {
    type Layer = TraceLayer<HttpMakeClassifier>;

    fn make(&self, _: Args) -> Result<Self::Layer, Error> {
        Ok(TraceLayer::new_for_http())
    }
}
