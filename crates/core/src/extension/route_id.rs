use crate::new_type;
use std::sync::Arc;

new_type!(
    #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
    RouteId,
    Arc<str>
);
