use std::net::SocketAddr;
use std::str::FromStr;

use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

use satex_core::config::Config;
use satex_core::Error;

use crate::router::make::MakeRouter;
use crate::serve::Serve;

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn detect() -> Result<Self, Error> {
        Config::detect().map(Self::new)
    }

    pub fn serve(&self) -> Serve {
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

        // 创建路由
        let router = match MakeRouter::make(&self.config) {
            Ok(router) => router,
            Err(e) => panic!("App create the router error: {}", e),
        };

        // 创建服务
        let addr = SocketAddr::from(self.config.server());
        Serve::new(addr, router)
    }
}
