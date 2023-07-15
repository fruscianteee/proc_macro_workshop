use proc_macro::TokenStream;
use quote::*;
use syn::DeriveInput;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = input.ident;
    let tokens = quote! {
        impl #name {
            pub fn builder() { }
        }
    };
    tokens.into()
}
