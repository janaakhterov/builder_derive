extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn builder_derive(ast: TokenStream) -> TokenStream {
    let ast = syn::parse(ast).unwrap();

    impl_builder(&ast)
}

fn impl_builder(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl Builder for #name {
            fn builder() {
                println!("Hello, I'm a builder");
            }
        }
    };

    gen.into()
}
