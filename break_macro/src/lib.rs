extern crate proc_macro;

use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Clear)]
pub fn derive_answer_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    input.ident = Ident::new(&(input.ident.to_string() + "Empty"), Span::call_site());
    match input.data {
        syn::Data::Struct(ref mut val) => val.fields = syn::Fields::Unit,
        syn::Data::Enum(ref mut val) => val.variants.clear(),
        syn::Data::Union(_) => panic!("Empty unions are not allowed in rust"),
    };
    input.to_token_stream().into()
}
