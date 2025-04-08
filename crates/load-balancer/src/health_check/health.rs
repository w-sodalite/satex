use arc_swap::ArcSwap;
use std::sync::Arc;

#[derive(Clone)]
struct HealthInner {
    /// Whether the endpoint is healthy to serve traffic
    healthy: bool,
    /// Whether the endpoint is allowed to serve traffic independent of its health
    enabled: bool,
    /// The counter for stateful transition between healthy and unhealthy.
    /// When [healthy] is true, this counts the number of consecutive health check failures
    /// so that the caller can flip the healthy when a certain threshold is met, and vise versa.
    consecutive_counter: usize,
}

/// Health of backends that can be updated atomically
pub(crate) struct Health(ArcSwap<HealthInner>);

impl Default for Health {
    fn default() -> Self {
        Health(ArcSwap::new(Arc::new(HealthInner {
            healthy: true,
            enabled: true,
            consecutive_counter: 0,
        })))
    }
}

impl Clone for Health {
    fn clone(&self) -> Self {
        let inner = self.0.load_full();
        Health(ArcSwap::new(inner))
    }
}

impl Health {
    pub fn ready(&self) -> bool {
        let h = self.0.load();
        h.healthy && h.enabled
    }

    pub fn enable(&self, enabled: bool) {
        let h = self.0.load();
        if h.enabled != enabled {
            // clone the inner
            let mut new_health = (**h).clone();
            new_health.enabled = enabled;
            self.0.store(Arc::new(new_health));
        };
    }

    // return true when the health is flipped
    pub fn observe_health(&self, health: bool, flip_threshold: usize) -> bool {
        let h = self.0.load();
        let mut flipped = false;
        if h.healthy != health {
            // opposite health observed, ready to increase the counter
            // clone the inner
            let mut new_health = (**h).clone();
            new_health.consecutive_counter += 1;
            if new_health.consecutive_counter >= flip_threshold {
                new_health.healthy = health;
                new_health.consecutive_counter = 0;
                flipped = true;
            }
            self.0.store(Arc::new(new_health));
        } else if h.consecutive_counter > 0 {
            // observing the same health as the current state.
            // reset the counter, if it is non-zero, because it is no longer consecutive
            let mut new_health = (**h).clone();
            new_health.consecutive_counter = 0;
            self.0.store(Arc::new(new_health));
        }
        flipped
    }
}
