pub mod health;
pub mod tcp;

use crate::Backend;
use async_trait::async_trait;
use satex_core::Error;

#[async_trait]
pub trait HealthStatusObserve {
    async fn observe(&self, backend: &Backend, success: bool);
}

#[async_trait]
pub trait HealthCheck {
    async fn check(&self, backend: &Backend) -> Result<(), Error>;
    async fn status_change(&self, backend: &Backend, success: bool);
    fn threshold(&self, success: bool) -> usize;
}

pub struct AlwaysHealthCheck(bool);

impl AlwaysHealthCheck {
    pub fn active() -> Self {
        AlwaysHealthCheck(true)
    }

    pub fn inactive() -> Self {
        AlwaysHealthCheck(false)
    }
}

#[async_trait]
impl HealthCheck for AlwaysHealthCheck {
    async fn check(&self, _backend: &Backend) -> Result<(), Error> {
        if self.0 {
            Ok(())
        } else {
            Err(Error::new("always inactive"))
        }
    }

    async fn status_change(&self, _backend: &Backend, _success: bool) {}

    fn threshold(&self, _success: bool) -> usize {
        1
    }
}
