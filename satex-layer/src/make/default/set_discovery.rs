use tower_http::add_extension::AddExtensionLayer;

use satex_core::config::Config;
use satex_core::Error;
use satex_discovery::{MakeServerDiscovery, MakeServerDiscoveryRegistry, NamedServerDiscovery};

use crate::make::default::MakeDefaultRouteServiceLayer;

#[derive(Default)]
pub struct MakeSetDiscoveryLayer;

impl MakeDefaultRouteServiceLayer for MakeSetDiscoveryLayer {
    type Layer = AddExtensionLayer<NamedServerDiscovery>;

    fn name(&self) -> &'static str {
        "SetDiscovery"
    }

    fn make(&self, config: &Config) -> Result<Self::Layer, Error> {
        let mut discoveries = vec![];
        for metadata in config.discovery() {
            let kind = metadata.kind();
            let make = MakeServerDiscoveryRegistry::get(kind)?;
            let discovery = make.make(metadata.args())?;
            discoveries.push(discovery);
        }
        Ok(AddExtensionLayer::new(NamedServerDiscovery::composite(
            discoveries,
        )))
    }
}
