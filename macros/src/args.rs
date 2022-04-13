use proc_macro2::Ident;
use syn::{parse::Parse, LitStr, Token};

#[derive(Debug, Clone)]
pub(crate) struct Args {
    pub(crate) key: Ident,
    pub(crate) val: LitStr,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        let _: Token![=] = input.parse()?;
        let val: LitStr = input.parse()?;

        Ok(Args { key, val })
    }
}
