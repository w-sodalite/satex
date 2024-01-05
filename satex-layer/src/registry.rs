use satex_core::registry;

use crate::make::concurrency_limit::MakeConcurrencyLimitLayer;
use crate::make::cors::MakeCorsLayer;
use crate::make::keep_host_header::MakeKeepHostHeaderLayer;
use crate::make::path_strip::MakePathStripLayer;
use crate::make::rate_limit::MakeRateLimitLayer;
use crate::make::request_body_limit::MakeRequestBodyLimitLayer;
use crate::make::set_header::{MakeSetRequestHeaderLayer, MakeSetResponseHeaderLayer};
use crate::make::set_status::MakeSetStatusLayer;
use crate::{ArcMakeRouteServiceLayer, MakeRouteServiceLayer, NamedRouteServiceLayer};

registry!(
    MakeRouteServiceLayerRegistry,
    ArcMakeRouteServiceLayer,
    NamedRouteServiceLayer,
    [
        MakeCorsLayer,
        MakeSetStatusLayer,
        MakePathStripLayer,
        MakeRateLimitLayer,
        MakeKeepHostHeaderLayer,
        MakeConcurrencyLimitLayer,
        MakeSetRequestHeaderLayer,
        MakeRequestBodyLimitLayer,
        MakeSetResponseHeaderLayer,
    ]
);