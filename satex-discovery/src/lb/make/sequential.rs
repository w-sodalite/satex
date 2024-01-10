use std::sync::atomic::{AtomicU64, Ordering};

use async_trait::async_trait;

use satex_core::config::args::Args;
use satex_core::endpoint::Endpoint;
use satex_core::essential::Essential;
use satex_core::Error;

use crate::lb::{LoadBalance, MakeLoadBalance};
use crate::selector::SortedEndpoint;
use crate::{__make_load_balance, valid_endpoints};

#[derive(Default)]
pub struct SequentialLoadBalance {
    count: AtomicU64,
}

#[async_trait]
impl LoadBalance for SequentialLoadBalance {
    async fn choose(
        &self,
        _: &Essential,
        endpoints: Vec<SortedEndpoint>,
    ) -> Result<Option<Endpoint>, Error> {
        let (mut endpoints, len) = valid_endpoints!(endpoints);
        let count = self.count.fetch_add(1, Ordering::Acquire);
        let index = (count % len as u64) as usize;
        Ok(Some(endpoints.remove(index).into()))
    }
}

__make_load_balance!(Sequential);

fn make(_: Args) -> Result<SequentialLoadBalance, Error> {
    Ok(SequentialLoadBalance::default())
}

#[cfg(test)]
mod test {
    use std::net::{Ipv4Addr, SocketAddr};

    use satex_core::config::args::Args;
    use satex_core::endpoint::Endpoint;
    use satex_core::essential::Essential;

    use crate::lb::make::new_sorted_endpoints;
    use crate::lb::make::sequential::MakeSequentialLoadBalance;
    use crate::lb::{LoadBalance, MakeLoadBalance};

    #[tokio::test]
    async fn test_choose() {
        let args = Args::default();
        let make = MakeSequentialLoadBalance::default();
        let lb = make.make(args).unwrap();
        let endpoints = new_sorted_endpoints(3);
        let endpoint = lb
            .choose(&Essential::default(), endpoints.clone())
            .await
            .unwrap();
        assert!(
            matches!(endpoint, Some(Endpoint::Ip(SocketAddr::V4(v4))) if *v4.ip() == Ipv4Addr::LOCALHOST && v4.port()==3000)
        );

        let endpoint = lb
            .choose(&Essential::default(), endpoints.clone())
            .await
            .unwrap();
        assert!(
            matches!(endpoint, Some(Endpoint::Ip(SocketAddr::V4(v4))) if *v4.ip() == Ipv4Addr::LOCALHOST && v4.port()==3001)
        );

        let endpoint = lb
            .choose(&Essential::default(), endpoints.clone())
            .await
            .unwrap();
        assert!(
            matches!(endpoint, Some(Endpoint::Ip(SocketAddr::V4(v4))) if *v4.ip() == Ipv4Addr::LOCALHOST && v4.port()==3002)
        );
    }
}
