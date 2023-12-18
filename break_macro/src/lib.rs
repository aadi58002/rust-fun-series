extern crate proc_macro;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Clear)]
pub fn derive_clear_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let empty_ident = Ident::new(&(input.ident.to_string() + "Empty"), Span::call_site());
    let quote = match input.data {
        syn::Data::Struct(_) => quote! { struct #empty_ident; },
        syn::Data::Enum(_) => quote! { enum #empty_ident {}},
        syn::Data::Union(_) => panic!("You can't have empty unions in rust 😊"),
    };
    quote.into()
}
