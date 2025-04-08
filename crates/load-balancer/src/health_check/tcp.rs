use crate::health_check::{HealthCheck, HealthStatusObserve};
use crate::Backend;
use async_trait::async_trait;
use satex_core::Error;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

pub struct TcpHealthCheck {
    connect_timeout: Duration,
    consecutive_success: usize,
    consecutive_failure: usize,
    status_observe: Option<Box<dyn HealthStatusObserve + Send + Sync>>,
}

impl TcpHealthCheck {
    pub fn with_connect_timeout(mut self, connect_timeout: Duration) -> Self {
        self.connect_timeout = connect_timeout;
        self
    }

    pub fn with_consecutive_success(mut self, consecutive_success: usize) -> Self {
        self.consecutive_success = consecutive_success;
        self
    }

    pub fn with_consecutive_failure(mut self, consecutive_failure: usize) -> Self {
        self.consecutive_failure = consecutive_failure;
        self
    }

    pub fn with_status_observe(
        mut self,
        status_observe: impl HealthStatusObserve + Send + Sync + 'static,
    ) -> Self {
        self.status_observe = Some(Box::new(status_observe));
        self
    }
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
