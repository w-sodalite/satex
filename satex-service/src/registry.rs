use satex_core::registry;

use crate::make::echo::MakeEchoService;
use crate::make::proxy::MakeProxyService;
use crate::make::r#static::MakeStaticService;
use crate::{ArcMakeRouteService, MakeRouteService, NamedRouteService};

registry!(
    MakeRouteServiceRegistry,
    ArcMakeRouteService,
    NamedRouteService,
    [MakeEchoService, MakeProxyService, MakeStaticService,]
);
