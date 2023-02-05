use std::{collections::HashMap, fs};

use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use rust_format::{Formatter, RustFmt};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Item {
    id: u32,
    name: String,
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "stackSize")]
    stack_size: u32,
}

pub fn build_items() {
    let items = fs::read_to_string("./data/1.19/items.json").expect("Unable to open items file");
    let items: Vec<Item> = serde_json::from_str(items.as_str()).unwrap();
    println!("cargo:error={:#?}", items);
    let mut items_rs = TokenStream2::new();

    let start = std::time::Instant::now();
    items_rs.extend(quote! {
        pub trait Item {
            const ID: u32;
            const NAME: &'static str;
            const DISPLAY_NAME: &'static str;
            const STACK_SIZE: u32 = 64u32;
        }
    });

    let mut item_struct_idents = HashMap::new();

    for item in &items {
        let item_ident = Ident::new_raw(
            item.name.to_case(Case::Pascal).as_str(),
            proc_macro2::Span::call_site(),
        );

        item_struct_idents.insert(item.name.clone(), item_ident.clone());

        items_rs.extend(quote! {
            pub struct #item_ident;
        });

        let Item {
            id,
            name,
            display_name,
            stack_size,
        } = item;

        let stack_size = if *stack_size == 64 {
            quote! {}
        } else {
            quote! { const STACK_SIZE: u32 = #stack_size; }
        };

        items_rs.extend(quote! {
            impl Item for #item_ident {
                const ID: u32 = #id;
                const NAME: &'static str = #name;
                const DISPLAY_NAME: &'static str = #display_name;
                #stack_size
            }
        });
    }

    let mut enum_inner = TokenStream2::new();
    for item in &items {
        let item_ident = item_struct_idents.get(&item.name).unwrap();
        let variant_ident = Ident::new(
            item.name.to_case(Case::Pascal).as_str(),
            proc_macro2::Span::call_site(),
        );

        enum_inner.extend(quote! {
            #variant_ident (#item_ident),
        });
    }
    items_rs.extend(quote! {
        pub enum Items {
            #enum_inner
        }
    });

    let mut get_id_inner = TokenStream2::new();
    for item in &items {
        let item_ident = item_struct_idents.get(&item.name).unwrap();
        let variant_ident = Ident::new(
            item.name.to_case(Case::Pascal).as_str(),
            proc_macro2::Span::call_site(),
        );

        get_id_inner.extend(quote! {
            Self::#variant_ident(_) => #item_ident::ID,
        });
    }

    items_rs.extend(quote! {
        impl Items {
            pub fn get_id(&self) -> u32 {
                match self {
                    #get_id_inner
                }
            }
        }
    });

    let output = RustFmt::default().format_str(items_rs.to_string()).unwrap();
    eprintln!("Time = {:#?}", start.elapsed());

    // panic!();
    fs::write("./src/items.rs", output).expect("Unable to write items.rs");
}
