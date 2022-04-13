use proc_macro2::{Ident, Span};
use syn::{LitInt, LitStr};

pub(crate) fn ident(str: String) -> Ident {
    Ident::new(&str, Span::call_site())
}

pub(crate) fn litstr(str: String) -> LitStr {
    LitStr::new(&str, Span::call_site())
}

pub(crate) fn litint(str: String) -> LitInt {
    LitInt::new(&str, Span::call_site())
}
