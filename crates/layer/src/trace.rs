use crate::make::MakeRouteLayer;
use satex_core::component::Args;
use satex_core::Error;
use satex_macro::Make;
use tower_http::trace::{HttpMakeClassifier, TraceLayer};

#[derive(Debug, Clone, Copy, Default, Make)]
#[make(name = "Trace")]
pub struct MakeTraceRouteLayer;

impl MakeRouteLayer for MakeTraceRouteLayer {
    type Layer = TraceLayer<HttpMakeClassifier>;

    fn make(&self, _: Args) -> Result<Self::Layer, Error> {
        Ok(TraceLayer::new_for_http())
    }
}
