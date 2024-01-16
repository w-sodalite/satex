use satex_core::config::ServeConfig;
use satex_core::{registry, Error};

use crate::make::default::set_discovery::MakeSetDiscoveryLayer;
use crate::make::default::set_http_client::MakeSetHttpClientLayer;
use crate::make::default::trace::MakeTraceLayer;
use crate::make::default::{ArcMakeDefaultRouteServiceLayer, MakeDefaultRouteServiceLayer};
use crate::NamedRouteServiceLayer;

registry!(
    MakeDefaultRouteServiceLayerRegistry,
    ArcMakeDefaultRouteServiceLayer,
    [
        MakeSetDiscoveryLayer,
        MakeSetHttpClientLayer,
        MakeTraceLayer
    ]
);

impl MakeDefaultRouteServiceLayerRegistry {
    pub fn make_all(config: &ServeConfig) -> Result<Vec<NamedRouteServiceLayer>, Error> {
        let makes = Self::all()?;
        makes.values().try_fold(vec![], |layers, make| {
            make.make(config)
                .map(|layer| layers.apply(|layers| layers.push(layer)))
        })
    }
}
