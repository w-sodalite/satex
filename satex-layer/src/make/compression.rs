use hyper::body::Body;
use hyper::Response;
use serde::Deserialize;
use tower_http::compression::predicate::{NotForContentType, SizeAbove};
use tower_http::compression::{CompressionLayer, Predicate};
use tower_http::CompressionLevel;

use satex_core::config::args::Args;
use satex_core::Error;

use crate::MakeRouteServiceLayer;

const DEFAULT_ABOVE_SIZE: u16 = 128;

#[derive(Deserialize)]
struct Config {
    #[serde(default)]
    gzip: Option<bool>,

    #[serde(default)]
    deflate: bool,

    #[serde(default)]
    br: bool,

    #[serde(default)]
    zstd: bool,

    #[serde(default)]
    level: Option<i32>,

    #[serde(default)]
    above_size: Option<u16>,

    #[serde(default)]
    exclude_content_types: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct MakeCompressionLayer;

impl MakeRouteServiceLayer for MakeCompressionLayer {
    type Layer = CompressionLayer<Variants>;

    fn name(&self) -> &'static str {
        "Compression"
    }

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        match args {
            Args::Shortcut(_) => Ok(CompressionLayer::default()
                .gzip(true)
                .quality(CompressionLevel::Default)
                .compress_when(Variants(vec![Variant::size_above(DEFAULT_ABOVE_SIZE)]))),
            Args::Complete(complete) => {
                let config = complete.deserialize::<Config>()?;
                let layer = CompressionLayer::default()
                    .gzip(config.gzip.unwrap_or(true))
                    .deflate(config.deflate)
                    .br(config.br)
                    .zstd(config.zstd)
                    .quality(config.level.map_or(CompressionLevel::Default, |level| {
                        CompressionLevel::Precise(level)
                    }));

                let mut variants = Variants::default();
                variants.push(Variant::size_above(
                    config.above_size.unwrap_or(DEFAULT_ABOVE_SIZE),
                ));
                if let Some(exclude_content_types) = config.exclude_content_types {
                    for exclude_content_type in exclude_content_types {
                        variants.push(Variant::ExcludeContentType(exclude_content_type));
                    }
                }
                Ok(layer.compress_when(variants))
            }
        }
    }
}

#[derive(Clone)]
pub enum Variant {
    SizeAbove(u16),
    ExcludeContentType(String),
}

impl Variant {
    fn size_above(size: u16) -> Self {
        Variant::SizeAbove(size)
    }

    fn exclude_content_type(content_type: String) -> Self {
        Variant::ExcludeContentType(content_type)
    }
}

impl Predicate for Variant {
    fn should_compress<B>(&self, response: &Response<B>) -> bool
    where
        B: Body,
    {
        match self {
            Variant::SizeAbove(size) => SizeAbove::new(*size).should_compress(response),
            Variant::ExcludeContentType(content_type) => {
                NotForContentType::new(content_type).should_compress(response)
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct Variants(Vec<Variant>);

impl Variants {
    pub fn push(&mut self, variant: Variant) {
        self.0.push(variant)
    }
}

impl Predicate for Variants {
    fn should_compress<B>(&self, response: &Response<B>) -> bool
    where
        B: Body,
    {
        self.0
            .iter()
            .all(|variant| variant.should_compress(response))
    }
}
