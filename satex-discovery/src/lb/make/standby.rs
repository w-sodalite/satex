use async_trait::async_trait;

use satex_core::config::args::Args;
use satex_core::endpoint::Endpoint;
use satex_core::Error;

use crate::lb::{Context, LoadBalance, MakeLoadBalance};
use crate::{__make_load_balance, valid_endpoints};

#[derive(Default)]
pub struct StandbyLoadBalance;

#[async_trait]
impl LoadBalance for StandbyLoadBalance {
    async fn choose<'a>(&self, context: Context<'a>) -> Result<Option<Endpoint>, Error> {
        let (mut endpoints, _) = valid_endpoints!(context.endpoints);
        Ok(Some(endpoints.remove(0).into()))
    }
}

__make_load_balance!(Standby);

fn make(_: Args) -> Result<StandbyLoadBalance, Error> {
    Ok(StandbyLoadBalance)
}

#[cfg(test)]
mod test {
    use satex_core::config::args::Args;
    use satex_core::essential::Essential;

    use crate::lb::make::new_endpoints;
    use crate::lb::make::standby::MakeStandbyLoadBalance;
    use crate::lb::{Context, LoadBalance, MakeLoadBalance};

    #[tokio::test]
    async fn test_choose() {
        let args = Args::default();
        let make = MakeStandbyLoadBalance::default();
        let lb = make.make(args).unwrap();
        let essential = Essential::default();
        let context = Context::new(&essential, new_endpoints(3000, 3));
        let endpoint = lb.choose(context).await.unwrap();
        assert!(endpoint.is_some());
        let endpoint = endpoint.unwrap();
        assert_eq!(endpoint.to_string(), "127.0.0.1:3000")
    }
}
