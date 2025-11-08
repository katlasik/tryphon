use proc_macro2::Ident;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{Type, TypePath};

pub(crate) fn is_option(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        let ident = &path
            .segments
            .first()
            .expect("Expecting at least 1 path segment")
            .ident;
        ident == "Option"
    } else {
        false
    }
}

pub(crate) fn ident_opt_to_str(field_name: &Option<Ident>) -> TokenStream2 {
    match field_name {
        Some(ident) => {
            let str = ident.to_string();
            quote! { Some(#str.to_string()) }
        }
        _ => quote! { None },
    }
}
