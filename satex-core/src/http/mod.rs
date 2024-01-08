use hyper::{Response, StatusCode};

pub use body::Body;

mod body;

///
/// # Arguments
///
/// * `body` ：HTTP包体
/// * `status`：HTTP响应状态码
///
/// # Examples
///
/// ```
/// let body = String::from("Hello World!");
/// let response = make_response(body,StatusCode::OK);
/// ```
///
pub fn make_response(body: impl Into<Body>, status: StatusCode) -> Response<Body> {
    let mut response = Response::new(body.into());
    *response.status_mut() = status;
    response
}
