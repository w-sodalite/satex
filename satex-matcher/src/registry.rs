use satex_core::registry;

use crate::make::header::MakeHeaderMatcher;
use crate::make::host::MakeHostMatcher;
use crate::make::method::MakeMethodMatcher;
use crate::make::path::MakePathMatcher;
use crate::make::query::MakeQueryMatcher;
use crate::make::remote_addr::MakeRemoteAddrMatcher;
use crate::make::time::MakeTimeMatcher;
use crate::{ArcMakeRouteMatcher, MakeRouteMatcher, NamedRouteMatcher};

registry!(
    MakeRouteMatcherRegistry,
    ArcMakeRouteMatcher,
    [
        MakeTimeMatcher,
        MakeHostMatcher,
        MakePathMatcher,
        MakeQueryMatcher,
        MakeMethodMatcher,
        MakeHeaderMatcher,
        MakeRemoteAddrMatcher,
    ]
);

impl MakeRouteMatcherRegistry {
    pub fn make_many(
        items: &[satex_core::config::metadata::Metadata],
    ) -> Result<Vec<NamedRouteMatcher>, ::satex_core::Error> {
        items.iter().try_fold(vec![], |targets, item| {
            Self::get(item.kind())
                .and_then(|make| make.make(item.args()))
                .map(|target| targets.apply(|targets| targets.push(target)))
        })
    }

    pub fn make_single(
        item: &satex_core::config::metadata::Metadata,
    ) -> Result<NamedRouteMatcher, ::satex_core::Error> {
        Self::get(item.kind()).and_then(|make| make.make(item.args()))
    }
}
