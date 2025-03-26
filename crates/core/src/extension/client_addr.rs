use crate::new_type;
use std::net::SocketAddr;

new_type!(
    #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    ClientAddr,
    SocketAddr
);
