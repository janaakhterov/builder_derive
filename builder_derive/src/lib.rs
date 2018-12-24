extern crate proc_macro;

mod builder;
mod utils;

use crate::proc_macro::TokenStream;
use syn::Data;
use syn::DeriveInput;
use syn::parse;
use crate::builder::impl_builder_struct;

#[proc_macro_derive(Builder)]
pub fn builder_derive(ast: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(ast).unwrap();

    match &ast.data {
        Data::Struct(data) => impl_builder_struct(&data, &ast.ident).into(),
        _ => unimplemented!(),
    }
}


