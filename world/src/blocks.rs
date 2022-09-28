use crate::{
    materials::{MaterialColor, Materials},
    sound::{SoundType, SoundTypes},
};

struct BlockProperties {
    material: Materials,
    material_color: Option<MaterialColor>,
    has_collision: bool,
    sound_type: SoundTypes,
    explosion_resistance: f32,
    destroy_time: f32,
    requires_tool: bool,
    // TODO: is randomly ticking
    friction: f32,
    speed_factor: f32,
    jump_factor: f32,
    can_occlude: bool,
    is_air: bool,
}

impl BlockProperties {
    pub fn new(material: Materials, material_color: Option<MaterialColor>) -> Self {
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
        pub mod block_properties {
            $(
                #[allow(non_camel_case_types)]
                pub type $name = $type;
            )*
        }
    };
}

macro_rules! blocks {
    ($($name: ident, $id: literal => {$properties: expr $(, $state_name: ident: {$($field: ident: $f_default: expr),*})?}),*) => {
        use serde::{Deserialize};

        block_props!(
            snowy => bool,
            lit => bool
        );

        #[derive(Deserialize, Debug)]
        #[serde(tag = "Name", content = "Properties")]
        pub enum Blocks {
            $(
                #[serde(rename = $id)]
                $name$((block_states::$state_name))?
            ),*
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
            material: Materials::Air,
            material_color: MaterialColor::Air,
            has_collision: false,
            sound_type: SoundType::Stone,
            ..BlockProperties::new()
        }
    },
    GrassBlock, "minecraft:grass_block" => {
        BlockProperties {
            material: Materials::Grass,
            material_color: MaterialColor::Grass,
            has_collision: true,
            sound_type: SoundType::Grass,
        },
        GrassBlockState: {
            snowy: false
        }
    }
);

pub mod properties {

    pub enum BlockProperties {
        Snowy(bool),
    }

    pub struct Property<T>
    where
        T: std::str::FromStr + std::fmt::Debug,
    {
        name: String,
        values: Vec<T>,
    }

    fn t() {
        let p = BlockProperties::Snowy(false);
    }
}
