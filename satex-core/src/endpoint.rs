use std::net::{SocketAddr, ToSocketAddrs};
use std::str::FromStr;

use serde::{Deserialize, Deserializer};

use crate::Error;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum Endpoint {
    Ip(SocketAddr),
    Raw(String),
}

impl<'de> Deserialize<'de> for Endpoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        Endpoint::from_str(&text).map_err(serde::de::Error::custom)
    }
}

impl FromStr for Endpoint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<SocketAddr>() {
            Ok(addr) => Ok(Endpoint::Ip(addr)),
            Err(_) => Ok(Endpoint::Raw(s.to_string())),
        }
    }
}

pub enum Iter {
    Option(std::option::IntoIter<SocketAddr>),
    Vec(std::vec::IntoIter<SocketAddr>),
}

impl Iterator for Iter {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Option(iter) => iter.next(),
            Iter::Vec(iter) => iter.next(),
        }
    }
}

impl ToSocketAddrs for Endpoint {
    type Iter = Iter;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        match self {
            Endpoint::Ip(addr) => addr.to_socket_addrs().map(Iter::Option),
            Endpoint::Raw(raw) => raw.to_socket_addrs().map(Iter::Vec),
        }
    }
}
