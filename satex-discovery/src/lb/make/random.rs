use async_trait::async_trait;
use rand::{thread_rng, Rng};

use satex_core::config::args::Args;
use satex_core::endpoint::Endpoint;
use satex_core::Error;

use crate::lb::{Context, LoadBalance, MakeLoadBalance};
use crate::{__make_load_balance, valid_endpoints};

pub struct RandomLoadBalance;

#[async_trait]
impl LoadBalance for RandomLoadBalance {
    async fn choose<'a>(&self, context: Context<'a>) -> Result<Option<Endpoint>, Error> {
        let (mut endpoints, len) = valid_endpoints!(context.endpoints);
        let index = thread_rng().gen_range(0..len);
        Ok(Some(endpoints.remove(index).into()))
    }
}

__make_load_balance!(Random);

fn make(_: Args) -> Result<RandomLoadBalance, Error> {
    Ok(RandomLoadBalance)
}

#[cfg(test)]
mod test {
    use satex_core::config::args::Args;
    use satex_core::essential::Essential;

    use crate::lb::make::new_endpoints;
    use crate::lb::make::random::MakeRandomLoadBalance;
    use crate::lb::{Context, LoadBalance, MakeLoadBalance};

    #[tokio::test]
    async fn test_choose() {
        let args = Args::default();
        let make = MakeRandomLoadBalance::default();
        let lb = make.make(args).unwrap();
        let essential = Essential::default();
        let context = Context::new(&essential, new_endpoints(3000, 3));
        let endpoint = lb.choose(context).await.unwrap();
        assert!(endpoint.is_some());
    }
}
