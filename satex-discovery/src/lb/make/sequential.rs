use std::sync::atomic::{AtomicU64, Ordering};

use async_trait::async_trait;

use satex_core::config::args::Args;
use satex_core::endpoint::Endpoint;
use satex_core::Error;

use crate::lb::{Context, LoadBalance, MakeLoadBalance};
use crate::{__make_load_balance, valid_endpoints};

#[derive(Default)]
pub struct SequentialLoadBalance {
    count: AtomicU64,
}

#[async_trait]
impl LoadBalance for SequentialLoadBalance {
    async fn choose<'a>(&self, context: Context<'a>) -> Result<Option<Endpoint>, Error> {
        let (mut endpoints, len) = valid_endpoints!(context.endpoints);
        let count = self.count.fetch_add(1, Ordering::Acquire);
        let index = (count % len as u64) as usize;
        Ok(Some(endpoints.remove(index).into()))
    }
}

__make_load_balance!(Sequential);

fn make(_: Args) -> Result<SequentialLoadBalance, Error> {
    Ok(SequentialLoadBalance::default())
}
