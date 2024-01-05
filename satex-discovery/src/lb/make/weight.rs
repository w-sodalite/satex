use async_trait::async_trait;
use rand::{thread_rng, Rng};

use satex_core::apply::Apply;
use satex_core::config::args::Args;
use satex_core::endpoint::Endpoint;
use satex_core::{satex_error, Error};

use crate::lb::{Context, LoadBalance, MakeLoadBalance};
use crate::{__load_balance, valid_endpoints};

pub struct WeightLoadBalance {
    ratios: Vec<usize>,
}

impl WeightLoadBalance {
    pub fn new(ratios: Vec<usize>) -> Self {
        Self { ratios }
    }
}

#[async_trait]
impl LoadBalance for WeightLoadBalance {
    async fn choose<'a>(&self, context: Context<'a>) -> Result<Option<Endpoint>, Error> {
        let (mut endpoints, _) = valid_endpoints!(context.endpoints);
        let mut ratios = vec![];
        let mut sum = 0;
        for endpoint in endpoints.iter() {
            let ratio = self
                .ratios
                .get(endpoint.index())
                .copied()
                .ok_or_else(|| satex_error!("Invalid endpoint index: {}", endpoint.index()))?;
            ratios.push(ratio);
            sum += ratio;
        }
        for (index, ratio) in ratios.into_iter().enumerate() {
            if thread_rng().gen_ratio(ratio as u32, sum as u32) {
                return Ok(Some(endpoints.remove(index).into()));
            } else {
                sum -= ratio;
            }
        }
        unreachable!()
    }
}

__load_balance!(Weight);

fn make(args: Args) -> Result<WeightLoadBalance, Error> {
    match args {
        Args::Shortcut(shortcut) => shortcut.into_iter().try_fold(vec![], |ratios, ratio| {
            ratio
                .parse::<usize>()
                .map(|ratio| ratios.apply(|ratios| ratios.push(ratio)))
                .map_err(|e| satex_error!(e))
        }),
        Args::Complete(complete) => complete.deserialize::<Vec<usize>>(),
    }
    .map(WeightLoadBalance::new)
}
