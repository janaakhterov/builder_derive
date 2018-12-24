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
use proc_macro2::TokenStream as TS;

#[proc_macro_derive(Builder)]
pub fn builder_derive(ast: TokenStream) -> TokenStream {
    let ast = parse(ast).unwrap();

    impl_builder(&ast)
}

fn impl_builder_struct_fields(data: &DataStruct) -> TS {
    let _fields = match &data.fields {
        Fields::Named(fields) => {
            fields.named.iter()
                .map(|field| {
                    // TODO: Not sure if clone is necessary
                    let ident = &field.ident.clone().unwrap();
                    let ty = &field.ty;
                    quote!{ #ident: Option<#ty> }
                }).collect::<Vec<TS>>()
        },
        _ => unimplemented!(),
    };

    quote!{
        #(#_fields),*
    }
}

fn impl_builder_struct_setters(data: &DataStruct) -> TS {
    let _setters = match &data.fields {
        Fields::Named(fields) => {
            fields.named.iter()
                .map(|field| {
                    // TODO: Not sure if clone is necessary
                    let ident = &field.ident.clone().unwrap();
                    let ty = &field.ty;
                    let gen = quote! {
                        fn #ident(&mut self, value: #ty) -> &mut Self {
                            self.#ident = Some(value);
                            self
                        }
                    };

                    // panic!(gen.to_string());
                    gen
                }).collect::<Vec<TS>>()
        },
        _ => unimplemented!(),
    };

    quote!{
        #(#_setters)*
    }
}

fn impl_builder(ast: &DeriveInput) -> TokenStream {
    let (fields, setters) = match &ast.data {
        Data::Struct(data) => {
            (impl_builder_struct_fields(data),
            impl_builder_struct_setters(data))
        },
        _ => unimplemented!(),
    };

    let name = format!("{}Builder", &ast.ident);
    let name = Ident::new(&name, Span::call_site());

    let gen = quote! {
        #[derive(Default, Debug)]
        pub struct #name {
            #fields
        }

        impl #name {
            fn build() {
                println!("Hello, I'm a builder for TestBuilder");
            }

            #setters
        }
    };

    gen.into()
}
