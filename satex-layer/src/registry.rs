use satex_core::registry;

use crate::make::compression::MakeCompressionLayer;
use crate::make::concurrency_limit::MakeConcurrencyLimitLayer;
use crate::make::cors::MakeCorsLayer;
use crate::make::keep_host_header::MakeKeepHostHeaderLayer;
use crate::make::path_strip::MakePathStripLayer;
use crate::make::rate_limit::MakeRateLimitLayer;
use crate::make::request_body_limit::MakeRequestBodyLimitLayer;
use crate::make::rewrite_path::MakeRewritePathLayer;
use crate::make::set_header::{MakeSetRequestHeaderLayer, MakeSetResponseHeaderLayer};
use crate::make::set_status::MakeSetStatusLayer;
use crate::make::x_forward::MakeXForwardLayer;
use crate::{ArcMakeRouteServiceLayer, MakeRouteServiceLayer, NamedRouteServiceLayer};

registry!(
    MakeRouteServiceLayerRegistry,
    ArcMakeRouteServiceLayer,
    NamedRouteServiceLayer,
    [
        MakeCorsLayer,
        MakeXForwardLayer,
        MakeSetStatusLayer,
        MakePathStripLayer,
        MakeRateLimitLayer,
        MakeRewritePathLayer,
        MakeCompressionLayer,
        MakeKeepHostHeaderLayer,
        MakeConcurrencyLimitLayer,
        MakeSetRequestHeaderLayer,
        MakeRequestBodyLimitLayer,
        MakeSetResponseHeaderLayer,
    ]
);
