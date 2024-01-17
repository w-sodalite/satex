use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::net::IpAddr;
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use tokio::spawn;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tracing::debug;

use satex_core::config::args::Args;
use satex_core::endpoint::Endpoint;
use satex_core::essential::Essential;
use satex_core::Error;

use crate::lb::make::MakeLoadBalance;
use crate::lb::LoadBalance;
use crate::selector::SortedEndpoint;
use crate::{__make_load_balance, valid_endpoints};

const DEFAULT_TIMEOUT_SECS: u64 = 1800;
const DEFAULT_INTERVAL_SECS: u64 = 10;

type ArcTable = Arc<RwLock<HashMap<IpAddr, (SortedEndpoint, Instant)>>>;

type WeakTable = Weak<RwLock<HashMap<IpAddr, (SortedEndpoint, Instant)>>>;

pub struct IpHashLoadBalance {
    table: ArcTable,
    timeout: Duration,
}

impl IpHashLoadBalance {
    pub fn new(timeout: Duration, interval: Duration) -> Self {
        let table = Arc::new(RwLock::new(HashMap::new()));
        Self::start_clean(Arc::downgrade(&table), timeout, interval);
        Self { table, timeout }
    }

    fn start_clean(tables: WeakTable, timeout: Duration, interval: Duration) {
        ///
        /// 获取所有的已经过期的节点
        ///
        #[inline]
        async fn get_expired_ips(tables: &ArcTable, timeout: Duration) -> HashSet<IpAddr> {
            tables
                .read()
                .await
                .iter()
                .filter(|(_, (_, instant))| instant.elapsed().gt(&timeout))
                .map(|(addr, _)| addr)
                .copied()
                .collect::<HashSet<_>>()
        }

        ///
        /// 删除所有的已经过期的节点
        ///
        #[inline]
        async fn remove_expired_ips(tables: &ArcTable, ips: HashSet<IpAddr>) {
            if !ips.is_empty() {
                let mut tables = tables.write().await;
                debug!("IpHash remove expired cache: {:?}", ips);
                ips.iter().for_each(|ip| {
                    tables.remove(ip);
                });
            }
        }

        // 启动定时清理任务
        spawn(async move {
            while let Some(tables) = tables.upgrade() {
                // get all expired ip collection
                let ips = get_expired_ips(&tables, timeout).await;

                // remove all expired ip
                remove_expired_ips(&tables, ips).await;

                // sleep interval
                sleep(interval).await;
            }
        });
    }
}

#[async_trait]
impl LoadBalance for IpHashLoadBalance {
    async fn choose(
        &self,
        essential: &Essential,
        mut endpoints: Vec<SortedEndpoint>,
    ) -> Result<Option<Endpoint>, Error> {
        let ip = essential.client_addr.ip();
        let mut table = self.table.write().await;
        let mut cached = table.get_mut(&ip);
        loop {
            match cached {
                Some((endpoint, instant)) => {
                    let index = endpoints
                        .iter()
                        .enumerate()
                        .filter(|(_, item)| *item == endpoint)
                        .map(|(index, _)| index)
                        .next();
                    match index {
                        Some(index) => {
                            if instant.elapsed().lt(&self.timeout) {
                                *instant = Instant::now();
                                return Ok(Some(endpoints.remove(index).into()));
                            } else {
                                table.remove(&ip);
                                cached = None;
                            }
                        }
                        None => {
                            cached = None;
                        }
                    }
                }
                None => {
                    let (mut endpoints, len) = valid_endpoints!(endpoints);
                    let mut hasher = DefaultHasher::new();
                    ip.hash(&mut hasher);
                    let hash = hasher.finish();
                    let index = (hash % len as u64) as usize;
                    return Ok(Some(endpoints.remove(index).into()));
                }
            }
        }
    }
}

__make_load_balance! {
    IpHash,
    #[serde(
        deserialize_with = "satex_core::serde::tot::as_u64",
        default = "Config::default_timeout"
    )]
    timeout: u64,

    #[serde(
        deserialize_with = "satex_core::serde::tot::as_u64",
        default = "Config::default_interval"
    )]
    interval: u64,
}

impl Config {
    pub fn new(timeout: u64, interval: u64) -> Self {
        Self { timeout, interval }
    }

    pub fn default_timeout() -> u64 {
        DEFAULT_TIMEOUT_SECS
    }

    pub fn default_interval() -> u64 {
        DEFAULT_INTERVAL_SECS
    }
}

fn make(args: Args) -> Result<IpHashLoadBalance, Error> {
    Config::try_from(args).map(|config: Config| {
        IpHashLoadBalance::new(
            Duration::from_secs(config.timeout),
            Duration::from_secs(config.interval),
        )
    })
}

#[cfg(test)]
mod test {
    use satex_core::config::args::{Args, Shortcut};
    use satex_core::essential::Essential;

    use crate::lb::make::ip_hash::MakeIpHashLoadBalance;
    use crate::lb::make::new_sorted_endpoints;
    use crate::lb::{LoadBalance, MakeLoadBalance};

    #[tokio::test]
    async fn test_choose() {
        let args = Args::Shortcut(Shortcut::from("30,10"));
        let make = MakeIpHashLoadBalance::default();
        let lb = make.make(args).unwrap();
        let endpoints = new_sorted_endpoints(8);
        let e1 = lb
            .choose(&Essential::default(), endpoints.clone())
            .await
            .unwrap();
        let e2 = lb.choose(&Essential::default(), endpoints).await.unwrap();
        assert!(matches!((e1, e2), (Some(e1), Some(e2)) if e1==e2))
    }
}
