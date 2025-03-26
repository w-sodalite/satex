use crate::args::find_arg;
use crate::symbol::Symbol;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error, Expr};

const MAKE: Symbol = Symbol("make");

const ATTR_NAME: &str = "name";

pub fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let name = get_name(&input)?;
    Ok(quote! {
        #[automatically_derived]
        impl satex_core::make::Make for #struct_name {
            fn name(&self) -> &'static str {
                #name
            }
        }
    })
}

fn get_name(input: &DeriveInput) -> syn::Result<Expr> {
    let arg = find_arg(&input.attrs, MAKE, ATTR_NAME)?;
    match arg {
        Some(arg) => Ok(arg.value),
        None => Err(Error::new_spanned(input, "Missing attribute `name`")),
    }
}
