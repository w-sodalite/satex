use async_trait::async_trait;

use satex_core::config::args::Args;
use satex_core::endpoint::Endpoint;
use satex_core::Error;

use crate::lb::{Context, LoadBalance, MakeLoadBalance};
use crate::{__load_balance, valid_endpoints};

#[derive(Default)]
pub struct StandbyLoadBalance;

#[async_trait]
impl LoadBalance for StandbyLoadBalance {
    async fn choose<'a>(&self, context: Context<'a>) -> Result<Option<Endpoint>, Error> {
        let (mut endpoints, _) = valid_endpoints!(context.endpoints);
        Ok(Some(endpoints.remove(0).into()))
    }
}

__load_balance!(Standby);

fn make(_: Args) -> Result<StandbyLoadBalance, Error> {
    Ok(StandbyLoadBalance)
}
