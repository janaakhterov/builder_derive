extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::Data;
use syn::DataStruct;
use syn::DeriveInput;
use syn::parse;
use proc_macro2::Ident;
use proc_macro2::Span;

#[proc_macro_derive(Builder)]
pub fn builder_derive(ast: TokenStream) -> TokenStream {
    let ast = parse(ast).unwrap();

    impl_builder(&ast)
}

fn impl_builder_struct(data: &DataStruct) -> TokenStream {

    let gen = quote!{
    
    };
    
    gen.into()
}

fn impl_builder(ast: &DeriveInput) -> TokenStream {
    let _body = match &ast.data {
        Data::Struct(data) => impl_builder_struct(data),
        _ => unimplemented!(),
    };

    let name = &ast.ident;
    let name = format!("{}Builder", name);
    let name = Ident::new(&name, Span::call_site());

    let gen = quote! {
        #[derive(Default)]
        pub struct #name {
            test: Option<i64>,
        }

        impl #name {
            fn build() {
                println!("Hello, I'm a builder for TestBuilder");
            }
        }
    };

    gen.into()
}
