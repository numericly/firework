use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::Span;
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
                    #name::#variant_ident => protocol_core::SerializeField::serialize(&<#typ as TryFrom<i32>>::try_from(#index).expect("Error converting variant identifier"), &mut writer),
                }
            }
            Fields::Named(fields) => {
                let param_fields = fields.named.iter().map(|it| &it.ident);
                let serializable_fields = fields.named.iter().map(|it| &it.ident);
                quote! {
                    #name::#variant_ident { #(#param_fields,)*} => 
                        {
                            protocol_core::SerializeField::serialize(&<#typ as TryFrom<i32>>::try_from(#index).expect("Error converting variant identifier"), &mut writer);
                            #(
                                #serializable_fields.serialize(&mut writer);
                            )*
                        }
                }
            }
            Fields::Unnamed(fields) => {
                let fields_count: usize = fields.unnamed.len();
                let mut args_stream = TokenStream2::new();

                let idents = {
                    let mut idents = Vec::with_capacity(fields_count);
                    for i in 0..fields_count {
                        let ident = Ident::new(&format!("field{}", i), Span::call_site());
                        args_stream.extend(quote!{ #ident, });
                        idents.push(ident);
                    }
                    idents
                };
                quote! {
                    #name::#variant_ident(#args_stream) => {
                        protocol_core::SerializeField::serialize(&<#typ as TryFrom<i32>>::try_from(#index).expect("Error converting variant identifier"), &mut writer);
                        #(
                            #idents.serialize(&mut writer);
                        )*
                    },
                }
            }
        });
    }

    quote! {
        impl protocol_core::SerializeField for #name {
            fn serialize<W: std::io::Write>(&self, mut writer: W) {
                match self {
                    #match_arms
                }
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
    let field_idents = fields.named.iter().map(|field| &field.ident);

    quote! {
        impl protocol_core::SerializeField for #name {
            fn serialize<W: std::io::Write>(&self, mut writer: W) {
                #(
                    self.#field_idents.serialize(&mut writer);
                )*
            }
        }
    }
}

fn handle_unnamed_struct(fields: FieldsUnnamed, name: &Ident) -> TokenStream2 {
    let fields_count: usize = fields.unnamed.len();
    let mut args_stream = TokenStream2::new();


    for i in 0..fields_count {
        let index = syn::Index::from(i);
        args_stream.extend(quote! {self.#index.serialize(&mut writer); });
    }

    quote! {
        impl protocol_core::SerializeField for #name {
            fn serialize<W: std::io::Write>(&self, mut writer: W) {
                #args_stream
            }
        }
    }
}
