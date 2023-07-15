use proc_macro::TokenStream;
use quote::*;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: syn::ItemStruct = syn::parse(input).unwrap();
    let name = input.ident;
    let mut fields = vec![];
    let mut default_fields = vec![];
    for field in input.fields {
        let ty = field.ty;
        let ident = field.ident;
        let x = quote! {
            #ident: Option<#ty>
        };
        fields.push(x);

        let y = quote! {
            #ident: None
        };
        default_fields.push(y);
    }

    let builer_name = format_ident!("{}Builder", name);
    let tokens = quote! {

        pub struct #builer_name {
            #(#fields),*
        }

        impl #name {
            pub fn builder() -> #builer_name {
                #builer_name {
                    #(#default_fields),*
                }
            }
        }
    };
    tokens.into()
}
