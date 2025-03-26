use hyper_rustls::{ConfigBuilderExt, HttpsConnector};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Builder;
use rustls::ClientConfig as ClientTlsConfig;
use satex_core::body::Body;
use satex_core::executor::SpawnLocalExecutor;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub(crate) type Client = hyper_util::client::legacy::Client<HttpsConnector<HttpConnector>, Body>;

impl From<ClientConfig> for Client {
    fn from(config: ClientConfig) -> Self {
        let mut builder = Builder::new(SpawnLocalExecutor::new());

        // basic
        if let Some(set_host) = config.set_host {
            builder.set_host(set_host);
        }
        if let Some(http09_responses) = config.http09_responses {
            builder.http09_responses(http09_responses);
        }
        if let Some(retry_canceled_requests) = config.retry_canceled_requests {
            builder.retry_canceled_requests(retry_canceled_requests);
        }
        if let Some(pool_max_idle_per_host) = config.pool_max_idle_per_host {
            builder.pool_max_idle_per_host(pool_max_idle_per_host);
        }
        if let Some(pool_idle_timeout_secs) = config.pool_idle_timeout_secs {
            builder.pool_idle_timeout(Duration::from_secs(pool_idle_timeout_secs));
        }

        // http1
        if let Some(http1_writev) = config.http1_writev {
            builder.http1_writev(http1_writev);
        }
        if let Some(http1_max_buf_size) = config.http1_max_buf_size {
            builder.http1_max_buf_size(http1_max_buf_size);
        }
        if let Some(http1_read_buf_exact_size) = config.http1_read_buf_exact_size {
            builder.http1_read_buf_exact_size(http1_read_buf_exact_size);
        }
        if let Some(http1_preserve_header_case) = config.http1_preserve_header_case {
            builder.http1_preserve_header_case(http1_preserve_header_case);
        }
        if let Some(http1_title_case_headers) = config.http1_title_case_headers {
            builder.http1_title_case_headers(http1_title_case_headers);
        }
        if let Some(http1_allow_obsolete_multiline_headers_in_responses) =
            config.http1_allow_obsolete_multiline_headers_in_responses
        {
            builder.http1_allow_obsolete_multiline_headers_in_responses(
                http1_allow_obsolete_multiline_headers_in_responses,
            );
        }
        if let Some(http1_allow_spaces_after_header_name_in_responses) =
            config.http1_allow_spaces_after_header_name_in_responses
        {
            builder.http1_allow_spaces_after_header_name_in_responses(
                http1_allow_spaces_after_header_name_in_responses,
            );
        }
        if let Some(http1_ignore_invalid_headers_in_responses) =
            config.http1_ignore_invalid_headers_in_responses
        {
            builder.http1_ignore_invalid_headers_in_responses(
                http1_ignore_invalid_headers_in_responses,
            );
        }

        // http2
        if let Some(http2_only) = config.http2_only {
            builder.http2_only(http2_only);
        }
        if let Some(http2_keep_alive_timeout_secs) = config.http2_keep_alive_timeout_secs {
            builder.http2_keep_alive_timeout(Duration::from_secs(http2_keep_alive_timeout_secs));
        }
        if let Some(http2_keep_alive_interval_secs) = config.http2_keep_alive_interval_secs {
            builder.http2_keep_alive_interval(Duration::from_secs(http2_keep_alive_interval_secs));
        }
        if let Some(http2_keep_alive_while_idle) = config.http2_keep_alive_while_idle {
            builder.http2_keep_alive_while_idle(http2_keep_alive_while_idle);
        }
        if let Some(http2_adaptive_window) = config.http2_adaptive_window {
            builder.http2_adaptive_window(http2_adaptive_window);
        }
        if let Some(http2_initial_connection_window_size) =
            config.http2_initial_connection_window_size
        {
            builder.http2_initial_connection_window_size(http2_initial_connection_window_size);
        }
        if let Some(http2_initial_stream_window_size) = config.http2_initial_stream_window_size {
            builder.http2_initial_stream_window_size(http2_initial_stream_window_size);
        }
        if let Some(http2_max_concurrent_reset_streams) = config.http2_max_concurrent_reset_streams
        {
            builder.http2_max_concurrent_reset_streams(http2_max_concurrent_reset_streams);
        }
        if let Some(http2_max_frame_size) = config.http2_max_frame_size {
            builder.http2_max_frame_size(http2_max_frame_size);
        }
        if let Some(http2_max_send_buf_size) = config.http2_max_send_buf_size {
            builder.http2_max_send_buf_size(http2_max_send_buf_size);
        }

        builder.build(connector(&config))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

fn connector(config: &ClientConfig) -> HttpsConnector<HttpConnector> {
    // http connector 配置
    let mut connector = HttpConnector::new();
    connector.enforce_http(false);
    connector.set_keepalive(config.keepalive_secs.map(Duration::from_secs));
    connector.set_keepalive_interval(config.keepalive_interval_secs.map(Duration::from_secs));
    connector.set_keepalive_retries(config.keepalive_retries);
    connector.set_nodelay(config.nodelay.unwrap_or(true));
    connector.set_reuse_address(config.reuse_address.unwrap_or(true));
    connector.set_connect_timeout(config.connect_timeout_secs.map(Duration::from_secs));
    connector.set_recv_buffer_size(config.recv_buffer_size);
    connector.set_send_buffer_size(config.send_buffer_size);
    connector
        .set_happy_eyeballs_timeout(config.happy_eyeballs_timeout_secs.map(Duration::from_secs));

    // http tls 配置
    let tls_config = ClientTlsConfig::builder()
        .with_native_roots()
        .map(|builder| builder.with_no_client_auth())
        .expect("TLS config create with native roots error!");

    // https connector 配置
    HttpsConnector::<HttpConnector>::builder()
        .with_tls_config(tls_config)
        .https_or_http()
        .enable_all_versions()
        .wrap_connector(connector)
}
