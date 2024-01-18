use std::str::FromStr;

use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

use satex_core::config::SatexConfig;
use satex_core::Error;

use crate::serve::{Serve, Serves};

pub struct App {
    config: SatexConfig,
}

impl App {
    pub fn new(config: SatexConfig) -> Self {
        Self { config }
    }

    pub fn detect() -> Result<Self, Error> {
        let config = SatexConfig::detect()?;
        Ok(Self::new(config))
    }

    pub fn run(&self) -> Serves {
        // 打印Banner
        let banner = include_str!("../banner.txt");
        println!("{}", banner);

        // 初始化日志
        let logging = self.config.tracing();
        tracing_subscriber::fmt()
            .with_ansi(logging.ansi())
            .with_max_level(Level::from_str(logging.max_level()).unwrap_or(Level::INFO))
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .with_level(logging.level())
            .with_file(logging.file())
            .with_thread_names(logging.thread_names())
            .with_line_number(logging.line_number())
            .init();

        let serves = self
            .config
            .load()
            .expect("Load all serve config error")
            .into_iter()
            .map(|serve_config| Serve::new(serve_config))
            .collect();
        Serves::new(serves)
    }
}
