use proc_macro::TokenStream;
use quote::*;
use syn::Type;

fn get_typepath_in_option(ty: &Type) -> Option<Type> {
    match ty {
        Type::Path(ref typepath) => {
            let Some(seg) = typepath.path.segments.first() else { return None };
            if seg.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(ref inner_ty) = seg.arguments {
                    if let Some(syn::GenericArgument::Type(ty)) = inner_ty.args.first() {
                        return Some(ty.clone());
                    }
                }
                None
            } else {
                None
            }
        }
        _ => None,
    }
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: syn::ItemStruct = syn::parse(input).unwrap();
    let name = input.ident;
    let mut fields = vec![];
    let mut default_fields = vec![];
    let mut methods = vec![];
    // Commandのフィールド
    let mut assign_fields = vec![];
    // Command を文字列に変換して、lowercaseにして、format_ident!()でASTに戻す
    let lower_name = format_ident!("{}", name.to_string().to_lowercase());

    for field in input.fields {
        let mut ty = field.ty;
        let inner_ty = get_typepath_in_option(&ty);
        if let Some(ref inner_ty) = inner_ty {
            ty = inner_ty.clone();
        }

        let field_name = field.ident;

        let assign_field = if inner_ty.is_some() {
            quote! {
                #field_name: self.#field_name.clone()
            }
        } else {
            quote! {
                #field_name: self.#field_name.clone().ok_or(format!("not found {}", stringify!(#field_name)))?
            }
        };

        assign_fields.push(assign_field);

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
        }

        // #builder_name is the struct name(CommandBuilder)
        impl #builder_name {
            #(#methods)*

            fn build(&self) -> Result<#name, Box<dyn std::error::Error>> {
                let #lower_name = #name {
                    #(#assign_fields),*
                };
                Ok(#lower_name)
            }
        }
    };
    tokens.into()
}
