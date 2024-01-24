use async_trait::async_trait;
use rand::{thread_rng, Rng};

use satex_core::config::args::Args;
use satex_core::endpoint::Endpoint;
use satex_core::essential::Essential;
use satex_core::Error;

use crate::lb::make::{make_load_balance, valid_endpoints};
use crate::lb::{LoadBalance, MakeLoadBalance};
use crate::selector::SortedEndpoint;

pub struct RandomLoadBalance;

#[async_trait]
impl LoadBalance for RandomLoadBalance {
    async fn choose(
        &self,
        _: &Essential,
        endpoints: Vec<SortedEndpoint>,
    ) -> Result<Option<Endpoint>, Error> {
        let (mut endpoints, len) = valid_endpoints!(endpoints);
        let index = thread_rng().gen_range(0..len);
        Ok(Some(endpoints.remove(index).into()))
    }
}

make_load_balance!(Random);

fn make(_: Args) -> Result<RandomLoadBalance, Error> {
    Ok(RandomLoadBalance)
}

#[cfg(test)]
mod test {
    use satex_core::config::args::Args;
    use satex_core::essential::Essential;

    use crate::lb::make::new_sorted_endpoints;
    use crate::lb::make::random::MakeRandomLoadBalance;
    use crate::lb::{LoadBalance, MakeLoadBalance};

    #[tokio::test]
    async fn test_choose() {
        let args = Args::default();
        let make = MakeRandomLoadBalance::default();
        let lb = make.make(args).unwrap();
        let endpoint = lb
            .choose(&Essential::default(), new_sorted_endpoints(3))
            .await
            .unwrap();
        assert!(endpoint.is_some());
    }
}
