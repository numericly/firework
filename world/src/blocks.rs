use crate::{materials::Materials, sound::SoundTypes};

pub struct BlockProperties {
    pub material: Materials,
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
    pub const fn new(material: Materials) -> Self {
        Self {
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

macro_rules! enum_property {
    ($name:ident { $($variant:ident => $val:literal),* $(,)? }) => {

        #[derive(Deserialize, Debug)]
        pub enum $name {
            $($variant),*
        }

        impl FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($val => Ok(Self::$variant)),*,
                    _ => Err(()),
                }
            }
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                match self {
                    $($name::$variant => $val.to_string()),*
                }
            }
        }
    }
}

macro_rules! block_prop {
    ($name: ident) => {
        pub type $name<T> = T;
    };
    ($name: ident, $type: ty) => {
        pub type $name = $type;
    };
}

macro_rules! block_props {
    ($($name: ident$( => $type: ty)?),*) => {

        $(
            #[allow(non_camel_case_types)]
            block_prop!($name$(, $type )?);
        )*
    };
}

macro_rules! field_type {
    ($field: ident) => {
        $field
    };
    ($field: ident, $field_ty: ty) => {
        $field<$field_ty>
    };
}

macro_rules! blocks {
    ($($name: ident, $id: literal => {$properties: expr $(, $state_name: ident {$($field: ident$(, $prop_ty: ty)?: $f_default: expr),*})?}),*) => {
        use serde::{Deserialize};

        #[derive(Deserialize, Debug)]
        #[serde(tag = "Name", content = "Properties")]
        pub enum BlockState {
            $(
                #[serde(rename = $id)]
                $name$((block_states::$state_name))?
            ),*
        }

        impl BlockState {
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

        pub mod block_properties {
            use serde::{Deserialize};
            use std::str::FromStr;
            use super::BlockProperties;

            #[derive(Deserialize, Debug)]
            pub struct ConstrainedInt<const MIN: u32, const MAX: u32> (pub u32);

            impl<const MIN: u32, const MAX: u32> FromStr for ConstrainedInt<MIN, MAX> {
                type Err = String;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    let val = s.parse::<u32>().map_err(|e| e.to_string())?;
                    if val > MAX {
                        return Err(format!("Value {} is greater than the maximum value {}", val, MAX));
                    }
                    if val < MIN {
                        return Err(format!("Value {} is less than the minimum value {}", val, MIN));
                    }

                    Ok(Self(val))
                }
            }

            impl<const MIN: u32, const MAX: u32> ConstrainedInt<MIN, MAX> {
            }

            enum_property! { Axis { X => "x", Y => "y", Z => "z" } }
            enum_property! { HorizontalAxis { X => "x", Z => "z" } }

            enum_property! { Facing { North => "north", East => "east", South => "south", West => "west", Up => "up", Down => "down" } }
            enum_property! { HopperFacing { North => "north", East => "east", South => "south", West => "west", Down => "down" } }
            enum_property! { HorizontalFacing { North => "north", East => "east", South => "south", West => "west" } }
            enum_property! { Orientation {
                DownEast => "down_east",
                DownNorth => "down_north",
                DownSouth => "down_south",
                DownWest => "down_west",
                UpEast => "up_east",
                UpNorth => "up_north",
                UpSouth => "up_south",
                UpWest => "up_west",
                WestUp => "west_up",
                NorthUp => "north_up",
                SouthUp => "south_up",
                EastUp => "east_up",
            }}
            enum_property! { AttachFace { Floor => "floor", Wall => "wall", Ceiling => "ceiling" } }
            enum_property! { BellAttach { Floor => "floor", Ceiling => "ceiling", SingleWall => "single_wall", DoubleWall => "double_wall" } }
            enum_property! { WallSide { None => "none", Low => "low", Tall => "tall" } }
            enum_property! { RedstoneSide { Up => "up", None => "none", Side => "side" } }
            enum_property! { DoubleBlockHalf { Lower => "lower", Upper => "upper" } }
            enum_property! { Half { Top => "top", Bottom => "bottom" } }
            enum_property! { RailShape {
                NorthSouth => "north_south",
                EastWest => "east_west",
                AscendingEast => "ascending_east",
                AscendingWest => "ascending_west",
                AscendingNorth => "ascending_north",
                AscendingSouth => "ascending_south",
                SouthEast => "south_east",
                SouthWest => "south_west",
                NorthWest => "north_west",
                NorthEast => "north_east"
            }}
            enum_property! { StraightRailShape {
                NorthSouth => "north_south",
                EastWest => "east_west",
                AscendingEast => "ascending_east",
                AscendingWest => "ascending_west",
                AscendingNorth => "ascending_north",
                AscendingSouth => "ascending_south"
            }}
            enum_property! { BedPart { Head => "head", Foot => "foot" } }
            enum_property! { ChestType { Single => "single", Left => "left", Right => "right" } }
            enum_property! { ComparatorMode { Compare => "compare", Subtract => "subtract" } }
            enum_property! { DoorHinge { Left => "left", Right => "right" } }
            enum_property! { NoteBlockInstrument {
                Harp => "harp",
                DoubleBass => "basedrum",
                Snare => "snare",
                Hat => "hat",
                Bells => "bass",
                Flute => "flute",
                Bell => "bell",
                Guitar => "guitar",
                Chime => "chime",
                Xylophone => "xylophone",
                IronXylophone => "iron_xylophone",
                CowBell => "cow_bell",
                Didgeridoo => "didgeridoo",
                Bit => "bit",
                Banjo => "banjo",
                Pling => "pling"
            }}
            enum_property! { PistonType { Normal => "normal", Sticky => "sticky" } }
            enum_property! { SlabType { Top => "top", Bottom => "bottom", Double => "double" } }
            enum_property! { StairsShape {
                Straight => "straight",
                InnerLeft => "inner_left",
                InnerRight => "inner_right",
                OuterLeft => "outer_left",
                OuterRight => "outer_right"
            }}
            enum_property! { StructureBlockMode { Save => "save", Load => "load", Corner => "corner", Data => "data" } }
            enum_property! { BambooLeaves { None => "none", Small => "small", Large => "large" } }
            enum_property! { Tilt { None => "none", Unstable => "unstable", Partial => "partial", Full => "full" } }
            enum_property! { VerticalDirection { Up => "up", Down => "down" } }
            enum_property! { DripStoneThickness { TipMerge => "tip_merge", Tip => "tip", Frustum => "frustum", Middle => "middle", Base => "base" } }
            enum_property! { SculkSensorPhase { Inactive => "inactive", Active => "active", CoolDown => "cooldown" } }

            block_props!(
                attached => bool,
                bottom => bool,
                conditional => bool,
                disarmed => bool,
                drag => bool,
                enabled => bool,
                extended => bool,
                eye => bool,
                falling => bool,
                hanging => bool,
                has_bottle_0 => bool,
                has_bottle_1 => bool,
                has_bottle_2 => bool,
                has_record => bool,
                has_book => bool,
                inverted => bool,
                in_wall => bool,
                lit => bool,
                locked => bool,
                occupied => bool,
                open => bool,
                persistent => bool,
                powered => bool,
                short => bool,
                signal_fire => bool,
                snowy => bool,
                triggered => bool,
                unstable => bool,
                waterlogged => bool,
                vine_end => bool,
                berries => bool,
                bloom => bool,
                shrieking => bool,
                can_summon => bool,
                axis,
                up => bool,
                down => bool,
                north,
                east,
                south,
                west,
                facing,
                orientation => Orientation,
                attach_face => AttachFace,
                attachment => BellAttach,
                half,
                shape,
                age,
                bites => ConstrainedInt::<0, 6>,
                candles => ConstrainedInt::<1, 4>,
                delay => ConstrainedInt::<1, 4>,
                distance,
                eggs => ConstrainedInt::<1, 4>,
                hatch => ConstrainedInt::<0, 2>,
                layers => ConstrainedInt::<1, 8>,
                level,
                honey_level => ConstrainedInt::<0, 5>,
                moisture => ConstrainedInt::<0, 7>,
                note => ConstrainedInt::<0, 24>,
                pickles => ConstrainedInt::<1, 4>,
                power => ConstrainedInt::<0, 15>,
                stage => ConstrainedInt::<0, 1>,
                charges => ConstrainedInt::<0, 4>,
                rotation => ConstrainedInt::<0, 15>,
                part => BedPart,
                chest_type => ChestType,
                mode,
                hinge => DoorHinge,
                instrument => NoteBlockInstrument,
                leaves => BambooLeaves,
                tilt => Tilt,
                vertical_direction => VerticalDirection,
                thickness => DripStoneThickness,
                phase => SculkSensorPhase
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
                            pub $field: field_type!($field$(, $prop_ty)?),
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
            ..BlockProperties::new(crate::materials::Materials::AIR)
        }
    },
    GrassBlock, "minecraft:grass_block" => {
        BlockProperties {
            has_collision: true,
            can_occlude: true,
            is_air: false,
            ..BlockProperties::new(crate::materials::Materials::GRASS)
        },
        GrassBlockState {
            snowy: false
        }
    }
);
