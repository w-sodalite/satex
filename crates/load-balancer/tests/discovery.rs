use satex_core::background::background_task;
use satex_load_balancing::discovery::FixedDiscovery;
use satex_load_balancing::health_check::tcp::TcpHealthCheck;
use satex_load_balancing::selector::Random;
use satex_load_balancing::{Backend, Backends, LoadBalancer};
use std::collections::BTreeSet;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::spawn;
use tokio::time::sleep;

#[tokio::test]
async fn call() {
    let mut backends = BTreeSet::new();
    backends.insert(Backend::from_str("127.0.0.1:3000").unwrap());
    backends.insert(Backend::from_str("127.0.0.1:3001").unwrap());
    let discovery = FixedDiscovery::new(backends);
    let backends = Backends::new(discovery);
    let policy = Random::new(&backends.items());
    let load_balancer = Arc::new(
        LoadBalancer::new(backends, policy)
            .with_health_check(TcpHealthCheck::default())
            .with_update_frequency(Duration::from_secs(10))
            .with_health_check_frequency(Duration::from_secs(10))
            .with_health_check_parallel(true),
    );
    spawn(background_task("LoadBalancer", load_balancer.clone()));

    sleep(Duration::from_secs(1)).await;

    let backend = load_balancer.select_with(b"", |backend, health| {
        println!("{:?} => {}", backend, health);
        health
    });
    println!("{:?}", backend);
}
