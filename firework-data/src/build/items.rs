use std::fs;

use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use rust_format::{Formatter, RustFmt};
use serde::Deserialize;

struct QuoteOption<T>(Option<T>);

impl<T: ToTokens> ToTokens for QuoteOption<T> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append_all(match self.0 {
            Some(ref t) => quote! { ::std::option::Option::Some(#t) },
            None => quote! { ::std::option::Option::None },
        });
    }
}

#[derive(Deserialize, Debug, Clone)]
struct Item {
    id: i32,
    name: String,
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "stackSize")]
    stack_size: usize,
    #[serde(rename = "attackDamage")]
    attack_damage: Option<f32>,
    #[serde(rename = "attackSpeed")]
    attack_speed: Option<f32>,
    #[serde(rename = "armorDefense")]
    armor_defense: Option<i32>,
    #[serde(rename = "armorToughness")]
    armor_toughness: Option<i32>,
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
    let mut get_attack_damage_inner = TokenStream2::new();
    let mut get_attack_speed_inner = TokenStream2::new();
    let mut get_armor_defense_inner = TokenStream2::new();
    let mut get_armor_toughness_inner = TokenStream2::new();
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
            attack_damage,
            attack_speed,
            armor_defense,
            armor_toughness,
        } = item;

        let attack_damage = QuoteOption(*attack_damage);
        let attack_speed = QuoteOption(*attack_speed);
        let armor_defense = QuoteOption(*armor_defense);
        let armor_toughness = QuoteOption(*armor_toughness);

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
        get_attack_damage_inner.extend(quote! {
            Self::#item_ident => #attack_damage,
        });
        get_attack_speed_inner.extend(quote! {
            Self::#item_ident => #attack_speed,
        });
        get_armor_defense_inner.extend(quote! {
            Self::#item_ident => #armor_defense,
        });
        get_armor_toughness_inner.extend(quote! {
            Self::#item_ident => #armor_toughness,
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
            pub fn get_id(&self) -> i32 {
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
            pub fn get_stack_size(&self) -> usize {
                match self {
                    #get_stack_size_inner
                }
            }
            pub fn from_id(id: i32) -> Option<Self> {
                match id {
                    #from_item_id_inner
                    _ => None
                }
            }
            // dude sorry about build times 💀 i like doubled the data crate
            pub fn get_attack_damage(&self) -> Option<f32> {
                match self {
                    #get_attack_damage_inner
                }
            }
            pub fn get_attack_speed(&self) -> Option<f32> {
                match self {
                    #get_attack_speed_inner
                }
            }
            pub fn get_armor_defense(&self) -> Option<i32> {
                match self {
                    #get_armor_defense_inner
                }
            }
            pub fn get_armor_toughness(&self) -> Option<i32> {
                match self {
                    #get_armor_toughness_inner
                }
            }
        }
    });

    let output = RustFmt::default().format_str(items_rs.to_string()).unwrap();

    // panic!();
    fs::write("./src/items.rs", output).expect("Unable to write items.rs");
}
