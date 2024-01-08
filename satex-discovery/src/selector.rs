use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use tokio::net::TcpStream;
use tokio::sync::Notify;
use tokio::time::sleep;
use tracing::{debug, warn};

use satex_core::apply::Apply;
use satex_core::endpoint::Endpoint;
use satex_core::task::spawn;
use satex_core::{satex_error, Error};

///
/// Status: 初始化
///
const STATUS_INITIAL: u8 = 0;

///
/// Status: 启动中
///
const STATUS_STARTING: u8 = 1;

///
/// Status: 启动完成
///
const STATUS_STARTED: u8 = 2;

///
/// Status: 已关闭
///
const STATUS_SHUTDOWN: u8 = 3;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct IndexedEndpoint {
    index: usize,
    endpoint: Endpoint,
}

impl IndexedEndpoint {
    pub fn new(index: usize, endpoint: Endpoint) -> Self {
        Self { index, endpoint }
    }

    pub fn split(self) -> (usize, Endpoint) {
        (self.index, self.endpoint)
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn endpoint(&self) -> &Endpoint {
        &self.endpoint
    }
}

impl From<IndexedEndpoint> for Endpoint {
    fn from(value: IndexedEndpoint) -> Self {
        value.endpoint
    }
}

#[derive(Debug, Clone)]
pub struct Selector {
    ///
    /// 选择器对应的服务名
    ///
    server: String,

    ///
    /// 当前选择器包含的所有地址集合
    ///
    endpoints: Arc<Vec<Endpoint>>,

    ///
    /// 有效的地址下标集合，这里使用同步的[`RwLock`]是因为即使出现竞争情况，等待的时间也非常少，所以不使用异步的[`RwLock`]。
    ///
    actives: Arc<RwLock<Vec<IndexedEndpoint>>>,

    ///
    /// 选择器检测[`Endpoint`]是否有效的间隔时间
    ///
    interval: Duration,

    ///
    /// 选择器运行状态：
    ///
    /// - [`STATUS_INITIAL`]
    /// - [`STATUS_STARTING`]
    /// - [`STATUS_STARTED`]
    /// - [`STATUS_SHUTDOWN`]
    status: Arc<AtomicU8>,

    ///
    /// 选择器启动完成后通知事件句柄
    ///
    notify: Arc<Notify>,
}

impl Display for Selector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Selector(server={})", self.server)
    }
}

impl Selector {
    ///
    /// 创建一个新的选择器
    ///
    /// # Arguments
    ///
    /// * `server`: 服务名
    /// * `endpoints`: 服务对应的地址集合
    /// * `interval`: 选择器检测时间间隔
    ///
    /// returns: Selector
    ///
    pub fn new(server: String, endpoints: Vec<Endpoint>, interval: Duration) -> Self {
        Selector {
            server,
            endpoints: Arc::new(endpoints),
            actives: Arc::new(RwLock::new(Vec::default())),
            interval,
            status: Arc::new(AtomicU8::new(0)),
            notify: Arc::new(Notify::new()),
        }
        .apply(|selector| selector.start())
    }

    ///
    ///
    /// 尝试升级当前状态到下一个状态，返回是否升级成功。
    ///
    /// # Arguments
    ///
    /// * `status`: 选择器状态
    /// * `current`: 选择器当前状态
    ///
    /// returns: bool
    ///
    fn try_upgrade_status(status: &AtomicU8, current: u8) -> bool {
        status
            .compare_exchange(current, current + 1, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    ///
    /// 启动选择器
    ///
    /// 选择器启动时会将逐步提升当前的状态，初始状态为[`STATUS_INITIAL`]。检测任务第一次开始执行时会将状态提升为[`STATUS_STARTING`]。第一次检测任务完成后
    /// 会将状态提升为[`STATUS_STARTED`]，此后会一直保持该状态。当调用[`Self::shutdown`]方法或选择器被[`Self::drop`]时，状态会提升为[`STATUS_SHUTDOWN`]，至此整个生命周期结束。
    ///
    fn start(&mut self) {
        if Self::try_upgrade_status(&self.status, STATUS_INITIAL) {
            let server = self.server.clone();
            let endpoints = self.endpoints.clone();
            let actives = self.actives.clone();
            let status = self.status.clone();
            let interval = self.interval;
            let notify = self.notify.clone();
            spawn(async move {
                loop {
                    let mut snapshots = Vec::with_capacity(endpoints.len());
                    for (index, endpoint) in endpoints.iter().enumerate() {
                        let stream = match endpoint {
                            Endpoint::Ip(addr) => TcpStream::connect(addr).await,
                            Endpoint::Domain(raw) => TcpStream::connect(raw.as_str()).await,
                        };
                        match stream {
                            Ok(_) => {
                                debug!(
                                    "Selector(server={}) validate endpoint [{:?}] status is valid!",
                                    server, endpoint
                                );
                                snapshots.push(IndexedEndpoint::new(index, endpoint.clone()));
                            }
                            Err(e) => {
                                warn!(
                                "Selector(server={}) validate endpoint [{:?}] status is invalid, connect error: {}",
                                server, endpoint, e
                            );
                            }
                        }
                    }

                    // 更新有效的节点下标
                    {
                        let mut actives = actives.write().expect(&format!(
                            "Selector(server={}) get write lock error!",
                            server
                        ));
                        *actives = snapshots;
                    }

                    // 提升当前状态：`STATUS_STARTING` -> `STATUS_STARTED`
                    if Self::try_upgrade_status(&status, STATUS_STARTING) {
                        debug!("Selector(server={}) start success!", server);
                        // notify resolve waiter
                        notify.notify_waiters();
                    }

                    sleep(interval).await;

                    // 如果当前状态不是`STATUS_STARTED`，则取消任务。
                    if status.load(Ordering::Relaxed) != STATUS_STARTED {
                        break;
                    }
                }
            });
        }
    }

    pub async fn select(&self) -> Result<Vec<IndexedEndpoint>, Error> {
        if self.status.load(Ordering::Relaxed) != 2 {
            self.notify.notified().await;
        }
        let actives = {
            self.actives
                .read()
                .map_err(|e| satex_error!("{} get write lock error: {}", self, e))?
                .clone()
        };
        Ok(actives)
    }
}

impl Drop for Selector {
    fn drop(&mut self) {
        self.status.store(STATUS_SHUTDOWN, Ordering::Relaxed);
    }
}
