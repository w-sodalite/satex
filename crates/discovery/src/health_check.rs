use crate::backend::Backend;
use async_trait::async_trait;
use satex_core::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

pub type BoxHealthStatusObserve = Box<dyn HealthStatusObserve + Send + Sync>;

#[async_trait]
pub trait HealthStatusObserve {
    async fn observe(&self, backend: &Backend, success: bool);
}

pub type ArcHealthCheck = Arc<dyn HealthCheck + Send + Sync>;

#[async_trait]
pub trait HealthCheck {
    async fn check(&self, backend: &Backend) -> Result<(), Error>;
    async fn status_change(&self, backend: &Backend, success: bool);
    fn threshold(&self, success: bool) -> usize;
}

pub struct AlwaysActiveHealthCheck;

#[async_trait]
impl HealthCheck for AlwaysActiveHealthCheck {
    async fn check(&self, _backend: &Backend) -> Result<(), Error> {
        Ok(())
    }

    async fn status_change(&self, _backend: &Backend, _success: bool) {}

    fn threshold(&self, _success: bool) -> usize {
        1
    }
}

pub struct TcpHealthCheck {
    pub connect_timeout: Duration,
    pub consecutive_success: usize,
    pub consecutive_failure: usize,
    pub status_observe: Option<BoxHealthStatusObserve>,
}

impl Default for TcpHealthCheck {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(1),
            consecutive_success: 1,
            consecutive_failure: 1,
            status_observe: None,
        }
    }
}

#[async_trait]
impl HealthCheck for TcpHealthCheck {
    async fn check(&self, backend: &Backend) -> Result<(), Error> {
        timeout(self.connect_timeout, TcpStream::connect(backend.addr))
            .await
            .map(|_| ())
            .map_err(Error::new)
    }

    async fn status_change(&self, backend: &Backend, success: bool) {
        if let Some(status_observe) = &self.status_observe {
            status_observe.observe(backend, success).await;
        }
    }

    fn threshold(&self, success: bool) -> usize {
        if success {
            self.consecutive_success
        } else {
            self.consecutive_failure
        }
    }
}
