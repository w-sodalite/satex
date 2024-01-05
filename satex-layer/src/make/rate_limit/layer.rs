use std::sync::Arc;
use std::time::Duration;

use leaky_bucket::RateLimiter;
use tower::Layer;

use crate::make::rate_limit::{Policy, RateLimit};

#[derive(Clone)]
pub struct RateLimitLayer {
    limiter: Arc<RateLimiter>,
    policy: Policy,
}

impl RateLimitLayer {
    pub fn new(policy: Policy, max: usize, refill: usize, interval: Duration, fair: bool) -> Self {
        let limiter = RateLimiter::builder()
            .initial(max)
            .max(max)
            .refill(refill)
            .interval(interval)
            .fair(fair)
            .build();
        Self {
            policy,
            limiter: Arc::new(limiter),
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimit<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimit::new(inner, self.limiter.clone(), self.policy)
    }
}
