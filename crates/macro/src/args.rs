use crate::symbol::Symbol;
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{AttrStyle, Attribute, Error, Expr, Lit, LitBool, LitStr, Path, Token};

pub(crate) fn find_args(attrs: &[Attribute], symbol: Symbol) -> syn::Result<Args> {
    match attrs
        .iter()
        .filter(|attr| matches!(attr.style, AttrStyle::Outer))
        .flat_map(|attr| attr.meta.require_list())
        .filter(|ms| ms.path == symbol)
        .map(|attr| attr.parse_args::<Args>())
        .next()
    {
        Some(args) => args,
        None => Err(Error::new(Span::call_site(), "missing attribute")),
    }
}

pub(crate) fn find_arg(
    attrs: &[Attribute],
    symbol: Symbol,
    name: &str,
) -> syn::Result<Option<Arg>> {
    let args = find_args(attrs, symbol)?;
    Ok(args.0.into_iter().find(|arg| arg.name.is_ident(name)))
}

pub(crate) struct Args(Punctuated<Arg, Token![,]>);

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Args(Punctuated::parse_terminated(input)?))
    }
}

pub(crate) struct Arg {
    pub name: Path,
    pub value: Expr,
}

impl Arg {
    #[allow(unused)]
    pub fn lit_str(&self) -> Option<&LitStr> {
        match &self.value {
            Expr::Lit(expr) => match &expr.lit {
                Lit::Str(lit_str) => Some(lit_str),
                _ => None,
            },
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn lit_bool(&self) -> Option<&LitBool> {
        match &self.value {
            Expr::Lit(expr) => match &expr.lit {
                Lit::Bool(lit_bool) => Some(lit_bool),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Path>()?;
        let _ = input.parse::<Token![=]>()?;
        let value = input.parse::<Expr>()?;
        Ok(Arg { name, value })
    }
}
