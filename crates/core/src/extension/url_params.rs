use http::Extensions;
use matchit::Params;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum UrlParams {
    Params(Vec<(Arc<str>, PercentDecodedStr)>),
    InvalidUtf8InPathParam { key: Arc<str> },
}

pub fn insert_url_params(extensions: &mut Extensions, params: Params<'_, '_>) {
    let current_params = extensions.get_mut();

    if let Some(UrlParams::InvalidUtf8InPathParam { .. }) = current_params {
        // nothing to do here since an error was stored earlier
        return;
    }

    let params = params
        .iter()
        .map(|(k, v)| {
            if let Some(decoded) = PercentDecodedStr::new(v) {
                Ok((Arc::from(k), decoded))
            } else {
                Err(Arc::from(k))
            }
        })
        .collect::<Result<Vec<_>, _>>();

    match (current_params, params) {
        (Some(UrlParams::InvalidUtf8InPathParam { .. }), _) => {
            unreachable!("we check for this state earlier in this method")
        }
        (_, Err(invalid_key)) => {
            extensions.insert(UrlParams::InvalidUtf8InPathParam { key: invalid_key });
        }
        (Some(UrlParams::Params(current)), Ok(params)) => {
            current.extend(params);
        }
        (None, Ok(params)) => {
            extensions.insert(UrlParams::Params(params));
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PercentDecodedStr(Arc<str>);

impl PercentDecodedStr {
    pub(crate) fn new<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        percent_encoding::percent_decode(s.as_ref().as_bytes())
            .decode_utf8()
            .ok()
            .map(|decoded| Self(decoded.as_ref().into()))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for PercentDecodedStr {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
