extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field};

#[proc_macro_derive(Clear)]
pub fn derive_clear_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let empty_ident = Ident::new(&(input.ident.to_string() + "Clear"), input.ident.span());
    let quote = match input.data {
        syn::Data::Struct(_) => quote! { struct #empty_ident; },
        syn::Data::Enum(_) => quote! { enum #empty_ident {}},
        syn::Data::Union(_) => panic!("You can't have empty unions in rust 😊"),
    };
    quote.into()
}

fn remove_string_field(field: &&Field) -> bool {
    match &field.ty {
        syn::Type::Path(syn::TypePath {
            path: syn::Path { segments, .. },
            ..
        }) => {
            if &segments.iter().nth(0).unwrap().ident.to_string() == "String" {
                false
            } else {
                true
            }
        }
        _ => true,
    }
}

#[proc_macro_derive(ClearString)]
pub fn derive_clear_string_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let str_empty_ident = Ident::new(
        &(input.ident.to_string() + "ClearString"),
        input.ident.span(),
    );
    TokenStream::from(match input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(syn::FieldsNamed {
                brace_token: _,
                named,
            }) => {
                let named: Vec<_> = named
                    .iter()
                    .filter(|val| remove_string_field(val))
                    .collect();
                quote! {
                    struct #str_empty_ident {
                        #(#named),*
                    }
                }
            }
            syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
                let unnamed: Vec<_> = unnamed
                    .iter()
                    .filter(|val| remove_string_field(val))
                    .collect();
                quote! {
                    struct #str_empty_ident (#(#unnamed),*);
                }
            }
            syn::Fields::Unit => {
                quote! {
                    struct #str_empty_ident;
                }
            }
        },
        syn::Data::Enum(data) => {
            let variants: Vec<_> = data
                .variants
                .iter()
                .filter(|val| match &val.fields {
                    syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
                        remove_string_field(&unnamed.iter().nth(0).unwrap())
                    }
                    _ => true,
                })
                .collect();
            if variants.len() != 0 {
                quote! {
                    enum #str_empty_ident{
                        #(#variants),*
                    }
                }
            } else {
                quote! {
                    enum #str_empty_ident {}
                }
            }
        }
        syn::Data::Union(_) => panic!("Empty unions are not allowed in rust"),
    })
}
