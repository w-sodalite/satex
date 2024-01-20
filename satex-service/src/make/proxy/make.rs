use std::time::Duration;

use hyper_rustls::{ConfigBuilderExt, HttpsConnector};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use rustls::ClientConfig as ClientTlsConfig;
use serde::Deserialize;

use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::proxy::{HttpClient, ProxyService};
use crate::{MakeRouteService, __make_service};

///
/// Keepalive超时时间
///
const DEFAULT_KEEPALIVE: Duration = Duration::from_secs(60);

__make_service! {
    Proxy,
    uri: String,
    #[serde(default)]
    client: ClientConfig
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ClientConfig {
    #[serde(default)]
    pool_max_idle_per_host: Option<usize>,

    #[serde(default)]
    pool_idle_timeout_secs: Option<u64>,

    #[serde(default)]
    retry_canceled_requests: Option<bool>,

    #[serde(default)]
    http09_responses: Option<bool>,

    #[serde(default)]
    set_host: Option<bool>,

    #[serde(default)]
    keepalive_secs: Option<u64>,

    #[serde(default)]
    keepalive_interval_secs: Option<u64>,

    #[serde(default)]
    nodelay: Option<bool>,

    #[serde(default)]
    reuse_address: Option<bool>,

    #[serde(default)]
    connect_timeout_secs: Option<u64>,

    #[serde(default)]
    keepalive_retries: Option<u32>,

    #[serde(default)]
    recv_buffer_size: Option<usize>,

    #[serde(default)]
    send_buffer_size: Option<usize>,

    #[serde(default)]
    happy_eyeballs_timeout_secs: Option<u64>,

    #[serde(default)]
    http1_writev: Option<bool>,

    #[serde(default)]
    http1_max_buf_size: Option<usize>,

    #[serde(default)]
    http1_read_buf_exact_size: Option<usize>,

    #[serde(default)]
    http1_preserve_header_case: Option<bool>,

    #[serde(default)]
    http1_title_case_headers: Option<bool>,

    #[serde(default)]
    http1_allow_obsolete_multiline_headers_in_responses: Option<bool>,

    #[serde(default)]
    http1_allow_spaces_after_header_name_in_responses: Option<bool>,

    #[serde(default)]
    http1_ignore_invalid_headers_in_responses: Option<bool>,

    #[serde(default)]
    http2_only: Option<bool>,

    #[serde(default)]
    http2_keep_alive_timeout_secs: Option<u64>,

    #[serde(default)]
    http2_keep_alive_interval_secs: Option<u64>,

    #[serde(default)]
    http2_keep_alive_while_idle: Option<bool>,

    #[serde(default)]
    http2_adaptive_window: Option<bool>,

    #[serde(default)]
    http2_initial_connection_window_size: Option<u32>,

    #[serde(default)]
    http2_initial_stream_window_size: Option<u32>,

    #[serde(default)]
    http2_max_concurrent_reset_streams: Option<usize>,

    #[serde(default)]
    http2_max_frame_size: Option<u32>,

    #[serde(default)]
    http2_max_send_buf_size: Option<usize>,
}

fn make(args: Args) -> Result<ProxyService, Error> {
    Config::try_from(args).map(|config| ProxyService::new(config.uri, make_client(config.client)))
}

fn make_connector(client_config: &ClientConfig) -> HttpsConnector<HttpConnector> {
    let mut connector = HttpConnector::new();
    connector.set_keepalive(Some(
        client_config
            .keepalive_secs
            .map(Duration::from_secs)
            .unwrap_or(DEFAULT_KEEPALIVE),
    ));
    connector.set_keepalive_interval(
        client_config
            .keepalive_interval_secs
            .map(|secs| Duration::from_secs(secs)),
    );
    connector.set_keepalive_retries(client_config.keepalive_retries);
    connector.set_nodelay(client_config.nodelay.unwrap_or(true));
    connector.set_reuse_address(client_config.reuse_address.unwrap_or(true));
    connector.set_connect_timeout(client_config.connect_timeout_secs.map(Duration::from_secs));
    connector.set_recv_buffer_size(client_config.recv_buffer_size);
    connector.set_send_buffer_size(client_config.send_buffer_size);
    connector.set_happy_eyeballs_timeout(
        client_config
            .happy_eyeballs_timeout_secs
            .map(Duration::from_secs),
    );
    // Client Tls配置
    let tls_config = ClientTlsConfig::builder()
        .with_native_roots()
        .map(|builder| builder.with_no_client_auth())
        .unwrap();
    HttpsConnector::from((connector, tls_config))
}

fn make_client(client_config: ClientConfig) -> HttpClient {
    let connector = make_connector(&client_config);
    let mut builder = Client::builder(TokioExecutor::new());

    // Basic
    if let Some(set_host) = client_config.set_host {
        builder.set_host(set_host);
    }
    if let Some(http09_responses) = client_config.http09_responses {
        builder.http09_responses(http09_responses);
    }
    if let Some(retry_canceled_requests) = client_config.retry_canceled_requests {
        builder.retry_canceled_requests(retry_canceled_requests);
    }
    if let Some(pool_max_idle_per_host) = client_config.pool_max_idle_per_host {
        builder.pool_max_idle_per_host(pool_max_idle_per_host);
    }
    if let Some(pool_idle_timeout_secs) = client_config.pool_idle_timeout_secs {
        builder.pool_idle_timeout(Duration::from_secs(pool_idle_timeout_secs));
    }

    // Http1
    if let Some(http1_writev) = client_config.http1_writev {
        builder.http1_writev(http1_writev);
    }
    if let Some(http1_max_buf_size) = client_config.http1_max_buf_size {
        builder.http1_max_buf_size(http1_max_buf_size);
    }
    if let Some(http1_read_buf_exact_size) = client_config.http1_read_buf_exact_size {
        builder.http1_read_buf_exact_size(http1_read_buf_exact_size);
    }
    if let Some(http1_preserve_header_case) = client_config.http1_preserve_header_case {
        builder.http1_preserve_header_case(http1_preserve_header_case);
    }
    if let Some(http1_title_case_headers) = client_config.http1_title_case_headers {
        builder.http1_title_case_headers(http1_title_case_headers);
    }
    if let Some(http1_allow_obsolete_multiline_headers_in_responses) =
        client_config.http1_allow_obsolete_multiline_headers_in_responses
    {
        builder.http1_allow_obsolete_multiline_headers_in_responses(
            http1_allow_obsolete_multiline_headers_in_responses,
        );
    }
    if let Some(http1_allow_spaces_after_header_name_in_responses) =
        client_config.http1_allow_spaces_after_header_name_in_responses
    {
        builder.http1_allow_spaces_after_header_name_in_responses(
            http1_allow_spaces_after_header_name_in_responses,
        );
    }
    if let Some(http1_ignore_invalid_headers_in_responses) =
        client_config.http1_ignore_invalid_headers_in_responses
    {
        builder
            .http1_ignore_invalid_headers_in_responses(http1_ignore_invalid_headers_in_responses);
    }

    // Http2
    if let Some(http2_only) = client_config.http2_only {
        builder.http2_only(http2_only);
    }
    builder.http2_keep_alive_timeout(
        client_config
            .http2_keep_alive_timeout_secs
            .map(Duration::from_secs)
            .unwrap_or(DEFAULT_KEEPALIVE),
    );
    if let Some(http2_keep_alive_interval_secs) = client_config.http2_keep_alive_interval_secs {
        builder.http2_keep_alive_interval(Duration::from_secs(http2_keep_alive_interval_secs));
    }
    if let Some(http2_keep_alive_while_idle) = client_config.http2_keep_alive_while_idle {
        builder.http2_keep_alive_while_idle(http2_keep_alive_while_idle);
    }
    if let Some(http2_adaptive_window) = client_config.http2_adaptive_window {
        builder.http2_adaptive_window(http2_adaptive_window);
    }
    if let Some(http2_initial_connection_window_size) =
        client_config.http2_initial_connection_window_size
    {
        builder.http2_initial_connection_window_size(http2_initial_connection_window_size);
    }
    if let Some(http2_initial_stream_window_size) = client_config.http2_initial_stream_window_size {
        builder.http2_initial_stream_window_size(http2_initial_stream_window_size);
    }
    if let Some(http2_max_concurrent_reset_streams) =
        client_config.http2_max_concurrent_reset_streams
    {
        builder.http2_max_concurrent_reset_streams(http2_max_concurrent_reset_streams);
    }
    if let Some(http2_max_frame_size) = client_config.http2_max_frame_size {
        builder.http2_max_frame_size(http2_max_frame_size);
    }
    if let Some(http2_max_send_buf_size) = client_config.http2_max_send_buf_size {
        builder.http2_max_send_buf_size(http2_max_send_buf_size);
    }

    builder.build(connector)
}
