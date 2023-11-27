use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse::Parse, parse_macro_input, Attribute, DeriveInput, FieldsNamed,
    FieldsUnnamed, LitStr, Token,
};

use crate::args::Args;

pub fn expand_from_error(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let enum_data = if let syn::Data::Enum(data) = &ast.data {
        data
    } else {
        panic!("{} is not an enum", name);
    };

    let mut display_stream = proc_macro2::TokenStream::new();
    let mut from_stream = proc_macro2::TokenStream::new();

    for variant_data in &enum_data.variants {
        let variant_name = &variant_data.ident;

        let attr = variant_data
            .attrs
            .iter()
            .find(|x| x.path().is_ident("from_error"))
            .unwrap();

        let (from_token, display_token) = match &variant_data.fields {
            syn::Fields::Named(named) => {
                struct_parse(name, variant_name, attr, named)
            }
            syn::Fields::Unnamed(unnamed) => {
                tuple_parse(name, variant_name, attr, unnamed)
            }
            syn::Fields::Unit => unit_parse(name, variant_name, attr),
        };

        from_stream.extend(from_token);
        display_stream.extend(display_token);
    }

    quote! {
        impl std::error::Error for #name {}

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #display_stream
                }
            }
        }

        impl From<#name> for AppError {
            fn from(e: #name) -> Self {
                match &e {
                    #from_stream
                }
            }
        }
    }
    .into()
}

#[derive(Debug)]
struct FromError {
    code: Option<Args>,
    status: Option<Args>,
}

impl Parse for FromError {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut r = FromError {
            code: None,
            status: None,
        };
        loop {
            let pair: Args = input.parse()?;
            if pair.key == "code" {
                r.code = Some(pair);
            } else if pair.key == "status" {
                r.status = Some(pair);
            }
            if !input.is_empty() {
                let _comma: Token![,] = input.parse()?;
            } else {
                break;
            }
        }

        if r.code.is_none() || r.status.is_none() {
            panic!("'code' and 'status' required.")
        }

        Ok(r)
    }
}

fn parse_attr(attr: &Attribute) -> (Ident, LitStr) {
    let attr: FromError = attr.parse_args().unwrap();

    let status = match attr.status {
        Some(s) => s,
        None => panic!("'status' required"),
    };

    let code = match attr.code {
        Some(c) => c,
        None => panic!("'code' required"),
    };

    let i18n_val = code.val;
    let status_val = status.val;
    let function_val = match status_val.value().as_str() {
        "forbidden" => Ident::new("forbidden", Span::call_site()),
        "internal_server_error" => {
            Ident::new("internal_server_error", Span::call_site())
        }
        "not_found" => Ident::new("not_found", Span::call_site()),
        "unauthorized" => Ident::new("unauthorized", Span::call_site()),
        "bad_request" => Ident::new("bad_request", Span::call_site()),
        &_ => panic!("'status' is not correct"),
    };

    (function_val, i18n_val)
}

fn unit_parse(
    name: &Ident,
    variant_name: &Ident,
    attr: &Attribute,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let (function_val, i18n_val) = parse_attr(attr);

    (
        quote! {
            #name::#variant_name => Self::#function_val(util::i18n::i18n(#i18n_val)),
        },
        quote! {
            #name::#variant_name => f.write_fmt(format_args!(
                "{}",
                util::i18n::i18n(#i18n_val)
            )),
        },
    )
}

fn tuple_parse(
    name: &Ident,
    variant_name: &Ident,
    attr: &Attribute,
    fields: &FieldsUnnamed,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut tuple_names = proc_macro2::TokenStream::new();

    for field in fields.unnamed.iter() {
        let fi = field.ident.as_ref().unwrap();
        let tuple_name = Ident::new(&format!("{}", fi), Span::call_site());
        tuple_names.extend(quote!(#tuple_name,));
    }

    let (function_val, i18n_val) = parse_attr(attr);

    (
        quote! {
            #name::#variant_name(#tuple_names) => Self::#function_val(util::i18n::i18n(#i18n_val)),
        },
        quote! {
            #name::#variant_name(#tuple_names) => f.write_fmt(format_args!(
                "{}",
                util::i18n::i18n(#i18n_val)
            )),
        },
    )
}

fn struct_parse(
    name: &Ident,
    variant_name: &Ident,
    attr: &Attribute,
    fields: &FieldsNamed,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut field_names = proc_macro2::TokenStream::new();

    for field in fields.named.iter() {
        let fi = field.ident.as_ref().unwrap();
        let field_name = Ident::new(
            &format!("{}", fi).to_ascii_lowercase(),
            Span::call_site(),
        );
        field_names.extend(quote!(#field_name,));
    }

    let (function_val, i18n_val) = parse_attr(attr);

    (
        quote! {
            #name::#variant_name{#field_names} => Self::#function_val(util::i18n::i18n(#i18n_val)),
        },
        quote! {
            #name::#variant_name{#field_names} => f.write_fmt(format_args!(
                "{}",
                util::i18n::i18n(#i18n_val)
            )),
        },
    )
}
