use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::FieldsNamed;
use syn::FieldsUnnamed;
use syn::parse_macro_input;
use syn::Fields;
use syn::{Data, DeriveInput};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(protocol))]
struct EnumOpts {
    typ: syn::Type,
}

pub fn derive_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match &input.data {
        Data::Enum(_) => gen_impl_enum(input),
        Data::Struct(_) => gen_impl_struct(input),
        _ => panic!("only enums are supported"),
    }
    .into()
}

fn gen_impl_enum(input: DeriveInput) -> TokenStream2 {
    let EnumOpts { typ } = EnumOpts::from_derive_input(&input).expect("Wrong options");
    let mut match_arms = TokenStream2::new();

    let name = &input.ident;

    let Data::Enum(enum_data) = input.data else {
        panic!("only enums are supported");
    };

    for (i, variant) in enum_data.variants.iter().enumerate() {
        let variant_ident = &variant.ident;
        let index = syn::Index::from(i);
        match_arms.extend(match &variant.fields {
            Fields::Unit => {
                quote! {
                    #index => #name::#variant_ident,
                }
            }
            Fields::Named(fields) => {
                let field_idents = fields.named.iter().map(|it| &it.ident);
                quote! {
                    #index => 
                        #name::#variant_ident {
                            #(
                                #field_idents: firework_protocol_core::DeserializeField::deserialize(&mut reader)?,
                            )*
                        },
                }
            }
            Fields::Unnamed(fields) => {
                let fields_count: usize = fields.unnamed.len();
                let mut args_stream = TokenStream2::new();

                for _ in 0..fields_count {
                    args_stream.extend(quote! {firework_protocol_core::DeserializeField::deserialize(&mut reader)?, });
                }

                quote! {
                    #index => #name::#variant_ident(#args_stream),
                }
            }
        });
    }

    quote! {
        impl firework_protocol_core::DeserializeField for #name {
            fn deserialize<R: std::io::Read>(mut reader: R) -> Result<Self, firework_protocol_core::DeserializeError>  {
                let variant_index: i32 = <i32 as TryFrom<#typ>>::try_from(firework_protocol_core::DeserializeField::deserialize(&mut reader)?).unwrap();
                Ok(match variant_index {
                    #match_arms
                    variant => return Err(firework_protocol_core::DeserializeError::InvalidVariantIndex(variant, stringify!(#name))),
                })
            }
        }
    }
}

fn gen_impl_struct(input: DeriveInput) -> TokenStream2 {
    let name = &input.ident;
    let Data::Struct(struct_data) = input.data else {
        panic!("only enums are supported");
    };
    match struct_data.fields {
        Fields::Named(fields) => handle_named_struct(fields, name),
        Fields::Unnamed(fields) => handle_unnamed_struct(fields, name),
        Fields::Unit => panic!("unit structs are not supported"),
    }
}

fn handle_named_struct(fields: FieldsNamed, name: &Ident) -> TokenStream2 {
    let field_idents = fields.named.iter().map(|it| &it.ident);

    quote! {
        impl firework_protocol_core::DeserializeField for #name {
            fn deserialize<R: std::io::Read>(mut reader: R) -> Result<Self, firework_protocol_core::DeserializeError>  {
                Ok(Self {
                    #(
                        #field_idents: firework_protocol_core::DeserializeField::deserialize(&mut reader)?,
                    )*
                })
            }
        }
    }
}

fn handle_unnamed_struct(fields: FieldsUnnamed, name: &Ident) -> TokenStream2 {
    let fields_count: usize = fields.unnamed.len();
    let mut args_stream = TokenStream2::new();

    for _ in 0..fields_count {
        args_stream.extend(quote! {firework_protocol_core::DeserializeField::deserialize(&mut reader)?, });
    }

    quote! {
        impl firework_protocol_core::DeserializeField for #name {
            fn deserialize<R: std::io::Read>(mut reader: R) -> Result<Self, firework_protocol_core::DeserializeError>  {
                Ok(Self(#args_stream))
            }
        }
    }
}
