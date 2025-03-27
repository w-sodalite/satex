mod make;
mod util;

use crate::make::Args;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn make(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);
    let input = parse_macro_input!(input as ItemStruct);
    make::expand(args, input)
        .map(|stream| stream.into())
        .expect("Expand attribute macro `configurable` error!")
}
