use firework::{
    data::items::Item,
    protocol::data_types::{Enchantment, ItemNbt, StackContents},
};
use rand::random;

const ARMOR_LOOT: [Item; 20] = [
    // iron armor, gold armor, leather armor, chain armor, diamond armor
    Item::IronHelmet,
    Item::IronChestplate,
    Item::IronLeggings,
    Item::IronBoots,
    Item::GoldenHelmet,
    Item::GoldenChestplate,
    Item::GoldenLeggings,
    Item::GoldenBoots,
    Item::LeatherHelmet,
    Item::LeatherChestplate,
    Item::LeatherLeggings,
    Item::LeatherBoots,
    Item::ChainmailHelmet,
    Item::ChainmailChestplate,
    Item::ChainmailLeggings,
    Item::ChainmailBoots,
    Item::DiamondHelmet,
    Item::DiamondChestplate,
    Item::DiamondLeggings,
    Item::DiamondBoots,
];

const WEAPON_LOOT: [Item; 25] = [
    // all variants of swords, axes, pickaxes, shovels, hoes
    Item::WoodenSword,
    Item::StoneSword,
    Item::IronSword,
    Item::GoldenSword,
    Item::DiamondSword,
    Item::WoodenAxe,
    Item::StoneAxe,
    Item::IronAxe,
    Item::GoldenAxe,
    Item::DiamondAxe,
    Item::WoodenPickaxe,
    Item::StonePickaxe,
    Item::IronPickaxe,
    Item::GoldenPickaxe,
    Item::DiamondPickaxe,
    Item::WoodenShovel,
    Item::StoneShovel,
    Item::IronShovel,
    Item::GoldenShovel,
    Item::DiamondShovel,
    Item::WoodenHoe,
    Item::StoneHoe,
    Item::IronHoe,
    Item::GoldenHoe,
    Item::DiamondHoe,
];

pub fn generate_chest_loot(_items: i32) -> Vec<Option<StackContents>> {
    let mut chest_contents = vec![None; 27];
    for _ in 0..4 {
        if !chest_contents.contains(&None) {
            break;
        }
        let mut index = rand::random::<usize>() % 27;
        while chest_contents[index].is_some() {
            index = rand::random::<usize>() % 27;
        }
        let item;
        let item_nbt = if random() {
            item = ARMOR_LOOT[rand::random::<usize>() % ARMOR_LOOT.len()];
            let enchantments = if random() {
                None
            } else {
                let level = if random() {
                    1
                } else if random() {
                    2
                } else if random() {
                    3
                } else {
                    4
                };
                Some(vec![Enchantment {
                    id: "minecraft:protection".to_string(),
                    level,
                }])
            };

            ItemNbt {
                enchantments,
                ..Default::default()
            }
        } else {
            item = WEAPON_LOOT[rand::random::<usize>() % WEAPON_LOOT.len()];
            let enchantments = if random() {
                None
            } else {
                let level = if random() {
                    1
                } else if random() {
                    2
                } else if random() {
                    3
                } else if random() {
                    4
                } else {
                    5
                };
                Some(vec![Enchantment {
                    id: "minecraft:sharpness".to_string(),
                    level,
                }])
            };
            ItemNbt {
                enchantments,
                ..Default::default()
            }
        };
        chest_contents[index] = Some(StackContents {
            id: item,
            count: 1,
            nbt: item_nbt,
        });
    }
    chest_contents
}
