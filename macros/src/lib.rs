//! Macros for AlgoliaObject

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    DeriveInput,
};

#[proc_macro_derive(AlgoliaObject)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    todo!()
}
