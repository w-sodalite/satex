use async_trait::async_trait;

use satex_core::config::args::Args;
use satex_core::endpoint::Endpoint;
use satex_core::essential::Essential;
use satex_core::Error;

use crate::lb::make::{make_load_balance, valid_endpoints};
use crate::lb::{LoadBalance, MakeLoadBalance};
use crate::selector::SortedEndpoint;

#[derive(Default)]
pub struct StandbyLoadBalance;

#[async_trait]
impl LoadBalance for StandbyLoadBalance {
    async fn choose(
        &self,
        _: &Essential,
        endpoints: Vec<SortedEndpoint>,
    ) -> Result<Option<Endpoint>, Error> {
        let (mut endpoints, _) = valid_endpoints!(endpoints);
        Ok(Some(endpoints.remove(0).into()))
    }
}

make_load_balance!(Standby);

fn make(_: Args) -> Result<StandbyLoadBalance, Error> {
    Ok(StandbyLoadBalance)
}

#[cfg(test)]
mod test {
    use satex_core::config::args::Args;
    use satex_core::essential::Essential;

    use crate::lb::make::new_sorted_endpoints;
    use crate::lb::make::standby::MakeStandbyLoadBalance;
    use crate::lb::{LoadBalance, MakeLoadBalance};

    #[tokio::test]
    async fn test_choose() {
        let args = Args::default();
        let make = MakeStandbyLoadBalance::default();
        let lb = make.make(args).unwrap();
        let endpoint = lb
            .choose(&Essential::default(), new_sorted_endpoints(3))
            .await
            .unwrap();
        assert!(endpoint.is_some());
        let endpoint = endpoint.unwrap();
        assert_eq!(endpoint.to_string(), "127.0.0.1:3000")
    }
}
