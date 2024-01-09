use hyper::{Response, StatusCode};

pub use body::Body;

mod body;

///
/// 根据指定的包体和状态码创建一个[Response]
///
/// # Arguments
///
/// * `body` ：HTTP包体
/// * `status`：HTTP响应状态码
///
/// # Examples
///
/// ```
/// use hyper::StatusCode;
/// use satex_core::http::make_response;
/// let body = String::from("Hello World!");
/// let response = make_response(body,StatusCode::OK);
/// assert_eq!(response.status(),StatusCode::OK);
/// ```
///
pub fn make_response(body: impl Into<Body>, status: StatusCode) -> Response<Body> {
    let mut response = Response::new(body.into());
    *response.status_mut() = status;
    response
}
