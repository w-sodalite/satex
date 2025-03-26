use http::{Response, StatusCode};

pub trait ResponseExt<B>: Sized {
    fn with_status(self, status: StatusCode) -> Self;
    fn with_body(self, body: B) -> Self;
}

impl<B> ResponseExt<B> for Response<B> {
    fn with_status(mut self, status: StatusCode) -> Self {
        *self.status_mut() = status;
        self
    }

    fn with_body(mut self, body: B) -> Response<B> {
        *self.body_mut() = body;
        self
    }
}
