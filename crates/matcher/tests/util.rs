use http::request::Parts;
use http::{Method, Request};
use satex_core::body::Body;

pub fn parts(uri: &str, method: Method) -> Parts {
    Request::builder()
        .uri(uri)
        .method(method)
        .body(Body::empty())
        .unwrap()
        .into_parts()
        .0
}
