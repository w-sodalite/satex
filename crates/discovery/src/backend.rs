use crate::Discovery;
use http::Extensions;
use std::collections::BTreeMap;
use std::future::poll_fn;
use std::net::SocketAddr;
use std::pin::pin;
use std::sync::Arc;
use std::task::Poll;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Backend {
    ///
    /// 后端服务地址
    ///
    pub addr: SocketAddr,

    ///
    /// 后端服务权重,负载均衡算法会使用到该权重值.
    ///
    pub weight: usize,

    ///
    /// 拓展信息
    ///
    pub extension: Extensions,
}

impl Backend {
    pub fn new(addr: SocketAddr) -> Self {
        Self::new_with_weight(addr, 1)
    }

    pub fn new_with_weight(addr: SocketAddr, weight: usize) -> Self {
        Self {
            addr,
            weight,
            extension: Extensions::new(),
        }
    }
}

pub struct Backends {
    backends: Arc<RwLock<BTreeMap<u64, Backend>>>,
}

impl Backends {
    pub fn new<D>(discovery: D) -> Self
    where
        D: Discovery + Send + Sync + 'static,
    {
        Self {
            backends: Default::default(),
        }
    }

    fn start<D>(backends: Arc<RwLock<BTreeMap<u64, Backend>>>, discovery: D)
    where
        D: Discovery + Send + Sync + 'static,
    {
        tokio::spawn(async move {
            poll_fn(move |cx| {
                let mut discovery = pin!(discovery);
                loop {
                    match discovery.as_mut().poll_discover(cx) {
                        Poll::Ready(_) => {}
                        Poll::Pending => {}
                    }
                }
            })
        });
    }
}
