#![doc = include_str!("../README.md")]

mod app;
pub use app::App;

pub mod config;
pub mod make_router;
pub mod registry;
pub mod watch;
