extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, PathArguments, Type};

#[proc_macro_derive(Sign)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let fields = match &input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(&input, "Expected named fields")
                    .into_compile_error()
                    .into()
            }
        },
        _ => {
            return syn::Error::new_spanned(&input, "Expected a struct")
                .into_compile_error()
                .into()
        }
    };

    let mut check_field = false;
    for field in fields {
        if "sign".eq(&field.ident.as_ref().unwrap().to_string()) {
            check_field = is_vec_of_u8(&field.ty);
        }
    }
    if !check_field {
        return syn::Error::new_spanned(&input, "Expected Field \"sign: Vec<u8>\"")
            .into_compile_error()
            .into();
    }

    let tokens = quote! {
        impl Signature for #ident {
            fn get_sign(&self) -> &Vec<u8> {
                &self.sign
            }

            fn set_sign(&mut self, sign: Vec<u8>) {
                self.sign = sign;
            }
        }
    };

    tokens.into()
}

fn is_vec_of_u8(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            !(type_path.path.segments.len() != 1
                || type_path.path.segments[0].ident != "Vec"
                || !match &type_path.path.segments[0].arguments {
                    PathArguments::AngleBracketed(angle_bracketed) => {
                        if angle_bracketed.args.len() == 1 {
                            if let syn::GenericArgument::Type(Type::Path(inner_path)) =
                                &angle_bracketed.args[0]
                            {
                                return inner_path.path.segments.len() == 1
                                    && inner_path.path.segments[0].ident == "u8";
                            }
                        }
                        false
                    }
                    _ => false,
                })
        }
        _ => false,
    }
}
