use crate::{ArcRouteMatcher, RouteMatcher};
use satex_core::component::Args;
use satex_core::make::Make;
use satex_core::util::try_downcast;
use satex_core::Error;
use std::sync::Arc;

pub trait MakeRouteMatcher: Make {
    ///
    /// 路由Matcher类型
    ///
    type Matcher: RouteMatcher;

    ///
    /// 根据参数创建路由Matcher
    ///
    /// # Arguments
    ///
    /// * `args`: 路由Matcher参数
    ///
    /// returns: Result<Self::Matcher, Error>
    ///
    fn make(&self, args: Args) -> Result<Self::Matcher, Error>;
}

#[derive(Clone)]
pub struct ArcMakeRouteMatcher(Arc<dyn MakeRouteMatcher<Matcher=ArcRouteMatcher> + Send + Sync>);

impl ArcMakeRouteMatcher {
    pub fn new<M>(make: M) -> Self
    where
        M: MakeRouteMatcher + Send + Sync + 'static,
        M::Matcher: RouteMatcher + Send + Sync + 'static,
    {
        try_downcast::<ArcMakeRouteMatcher, _>(make)
            .unwrap_or_else(|make| Self(Arc::new(Map(make))))
    }
}

impl Make for ArcMakeRouteMatcher {
    fn name(&self) -> &'static str {
        self.0.name()
    }
}

impl MakeRouteMatcher for ArcMakeRouteMatcher {
    type Matcher = ArcRouteMatcher;

    fn make(&self, args: Args) -> Result<Self::Matcher, Error> {
        self.0.make(args)
    }
}

struct Map<M>(M);

impl<M> Make for Map<M>
where
    M: MakeRouteMatcher,
    M::Matcher: RouteMatcher,
{
    fn name(&self) -> &'static str {
        self.0.name()
    }
}

impl<M> MakeRouteMatcher for Map<M>
where
    M: MakeRouteMatcher,
    M::Matcher: RouteMatcher + Send + Sync + 'static,
{
    type Matcher = ArcRouteMatcher;

    fn make(&self, args: Args) -> Result<Self::Matcher, Error> {
        self.0.make(args).map(ArcRouteMatcher::new)
    }
}
