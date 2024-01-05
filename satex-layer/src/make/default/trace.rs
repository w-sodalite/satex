use satex_core::{config::Config, Error};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultMakeSpan, TraceLayer},
};

use crate::MakeDefaultRouteServiceLayer;

#[derive(Default)]
pub struct MakeTraceLayer;

impl MakeDefaultRouteServiceLayer for MakeTraceLayer {
    type Layer = TraceLayer<SharedClassifier<ServerErrorsAsFailures>>;

    fn name(&self) -> &'static str {
        "Trace"
    }

    fn make(&self, config: &Config) -> Result<Self::Layer, Error> {
        let trace = config.router().trace();
        Ok(TraceLayer::new_for_http().make_span_with(
            DefaultMakeSpan::default()
                .level(trace.level())
                .include_headers(trace.include_headers()),
        ))
    }
}
