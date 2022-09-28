use crate::{
    materials::{MaterialColor, Materials},
    sound::SoundTypes,
};

pub struct BlockProperties {
    pub material: Materials,
    pub material_color: Option<MaterialColor>,
    pub has_collision: bool,
    pub sound_type: SoundTypes,
    pub explosion_resistance: f32,
    pub destroy_time: f32,
    pub requires_tool: bool,
    // TODO: is randomly ticking
    pub friction: f32,
    pub speed_factor: f32,
    pub jump_factor: f32,
    pub can_occlude: bool,
    pub is_air: bool,
}

impl BlockProperties {
    pub const fn new(material: Materials, material_color: Option<MaterialColor>) -> Self {
        Self {
            material_color,
            material,
            has_collision: true,
            sound_type: SoundTypes::STONE,
            explosion_resistance: 0.0,
            destroy_time: 0.0,
            requires_tool: false,
            friction: 0.6,
            speed_factor: 1.0,
            jump_factor: 1.0,
            can_occlude: true,
            is_air: false,
        }
    }
}

macro_rules! block_props {
    ($($name: ident => $type: ty),*) => {
            $(
                #[allow(non_camel_case_types)]
                pub type $name = $type;
            )*
    };
}

macro_rules! blocks {
    ($($name: ident, $id: literal => {$properties: expr $(, $state_name: ident: {$($field: ident: $f_default: expr),*})?}),*) => {
        use serde::{Deserialize};

        #[derive(Deserialize, Debug)]
        #[serde(tag = "Name", content = "Properties")]
        pub enum Blocks {
            $(
                #[serde(rename = $id)]
                $name$((block_states::$state_name))?
            ),*
        }

        impl Blocks {
            pub fn get_properties(&self) -> &BlockProperties {
                #[allow(unused_variables, non_snake_case)]
                match self {
                    $(
                        Self::$name$(($state_name))?
                            => &block_properties::$name
                    ),*
                }
            }
        }

        pub mod block {
            use super::block_states;
            use super::BlockProperties;
            use super::block_properties;

            use serde::{Deserialize};

            $(
                #[derive(Deserialize)]
                #[serde(default)]
                pub struct $name {
                    $(state: block_states::$state_name,)?
                    #[serde(skip)]
                    properties: BlockProperties
                }

                impl Default for $name {
                    fn default() -> Self {
                        Self {
                            $(state: block_states::$state_name::new(),)?
                            properties: block_properties::$name
                        }
                    }
                }
            )*
        }

        pub mod block_properties {
            use super::BlockProperties;

            block_props!(
                snowy => bool,
                lit => bool
            );

            $(
                #[allow(non_upper_case_globals)]
                pub const $name: BlockProperties = $properties;
            )*
        }

        pub mod block_states {
            use super::block_properties::*;
            use serde::{Deserialize};

            $(
                $(
                    #[derive(Deserialize, Debug)]
                    pub struct $state_name {
                        $(
                            #[serde(deserialize_with = "from_string")]
                            pub $field: $field,
                        )*
                    }

                    impl $state_name {
                        pub fn new() -> Self {
                            Self {
                                $(
                                    $field: $f_default,
                                )*
                            }
                        }
                    }
                )?
            )*

            pub fn from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
            where
                D: serde::Deserializer<'de>,
                T: serde::Deserialize<'de> + std::str::FromStr,
                <T as std::str::FromStr>::Err: std::fmt::Debug
            {
                let val: String = Deserialize::deserialize(deserializer)?;

                Ok(val.parse().unwrap())
            }
        }
    };
}

blocks!(
    Air, "minecraft:air" => {
        BlockProperties {
            has_collision: false,
            can_occlude: false,
            is_air: true,
            ..BlockProperties::new(crate::materials::Materials::AIR, None)
        }
    },
    GrassBlock, "minecraft:grass_block" => {
        BlockProperties {
            ..BlockProperties::new(crate::materials::Materials::GRASS, None)
        },
        GrassBlockState: {
            snowy: false
        }
    }
);

trait LightLevel {
    fn get_light_level(&self) -> u8 {
        0
    }
}

impl LightLevel for Blocks {
    fn get_light_level(&self) -> u8 {
        match self {
            Blocks::GrassBlock(_) => 15,
            _ => 0,
        }
    }
}
