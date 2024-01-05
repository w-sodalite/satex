use satex_core::registry;

use crate::make::builtin::MakeBuiltinDiscovery;
use crate::{ArcMakeServerDiscovery, MakeServerDiscovery, NamedServerDiscovery};

registry!(
    MakeServerDiscoveryRegistry,
    ArcMakeServerDiscovery,
    NamedServerDiscovery,
    [MakeBuiltinDiscovery]
);
