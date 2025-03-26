mod args;
mod make;
mod configurable;
mod symbol;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

///
/// derive [satex_core::metadata::Configurable]
///
#[proc_macro_derive(Configurable, attributes(configurable))]
pub fn configurable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    configurable::expand(input)
        .map(|stream| stream.into())
        .expect("Cannot derive trait: [Configurable]")
}

///
/// auto derive [satex_core::make::Make]
///
#[proc_macro_derive(Make, attributes(make))]
pub fn classify(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    make::expand(input)
        .map(|stream| stream.into())
        .expect("Cannot derive trait: [Named]")
}
