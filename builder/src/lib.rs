use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _ = input;
    let ast = parse_macro_input!(input as DeriveInput);
    // eprintln!("{:#?}", &ast.data);
    let name = ast.ident.clone();
    let builder_name = syn::Ident::new(&format!("{}Builder", name), name.span());
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed {
            named: ref field, ..
        }),
        ..
    }) = ast.data
    {
        field
    } else {
        unimplemented!();
    };
    let fields_optionized = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote!(#name: std::option::Option<#ty>)
    });
    let setter_methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote!(
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        )
    });
    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote!(
            #name: self.#name.clone().ok_or(concat!('\'', stringify!(#name), "\' is none"))?
        )
    });
    let default_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote!(
            #name: None
        )
    });
    quote!(
        pub struct #builder_name {
            #(#fields_optionized,)*
        }
        impl #builder_name {
            #(#setter_methods)*

            pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(#name {
                    #(#build_fields,)*
                })
            }
        }
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#default_fields,)*
                }
            }
        }
    ).into()
    // quote! {
    //     pub struct #builder_name {
    //         #(#fields_optionized,)*
    //     }
    //     impl #builder_name {
    //         #(#setter_methods)*

    //         pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
    //             Ok(#name {
    //                 #(#build_fields,)*
    //             })
    //         }
    //     }

    //     impl #name {
    //         pub fn builder() -> #builder_name {
    //             #builder_name {
    //                 #(#default_fields,)*
    //             }
    //         }
    //     }
    // }
    // .into()
    // TokenStream::new().into()
}
