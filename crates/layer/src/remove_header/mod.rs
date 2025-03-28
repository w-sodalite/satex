mod remove_request_header;
mod remove_response_header;

use http::{HeaderName, Request, Response};
pub use remove_request_header::{
    MakeRemoveRequestHeaderRouteLayer, RemoveRequestHeader, RemoveRequestHeaderLayer,
};
pub use remove_response_header::{
    MakeRemoveResponseHeaderRouteLayer, RemoveResponseHeader, RemoveResponseHeaderLayer,
    ResponseFuture,
};

pub trait Removable {
    fn remove(&mut self, name: &HeaderName);
}

impl<ResBody> Removable for Response<ResBody> {
    fn remove(&mut self, name: &HeaderName) {
        let headers = self.headers_mut();
        if headers.contains_key(name) {
            headers.remove(name);
        }
    }
}

impl<ReqBody> Removable for Request<ReqBody> {
    fn remove(&mut self, name: &HeaderName) {
        let headers = self.headers_mut();
        if headers.contains_key(name) {
            headers.remove(name);
        }
    }
}
