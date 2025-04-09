use rand::{rng, Rng};
use std::hash::Hasher;
use std::sync::atomic::{AtomicU64, Ordering};

pub trait Algorithm {
    fn new() -> Self;
    fn next(&self, key: &[u8]) -> u64;
}

impl<H> Algorithm for H
where
    H: Hasher + Default,
{
    fn new() -> Self {
        Default::default()
    }

    fn next(&self, key: &[u8]) -> u64 {
        let mut hasher = H::default();
        hasher.write(key);
        hasher.finish()
    }
}

pub struct Random;

impl Algorithm for Random {
    fn new() -> Self {
        Self
    }

    fn next(&self, _: &[u8]) -> u64 {
        let mut rng = rng();
        rng.random()
    }
}

pub struct RoundRobin(AtomicU64);

impl Algorithm for RoundRobin {
    fn new() -> Self {
        Self(AtomicU64::new(0))
    }

    fn next(&self, _: &[u8]) -> u64 {
        self.0.fetch_add(1, Ordering::Relaxed)
    }
}
