use crate::{
    biomes::{create_biome_map, Biome},
    blocks::{create_global_palette, deserialize_content},
};
use blocks::Block;
use lazy_static::lazy_static;
use serde::{
    de::{self, IgnoredAny, MapAccess, Visitor},
    Deserialize, Deserializer,
    __private::de::{
        Content, ContentDeserializer, TagContentOtherField, TagContentOtherFieldVisitor,
        TagOrContentField,
    },
};
use std::collections::HashMap;
use std::{array, str::FromStr, vec};

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

pub trait Palette {
    fn get(&self) -> i32;
}

lazy_static! {
    static ref BLOCK_PALETTE: HashMap<Block, usize> = create_global_palette();
}

impl Palette for Block {
    fn get(&self) -> i32 {
        *BLOCK_PALETTE
            .get(self)
            .expect("Block not found in global palette") as i32
    }
}

lazy_static! {
    static ref BIOME_PALETTE: HashMap<Biome, usize> = create_biome_map();
}

impl Palette for Biome {
    fn get(&self) -> i32 {
        *BIOME_PALETTE
            .get(self)
            .expect("Biome not found in global palette") as i32
    }
}

trait Values {
    type ValueIterator: Iterator;
    fn possible_values() -> Self::ValueIterator
    where
        Self: Sized;
}

impl<const MIN: u32, const MAX: u32> Values for ConstrainedInt<MIN, MAX> {
    type ValueIterator = vec::IntoIter<ConstrainedInt<MIN, MAX>>;
    fn possible_values() -> Self::ValueIterator {
        let values: Vec<ConstrainedInt<MIN, MAX>> = (MIN..=MAX)
            .into_iter()
            .map(|v| ConstrainedInt::<MIN, MAX>(v))
            .collect();
        values.into_iter()
    }
}

impl Values for bool {
    type ValueIterator = array::IntoIter<bool, 2>;
    fn possible_values() -> Self::ValueIterator {
        const VALUES: [bool; 2] = [true, false];
        return VALUES.into_iter();
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
                    while let Some(key) = map.next_key_seed(TagContentOtherFieldVisitor {
                        tag: "Name",
                        content: "Properties",
                    })? {
                        match key {
                            TagContentOtherField::Other => {
                                map.next_value::<IgnoredAny>()?;
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
                        let tag = map.next_value::<String>()?;
                        match read_key(&mut map)? {
                            Some(TagOrContentField::Content) => {
                                let data_map: std::collections::HashMap<String, String> =
                                    map.next_value()?;
                                let content = deserialize_content::<T>(&tag, Some(&data_map))
                                    .map_err(|e| de::Error::custom(e));
                                let None = read_key(&mut map)? else {
                                    return Err(de::Error::custom("did not expect more key-pairs for block"))
                                };
                                content
                            }
                            Some(TagOrContentField::Tag) => Err(de::Error::custom("duplicate tag")),
                            None => deserialize_content::<T>(&tag, None)
                                .map_err(|e| de::Error::custom(e)),
                        }
                    }
                    Some(TagOrContentField::Content) => {
                        let content = map.next_value::<Content>()?;

                        match read_key(&mut map)? {
                            Some(TagOrContentField::Tag) => {
                                let deserializer = ContentDeserializer::<T::Error>::new(content);
                                let tag = map.next_value::<String>()?;
                                let data_map = Deserialize::deserialize(deserializer)?;

                                let content =
                                    deserialize_content::<T>(tag.as_str(), Some(&data_map))
                                        .map_err(|e| de::Error::custom(e));
                                let None = read_key(&mut map)? else {
                                        return Err(de::Error::custom("did not expect more key-pairs for block"))
                                    };
                                content
                            }
                            Some(TagOrContentField::Content) => {
                                Err(de::Error::duplicate_field("Properties"))
                            }
                            None => Err(de::Error::missing_field("Name")),
                        }
                    }
                    None => {
                        println!("air");
                        Ok(Block::Air(blocks::Air {}))
                    }
                }
            }
        }
        Ok(deserializer.deserialize_map(BlockVisitor)?)
    }
}

impl<'de> Deserialize<'de> for Biome {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        struct __Visitor;

        impl<'de> serde::de::Visitor<'de> for __Visitor {
            type Value = Biome;
            fn expecting(
                &self,
                f: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                f.write_str("biome string")
            }
            fn visit_str<Err>(self, __value: &str) -> serde::__private::Result<Self::Value, Err>
            where
                Err: serde::de::Error,
            {
                biomes::visit_str(__value)
            }
        }

        __deserializer.deserialize_str(__Visitor)
    }
}

pub mod biomes;
#[allow(non_camel_case_types, dead_code)]
pub mod blocks;
pub mod tags;
