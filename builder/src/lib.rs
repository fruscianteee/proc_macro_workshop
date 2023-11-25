use proc_macro::TokenStream;
use quote::*;
use syn::{parenthesized, Expr, Lit, LitStr, Meta, Path, Type};

// Option<syn::Type> から syn::Type を取り出す
// Type::Path(
//     TypePath {
//         qself: None,
//         path: Path {
//             segments: [
//                 PathSegment {
//                     ident: "Option",
//                     arguments: PathArguments::AngleBracketed(
//                         AngleBracketedGenericArguments {
//                             args: [
//                                 GenericArgument::Type(
//                                     ... // ここに `Option<T>`のTが入る
//                                 ),
//                             ],
//                         },
//                     ),
//                 },
//             ],
//         },
//     },
// )
fn get_type_in(ty: &Type, ident: &str) -> Option<Type> {
    if let Type::Path(ref typepath) = ty {
        let Some(seg) = typepath.path.segments.first() else {
            return None;
        };
        if seg.ident == ident {
            if let syn::PathArguments::AngleBracketed(ref inner_ty) = seg.arguments {
                if let Some(syn::GenericArgument::Type(ty)) = inner_ty.args.first() {
                    return Some(ty.clone());
                }
            }
        }
    }
    None
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: syn::ItemStruct = syn::parse(input).unwrap();
    let structure_name = input.ident;
    let mut fields = vec![];
    let mut methods = vec![];
    // Commandのフィールド
    let mut fields_for_build = vec![];
    // Command を文字列に変換して、lowercaseにして、format_ident!()でASTに戻す
    let lower_name = format_ident!("{}", structure_name.to_string().to_lowercase());

    for field in input.fields {
        let mut ty = field.ty;
        let inner_ty = get_type_in(&ty, "Option");
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

        fields_for_build.push(assign_field);

        // get attributes
        let attrs: Vec<_> = field
            .attrs
            .into_iter()
            .filter(|attr| {
                let ident = attr.path().get_ident();
                ident.map(|i| i == "builder").unwrap_or(false)
            })
            .collect();

        let mut generated_method = false;
        for attr in attrs {
            let inner_ty = get_type_in(&ty, "Vec");
            let Some(ref inner_ty) = inner_ty else {
                panic!("field {} is not Vec", field_name.unwrap());
            };

            let tokenstream = attr.meta.to_token_stream();
            let meta: syn::MetaList = syn::parse(tokenstream.into()).unwrap();
            let meta: syn::MetaNameValue =
                syn::parse(meta.tokens.to_token_stream().into()).unwrap();

            if let Expr::Lit(expr) = &meta.value {
                if let Lit::Str(lit_str) = &expr.lit {
                    let x: Path = lit_str.parse().unwrap();
                    let method_ident = x.get_ident().unwrap();

                    dbg!(&method_ident.into_token_stream());

                    methods.push(quote! {
                        fn #method_ident(&mut self, x: #inner_ty) -> &mut Self {
                            if self.#field_name.is_none() {
                                self.#field_name = Some(x);
                            }
                            self.#field_name.push(x);
                            self
                        }
                    });
                    generated_method = true;
                }
            }
        }

        if !generated_method {
            methods.push(quote! {
                fn #field_name(&mut self, #field_name: #ty) -> &mut Self {
                    self.#field_name = Some(#field_name);
                    self
                }
            });
        }

        let field = quote! {
            #field_name: Option<#ty>
        };
        fields.push(field);
    }

    let builder_name = format_ident!("{}Builder", structure_name);
    let tokens = quote! {
        #[derive(Default)]
        pub struct #builder_name {
            #(#fields),*
        }

        // #name is the struct name(Command)
        impl #structure_name {
            pub fn builder() -> #builder_name {
                // #builder_name is the struct name(CommandBuilder)
                #builder_name::default()
            }
        }

        // #builder_name is the struct name(CommandBuilder)
        impl #builder_name {
            #(#methods)*

            fn build(&self) -> Result<#structure_name, Box<dyn std::error::Error>> {
                let #lower_name = #structure_name {
                    #(#fields_for_build),*
                };
                Ok(#lower_name)
            }
        }
    };
    tokens.into()
}
