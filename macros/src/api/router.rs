use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Ident, ItemFn, LitStr, Token};

pub(crate) fn expand_router(args: TokenStream, item: TokenStream) -> TokenStream {
    let ft = parse_macro_input!(item as ItemFn);
    let name = &ft.sig.ident;
    let req_router = parse_macro_input!(args as ReqRouter);
    let path = &req_router.path().val;
    let method = &req_router.method().val;

    let func_name = format!("{}_{}", method.value(), name);
    let method = Ident::new(&method.value(), Span::call_site());
    let func_name = Ident::new(&func_name, Span::call_site());

    quote! {
        pub fn #func_name() -> axum::Router {
            #ft

            axum::Router::new().route(#path, axum::routing::#method(#name))
        }
    }
    .into()
}
struct Args {
    key: Ident,
    val: LitStr,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        let _: Token![=] = input.parse()?;
        let val: LitStr = input.parse()?;

        Ok(Args { key, val })
    }
}

struct ReqRouter {
    path: Option<Args>,
    method: Args,
}

impl Default for ReqRouter {
    fn default() -> Self {
        Self {
            path: None,
            method: Args {
                key: Ident::new("method", Span::call_site()),
                val: LitStr::new("get", Span::call_site()),
            },
        }
    }
}

impl Parse for ReqRouter {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut r = ReqRouter::default();
        loop {
            let pair: Args = input.parse()?;
            if pair.key == "path" {
                r.path = Some(pair);
            } else if pair.key == "method" {
                r.method = pair;
            }
            if !input.is_empty() {
                let _comma: Token![,] = input.parse()?;
            } else {
                break;
            }
        }

        Ok(r)
    }
}

impl ReqRouter {
    fn path(&self) -> &Args {
        if let Some(p) = &self.path {
            return p;
        }
        panic!("'path' doesn't exist");
    }

    fn method(&self) -> &Args {
        &self.method
    }
}
