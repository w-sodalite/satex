use hyper::header::HeaderValue;
use tower_http::set_header::MakeHeaderValue;

#[derive(Default, Debug, Clone)]
pub struct FixedMakeHeaderValue(Option<HeaderValue>);

impl FixedMakeHeaderValue {
    pub fn new(value: HeaderValue) -> Self {
        Self(Some(value))
    }
}

impl<T> MakeHeaderValue<T> for FixedMakeHeaderValue {
    fn make_header_value(&mut self, _: &T) -> Option<HeaderValue> {
        self.0.take()
    }
}
