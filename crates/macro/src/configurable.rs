use crate::args::find_arg;
use crate::symbol::Symbol;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Error};

///
/// configurable attr symbol
///
const CONFIGURABLE: Symbol = Symbol("configurable");

///
/// Derive宏属性: [shortcut_mode]
///
const ATTR_SHORTCUT_MODE: &str = "shortcut_mode";

///
/// Derive宏属性: [companion]
///
const ATTR_COMPANION: &str = "companion";

///
/// 默认的快捷参数模式: [ShortcutMode::Object]
///
const DEFAULT_SHORTCUT_MODE: &str = "Object";

pub(crate) fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let companion = get_companion(&input)?;
    let shortcut_mode = get_shortcut_mode(&input)?;
    match &input.data {
        Data::Struct(data) => {
            let fields = data
                .fields
                .iter()
                .flat_map(|field| &field.ident)
                .map(|ident| ident.to_string());
            Ok(quote! {
                #[automatically_derived]
                impl satex_core::component::Configurable for #struct_name {
                    const FIELDS: &'static [&'static str] = &[#( #fields ),*];
                    const SHORTCUT_MODE: satex_core::component::ShortcutMode = satex_core::component::ShortcutMode::#shortcut_mode;
                    const COMPANION: &'static str = #companion;
                }
            })
        }
        _ => Err(Error::new_spanned(
            input,
            "configurable only supports struct",
        )),
    }
}

///
/// 获取属性中的[shortcut_mode]值
///
fn get_shortcut_mode(input: &DeriveInput) -> syn::Result<Ident> {
    let arg = find_arg(&input.attrs, CONFIGURABLE, ATTR_SHORTCUT_MODE)?;
    match arg {
        Some(arg) => match arg.lit_str() {
            Some(lit_str) => Ok(Ident::new(&lit_str.value(), Span::call_site())),
            None => Ok(Ident::new(DEFAULT_SHORTCUT_MODE, Span::call_site())),
        },
        None => Ok(Ident::new(DEFAULT_SHORTCUT_MODE, Span::call_site())),
    }
}

///
/// 获取属性中的[companion]值
///
fn get_companion(input: &DeriveInput) -> syn::Result<String> {
    let arg = find_arg(&input.attrs, CONFIGURABLE, ATTR_COMPANION)?;
    match arg {
        Some(arg) => match arg.lit_str() {
            Some(lit_str) => Ok(lit_str.value()),
            None => Err(Error::new_spanned(
                input,
                "attribute `companion` must be a literal string",
            )),
        },
        None => Err(Error::new_spanned(input, "missing attribute `companion`")),
    }
}
