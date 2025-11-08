mod struct_type;
mod utils;

use crate::struct_type::StructType;
use crate::utils::{ident_opt_to_str, is_option};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Expr, ExprLit, Field, Lit, Path, Type, parse_macro_input};
fn find_attrs(field: &Field, compile_errors_stream: &mut TokenStream) -> (Vec<String>, bool) {
    let mut loaders: Vec<String> = Vec::new();

    let mut is_nested_config = false;

    for attr in &field.attrs {
        if attr.path().is_ident("config") {
            is_nested_config = true;
        } else if attr.path().is_ident("env") {
            match attr.parse_args::<Expr>() {
                Ok(Expr::Lit(ExprLit {
                    lit: Lit::Str(token),
                    ..
                })) => {
                    let value = token.value().trim().to_string();

                    if value.is_empty() {
                        let error_stream: TokenStream = Error::new(
                            attr.meta.path().span(),
                            "Environment variable can't be empty nor blank",
                        )
                        .to_compile_error()
                        .into();
                        compile_errors_stream.extend(error_stream);
                    } else {
                        loaders.push(value);
                    }
                }
                _ => {
                    let error_stream: TokenStream =
                        Error::new(attr.meta.path().span(), "Expecting a string literal")
                            .to_compile_error()
                            .into();
                    compile_errors_stream.extend(error_stream);
                }
            }
        }
    }

    if loaders.is_empty() && !is_nested_config {
        let error_stream: TokenStream = Error::new(field.span(), "No env attribute found")
            .to_compile_error()
            .into();
        compile_errors_stream.extend(error_stream);
    } else if is_nested_config && !loaders.is_empty() {
        let error_stream: TokenStream = Error::new(field.span(), "You can either mark field as nested config or provide env variables to read from, not both.").to_compile_error().into();
        compile_errors_stream.extend(error_stream);
    }

    (loaders, is_nested_config)
}

fn find_default_attr(
    field: &Field,
    compile_error_stream: &mut TokenStream,
) -> Option<TokenStream2> {
    let mut default_value = None;

    let field_type = &field.ty;

    for attr in &field.attrs {
        if attr.path().is_ident("default") {
            if default_value.is_some() {
                let error_stream: TokenStream = Error::new(
                    attr.path().span(),
                    "You can define only one default attribute",
                )
                .to_compile_error()
                .into();
                compile_error_stream.extend(error_stream);
            }

            match attr.parse_args::<Expr>() {
                Ok(Expr::Lit(ExprLit {
                    lit: Lit::Str(token),
                    ..
                })) => {
                    default_value = Some(quote! {
                      {
                        let tmp: #field_type = #token.to_string();
                        tmp
                      }
                    });
                }
                Ok(Expr::Lit(ExprLit { lit, .. })) => {
                    default_value = Some(quote! {
                      {
                        let tmp: #field_type = #lit;
                        tmp
                      }
                    });
                }
                _ => {
                    let error_stream: TokenStream =
                        Error::new(attr.path().span(), "Expecting a literal value")
                            .to_compile_error()
                            .into();
                    compile_error_stream.extend(error_stream);
                }
            }
        }
    }

    default_value
}

fn build_loading_expr(
    field_name: &Option<Ident>,
    field_idx: usize,
    env_attrs: Vec<String>,
    default_value: Option<TokenStream2>,
    field_type: &Type,
) -> TokenStream2 {
    let is_option = is_option(field_type);
    let field_name = ident_opt_to_str(field_name);

    let handle_missing_value = if is_option {
        quote! {
          Ok(None)
        }
    } else if let Some(default) = default_value {
        quote! {
          Ok(#default)
        }
    } else {
        quote! {
          Err(
            tryphon::ConfigFieldError::MissingValue {
              field_name: #field_name,
              field_idx: #field_idx,
              env_vars: vec![#(#env_attrs,)*].into_iter().map(String::from).collect()
            }
          )
        }
    };

    if !env_attrs.is_empty() {
        let mut iterator = env_attrs.iter();
        let first_env_name = iterator
            .next()
            .expect("Expecting at least one loader")
            .clone();
        let mut loading_expr = quote! {
          std::env::var(#first_env_name).map(|v| (v, #first_env_name.to_string()))
        };

        for next_env_name in iterator {
            loading_expr = quote! {
              #loading_expr.or_else(|_| {  std::env::var(#next_env_name).map(|v| (v, #next_env_name.to_string())) })
            };
        }

        quote! {
          match #loading_expr {
            Ok((raw, env_var_name)) => {
              <#field_type as tryphon::ConfigValueDecoder>::decode(raw.clone()).map_err(
                |message|{
                  tryphon::ConfigFieldError::ParsingError {
                    field_name: #field_name,
                    field_idx: #field_idx,
                    raw: raw.clone(),
                    message,
                    env_var_name
                  }
                })
            },
            Err(std::env::VarError::NotPresent) => #handle_missing_value,
            Err(e @ std::env::VarError::NotUnicode(_)) => Err(tryphon::ConfigFieldError::Other {
              message: e.to_string(),
              field_name: #field_name,
              field_idx: #field_idx,
            })
          }
        }
    } else {
        TokenStream2::new()
    }
}

fn build_nested_config_expr(field: &Field, field_idx: usize) -> TokenStream2 {
    let field_type = &field.ty;
    let field_name = ident_opt_to_str(&field.ident);

    quote! {
      <#field_type as Config>::load().map_err(|error| tryphon::ConfigFieldError::Nested {
        field_name: #field_name,
        error,
        field_idx: #field_idx,
      })
    }
}

fn build_loading_for_struct(
    struct_name: TokenStream2,
    fields: Vec<&Field>,
    compile_errors_stream: &mut TokenStream,
) -> TokenStream2 {
    let mut loading_exprs = Vec::new();

    let struct_type = StructType::from_fields(&fields);

    for (field_idx, field) in fields.iter().enumerate() {
        let field_type = &field.ty;
        let default_attr = find_default_attr(field, compile_errors_stream);
        let (env_attrs, is_nested_config) = find_attrs(field, compile_errors_stream);
        if !env_attrs.is_empty() {
            loading_exprs.push((
                field.ident.clone(),
                field_idx,
                build_loading_expr(&field.ident, field_idx, env_attrs, default_attr, field_type),
            ));
        } else if is_nested_config {
            loading_exprs.push((
                field.ident.clone(),
                field_idx,
                build_nested_config_expr(field, field_idx),
            ));
        }
    }

    let errors_gathering = (0..loading_exprs.len()).map(|idx| {
        let idx = syn::Index::from(idx);

        quote! {
          temp_tuple.#idx.as_ref().err()
        }
    });

    let loading_exprs_vals = loading_exprs
        .iter()
        .map(|v| v.2.clone())
        .collect::<Vec<_>>();

    let struct_expr = if struct_type == StructType::Tuple {
        let build_struct_fields = loading_exprs.iter().map(|(_, idx, _)| {
            let idx = syn::Index::from(*idx);

            quote! {
              temp_tuple.#idx.unwrap()
            }
        });

        quote! {
          #struct_name (
             #(#build_struct_fields ,)*
          )
        }
    } else if struct_type == StructType::Named {
        let build_struct_fields = loading_exprs.iter().map(|(field_name, idx, _)| {
            let name = field_name.clone().unwrap();

            let idx = syn::Index::from(*idx);

            quote! {
              #name: temp_tuple.#idx.unwrap()
            }
        });

        quote! {
          #struct_name {
             #(#build_struct_fields ,)*
          }
        }
    } else {
        quote! {
          #struct_name
        }
    };

    if struct_type != StructType::Unit {
        quote! {
          {
            let temp_tuple = (#(#loading_exprs_vals ,)*);

            let field_errors = vec![#(#errors_gathering,)*].iter().cloned().flatten().cloned().collect::<Vec<_>>();
            if field_errors.is_empty() {
              Ok(#struct_expr)
            } else {
              Err(tryphon::ConfigError {
                field_errors
              })
            }
          }
        }
    } else {
        quote! {
          Ok(#struct_expr)
        }
    }
}

#[proc_macro_derive(Config, attributes(env, default, config))]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = ast.ident;

    let mut compile_errors_stream = TokenStream::new();

    let building_expr = match ast.data {
        Data::Struct(syn::DataStruct { ref fields, .. }) => {
            let name = quote! { #struct_name };

            build_loading_for_struct(name, fields.iter().collect(), &mut compile_errors_stream)
        }
        Data::Enum(syn::DataEnum { ref variants, .. }) => {
            let building_exprs = variants
                .iter()
                .map(|v| {
                    let variant_name = &v.ident;
                    let full_variant_name = format!("{struct_name}::{variant_name}");

                    let path: Path = syn::parse_str(&full_variant_name).unwrap();

                    let name = quote! { #path };

                    build_loading_for_struct(
                        name,
                        v.fields.iter().collect(),
                        &mut compile_errors_stream,
                    )
                })
                .collect::<Vec<_>>();

            let mut iter = building_exprs.iter();
            let mut acc = iter.next().expect("Expecting at least one element").clone();

            for next in iter {
                acc = quote! {
                  #acc.or_else(|_| { #next })
                };
            }

            acc
        }
        Data::Union(_) => {
            Error::new(Span::call_site(), "Union type is not supported!").to_compile_error()
        }
    };

    if compile_errors_stream.is_empty() {
        quote! {
          impl tryphon::Config for #struct_name {

              fn load() -> Result<Self, tryphon::ConfigError> {
                #building_expr
              }
          }
        }
        .into()
    } else {
        compile_errors_stream
    }
}

#[proc_macro_derive(ConfigValueDecoder)]
pub fn derive_config_value_decoder(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ast.data {
        Data::Enum(syn::DataEnum { ref variants, .. }) => {
            let enum_name = ast.ident;

            let mut cases = vec![];

            for variant in variants {
                if variant.fields.is_empty() {
                    let variant_name = &variant.ident.to_string();

                    let variant_name_lowercased = variant_name.to_lowercase();

                    let full_variant_name = format!("{enum_name}::{variant_name}");

                    let path: Path = syn::parse_str(&full_variant_name).unwrap();

                    cases.push(quote! {
                      #variant_name_lowercased => std::result::Result::Ok(#path)
                    });
                } else {
                    return Error::new(
                        Span::call_site(),
                        "You can only derive ConfigValueDecoder for enums without fields",
                    )
                    .to_compile_error()
                    .into();
                }
            }

            quote! {
              impl tryphon::ConfigValueDecoder for #enum_name {
                fn decode(raw: String) -> Result<Self, String> {
                    match raw.to_lowercase().as_str() {
                      #(#cases ,)*
                      _ => Err(format!("Invalid log level: {}", raw)),
                    }

                }
              }
            }
            .into()
        }
        _ => Error::new(
            Span::call_site(),
            "You can only derive ConfigValueDecoder for enums without fields",
        )
        .to_compile_error()
        .into(),
    }
}
