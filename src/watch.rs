use crate::config::Config;
use crate::make_router::MakeRouter;
use crate::registry::Registry;
use async_stream::stream;
use futures::Stream;
use satex_core::Error;
use satex_server::router::Event;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tokio::fs::metadata;
use tokio::spawn;
use tokio::sync::mpsc::{channel, Sender};
use tokio::time::sleep;
use tracing::error;

pub struct ConfigFileWatchEvents;

impl ConfigFileWatchEvents {
    pub fn events(
        registry: Registry,
        file: PathBuf,
        interval: Duration,
    ) -> impl Stream<Item=Event> {
        let (tx, mut rx) = channel(1024);

        // spawn watch task
        spawn(async move {
            if let Err(e) = watch(tx, registry, file, interval).await {
                error!("Watch config file error: {}", e);
            }
        });

        stream! {
            while let Some(event) = rx.recv().await {
                yield event;
            }
        }
    }
}

async fn watch(
    tx: Sender<Event>,
    registry: Registry,
    file: impl AsRef<Path>,
    interval: Duration,
) -> Result<(), Error> {
    let make_router = MakeRouter::new(registry);
    let mut modified = get_modified(&file).await?;
    loop {
        sleep(interval).await;
        let last_modified = get_modified(&file).await?;
        if last_modified > modified {
            modified = last_modified;
            let config = Config::from_yaml(&file)?;
            let router = make_router.make(&config)?;
            tx.send(Event::Set(router)).await.map_err(Error::new)?;
        }
    }
}

#[inline]
async fn get_modified(path: impl AsRef<Path>) -> Result<SystemTime, Error> {
    metadata(path)
        .await
        .and_then(|m| m.modified())
        .map_err(Error::new)
}
