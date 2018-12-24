use crate::utils::unzip;
use proc_macro2::Ident;
use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::DataStruct;
use syn::Fields;

pub(crate) fn impl_builder_struct(data: &DataStruct, derived_struct: &Ident) -> TS {
    let name = format!("{}Builder", derived_struct);
    let name = Ident::new(&name, derived_struct.span());

    let (_fields, _setters, _check_options, _idents_for_struct): (
        Vec<TS>,
        Vec<TS>,
        Vec<TS>,
        Vec<TS>,
    ) = match &data.fields {
        Fields::Named(fields) => {
            unzip(
                fields
                    .named
                    .iter()
                    .map(|field| {
                        // TODO: Not sure if clone is necessary
                        let ident = &field.ident.clone().unwrap();
                        let ty = &field.ty;

                        let option_field = quote! { #ident: Option<#ty> };

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

                        let ident_for_struct = quote! { #ident };

                        (option_field, setter, check_option, ident_for_struct)
                    })
                    .collect::<Vec<(TS, TS, TS, TS)>>(),
            )
        }
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
