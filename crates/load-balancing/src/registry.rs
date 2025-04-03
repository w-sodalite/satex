use crate::LoadBalancer;
use std::collections::HashMap;

#[derive(Default)]
pub struct LoadBalancerRegistry {
    cache: HashMap<String, LoadBalancer>,
}

impl LoadBalancerRegistry {
    pub fn register(&mut self, name: impl ToString, load_balancer: LoadBalancer) {
        self.cache.insert(name.to_string(), load_balancer);
    }

    pub fn get(&self, name: &str) -> Option<&LoadBalancer> {
        self.cache.get(name)
    }
}
