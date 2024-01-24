use async_trait::async_trait;
use rand::{thread_rng, Rng};

use satex_core::apply::Apply;
use satex_core::config::args::Args;
use satex_core::endpoint::Endpoint;
use satex_core::essential::Essential;
use satex_core::{satex_error, Error};

use crate::lb::make::{make_load_balance, valid_endpoints};
use crate::lb::{LoadBalance, MakeLoadBalance};
use crate::selector::SortedEndpoint;

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
    async fn choose(
        &self,
        _: &Essential,
        endpoints: Vec<SortedEndpoint>,
    ) -> Result<Option<Endpoint>, Error> {
        let (mut endpoints, _) = valid_endpoints!(endpoints);
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
        let mut enumerate = ratios.into_iter().enumerate();
        loop {
            if let Some((index, ratio)) = enumerate.next() {
                if ratio == sum {
                    break Ok(Some(endpoints.remove(index).into()));
                }
                if thread_rng().gen_ratio(ratio as u32, sum as u32) {
                    break Ok(Some(endpoints.remove(index).into()));
                } else {
                    sum -= ratio;
                }
            }
        }
    }
}

make_load_balance!(Weight);

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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use satex_core::config::args::{Args, Shortcut};
    use satex_core::endpoint::Endpoint;
    use satex_core::essential::Essential;

    use crate::lb::make::new_sorted_endpoints;
    use crate::lb::make::weight::MakeWeightLoadBalance;
    use crate::lb::{LoadBalance, MakeLoadBalance};

    #[tokio::test]
    async fn test_choose() {
        let args = Args::Shortcut(Shortcut::from("1,1,2"));
        let make = MakeWeightLoadBalance::default();
        let lb = make.make(args).unwrap();
        let endpoints = new_sorted_endpoints(3);
        let size = 10000;
        let mut targets = vec![];
        for _ in 0..size {
            targets.push(
                lb.choose(&Essential::default(), endpoints.clone())
                    .await
                    .unwrap(),
            );
        }
        let counter = targets.into_iter().flatten().collect::<Counter>();
        println!("Sum: {}", counter.sum);
        counter
            .endpoints
            .into_iter()
            .for_each(|(endpoint, (count, ratio))| {
                println!("{}: {},{}", endpoint, count, ratio);
            });
    }

    #[derive(Debug)]
    struct Counter {
        sum: usize,
        endpoints: HashMap<Endpoint, (usize, f32)>,
    }

    impl FromIterator<Endpoint> for Counter {
        fn from_iter<T: IntoIterator<Item = Endpoint>>(iter: T) -> Self {
            let mut endpoints = HashMap::new();
            for endpoint in iter {
                let (count, _) = endpoints.entry(endpoint).or_insert((0, 0.));
                *count = *count + 1;
            }
            let sum = endpoints
                .iter()
                .map(|(_, (count, _))| *count)
                .sum::<usize>();
            endpoints
                .iter_mut()
                .for_each(|(_, (count, ratio))| *ratio = (*count as f32) / (sum as f32));
            Self { sum, endpoints }
        }
    }
}
