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
use failure::Error;
use failure::err_msg;
use std::result::Result;

#[proc_macro_derive(Builder)]
pub fn builder_derive(ast: TokenStream) -> TokenStream {
    let ast = parse(ast).unwrap();

    match &ast.data {
        Data::Struct(data) => impl_builder_struct(data, &ast.ident).into(),
        _ => unimplemented!(),
    }
}

fn unzip(vec: Vec<(TS, TS, TS, TS)>) -> (Vec<TS>, Vec<TS>, Vec<TS>, Vec<TS>) {
    let mut first: Vec<TS> = Vec::new();
    let mut second: Vec<TS> = Vec::new();
    let mut third: Vec<TS> = Vec::new();
    let mut forth: Vec<TS> = Vec::new();


    for (a, b ,c ,d) in vec {
        first.push(a);
        second.push(b);
        third.push(c);
        forth.push(d);
    }

    (first, second, third, forth)
}

fn impl_builder_struct(data: &DataStruct, derived_struct: &Ident) -> TS {
    let name = format!("{}Builder", derived_struct);
    let name = Ident::new(&name, derived_struct.span());

    let (_fields, 
         _setters, 
         _check_options, 
         _idents_for_struct): (Vec<TS>, Vec<TS>, Vec<TS>, Vec<TS>) = match &data.fields {
        Fields::Named(fields) => {
            unzip(fields.named.iter().map(|field| {
                // TODO: Not sure if clone is necessary
                let ident = &field.ident.clone().unwrap();
                let ty = &field.ty;

                let option_field = quote!{ #ident: Option<#ty> };

                let setter = quote! {
                    fn #ident(&mut self, value: #ty) -> &mut Self {
                        self.#ident = Some(value);
                        self
                    }
                };

                let check_option = quote! {
                    let #ident = self.#ident
                        .expect("Builder wasn't completely initialized");
                };

                let ident_for_struct = quote!{ #ident };

                (option_field, setter, check_option, ident_for_struct)
            }).collect::<Vec<(TS, TS, TS, TS)>>())
        },
        _ => unimplemented!(),
    };

    let gen = quote! {
        #[derive(Default, Debug)]
        pub struct #name {
            #(#_fields),*
        }

        impl #name {
            #(#_setters)*

            fn build(self) -> Result<#derived_struct, Error> {
                #(#_check_options)*

                Ok(#derived_struct {
                    #(#_idents_for_struct),*
                })
            }
        }
    };

    gen
}
