use proc_macro::TokenStream;
use quote::*;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: syn::ItemStruct = syn::parse(input).unwrap();
    let name = input.ident;
    let mut fields = vec![];
    let mut default_fields = vec![];
    let mut field_names = vec![];
    let mut methods = vec![];
    for field in input.fields {
        let ty = field.ty;
        let field_name = field.ident;

        field_names.push(quote! {
            println!("{}", stringify!(#field_name));
        });

        methods.push(quote! {
            fn #field_name(&mut self, #field_name: #ty) -> &mut Self {
                self.#field_name = Some(#field_name);
                self
            }
        });
        
        let x = quote! {
            #field_name: Option<#ty>
        };
        fields.push(x);

        let y = quote! {
            #field_name: None
        };
        default_fields.push(y);
    }

    let builder_name = format_ident!("{}Builder", name);
    let tokens = quote! {

        pub struct #builder_name {
            #(#fields),*
        }

        // #name is the struct name(Command)
        impl #name {
            pub fn builder() -> #builder_name {
                // #builder_name is the struct name(CommandBuilder)
                #builder_name {
                    #(#default_fields),*
                }
            }

            pub fn println() {
                #(#field_names)*
            }
        }

        // #builder_name is the struct name(CommandBuilder)
        impl #builder_name {
            #(#methods)*
        }
    };
    tokens.into()
}
