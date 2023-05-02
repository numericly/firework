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
    let items = fs::read_to_string("./data/1.19.4/items.json").expect("Unable to open items file");
    let items: Vec<Item> = serde_json::from_str(items.as_str()).unwrap();
    println!("cargo:error={:#?}", items);
    let mut items_rs = TokenStream2::new();

    let mut item_enum_inner = TokenStream2::new();
    let mut get_id_inner = TokenStream2::new();
    let mut get_name_inner = TokenStream2::new();
    let mut get_display_name_inner = TokenStream2::new();
    let mut get_stack_size_inner = TokenStream2::new();
    let mut from_item_id_inner = TokenStream2::new();

    for item in &items {
        let item_ident = Ident::new_raw(
            item.name.to_case(Case::Pascal).as_str(),
            proc_macro2::Span::call_site(),
        );

        item_enum_inner.extend(quote! {
            #item_ident,
        });

        let Item {
            id,
            name,
            display_name,
            stack_size,
        } = item;

        get_id_inner.extend(quote! {
            Self::#item_ident => #id,
        });
        get_name_inner.extend(quote! {
            Self::#item_ident => #name,
        });
        get_display_name_inner.extend(quote! {
            Self::#item_ident => #display_name,
        });
        get_stack_size_inner.extend(quote! {
            Self::#item_ident => #stack_size,
        });
        from_item_id_inner.extend(quote! {
            #id => Some(Self::#item_ident),
        });
    }

    items_rs.extend(quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Item {
            #item_enum_inner
        }

        impl Item {
            pub fn get_id(&self) -> u32 {
                match self {
                    #get_id_inner
                }
            }
            pub fn get_name(&self) -> &str {
                match self {
                    #get_name_inner
                }
            }
            pub fn get_display_name(&self) -> &str {
                match self {
                    #get_display_name_inner
                }
            }
            pub fn get_stack_size(&self) -> u32 {
                match self {
                    #get_stack_size_inner
                }
            }
            pub fn from_id(id: u32) -> Option<Self> {
                match id {
                    #from_item_id_inner
                    _ => None
                }
            }
        }
    });

    let output = RustFmt::default().format_str(items_rs.to_string()).unwrap();

    // panic!();
    fs::write("./src/items.rs", output).expect("Unable to write items.rs");
}
