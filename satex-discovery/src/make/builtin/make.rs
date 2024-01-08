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
    let mut items = HashMap::with_capacity(capacity);
    configs.into_iter().try_for_each(|config| {
        match config.lb {
            Some(lb) => {
                MakeLoadBalanceRegistry::get(lb.kind()).and_then(|make| make.make(lb.args()))
            }
            None => Ok(NamedLoadBalance::default()),
        }
        .map(|lb| {
            let server = config.server;
            items.insert(
                server.clone(),
                (
                    Selector::new(
                        server,
                        config.uris,
                        Duration::from_secs(config.interval.unwrap_or(DEFAULT_INTERVAL)),
                    ),
                    lb,
                ),
            );
        })
    })?;
    Ok(BuiltinDiscovery::new(items))
}

#[cfg(test)]
mod test {
    use std::net::SocketAddr;
    use std::str::FromStr;

    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;
    use tokio::spawn;
    use tokio::sync::oneshot::{channel, Sender};

    use satex_core::config::metadata::Metadata;
    use satex_core::essential::Essential;

    use crate::make::builtin::MakeBuiltinDiscovery;
    use crate::{MakeServerDiscovery, ServerDiscovery};

    #[tokio::test]
    async fn test_make() {
        async fn start_server(tx: Sender<()>) {
            let addr = "127.0.0.1:3000".parse::<SocketAddr>().unwrap();
            let listener = TcpListener::bind(addr).await.unwrap();
            spawn(async move {
                tx.send(()).unwrap();
                loop {
                    let (mut stream, _) = listener.accept().await.unwrap();
                    stream.write(b"").await.unwrap();
                    break;
                }
            });
        }
        let (tx, rx) = channel();
        start_server(tx).await;
        rx.await.unwrap();

        let yaml = r#"
                          kind: Builtin
                          args:
                            - server: server1
                              uris:
                                - 127.0.0.1:3000
                              interval: 10
                              lb: Random
                        "#;
        let metadata = Metadata::from_str(yaml).unwrap();
        let args = metadata.args();
        let make = MakeBuiltinDiscovery::default();
        let discovery = make.make(args).unwrap();
        let endpoint = discovery
            .resolve(&Essential::default(), "server1")
            .await
            .unwrap();
        assert!(endpoint.is_some());
    }
}
