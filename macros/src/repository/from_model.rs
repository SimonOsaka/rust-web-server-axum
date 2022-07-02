use crate::args::Args;
use crate::util::{ident, litint, litstr};
use heck::ToUpperCamelCase;
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::Type;
use syn::{parse::Parse, parse_macro_input, DeriveInput, LitStr, Token};

pub fn expand_from_model(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let struct_name_ident = &ast.ident;

    let attr_opt = &ast.attrs.iter().find(|x| x.path.is_ident("from_model"));
    let attr = if let Some(a) = attr_opt {
        a
    } else {
        panic!("add struct macro 'from_model'");
    };

    let arg: Args = match attr.parse_args() {
        Ok(arg) => arg,
        Err(_) => panic!("add struct attribute 'table_name' #[from_model(table_name = \"...\")"),
    };
    let table_name_litstr = arg.val;

    let struct_data = if let syn::Data::Struct(data) = &ast.data {
        data
    } else {
        panic!("{} is not a struct", struct_name_ident);
    };

    let mut non_from_model_column_names_ts = TokenStream2::new();
    let mut non_from_model_column_values_ts = TokenStream2::new();
    let mut primary_keys_ts = TokenStream2::new();
    let mut input_primary_keys_ts = TokenStream2::new();
    let mut where_primary_keys_ts = TokenStream2::new();
    let mut update_sets_ts = TokenStream2::new();
    let mut single_fields_ts = TokenStream2::new();
    let mut multi_fields_ts = TokenStream2::new();
    let mut model_fields_ts = TokenStream2::new();
    let mut conditions_ts = TokenStream2::new();

    let mut primary_key_opt = None;

    let mut multi_struct_impl_ts = TokenStream2::new();
    // fields
    let struct_data_fields_len = struct_data.fields.len();
    for (i, variant_data) in struct_data.fields.iter().enumerate() {
        let variant_name_ident = if let Some(ident) = variant_data.ident.as_ref() {
            ident
        } else {
            panic!("Not tuple struct");
        };
        // attribute
        let from_model_attr_opt = variant_data
            .attrs
            .iter()
            .find(|x| x.path.is_ident("from_model"));

        let variant_name_litstr = litstr(format!("{}", variant_name_ident));

        let mut left_paren = "";
        let mut end_paren_alias = "".to_string();

        if i == 0 {
            left_paren = "(";
        } else if i == struct_data_fields_len - 1 {
            let v = table_name_litstr.value();
            end_paren_alias = format!(") AS \"{}\"", v);
        }

        let multi_fields_name_litstr = litstr(format!(
            "{left_paren}{}.{}{end_paren_alias}",
            table_name_litstr.value().as_str(),
            variant_name_ident,
        ));

        let single_fields_name_litstr = litstr(format!(
            "{}.{}",
            table_name_litstr.value().as_str(),
            variant_name_ident,
        ));

        let model_fields_name_ident = ident(
            variant_name_litstr
                .value()
                .as_str()
                .to_upper_camel_case()
                .to_string(),
        );

        single_fields_ts.extend(quote!(#single_fields_name_litstr,));
        multi_fields_ts.extend(quote!(#multi_fields_name_litstr,));
        model_fields_ts.extend(quote! {
            #model_fields_name_ident(crate::db::types::Operation),
        });
        let struct_name_fields_ident = ident(format!("{}Fields", struct_name_ident));
        let table_name_variant_name_litstr = litstr(format!(
            "{}.{}",
            table_name_litstr.value(),
            variant_name_litstr.value()
        ));
        let ty = &variant_data.ty;
        if let Type::Path(ty) = ty {
            if ty.path.is_ident("ID") || ty.path.is_ident("i64") || ty.path.is_ident("i16") {
                conditions_ts.extend(quote! {
                    #struct_name_fields_ident::#model_fields_name_ident(op) => match op {
                        crate::db::types::Operation::Eq(value) => {
                            if let crate::db::types::Value::Integer(v) = value {
                                sql_builder.and_where_eq(#table_name_variant_name_litstr, params.add_value(v));
                            }
                        },
                        crate::db::types::Operation::Between(lv, rv) => {
                            if let (crate::db::types::Value::Integer(lv), crate::db::types::Value::Integer(rv)) =
                    (lv, rv) {
                            sql_builder.and_where_between(#table_name_variant_name_litstr, params.add_value(lv), params.add_value(rv));
                            }
                        },
                    },
                });
            } else if ty.path.is_ident("String") {
                conditions_ts.extend(quote! {
                    #struct_name_fields_ident::#model_fields_name_ident(op) => match op {
                        crate::db::types::Operation::Eq(value) => {
                            if let crate::db::types::Value::String(v) = value {
                                sql_builder.and_where_eq(#table_name_variant_name_litstr, params.add_value(v));
                            }
                        },
                        crate::db::types::Operation::Between(lv, rv) => {
                            panic!("{} Not support 'crate::db::types::Operation::Between'", #table_name_variant_name_litstr);
                        },
                    },
                });
            } else {
                conditions_ts.extend(quote! {
                    #struct_name_fields_ident::#model_fields_name_ident(op) => match op {
                        crate::db::types::Operation::Eq(_) => {
                            panic!("{} Not support 'crate::db::types::Operation::Between'", #table_name_variant_name_litstr);
                        },
                        crate::db::types::Operation::Between(_, _) => {
                            panic!("{} Not support 'crate::db::types::Operation::Between'", #table_name_variant_name_litstr);
                        },
                    },
                });
            }
        }

        match from_model_attr_opt {
            // from_model exist
            Some(from_model) => {
                let from_model_ident_opt = from_model.parse_args::<Ident>().ok();
                match from_model_ident_opt {
                    // from_model(primary_key)
                    Some(ident) => {
                        if "primary_key" == ident.to_string().as_str() {
                            primary_key_opt = Some(variant_name_litstr.clone());
                            primary_keys_ts.extend(quote! {
                                .and_where_eq(#variant_name_litstr, params.add_value(&self.#variant_name_ident))
                            });
                            input_primary_keys_ts.extend(quote! {
                                #variant_name_ident: vars::types::ID,
                            });
                            where_primary_keys_ts.extend(quote! {
                                .and_where_eq(#variant_name_litstr, params.add_value(#variant_name_ident))
                            });
                        }
                    }
                    // from_model(key = val, ..)
                    None => {
                        let foreign_attr_opt = from_model.parse_args::<ForeignAttr>().ok();
                        match foreign_attr_opt {
                            Some(foreign_attr) => {
                                let t2_litstr = match foreign_attr.table_name {
                                    Some(k) => k.val,
                                    None => {
                                        panic!("no table_name on {}", variant_name_litstr.value())
                                    }
                                };
                                let t2_primary_key_litstr = match foreign_attr.primary_key {
                                    Some(k) => k.val,
                                    None => {
                                        panic!("no primary_key on {}", variant_name_litstr.value())
                                    }
                                };
                                let m2_litstr = match foreign_attr.model {
                                    Some(k) => k.val,
                                    None => {
                                        panic!("no model on {}", variant_name_litstr.value())
                                    }
                                };
                                let t1_primary_key_litstr = match primary_key_opt.clone() {
                                    Some(pk) => pk,
                                    None => panic!("no primary_key on {}", struct_name_ident),
                                };
                                let me = multi_expand(
                                    struct_name_ident,
                                    m2_litstr,
                                    &table_name_litstr,
                                    t2_litstr,
                                    t1_primary_key_litstr,
                                    &variant_name_litstr,
                                    t2_primary_key_litstr,
                                );
                                if !primary_keys_ts.is_empty() {
                                    multi_struct_impl_ts.extend(me);
                                }
                                // update sets
                                non_from_model_column_names_ts
                                    .extend(quote!(#variant_name_litstr,));
                                non_from_model_column_values_ts
                                    .extend(quote!(params.add_value(&self.#variant_name_ident),));
                                update_sets_ts.extend(quote!        (sql_builder.set(#variant_name_litstr,  params.add_value(&self.#variant_name_ident));),
                                );
                            }
                            None => panic!("No foreign"),
                        }
                    }
                }
            }
            None => {
                // update sets
                non_from_model_column_names_ts.extend(quote!(#variant_name_litstr,));
                non_from_model_column_values_ts
                    .extend(quote!(params.add_value(&self.#variant_name_ident),));
                update_sets_ts.extend(
                    quote!(sql_builder.set(#variant_name_litstr, params.add_value(&self.#variant_name_ident));),
                );
            }
        }
    }

    let struct_name_litstr = litstr(struct_name_ident.to_string());
    let fields_expand_ts = fields_expand(
        &struct_name_litstr,
        &single_fields_ts,
        &multi_fields_ts,
        &model_fields_ts,
        &conditions_ts,
        struct_data.fields.len(),
    );
    let insert_expand_ts = insert_expand(
        &table_name_litstr,
        &non_from_model_column_names_ts,
        &non_from_model_column_values_ts,
    );
    let get_expand_ts = get_expand(struct_name_ident, &table_name_litstr);
    let delete_expand_ts = delete_expand(&table_name_litstr, &primary_keys_ts);
    let update_expand_ts = update_expand(&table_name_litstr, &primary_keys_ts, &update_sets_ts);

    let mut struct_impl_ts = TokenStream2::new();
    if !primary_keys_ts.is_empty() {
        // primary_key exist
        if struct_data.fields.len() > 1 {
            struct_impl_ts.extend(update_expand_ts);
            // get_expand
            struct_impl_ts.extend(get_expand_ts);
        }
        struct_impl_ts.extend(delete_expand_ts);
    } else {
        // primary_key not exist
        struct_impl_ts.extend(insert_expand_ts);
    }

    quote! {
        #fields_expand_ts

        impl #struct_name_ident {
            #struct_impl_ts
        }

        #multi_struct_impl_ts
    }
    .into()
}

#[derive(Debug, Clone)]
struct ForeignAttr {
    table_name: Option<Args>,
    model: Option<Args>,
    primary_key: Option<Args>,
}
impl Parse for ForeignAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut r = ForeignAttr {
            table_name: None,
            model: None,
            primary_key: None,
        };
        loop {
            let pair: Args = input.parse()?;
            if pair.key == "table_name" {
                r.table_name = Some(pair);
            } else if pair.key == "model" {
                r.model = Some(pair);
            } else if pair.key == "primary_key" {
                r.primary_key = Some(pair);
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

fn single_column_names(model_name_uppercase: String) -> Ident {
    ident(format!("{}_SINGLE_FIELDS", model_name_uppercase))
}

fn multi_column_names(model_name_uppercase: String) -> Ident {
    ident(format!("{}_MULTI_FIELDS", model_name_uppercase))
}

fn fields_name(model_name: String) -> Ident {
    ident(format!("{}Fields", model_name))
}

fn fields_sqlquery_fn_name(model_name: String) -> Ident {
    ident(format!(
        "{}_fields_sqlquery",
        model_name.as_str().to_ascii_lowercase()
    ))
}

fn insert_expand(
    table_name: &LitStr,
    column_names: &TokenStream2,
    column_values: &TokenStream2,
) -> TokenStream2 {
    quote! {
        #[tracing::instrument(skip(transaction), err)]
        pub async fn insert<'a>(&self, transaction: Option<&'a mut sqlx::Transaction<'static, sqlx::Postgres>>,) -> Result<vars::types::ID, sqlx::Error> {
            use crate::db::write::SqlWriter;

            let mut params = crate::db::SqlParams::new();

            let mut sql_builder = sql_builder::SqlBuilder::insert_into(#table_name);

            sql_builder
                .fields(&[
                    #column_names
                ])
                .values(&[
                    #column_values
                ])
                .returning_id();

            let id = sql_builder.insert_one(params, transaction).await?;
            tracing::debug!("inserted id: {:?}", id);

            Ok(id)
        }
    }
}

fn delete_expand(table_name: &LitStr, primary_keys: &TokenStream2) -> TokenStream2 {
    quote! {
        #[tracing::instrument(skip(transaction), err)]
        pub async fn delete<'a>(
            &self,
            transaction: Option<&'a mut sqlx::Transaction<'static, sqlx::Postgres>>,
        ) -> Result<bool, sqlx::Error> {
            use crate::db::write::SqlWriter;

            let mut params = crate::db::SqlParams::new();
            let mut sql_builder = sql_builder::SqlBuilder::update_table(#table_name);
            sql_builder
                .set("is_deleted", 1)
                .and_where_eq("is_deleted", 0)
                #primary_keys;

            let affect_rows = sql_builder.delete_one(params, transaction).await?;

            Ok(affect_rows == 1)
        }
    }
}

fn update_expand(
    table_name_litstr: &LitStr,
    primary_keys_ts: &TokenStream2,
    update_sets_ts: &TokenStream2,
) -> TokenStream2 {
    quote! {
        #[tracing::instrument(skip(transaction), err)]
        pub async fn update<'a>(
            &self,
            transaction: Option<&'a mut sqlx::Transaction<'static, sqlx::Postgres>>,
        ) -> Result<bool, sqlx::Error> {
            use crate::db::write::SqlWriter;

            let mut params = crate::db::SqlParams::new();
            let mut sql_builder = sql_builder::SqlBuilder::update_table(#table_name_litstr);
            #update_sets_ts
            sql_builder
                .and_where_eq("is_deleted", 0)
                #primary_keys_ts;

            let affect_rows = sql_builder.update_one(params, transaction).await?;

            Ok(affect_rows == 1)
        }
    }
}

fn get_expand(struct_name_ident: &Ident, table_name: &LitStr) -> TokenStream2 {
    let fields_name = single_column_names(struct_name_ident.to_string().to_ascii_uppercase());

    let fields_name_ident = ident(format!("{}Fields", struct_name_ident));

    let fields_name_sqlquery_ident = fields_sqlquery_fn_name(struct_name_ident.to_string());

    quote! {
        #[tracing::instrument(skip(transaction), err)]
        pub async fn get<'a>(
            fields: Vec<#fields_name_ident>,
            transaction: Option<&'a mut sqlx::Transaction<'static, sqlx::Postgres>>,
        ) -> Result<Option<#struct_name_ident>, sqlx::Error> {
            use crate::db::read::SqlReader;

            let mut params = crate::db::SqlParams::new();
            let mut sql_builder = sql_builder::SqlBuilder::select_from(#table_name);
            sql_builder.fields(#fields_name).and_where_eq("is_deleted", 0);

            for field in fields {
                #fields_name_sqlquery_ident(field, &mut sql_builder, &mut params);
            }

            let my = sql_builder.query_one_optinal(params, transaction).await?;
            Ok(my)
        }
    }
}

fn fields_expand(
    model_name: &LitStr,
    single_fields: &TokenStream2,
    multi_fields: &TokenStream2,
    model_fields: &TokenStream2,
    conditions_ts: &TokenStream2,
    len: usize,
) -> TokenStream2 {
    // MYUSERS_SINGLE_FIELDS
    let single_name_ident = single_column_names(model_name.value().to_ascii_uppercase());
    let size = litint(len.to_string());

    // MYUSERS_MULTI_FIELDS
    let multi_name_ident = multi_column_names(model_name.value().to_ascii_uppercase());

    // MyUsersFields
    let fields_name_ident = fields_name(model_name.value());

    // myusers_fields_sqlquery
    let fields_name_sqlquery_ident = fields_sqlquery_fn_name(model_name.value());

    quote! {
        pub(crate) const #single_name_ident: &[&str; #size] = &[
            #single_fields
        ];

        pub(crate) const #multi_name_ident: &[&str; #size] = &[
            #multi_fields
        ];

        #[derive(Debug)]
        pub enum #fields_name_ident {
            #model_fields
        }

        pub(crate) fn #fields_name_sqlquery_ident(field: #fields_name_ident, sql_builder: &mut sql_builder::SqlBuilder, params: &mut crate::db::SqlParams) {
            match field {
                #conditions_ts
            }
        }
    }
}

fn multi_expand(
    m1_ident: &Ident,
    m2_litstr: LitStr,
    t1_litstr: &LitStr,
    t2_litstr: LitStr,
    t1_primary_key_litstr: LitStr,
    t1_foreign_key_litstr: &LitStr,
    t2_primary_key_litstr: LitStr,
) -> TokenStream2 {
    let m2_ident = ident(m2_litstr.value());
    let struct_name_ident = ident(format!("{}{}", m1_ident, m2_ident));
    let t1_ident = ident(t1_litstr.value());

    let t1_multi_fields_name_ident = multi_column_names(m1_ident.to_string().to_ascii_uppercase());
    let t2_ident = ident(t2_litstr.value());
    let t2_multi_fields_name_ident = multi_column_names(m2_litstr.value().to_ascii_uppercase());
    let on = litstr(format!(
        "{t1}.{t1_fk} = {t2}.{t2_pk} and {t2}.is_deleted = 0",
        t1 = t1_litstr.value(),
        t2 = t2_litstr.value(),
        t1_fk = t1_foreign_key_litstr.value(),
        t2_pk = t2_primary_key_litstr.value()
    ));
    let m1_fields_name_ident = ident(format!("{}Fields", m1_ident));
    let m2_fields_name_ident = ident(format!("{}Fields", m2_ident));

    let fields1_name_sqlquery_ident = fields_sqlquery_fn_name(m1_ident.to_string());
    let fields2_name_sqlquery_ident = fields_sqlquery_fn_name(m2_litstr.value());

    let t1_fields_ident = ident(format!("{}_fields", t1_ident));
    let t2_fields_ident = ident(format!("{}_fields", t2_ident));

    quote! {
        #[derive(Debug, sqlx::FromRow)]
        pub struct #struct_name_ident {
            pub #t1_ident: #m1_ident,
            pub #t2_ident: #m2_ident,
        }

        impl #struct_name_ident {
            #[tracing::instrument(skip(transaction), err)]
            pub async fn find<'a>(
                #t1_fields_ident: Vec<#m1_fields_name_ident>,
                #t2_fields_ident: Vec<#m2_fields_name_ident>,
                transaction: Option<&'a mut sqlx::Transaction<'static, sqlx::Postgres>>,
            ) -> Result<Vec<(#m1_ident, #m2_ident)>, sqlx::Error> {
                use crate::db::read::SqlReader;
                use sql_builder::SqlName;

                let mut params = crate::db::SqlParams::new();
                let mut sql_builder = sql_builder::SqlBuilder::select_from(#t1_litstr);

                sql_builder
                    .fields(#t1_multi_fields_name_ident)
                    .fields(#t2_multi_fields_name_ident)
                    .left()
                    .join(#t2_litstr)
                    .on(#on)
                    .and_where_eq(sql_builder::name!(#t1_litstr, "is_deleted"), 0);

                    for field in #t1_fields_ident {
                       #fields1_name_sqlquery_ident(field, &mut sql_builder, &mut params);
                    }

                    for field in #t2_fields_ident {
                        #fields2_name_sqlquery_ident(field, &mut sql_builder, &mut params);
                     }

                    sql_builder.order_desc(sql_builder::name!(#t1_litstr, #t1_primary_key_litstr));

                let list: Vec<#struct_name_ident> = sql_builder.query_list(params, transaction).await?;

                let c = list
                    .into_iter()
                    .map(|result| (result.#t1_ident, result.#t2_ident))
                    .collect();
                Ok(c)
            }
        }
    }
}
