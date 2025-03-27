use proc_macro2::Ident;
use quote::format_ident;
use std::marker::PhantomData;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Expr, Lit, LitStr, Token};

macro_rules! arg {
    ($name:ident,$ty:ty) => {
        pub struct $name<T> {
            pub value: $ty,
            _p: PhantomData<T>
        }

        impl<T: Parse> Parse for $name<T> {
            fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
                let _ = input.parse::<T>()?;
                let _ = input.parse::<Token![=]>()?;
                let value = input.parse()?;
                Ok(Self {
                    value,
                    _p: PhantomData,
                })
            }
        }
    };
}

arg!(ExprArg, Expr);
arg!(LitStrArg, LitStr);

impl<T> ExprArg<T> {
    pub fn require_ident(&self) -> syn::Result<Ident> {
        get_expr_ident(&self.value).ok_or_else(|| {
            Error::new_spanned(&self.value, "only identifier or string literal allowed")
        })
    }
}

pub fn get_expr_ident(expr: &Expr) -> Option<Ident> {
    match expr {
        Expr::Lit(lit) => {
            if let Lit::Str(lit) = &lit.lit {
                Some(format_ident!("{}", lit.value()))
            } else {
                None
            }
        }
        Expr::Path(path) => path.path.get_ident().cloned(),
        _ => None,
    }
}
