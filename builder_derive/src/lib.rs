extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::parse;
use proc_macro2::Ident;
use proc_macro2::Span;

#[proc_macro_derive(Builder)]
pub fn builder_derive(ast: TokenStream) -> TokenStream {
    let ast = parse(ast).unwrap();

    impl_builder(&ast)
}

fn impl_builder(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let builder_name = format!("{}Builder", name);
    let derive_default = "#[derive(Default)]";
    let ident = Ident::new(&builder_name, Span::call_site());

    let gen = quote! {
        impl #name {
            fn build() {
                println!("Hello, I'm a builder");
            }
        }

        pub struct #ident {
            test: Option<i64>,
        }
    };

    gen.into()
}
