use crate::health_check::{HealthCheck, HealthStatusObserve};
use crate::Backend;
use async_trait::async_trait;
use http::{HeaderMap, Response};
use satex_core::body::Body;
use satex_core::Error;
use std::time::Duration;

type Validator = Box<dyn Fn(&Response<Body>) -> Result<(), Error> + Send + Sync>;

pub struct HttpHealthCheck {
    path: String,
    headers: HeaderMap,
    validator: Validator,
    connect_timeout: Duration,
    override_port: Option<u16>,
    consecutive_success: usize,
    consecutive_failure: usize,
    status_observe: Option<Box<dyn HealthStatusObserve + Send + Sync>>,
}

impl HttpHealthCheck {
    pub fn new(path: String) -> Self {
        Self {
            path,
            consecutive_success: 1,
            consecutive_failure: 1,
            headers: Default::default(),
            validator: Box::new(|response| {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Err(Error::new(format!(
                        "response status expecting 20x, but it is {}",
                        response.status().as_u16()
                    )))
                }
            }),
            connect_timeout: Duration::from_secs(1),
            override_port: None,
            status_observe: None,
        }
    }

    pub fn with_headers(self, headers: HeaderMap) -> Self {
        Self { headers, ..self }
    }

    pub fn with_validator(
        self,
        validator: impl Fn(&Response<Body>) -> Result<(), Error> + Send + Sync + 'static,
    ) -> Self {
        Self {
            validator: Box::new(validator),
            ..self
        }
    }

    pub fn with_connect_timeout(self, connect_timeout: Duration) -> Self {
        Self {
            connect_timeout,
            ..self
        }
    }

    pub fn with_override_port(self, port: u16) -> Self {
        Self {
            override_port: Some(port),
            ..self
        }
    }

    pub fn with_status_observe(
        self,
        status_observe: impl HealthStatusObserve + Send + Sync + 'static,
    ) -> Self {
        Self {
            status_observe: Some(Box::new(status_observe)),
            ..self
        }
    }

    pub fn with_consecutive_success(self, consecutive_success: usize) -> Self {
        Self {
            consecutive_success,
            ..self
        }
    }

    pub fn with_consecutive_failure(self, consecutive_failure: usize) -> Self {
        Self {
            consecutive_failure,
            ..self
        }
    }
}

#[async_trait]
impl HealthCheck for HttpHealthCheck {
    async fn check(&self, backend: &Backend) -> Result<(), Error> {
        todo!()
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
