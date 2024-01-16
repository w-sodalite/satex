use std::net::SocketAddr;
use std::str::FromStr;

use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

use satex_core::config::SatexConfig;
use satex_core::Error;

use crate::router::make::MakeRouter;
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

    pub fn serve(&self) -> Serves {
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

        let mut serves = vec![];
        let serve_configs = self.config.load().expect("App load serve config error!");
        for serve_config in serve_configs {
            // 创建路由
            let router = match MakeRouter::make(&serve_config) {
                Ok(router) => router,
                Err(e) => panic!("App create the router error: {}", e),
            };

            // 创建服务
            let addr = SocketAddr::from(serve_config.server());
            serves.push(Serve::new(addr, router, serve_config.tls().clone()))
        }
        Serves::new(serves)
    }
}
