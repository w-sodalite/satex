use crate::load_balancer::LoadBalancer;
use async_trait::async_trait;
use satex_core::background::BackgroundTask;
use std::time::{Duration, Instant};
use tracing::error;

#[async_trait]
impl BackgroundTask for LoadBalancer {
    async fn run(&self) {
        // 136 years
        const NEVER: Duration = Duration::from_secs(u32::MAX as u64);
        let mut now = Instant::now();
        // run update and health check once
        let mut next_update = now;
        let mut next_health_check = now;
        loop {
            if next_update <= now {
                if let Err(e) = self.update().await {
                    error!("load balancer update error: {}", e);
                }
                next_update = now + self.update_frequency.unwrap_or(NEVER);
            }

            if next_health_check <= now {
                self.backends
                    .run_health_check(self.health_check_parallel)
                    .await;
                next_health_check = now + self.health_check_frequency.unwrap_or(NEVER);
            }

            if self.update_frequency.is_none() && self.health_check_frequency.is_none() {
                return;
            }
            let to_wake = std::cmp::min(next_update, next_health_check);
            tokio::time::sleep_until(to_wake.into()).await;
            now = Instant::now();
        }
    }
}
