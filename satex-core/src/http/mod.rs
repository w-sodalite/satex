use hyper::{Response, StatusCode};

pub use body::Body;

mod body;

pub fn make_response(body: impl Into<Body>, status: StatusCode) -> Response<Body> {
    let mut response = Response::new(body.into());
    *response.status_mut() = status;
    response
}
