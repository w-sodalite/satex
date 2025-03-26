use crate::new_type;
use http::Uri;

new_type!(
    #[derive(Debug, Clone)]
    RawUri,
    Uri
);
