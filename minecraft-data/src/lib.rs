use crate::blocks::deserialize_content;
use blocks::{Block, GrassBlock};
use serde::{
    de::{self, IgnoredAny, MapAccess, Visitor},
    Deserialize, Deserializer,
    __private::de::{
        Content, ContentDeserializer, TagContentOtherField, TagContentOtherFieldVisitor,
        TagOrContentField,
    },
};
use std::{collections::HashMap, str::FromStr};

use crate::blocks::Stone;

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct ConstrainedInt<const MIN: u32, const MAX: u32>(pub u32);

impl<const MIN: u32, const MAX: u32> FromStr for ConstrainedInt<MIN, MAX> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.parse::<u32>().map_err(|e| e.to_string())?;
        if val > MAX {
            return Err(format!(
                "Value {} is greater than the maximum value {}",
                val, MAX
            ));
        }
        if val < MIN {
            return Err(format!(
                "Value {} is less than the minimum value {}",
                val, MIN
            ));
        }

        Ok(Self(val))
    }
}

pub trait BlockProperties {
    const DISPLAY_NAME: &'static str;
    const HARDNESS: f32;
    const RESISTANCE: f32;
    const STACK_SIZE: u8;
    const DIGGABLE: bool;
    const TRANSPARENT: bool;
    const EMIT_LIGHT: u8;
    const FILTER_LIGHT: u8;
}

#[derive(Debug)]
pub enum Blocks {
    Stone(Stone),
    GrassBlock(GrassBlock),
}

// fn deserialize_content<'de, T: MapAccess<'de>>(
//     tag: &str,
//     map: Option<HashMap<String, String>>,
// ) -> Result<Blocks, T::Error> {
//     match tag {
//         "minecraft:stone" => Ok(Blocks::Stone(Stone {})),
//         "minecraft:grass_block" => {
//             let Some(map) = map else {
//                 return Err(de::Error::custom("Missing properties for \"grass_block\""))
//             };
//             Ok(Blocks::GrassBlock(GrassBlock {
//                 snowy: if let Some(snowy) = map.get("snowy") {
//                     snowy.parse().map_err(de::Error::custom)?
//                 } else {
//                     return Err(de::Error::custom(
//                         "missing property \"snowy\" in \"grass_block\"",
//                     ));
//                 },
//             }))
//         }
//         _ => Err(de::Error::custom(format!("unknown block: {}", tag))),
//     }
// }

impl<'de> Deserialize<'de> for Block {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BlockVisitor;

        impl<'de> Visitor<'de> for BlockVisitor {
            type Value = Block;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a block")
            }

            fn visit_map<T: MapAccess<'de>>(self, mut map: T) -> Result<Self::Value, T::Error> {
                fn read_key<'de, T: MapAccess<'de>>(
                    map: &mut T,
                ) -> Result<Option<TagOrContentField>, T::Error> {
                    let mut tag_or_content: Option<TagOrContentField> = None;
                    while let Some(key) = MapAccess::next_key_seed(
                        map,
                        TagContentOtherFieldVisitor {
                            tag: "Name",
                            content: "Properties",
                        },
                    )? {
                        match key {
                            TagContentOtherField::Other => {
                                MapAccess::next_value::<IgnoredAny>(map)?;
                                continue;
                            }
                            TagContentOtherField::Tag => {
                                tag_or_content = Some(TagOrContentField::Tag);
                                break;
                            }
                            TagContentOtherField::Content => {
                                tag_or_content = Some(TagOrContentField::Content);
                                break;
                            }
                        }
                    }
                    Ok(tag_or_content)
                }

                match read_key(&mut map)? {
                    Some(TagOrContentField::Tag) => {
                        let tag = MapAccess::next_value::<&str>(&mut map)?;
                        match read_key(&mut map)? {
                            Some(TagOrContentField::Content) => {
                                let map = map.next_value()?;
                                deserialize_content::<T>(&tag, Some(&map))
                                    .map_err(|e| de::Error::custom(e))
                            }
                            Some(TagOrContentField::Tag) => Err(de::Error::custom("duplicate tag")),
                            None => deserialize_content::<T>(&tag, None)
                                .map_err(|e| de::Error::custom(e)),
                        }
                    }
                    Some(TagOrContentField::Content) => {
                        let content = MapAccess::next_value::<Content>(&mut map)?;

                        match read_key(&mut map)? {
                            Some(TagOrContentField::Tag) => {
                                let deserializer = ContentDeserializer::<T::Error>::new(content);
                                let tag = MapAccess::next_value::<&str>(&mut map)?;
                                let map = Deserialize::deserialize(deserializer)?;

                                deserialize_content::<T>(tag, Some(&map))
                                    .map_err(|e| de::Error::custom(e))
                            }
                            Some(TagOrContentField::Content) => {
                                Err(de::Error::duplicate_field("Properties"))
                            }
                            None => Err(de::Error::missing_field("Name")),
                        }
                    }
                    None => Err(<T::Error as de::Error>::missing_field("t")),
                }
            }
        }

        let start = std::time::Instant::now();
        let data = deserializer.deserialize_map(BlockVisitor)?;
        println!("deserialization took {:?}", start.elapsed());

        Ok(data)
    }
}

pub fn ref_or_error<'a>(
    map: Option<&'a HashMap<String, String>>,
) -> Result<&'a HashMap<String, String>, String> {
    if let Some(map) = map {
        Ok(map)
    } else {
        Err("Missing properties".to_string())
    }
}

#[allow(non_camel_case_types, dead_code)]
pub mod blocks;
