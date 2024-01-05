use std::collections::HashMap;
use std::time::Duration;

use satex_core::config::args::Args;
use satex_core::config::metadata::Metadata;
use satex_core::endpoint::Endpoint;
use satex_core::satex_error;
use satex_core::Error;

use crate::lb::{MakeLoadBalance, MakeLoadBalanceRegistry, NamedLoadBalance};
use crate::make::builtin::BuiltinDiscovery;
use crate::selector::Selector;
use crate::{MakeServerDiscovery, __discovery};

const DEFAULT_INTERVAL: u64 = 10;

__discovery! {
    Builtin,
    Unsupported,
    server: String,
    uris: Vec<Endpoint>,
    interval: Option<u64>,
    lb: Option<Metadata>,
}

fn make(args: Args) -> Result<BuiltinDiscovery, Error> {
    let configs = match args {
        Args::Shortcut(_) => Err(satex_error!("Shortcut not supported!")),
        Args::Complete(complete) => complete.deserialize::<Vec<Config>>(),
    }?;
    let capacity = configs.len();
    let mut lbs = HashMap::with_capacity(capacity);
    let mut selectors = HashMap::with_capacity(capacity);
    configs.into_iter().try_for_each(|config| {
        match config.lb {
            Some(lb) => {
                MakeLoadBalanceRegistry::get(lb.kind()).and_then(|make| make.make(lb.args()))
            }
            None => Ok(NamedLoadBalance::default()),
        }
        .map(|lb| {
            let server = config.server;
            lbs.insert(server.clone(), lb);
            selectors.insert(
                server.clone(),
                Selector::new(
                    server,
                    config.uris,
                    Duration::from_secs(config.interval.unwrap_or(DEFAULT_INTERVAL)),
                ),
            );
        })
    })?;
    Ok(BuiltinDiscovery::new(selectors, lbs))
}
