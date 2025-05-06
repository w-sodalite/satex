use crate::config::Config;
use crate::make_router::MakeRouter;
use crate::registry::Registry;
use futures::stream::Empty;
use futures::Stream;
use satex_core::Error;
use satex_server::router::{Event, MakeRouterService};
use satex_server::Server;
use std::net::SocketAddr;
use tokio::spawn;
use tracing::Level;

type Unit = Empty<Event>;

pub struct App<S = Unit> {
    name: String,
    config: Config,
    registry: Registry,
    events: Option<S>,
}

impl App {
    pub fn new(name: impl Into<String>, config: Config, registry: Registry) -> Self {
        Self {
            name: name.into(),
            config,
            registry,
            events: None,
        }
    }

    pub fn with_events<S>(self, events: S) -> App<S> {
        App {
            name: self.name,
            config: self.config,
            registry: self.registry,
            events: Some(events),
        }
    }
}

impl<S> App<S>
where
    S: Stream<Item=Event> + Send + 'static,
{
    pub async fn run(self) -> Result<(), Error> {
        let App {
            name,
            config,
            registry,
            events,
            ..
        } = self;

        // 初始化Tracing
        setup(&config);

        // 创建路由
        let make_router = MakeRouter::new(registry);
        let router = make_router.make(&config)?;
        let make_service = match events {
            Some(events) => router.into_dynamic_service(events, |future| {
                spawn(future);
            }),
            None => router.into_static_service(),
        };

        // 启动服务
        let addr = SocketAddr::new(config.server.host, config.server.port);
        serve(&config, make_service)
            .bind(name, addr)?
            .await
            .map_err(Error::new)
    }
}

fn setup(config: &Config) {
    let tracing = &config.tracing;
    tracing_subscriber::fmt()
        .with_max_level(Level::from(tracing.max_level))
        .with_ansi(tracing.display_ansi)
        .with_file(tracing.display_file)
        .with_thread_names(tracing.display_thread_names)
        .with_thread_ids(tracing.display_thread_ids)
        .with_line_number(tracing.display_line_number)
        .with_level(tracing.display_level)
        .with_target(tracing.display_target)
        .init();
}

fn serve(config: &Config, make_service: MakeRouterService) -> Server<MakeRouterService> {
    let mut builder = Server::builder();
    if let Some(workers) = config.server.workers {
        builder = builder.workers(workers);
    }
    if let Some(max_concurrent_connections) = config.server.max_concurrent_connections {
        builder = builder.max_concurrent_connections(max_concurrent_connections);
    }
    if let Some(backlog) = config.server.backlog {
        builder = builder.backlog(backlog);
    }
    if config.server.tls.enabled {
        let mut builder = builder
            .tls()
            .alpn_protocols(&config.server.tls.alpn_protocols);
        if let Some(certs) = &config.server.tls.certs {
            builder = builder.certs(certs);
        }
        if let Some(private_key) = &config.server.tls.private_key {
            builder = builder.private_key(private_key);
        }
        builder.make_service(make_service)
    } else {
        builder.make_service(make_service)
    }
}
