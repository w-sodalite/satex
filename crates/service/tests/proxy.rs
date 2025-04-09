use http::{Extensions, Method, Request, Response, StatusCode};
use http_body_util::BodyExt;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use hyper_util::server::conn::auto::Builder;
use satex_core::body::Body;
use satex_core::component::Args;
use satex_core::digest::DefaultDigester;
use satex_core::executor::SpawnLocalExecutor;
use satex_service::make::MakeRouteService;
use satex_service::proxy::{MakeProxyRouteService, ProxyRouteService};
use serde_yaml::Value;
use std::convert::Infallible;
use std::net::Ipv4Addr;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::task::{JoinHandle, LocalSet, spawn_local};
use tokio::time::sleep;
use tower::Service;

fn start_server() -> JoinHandle<()> {
    spawn_local(async {
        let listener = TcpListener::bind((Ipv4Addr::new(0, 0, 0, 0), 34567))
            .await
            .unwrap();
        let builder = Builder::new(SpawnLocalExecutor::new());
        while let Ok((stream, _)) = listener.accept().await {
            builder
                .serve_connection_with_upgrades(
                    TokioIo::new(stream),
                    service_fn(|_| async move {
                        Ok::<_, Infallible>(Response::new(Body::from("Hello World!")))
                    }),
                )
                .await
                .unwrap();
        }
    })
}

async fn call(mut service: ProxyRouteService<DefaultDigester>) {
    let request = Request::builder()
        .uri("/")
        .method(Method::GET)
        .body(Body::empty())
        .unwrap();

    let response = service.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap();
    let bytes = body.to_bytes();
    assert_eq!(&bytes[..], b"Hello World!")
}

#[tokio::test]
async fn make_with_shortcut() {
    let ls = LocalSet::new();
    ls.spawn_local(async move {
        start_server();
        sleep(Duration::from_secs(1)).await;

        let service = MakeProxyRouteService
            .make(Args::Shortcut(Some("http://127.0.0.1:34567/")), &Extensions::default())
            .unwrap();
        call(service).await
    });
}

#[tokio::test]
async fn make_with_full() {
    let ls = LocalSet::new();
    ls.spawn_local(async move {
        start_server();
        sleep(Duration::from_secs(1)).await;

        let value = serde_yaml::from_str::<Value>(r#"uri: http://127.0.0.1:34567"#).unwrap();
        let args = Args::Full(&value);
        let service = MakeProxyRouteService.make(args, &Extensions::default()).unwrap();
        call(service).await
    });
}
