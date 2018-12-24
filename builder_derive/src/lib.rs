extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::Data;
use syn::DataStruct;
use syn::DeriveInput;
use syn::parse;
use syn::Fields;
use proc_macro2::Ident;
use proc_macro2::Span;

#[proc_macro_derive(Builder)]
pub fn builder_derive(ast: TokenStream) -> TokenStream {
    let ast = parse(ast).unwrap();

    impl_builder(&ast)
}

fn impl_builder_struct(data: &DataStruct) -> proc_macro2::TokenStream {
    let _fields = match &data.fields {
        Fields::Named(fields) => {
            fields.named.iter()
                .map(|field| {
                    let ident = &field.ident;

                    // TODO: Not sure if clone is necessary
                    let ident = ident.clone().unwrap();

                    let ty = &field.ty;
                    quote!{ 
                        #ident: Option<#ty>
                    }
                }).collect::<Vec<proc_macro2::TokenStream>>()
        },
        _ => unimplemented!(),
    };

    quote!{
        #(#_fields),*
    }
}

fn impl_builder(ast: &DeriveInput) -> TokenStream {
    let body = match &ast.data {
        Data::Struct(data) => impl_builder_struct(data),
        _ => unimplemented!(),
    };

    let name = &ast.ident;
    let name = format!("{}Builder", name);
    let name = Ident::new(&name, Span::call_site());

    let gen = quote! {
        #[derive(Default, Debug)]
        pub struct #name {
            #body
        }

        impl #name {
            fn build() {
                println!("Hello, I'm a builder for TestBuilder");
            }
        }
    };

    gen.into()
}
