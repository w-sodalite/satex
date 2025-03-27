use crate::util::ExprArg;
use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Error, ItemStruct, Token};

///
/// 默认的快捷参数模式: [ShortcutMode::Object]
///
const DEFAULT_SHORTCUT_MODE: &str = "Object";

pub(crate) fn expand(args: Args, input: ItemStruct) -> syn::Result<TokenStream> {
    let make_kind = args
        .kind
        .ok_or_else(|| Error::new(Span::call_site(), "Miss attribute `kind`"))?;

    let shortcut_mode = args
        .shortcut_mode
        .unwrap_or_else(|| Ident::new(DEFAULT_SHORTCUT_MODE, Span::call_site()));

    // 字段名集合
    let field_names = input
        .fields
        .iter()
        .flat_map(|field| &field.ident)
        .map(|ident| ident.to_string());

    let make_name = &input.ident;
    let generics = input.generics;
    let fields = input.fields.iter();

    Ok(quote! {

        #[automatically_derived]
        impl satex_core::component::Configurable for Config {
            const FIELDS: &'static [&'static str] = &[#( #field_names ),*];
            const SHORTCUT_MODE: satex_core::component::ShortcutMode = satex_core::component::ShortcutMode::#shortcut_mode;
            const COMPANION: &'static str = stringify!(#make_name);
        }

        #[derive(serde::Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct Config #generics{
            #(#fields),*
        }

        #[derive(Debug, Clone, Copy, Default)]
        pub struct #make_name;

        impl satex_core::make::Make for #make_name {
            fn name(&self) -> &'static str {
                stringify!(#make_kind)
            }
        }
    })
}

#[derive(Default)]
pub struct Args {
    kind: Option<Ident>,
    shortcut_mode: Option<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Args::default();
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::kind) {
                if args.kind.is_some() {
                    return Err(input.error("duplicate attribute `kind`"));
                }
                let arg = input.parse::<ExprArg<kw::kind>>()?;
                args.kind = Some(arg.require_ident()?);
            } else if lookahead.peek(kw::shortcut_mode) {
                if args.shortcut_mode.is_some() {
                    return Err(input.error("duplicate attribute `shortcut_mode`"));
                }
                let arg = input.parse::<ExprArg<kw::shortcut_mode>>()?;
                args.shortcut_mode = Some(arg.require_ident()?);
            } else if lookahead.peek(Token![,]) {
                let _ = input.parse::<Token![,]>()?;
            } else {
                let _ = input.parse::<TokenTree>()?;
            }
        }
        Ok(args)
    }
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(kind);
    custom_keyword!(shortcut_mode);
}
