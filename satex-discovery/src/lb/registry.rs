use satex_core::registry;

use crate::lb::make::ip_hash::MakeIpHashLoadBalance;
use crate::lb::make::random::MakeRandomLoadBalance;
use crate::lb::make::sequential::MakeSequentialLoadBalance;
use crate::lb::make::standby::MakeStandbyLoadBalance;
use crate::lb::make::weight::MakeWeightLoadBalance;
use crate::lb::make::ArcMakeLoadBalance;
use crate::lb::make::MakeLoadBalance;
use crate::lb::NamedLoadBalance;

registry!(
    MakeLoadBalanceRegistry,
    ArcMakeLoadBalance,
    NamedLoadBalance,
    [
        MakeWeightLoadBalance,
        MakeRandomLoadBalance,
        MakeSequentialLoadBalance,
        MakeStandbyLoadBalance,
        MakeIpHashLoadBalance
    ]
);
