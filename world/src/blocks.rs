// use crate::{materials::Materials, sound::SoundTypes};

// pub struct BlockProperties {
//     pub material: Materials,
//     pub has_collision: bool,
//     pub sound_type: SoundTypes,
//     pub explosion_resistance: f32,
//     pub destroy_time: f32,
//     pub requires_tool: bool,
//     // TODO: is randomly ticking
//     pub friction: f32,
//     pub speed_factor: f32,
//     pub jump_factor: f32,
//     pub can_occlude: bool,
//     pub is_air: bool,
// }

// impl BlockProperties {
//     pub const fn new(material: Materials) -> Self {
//         Self {
//             material,
//             has_collision: true,
//             sound_type: SoundTypes::STONE,
//             explosion_resistance: 0.0,
//             destroy_time: 0.0,
//             requires_tool: false,
//             friction: 0.6,
//             speed_factor: 1.0,
//             jump_factor: 1.0,
//             can_occlude: true,
//             is_air: false,
//         }
//     }
// }

// macro_rules! enum_property {
//     ($name:ident { $($variant:ident => $val:literal),* $(,)? }) => {

//         #[derive(Deserialize, Debug)]
//         pub enum $name {
//             $($variant),*
//         }

//         impl FromStr for $name {
//             type Err = ();

//             fn from_str(s: &str) -> Result<Self, Self::Err> {
//                 match s {
//                     $($val => Ok(Self::$variant)),*,
//                     _ => Err(()),
//                 }
//             }
//         }

//         impl ToString for $name {
//             fn to_string(&self) -> String {
//                 match self {
//                     $($name::$variant => $val.to_string()),*
//                 }
//             }
//         }
//     }
// }

// macro_rules! block_prop {
//     ($name: ident) => {
//         #[allow(non_camel_case_types)]
//         pub type $name<T> = T;
//     };
//     ($name: ident, $type: ty) => {
//         #[allow(non_camel_case_types)]
//         pub type $name = $type;
//     };
// }

// macro_rules! block_props {
//     ($($name: ident$( => $type: ty)?),*) => {

//         $(
//             block_prop!($name$(, $type )?);
//         )*
//     };
// }

// macro_rules! field_type {
//     ($field: ident) => {
//         $field
//     };
//     ($field: ident, $field_ty: ty) => {
//         $field<$field_ty>
//     };
// }

// macro_rules! blocks {
//     ($($name: ident, $id: literal => {$properties: expr $(, $state_name: ident {$($field: ident$(, $prop_ty: ty)?: $f_default: expr),*})?}),*) => {
//         use serde::{Deserialize};

//         #[derive(Deserialize, Debug)]
//         #[serde(tag = "Name", content = "Properties")]
//         pub enum BlockState {
//             $(
//                 #[serde(rename = $id)]
//                 $name$((block_states::$state_name))?
//             ),*
//         }

//         impl BlockState {
//             pub fn get_properties(&self) -> &BlockProperties {
//                 #[allow(unused_variables, non_snake_case)]
//                 match self {
//                     $(
//                         Self::$name$(($state_name))?
//                             => &block_properties::$name
//                     ),*
//                 }
//             }
//         }

//         pub mod block_properties {
//             use serde::{Deserialize};
//             use std::str::FromStr;
//             use super::BlockProperties;

//             #[derive(Deserialize, Debug)]
//             pub struct ConstrainedInt<const MIN: u32, const MAX: u32> (pub u32);

//             impl<const MIN: u32, const MAX: u32> FromStr for ConstrainedInt<MIN, MAX> {
//                 type Err = String;

//                 fn from_str(s: &str) -> Result<Self, Self::Err> {
//                     let val = s.parse::<u32>().map_err(|e| e.to_string())?;
//                     if val > MAX {
//                         return Err(format!("Value {} is greater than the maximum value {}", val, MAX));
//                     }
//                     if val < MIN {
//                         return Err(format!("Value {} is less than the minimum value {}", val, MIN));
//                     }

//                     Ok(Self(val))
//                 }
//             }

//             impl<const MIN: u32, const MAX: u32> ConstrainedInt<MIN, MAX> {
//             }

//             enum_property! { Axis { X => "x", Y => "y", Z => "z" } }
//             enum_property! { HorizontalAxis { X => "x", Z => "z" } }

//             enum_property! { Facing { North => "north", East => "east", South => "south", West => "west", Up => "up", Down => "down" } }
//             enum_property! { HopperFacing { North => "north", East => "east", South => "south", West => "west", Down => "down" } }
//             enum_property! { HorizontalFacing { North => "north", East => "east", South => "south", West => "west" } }
//             enum_property! { Orientation {
//                 DownEast => "down_east",
//                 DownNorth => "down_north",
//                 DownSouth => "down_south",
//                 DownWest => "down_west",
//                 UpEast => "up_east",
//                 UpNorth => "up_north",
//                 UpSouth => "up_south",
//                 UpWest => "up_west",
//                 WestUp => "west_up",
//                 NorthUp => "north_up",
//                 SouthUp => "south_up",
//                 EastUp => "east_up",
//             }}
//             enum_property! { AttachFace { Floor => "floor", Wall => "wall", Ceiling => "ceiling" } }
//             enum_property! { BellAttach { Floor => "floor", Ceiling => "ceiling", SingleWall => "single_wall", DoubleWall => "double_wall" } }
//             enum_property! { WallSide { None => "none", Low => "low", Tall => "tall" } }
//             enum_property! { RedstoneSide { Up => "up", None => "none", Side => "side" } }
//             enum_property! { DoubleBlockHalf { Lower => "lower", Upper => "upper" } }
//             enum_property! { Half { Top => "top", Bottom => "bottom" } }
//             enum_property! { RailShape {
//                 NorthSouth => "north_south",
//                 EastWest => "east_west",
//                 AscendingEast => "ascending_east",
//                 AscendingWest => "ascending_west",
//                 AscendingNorth => "ascending_north",
//                 AscendingSouth => "ascending_south",
//                 SouthEast => "south_east",
//                 SouthWest => "south_west",
//                 NorthWest => "north_west",
//                 NorthEast => "north_east"
//             }}
//             enum_property! { StraightRailShape {
//                 NorthSouth => "north_south",
//                 EastWest => "east_west",
//                 AscendingEast => "ascending_east",
//                 AscendingWest => "ascending_west",
//                 AscendingNorth => "ascending_north",
//                 AscendingSouth => "ascending_south"
//             }}
//             enum_property! { BedPart { Head => "head", Foot => "foot" } }
//             enum_property! { ChestType { Single => "single", Left => "left", Right => "right" } }
//             enum_property! { ComparatorMode { Compare => "compare", Subtract => "subtract" } }
//             enum_property! { DoorHinge { Left => "left", Right => "right" } }
//             enum_property! { NoteBlockInstrument {
//                 Harp => "harp",
//                 DoubleBass => "basedrum",
//                 Snare => "snare",
//                 Hat => "hat",
//                 Bells => "bass",
//                 Flute => "flute",
//                 Bell => "bell",
//                 Guitar => "guitar",
//                 Chime => "chime",
//                 Xylophone => "xylophone",
//                 IronXylophone => "iron_xylophone",
//                 CowBell => "cow_bell",
//                 Didgeridoo => "didgeridoo",
//                 Bit => "bit",
//                 Banjo => "banjo",
//                 Pling => "pling"
//             }}
//             enum_property! { PistonType { Normal => "normal", Sticky => "sticky" } }
//             enum_property! { SlabType { Top => "top", Bottom => "bottom", Double => "double" } }
//             enum_property! { StairsShape {
//                 Straight => "straight",
//                 InnerLeft => "inner_left",
//                 InnerRight => "inner_right",
//                 OuterLeft => "outer_left",
//                 OuterRight => "outer_right"
//             }}
//             enum_property! { StructureBlockMode { Save => "save", Load => "load", Corner => "corner", Data => "data" } }
//             enum_property! { BambooLeaves { None => "none", Small => "small", Large => "large" } }
//             enum_property! { Tilt { None => "none", Unstable => "unstable", Partial => "partial", Full => "full" } }
//             enum_property! { VerticalDirection { Up => "up", Down => "down" } }
//             enum_property! { DripStoneThickness { TipMerge => "tip_merge", Tip => "tip", Frustum => "frustum", Middle => "middle", Base => "base" } }
//             enum_property! { SculkSensorPhase { Inactive => "inactive", Active => "active", CoolDown => "cooldown" } }

//             block_props!(
//                 attached => bool,
//                 bottom => bool,
//                 conditional => bool,
//                 disarmed => bool,
//                 drag => bool,
//                 enabled => bool,
//                 extended => bool,
//                 eye => bool,
//                 falling => bool,
//                 hanging => bool,
//                 has_bottle_0 => bool,
//                 has_bottle_1 => bool,
//                 has_bottle_2 => bool,
//                 has_record => bool,
//                 has_book => bool,
//                 inverted => bool,
//                 in_wall => bool,
//                 lit => bool,
//                 locked => bool,
//                 occupied => bool,
//                 open => bool,
//                 persistent => bool,
//                 powered => bool,
//                 short => bool,
//                 signal_fire => bool,
//                 snowy => bool,
//                 triggered => bool,
//                 unstable => bool,
//                 waterlogged => bool,
//                 vine_end => bool,
//                 berries => bool,
//                 bloom => bool,
//                 shrieking => bool,
//                 can_summon => bool,
//                 axis,
//                 up => bool,
//                 down => bool,
//                 north,
//                 east,
//                 south,
//                 west,
//                 facing,
//                 orientation => Orientation,
//                 face => AttachFace,
//                 attachment => BellAttach,
//                 half,
//                 shape,
//                 age,
//                 bites => ConstrainedInt::<0, 6>,
//                 candles => ConstrainedInt::<1, 4>,
//                 delay => ConstrainedInt::<1, 4>,
//                 distance,
//                 eggs => ConstrainedInt::<1, 4>,
//                 hatch => ConstrainedInt::<0, 2>,
//                 layers => ConstrainedInt::<1, 8>,
//                 level,
//                 honey_level => ConstrainedInt::<0, 5>,
//                 moisture => ConstrainedInt::<0, 7>,
//                 note => ConstrainedInt::<0, 24>,
//                 pickles => ConstrainedInt::<1, 4>,
//                 power => ConstrainedInt::<0, 15>,
//                 stage => ConstrainedInt::<0, 1>,
//                 charges => ConstrainedInt::<0, 4>,
//                 rotation => ConstrainedInt::<0, 15>,
//                 part => BedPart,
//                 r#type,
//                 chest_type => ChestType,
//                 mode,
//                 hinge => DoorHinge,
//                 instrument => NoteBlockInstrument,
//                 leaves => BambooLeaves,
//                 tilt => Tilt,
//                 vertical_direction => VerticalDirection,
//                 thickness => DripStoneThickness,
//                 sculk_sensor_phase => SculkSensorPhase
//             );

//             $(
//                 #[allow(non_upper_case_globals)]
//                 pub const $name: BlockProperties = $properties;
//             )*
//         }

//         pub mod block_states {
//             use super::block_properties::*;
//             use serde::{Deserialize};

//             $(
//                 $(
//                     #[derive(Deserialize, Debug)]
//                     #[serde(rename_all = "snake_case")]
//                     pub struct $state_name {
//                         $(
//                             #[serde(deserialize_with = "from_string")]
//                             pub $field: field_type!($field$(, $prop_ty)?),
//                         )*
//                     }

//                     impl $state_name {
//                         pub fn new() -> Self {
//                             Self {
//                                 $(
//                                     $field: $f_default,
//                                 )*
//                             }
//                         }
//                     }
//                 )?
//             )*

//             pub fn from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
//             where
//                 D: serde::Deserializer<'de>,
//                 T: serde::Deserialize<'de> + std::str::FromStr,
//                 <T as std::str::FromStr>::Err: std::fmt::Debug
//             {
//                 let val: String = Deserialize::deserialize(deserializer)?;

//                 Ok(val.parse().unwrap())
//             }
//         }
//     };
// }

// blocks!(
//     Air, "minecraft:air" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             is_air: true,
//             ..BlockProperties::new(crate::materials::Materials::AIR)
//         }
//     },
//     Stone, "minecraft:stone" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     Granite, "minecraft:granite" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PolishedGranite, "minecraft:polished_granite" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     Diorite, "minecraft:diorite" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PolishedDiorite, "minecraft:polished_diorite" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     Andesite, "minecraft:andesite" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PolishedAndesite, "minecraft:polished_andesite" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     GrassBlock, "minecraft:grass_block" => {
//         BlockProperties {
//             explosion_resistance: 0.6,
//             destroy_time: 0.6,
//             ..BlockProperties::new(crate::materials::Materials::GRASS)
//         },
//         GrassBlockProperties {
//             snowy: false
//         }
//     },
//     Dirt, "minecraft:dirt" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::DIRT)
//         }
//     },
//     CoarseDirt, "minecraft:coarse_dirt" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::DIRT)
//         }
//     },
//     Podzol, "minecraft:podzol" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::DIRT)
//         },
//         PodzolProperties {
//             snowy: false
//         }
//     },
//     Cobblestone, "minecraft:cobblestone" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     OakPlanks, "minecraft:oak_planks" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         }
//     },
//     SprucePlanks, "minecraft:spruce_planks" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         }
//     },
//     BirchPlanks, "minecraft:birch_planks" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         }
//     },
//     JunglePlanks, "minecraft:jungle_planks" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         }
//     },
//     AcaciaPlanks, "minecraft:acacia_planks" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         }
//     },
//     DarkOakPlanks, "minecraft:dark_oak_planks" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         }
//     },
//     MangrovePlanks, "minecraft:mangrove_planks" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         }
//     },
//     OakSapling, "minecraft:oak_sapling" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         OakSaplingProperties {
//             stage: ConstrainedInt(0)
//         }
//     },
//     SpruceSapling, "minecraft:spruce_sapling" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         SpruceSaplingProperties {
//             stage: ConstrainedInt(0)
//         }
//     },
//     BirchSapling, "minecraft:birch_sapling" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         BirchSaplingProperties {
//             stage: ConstrainedInt(0)
//         }
//     },
//     JungleSapling, "minecraft:jungle_sapling" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         JungleSaplingProperties {
//             stage: ConstrainedInt(0)
//         }
//     },
//     AcaciaSapling, "minecraft:acacia_sapling" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         AcaciaSaplingProperties {
//             stage: ConstrainedInt(0)
//         }
//     },
//     DarkOakSapling, "minecraft:dark_oak_sapling" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         DarkOakSaplingProperties {
//             stage: ConstrainedInt(0)
//         }
//     },
//     MangrovePropagule, "minecraft:mangrove_propagule" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         MangrovePropaguleProperties {
//             age, ConstrainedInt<0, 4>: ConstrainedInt(0),
//             hanging: false,
//             stage: ConstrainedInt(0),
//             waterlogged: false
//         }
//     },
//     Bedrock, "minecraft:bedrock" => {
//         BlockProperties {
//             explosion_resistance: 3600000.0,
//             destroy_time: -1.0,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     Water, "minecraft:water" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 100.0,
//             destroy_time: 100.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER)
//         },
//         WaterProperties {
//             level, ConstrainedInt<0, 15>: ConstrainedInt(0)
//         }
//     },
//     Lava, "minecraft:lava" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 100.0,
//             destroy_time: 100.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::LAVA)
//         },
//         LavaProperties {
//             level, ConstrainedInt<0, 15>: ConstrainedInt(0)
//         }
//     },
//     Sand, "minecraft:sand" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     RedSand, "minecraft:red_sand" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     Gravel, "minecraft:gravel" => {
//         BlockProperties {
//             explosion_resistance: 0.6,
//             destroy_time: 0.6,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     GoldOre, "minecraft:gold_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DeepslateGoldOre, "minecraft:deepslate_gold_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 4.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     IronOre, "minecraft:iron_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DeepslateIronOre, "minecraft:deepslate_iron_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 4.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     CoalOre, "minecraft:coal_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DeepslateCoalOre, "minecraft:deepslate_coal_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 4.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     NetherGoldOre, "minecraft:nether_gold_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     OakLog, "minecraft:oak_log" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         OakLogProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     SpruceLog, "minecraft:spruce_log" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         SpruceLogProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     BirchLog, "minecraft:birch_log" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BirchLogProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     JungleLog, "minecraft:jungle_log" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         JungleLogProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     AcaciaLog, "minecraft:acacia_log" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         AcaciaLogProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     DarkOakLog, "minecraft:dark_oak_log" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         DarkOakLogProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     MangroveLog, "minecraft:mangrove_log" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MangroveLogProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     MangroveRoots, "minecraft:mangrove_roots" => {
//         BlockProperties {
//             explosion_resistance: 0.7,
//             destroy_time: 0.7,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MangroveRootsProperties {
//             waterlogged: false
//         }
//     },
//     MuddyMangroveRoots, "minecraft:muddy_mangrove_roots" => {
//         BlockProperties {
//             explosion_resistance: 0.7,
//             destroy_time: 0.7,
//             ..BlockProperties::new(crate::materials::Materials::DIRT)
//         },
//         MuddyMangroveRootsProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedSpruceLog, "minecraft:stripped_spruce_log" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         StrippedSpruceLogProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedBirchLog, "minecraft:stripped_birch_log" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         StrippedBirchLogProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedJungleLog, "minecraft:stripped_jungle_log" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         StrippedJungleLogProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedAcaciaLog, "minecraft:stripped_acacia_log" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         StrippedAcaciaLogProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedDarkOakLog, "minecraft:stripped_dark_oak_log" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         StrippedDarkOakLogProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedOakLog, "minecraft:stripped_oak_log" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         StrippedOakLogProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedMangroveLog, "minecraft:stripped_mangrove_log" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         StrippedMangroveLogProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     OakWood, "minecraft:oak_wood" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         OakWoodProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     SpruceWood, "minecraft:spruce_wood" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         SpruceWoodProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     BirchWood, "minecraft:birch_wood" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BirchWoodProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     JungleWood, "minecraft:jungle_wood" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         JungleWoodProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     AcaciaWood, "minecraft:acacia_wood" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         AcaciaWoodProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     DarkOakWood, "minecraft:dark_oak_wood" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         DarkOakWoodProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     MangroveWood, "minecraft:mangrove_wood" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MangroveWoodProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedOakWood, "minecraft:stripped_oak_wood" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         StrippedOakWoodProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedSpruceWood, "minecraft:stripped_spruce_wood" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         StrippedSpruceWoodProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedBirchWood, "minecraft:stripped_birch_wood" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         StrippedBirchWoodProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedJungleWood, "minecraft:stripped_jungle_wood" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         StrippedJungleWoodProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedAcaciaWood, "minecraft:stripped_acacia_wood" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         StrippedAcaciaWoodProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedDarkOakWood, "minecraft:stripped_dark_oak_wood" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         StrippedDarkOakWoodProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedMangroveWood, "minecraft:stripped_mangrove_wood" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         StrippedMangroveWoodProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     OakLeaves, "minecraft:oak_leaves" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::LEAVES)
//         },
//         OakLeavesProperties {
//             distance, ConstrainedInt<1,7>: ConstrainedInt(0),
//             persistent: false,
//             waterlogged: false
//         }
//     },
//     SpruceLeaves, "minecraft:spruce_leaves" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::LEAVES)
//         },
//         SpruceLeavesProperties {
//             distance, ConstrainedInt<1,7>: ConstrainedInt(0),
//             persistent: false,
//             waterlogged: false
//         }
//     },
//     BirchLeaves, "minecraft:birch_leaves" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::LEAVES)
//         },
//         BirchLeavesProperties {
//             distance, ConstrainedInt<1,7>: ConstrainedInt(0),
//             persistent: false,
//             waterlogged: false
//         }
//     },
//     JungleLeaves, "minecraft:jungle_leaves" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::LEAVES)
//         },
//         JungleLeavesProperties {
//             distance, ConstrainedInt<1,7>: ConstrainedInt(0),
//             persistent: false,
//             waterlogged: false
//         }
//     },
//     AcaciaLeaves, "minecraft:acacia_leaves" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::LEAVES)
//         },
//         AcaciaLeavesProperties {
//             distance, ConstrainedInt<1,7>: ConstrainedInt(0),
//             persistent: false,
//             waterlogged: false
//         }
//     },
//     DarkOakLeaves, "minecraft:dark_oak_leaves" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::LEAVES)
//         },
//         DarkOakLeavesProperties {
//             distance, ConstrainedInt<1,7>: ConstrainedInt(0),
//             persistent: false,
//             waterlogged: false
//         }
//     },
//     MangroveLeaves, "minecraft:mangrove_leaves" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::LEAVES)
//         },
//         MangroveLeavesProperties {
//             distance, ConstrainedInt<1,7>: ConstrainedInt(0),
//             persistent: false,
//             waterlogged: false
//         }
//     },
//     AzaleaLeaves, "minecraft:azalea_leaves" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::LEAVES)
//         },
//         AzaleaLeavesProperties {
//             distance, ConstrainedInt<1,7>: ConstrainedInt(0),
//             persistent: false,
//             waterlogged: false
//         }
//     },
//     FloweringAzaleaLeaves, "minecraft:flowering_azalea_leaves" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::LEAVES)
//         },
//         FloweringAzaleaLeavesProperties {
//             distance, ConstrainedInt<1,7>: ConstrainedInt(0),
//             persistent: false,
//             waterlogged: false
//         }
//     },
//     Sponge, "minecraft:sponge" => {
//         BlockProperties {
//             explosion_resistance: 0.6,
//             destroy_time: 0.6,
//             ..BlockProperties::new(crate::materials::Materials::SPONGE)
//         }
//     },
//     WetSponge, "minecraft:wet_sponge" => {
//         BlockProperties {
//             explosion_resistance: 0.6,
//             destroy_time: 0.6,
//             ..BlockProperties::new(crate::materials::Materials::SPONGE)
//         }
//     },
//     Glass, "minecraft:glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     LapisOre, "minecraft:lapis_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DeepslateLapisOre, "minecraft:deepslate_lapis_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 4.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     LapisBlock, "minecraft:lapis_block" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     Dispenser, "minecraft:dispenser" => {
//         BlockProperties {
//             explosion_resistance: 3.5,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DispenserProperties {
//             facing, Facing: Facing::North,
//             triggered: false
//         }
//     },
//     Sandstone, "minecraft:sandstone" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     ChiseledSandstone, "minecraft:chiseled_sandstone" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     CutSandstone, "minecraft:cut_sandstone" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     NoteBlock, "minecraft:note_block" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         NoteBlockProperties {
//             instrument: NoteBlockInstrument::Harp,
//             note: ConstrainedInt(0),
//             powered: false
//         }
//     },
//     WhiteBed, "minecraft:white_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         WhiteBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     OrangeBed, "minecraft:orange_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         OrangeBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     MagentaBed, "minecraft:magenta_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         MagentaBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     LightBlueBed, "minecraft:light_blue_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         LightBlueBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     YellowBed, "minecraft:yellow_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         YellowBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     LimeBed, "minecraft:lime_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         LimeBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     PinkBed, "minecraft:pink_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         PinkBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     GrayBed, "minecraft:gray_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         GrayBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     LightGrayBed, "minecraft:light_gray_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         LightGrayBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     CyanBed, "minecraft:cyan_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         CyanBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     PurpleBed, "minecraft:purple_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         PurpleBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     BlueBed, "minecraft:blue_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         BlueBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     BrownBed, "minecraft:brown_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         BrownBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     GreenBed, "minecraft:green_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         GreenBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     RedBed, "minecraft:red_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         RedBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     BlackBed, "minecraft:black_bed" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         },
//         BlackBedProperties {
//             facing, Facing: Facing::North,
//             occupied: false,
//             part: BedPart::Foot
//         }
//     },
//     PoweredRail, "minecraft:powered_rail" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.7,
//             destroy_time: 0.7,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         PoweredRailProperties {
//             powered: false,
//             shape, StraightRailShape: StraightRailShape::NorthSouth,
//             waterlogged: false
//         }
//     },
//     DetectorRail, "minecraft:detector_rail" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.7,
//             destroy_time: 0.7,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         DetectorRailProperties {
//             powered: false,
//             shape, StraightRailShape: StraightRailShape::NorthSouth,
//             waterlogged: false
//         }
//     },
//     StickyPiston, "minecraft:sticky_piston" => {
//         BlockProperties {
//             explosion_resistance: 1.5,
//             destroy_time: 1.5,
//             ..BlockProperties::new(crate::materials::Materials::PISTON)
//         },
//         StickyPistonProperties {
//             extended: false,
//             facing, Facing: Facing::North
//         }
//     },
//     Cobweb, "minecraft:cobweb" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 4.0,
//             destroy_time: 4.0,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WEB)
//         }
//     },
//     Grass, "minecraft:grass" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_PLANT)
//         }
//     },
//     Fern, "minecraft:fern" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_PLANT)
//         }
//     },
//     DeadBush, "minecraft:dead_bush" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_PLANT)
//         }
//     },
//     Seagrass, "minecraft:seagrass" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_WATER_PLANT)
//         }
//     },
//     TallSeagrass, "minecraft:tall_seagrass" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_WATER_PLANT)
//         },
//         TallSeagrassProperties {
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower
//         }
//     },
//     Piston, "minecraft:piston" => {
//         BlockProperties {
//             explosion_resistance: 1.5,
//             destroy_time: 1.5,
//             ..BlockProperties::new(crate::materials::Materials::PISTON)
//         },
//         PistonProperties {
//             extended: false,
//             facing, Facing: Facing::North
//         }
//     },
//     PistonHead, "minecraft:piston_head" => {
//         BlockProperties {
//             explosion_resistance: 1.5,
//             destroy_time: 1.5,
//             ..BlockProperties::new(crate::materials::Materials::PISTON)
//         },
//         PistonHeadProperties {
//             facing, Facing: Facing::North,
//             short: false,
//             r#type, PistonType: PistonType::Normal
//         }
//     },
//     WhiteWool, "minecraft:white_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     OrangeWool, "minecraft:orange_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     MagentaWool, "minecraft:magenta_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     LightBlueWool, "minecraft:light_blue_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     YellowWool, "minecraft:yellow_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     LimeWool, "minecraft:lime_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     PinkWool, "minecraft:pink_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     GrayWool, "minecraft:gray_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     LightGrayWool, "minecraft:light_gray_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     CyanWool, "minecraft:cyan_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     PurpleWool, "minecraft:purple_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     BlueWool, "minecraft:blue_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     BrownWool, "minecraft:brown_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     GreenWool, "minecraft:green_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     RedWool, "minecraft:red_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     BlackWool, "minecraft:black_wool" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             ..BlockProperties::new(crate::materials::Materials::WOOL)
//         }
//     },
//     MovingPiston, "minecraft:moving_piston" => {
//         BlockProperties {
//             destroy_time: -1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PISTON)
//         },
//         MovingPistonProperties {
//             facing, Facing: Facing::North,
//             r#type, PistonType: PistonType::Normal
//         }
//     },
//     Dandelion, "minecraft:dandelion" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     Poppy, "minecraft:poppy" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     BlueOrchid, "minecraft:blue_orchid" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     Allium, "minecraft:allium" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     AzureBluet, "minecraft:azure_bluet" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     RedTulip, "minecraft:red_tulip" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     OrangeTulip, "minecraft:orange_tulip" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     WhiteTulip, "minecraft:white_tulip" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     PinkTulip, "minecraft:pink_tulip" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     OxeyeDaisy, "minecraft:oxeye_daisy" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     Cornflower, "minecraft:cornflower" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     WitherRose, "minecraft:wither_rose" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     LilyOfTheValley, "minecraft:lily_of_the_valley" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     BrownMushroom, "minecraft:brown_mushroom" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     RedMushroom, "minecraft:red_mushroom" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     GoldBlock, "minecraft:gold_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     IronBlock, "minecraft:iron_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     Bricks, "minecraft:bricks" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     Tnt, "minecraft:tnt" => {
//         BlockProperties {
//             ..BlockProperties::new(crate::materials::Materials::EXPLOSIVE)
//         },
//         TntProperties {
//             unstable: false
//         }
//     },
//     Bookshelf, "minecraft:bookshelf" => {
//         BlockProperties {
//             explosion_resistance: 1.5,
//             destroy_time: 1.5,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         }
//     },
//     MossyCobblestone, "minecraft:mossy_cobblestone" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     Obsidian, "minecraft:obsidian" => {
//         BlockProperties {
//             explosion_resistance: 1200.0,
//             destroy_time: 50.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     Torch, "minecraft:torch" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     WallTorch, "minecraft:wall_torch" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         WallTorchProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     Fire, "minecraft:fire" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::FIRE)
//         },
//         FireProperties {
//             age, ConstrainedInt<0,15>: ConstrainedInt(0),
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             up: false,
//             west, bool: false
//         }
//     },
//     SoulFire, "minecraft:soul_fire" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::FIRE)
//         }
//     },
//     Spawner, "minecraft:spawner" => {
//         BlockProperties {
//             explosion_resistance: 5.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     OakStairs, "minecraft:oak_stairs" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         OakStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     Chest, "minecraft:chest" => {
//         BlockProperties {
//             explosion_resistance: 2.5,
//             destroy_time: 2.5,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         ChestProperties {
//             facing, Facing: Facing::North,
//             r#type, ChestType: ChestType::Single,
//             waterlogged: false
//         }
//     },
//     RedstoneWire, "minecraft:redstone_wire" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         RedstoneWireProperties {
//             east, RedstoneSide: RedstoneSide::None,
//             north, RedstoneSide: RedstoneSide::None,
//             power: ConstrainedInt(0),
//             south, RedstoneSide: RedstoneSide::None,
//             west, RedstoneSide: RedstoneSide::None
//         }
//     },
//     DiamondOre, "minecraft:diamond_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DeepslateDiamondOre, "minecraft:deepslate_diamond_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 4.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DiamondBlock, "minecraft:diamond_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     CraftingTable, "minecraft:crafting_table" => {
//         BlockProperties {
//             explosion_resistance: 2.5,
//             destroy_time: 2.5,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         }
//     },
//     Wheat, "minecraft:wheat" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         WheatProperties {
//             age, ConstrainedInt<0,7>: ConstrainedInt(0)
//         }
//     },
//     Farmland, "minecraft:farmland" => {
//         BlockProperties {
//             explosion_resistance: 0.6,
//             destroy_time: 0.6,
//             ..BlockProperties::new(crate::materials::Materials::DIRT)
//         },
//         FarmlandProperties {
//             moisture: ConstrainedInt(0)
//         }
//     },
//     Furnace, "minecraft:furnace" => {
//         BlockProperties {
//             explosion_resistance: 3.5,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         FurnaceProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             lit: false
//         }
//     },
//     OakSign, "minecraft:oak_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         OakSignProperties {
//             rotation: ConstrainedInt(0),
//             waterlogged: false
//         }
//     },
//     SpruceSign, "minecraft:spruce_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         SpruceSignProperties {
//             rotation: ConstrainedInt(0),
//             waterlogged: false
//         }
//     },
//     BirchSign, "minecraft:birch_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BirchSignProperties {
//             rotation: ConstrainedInt(0),
//             waterlogged: false
//         }
//     },
//     AcaciaSign, "minecraft:acacia_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         AcaciaSignProperties {
//             rotation: ConstrainedInt(0),
//             waterlogged: false
//         }
//     },
//     JungleSign, "minecraft:jungle_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         JungleSignProperties {
//             rotation: ConstrainedInt(0),
//             waterlogged: false
//         }
//     },
//     DarkOakSign, "minecraft:dark_oak_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         DarkOakSignProperties {
//             rotation: ConstrainedInt(0),
//             waterlogged: false
//         }
//     },
//     MangroveSign, "minecraft:mangrove_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MangroveSignProperties {
//             rotation: ConstrainedInt(0),
//             waterlogged: false
//         }
//     },
//     OakDoor, "minecraft:oak_door" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         OakDoorProperties {
//             facing, Facing: Facing::North,
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower,
//             hinge: DoorHinge::Left,
//             open: false,
//             powered: false
//         }
//     },
//     Ladder, "minecraft:ladder" => {
//         BlockProperties {
//             explosion_resistance: 0.4,
//             destroy_time: 0.4,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         LadderProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             waterlogged: false
//         }
//     },
//     Rail, "minecraft:rail" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.7,
//             destroy_time: 0.7,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         RailProperties {
//             shape, StraightRailShape: StraightRailShape::NorthSouth,
//             waterlogged: false
//         }
//     },
//     CobblestoneStairs, "minecraft:cobblestone_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         CobblestoneStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     OakWallSign, "minecraft:oak_wall_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         OakWallSignProperties {
//             facing, Facing: Facing::North,
//             waterlogged: false
//         }
//     },
//     SpruceWallSign, "minecraft:spruce_wall_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         SpruceWallSignProperties {
//             facing, Facing: Facing::North,
//             waterlogged: false
//         }
//     },
//     BirchWallSign, "minecraft:birch_wall_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BirchWallSignProperties {
//             facing, Facing: Facing::North,
//             waterlogged: false
//         }
//     },
//     AcaciaWallSign, "minecraft:acacia_wall_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         AcaciaWallSignProperties {
//             facing, Facing: Facing::North,
//             waterlogged: false
//         }
//     },
//     JungleWallSign, "minecraft:jungle_wall_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         JungleWallSignProperties {
//             facing, Facing: Facing::North,
//             waterlogged: false
//         }
//     },
//     DarkOakWallSign, "minecraft:dark_oak_wall_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         DarkOakWallSignProperties {
//             facing, Facing: Facing::North,
//             waterlogged: false
//         }
//     },
//     MangroveWallSign, "minecraft:mangrove_wall_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MangroveWallSignProperties {
//             facing, Facing: Facing::North,
//             waterlogged: false
//         }
//     },
//     Lever, "minecraft:lever" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         LeverProperties {
//             face: AttachFace::Wall,
//             facing, Facing: Facing::North,
//             powered: false
//         }
//     },
//     StonePressurePlate, "minecraft:stone_pressure_plate" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         StonePressurePlateProperties {
//             powered: false
//         }
//     },
//     IronDoor, "minecraft:iron_door" => {
//         BlockProperties {
//             explosion_resistance: 5.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         IronDoorProperties {
//             facing, Facing: Facing::North,
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower,
//             hinge: DoorHinge::Left,
//             open: false,
//             powered: false
//         }
//     },
//     OakPressurePlate, "minecraft:oak_pressure_plate" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         OakPressurePlateProperties {
//             powered: false
//         }
//     },
//     SprucePressurePlate, "minecraft:spruce_pressure_plate" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         SprucePressurePlateProperties {
//             powered: false
//         }
//     },
//     BirchPressurePlate, "minecraft:birch_pressure_plate" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BirchPressurePlateProperties {
//             powered: false
//         }
//     },
//     JunglePressurePlate, "minecraft:jungle_pressure_plate" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         JunglePressurePlateProperties {
//             powered: false
//         }
//     },
//     AcaciaPressurePlate, "minecraft:acacia_pressure_plate" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         AcaciaPressurePlateProperties {
//             powered: false
//         }
//     },
//     DarkOakPressurePlate, "minecraft:dark_oak_pressure_plate" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         DarkOakPressurePlateProperties {
//             powered: false
//         }
//     },
//     MangrovePressurePlate, "minecraft:mangrove_pressure_plate" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MangrovePressurePlateProperties {
//             powered: false
//         }
//     },
//     RedstoneOre, "minecraft:redstone_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         RedstoneOreProperties {
//             lit: false
//         }
//     },
//     DeepslateRedstoneOre, "minecraft:deepslate_redstone_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 4.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeepslateRedstoneOreProperties {
//             lit: false
//         }
//     },
//     RedstoneTorch, "minecraft:redstone_torch" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         RedstoneTorchProperties {
//             lit: true
//         }
//     },
//     RedstoneWallTorch, "minecraft:redstone_wall_torch" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         RedstoneWallTorchProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             lit: true
//         }
//     },
//     StoneButton, "minecraft:stone_button" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         StoneButtonProperties {
//             face: AttachFace::Wall,
//             facing, Facing: Facing::North,
//             powered: false
//         }
//     },
//     Snow, "minecraft:snow" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::TOP_SNOW)
//         },
//         SnowProperties {
//             layers: ConstrainedInt(1)
//         }
//     },
//     Ice, "minecraft:ice" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             friction: 0.98,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::ICE)
//         }
//     },
//     SnowBlock, "minecraft:snow_block" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::SNOW)
//         }
//     },
//     Cactus, "minecraft:cactus" => {
//         BlockProperties {
//             explosion_resistance: 0.4,
//             destroy_time: 0.4,
//             ..BlockProperties::new(crate::materials::Materials::CACTUS)
//         },
//         CactusProperties {
//             age, ConstrainedInt<0, 15>: ConstrainedInt(0)
//         }
//     },
//     Clay, "minecraft:clay" => {
//         BlockProperties {
//             explosion_resistance: 0.6,
//             destroy_time: 0.6,
//             ..BlockProperties::new(crate::materials::Materials::CLAY)
//         }
//     },
//     SugarCane, "minecraft:sugar_cane" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         SugarCaneProperties {
//             age, ConstrainedInt<0, 15>: ConstrainedInt(0)
//         }
//     },
//     Jukebox, "minecraft:jukebox" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         JukeboxProperties {
//             has_record: false
//         }
//     },
//     OakFence, "minecraft:oak_fence" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         OakFenceProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     Pumpkin, "minecraft:pumpkin" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::VEGETABLE)
//         }
//     },
//     Netherrack, "minecraft:netherrack" => {
//         BlockProperties {
//             explosion_resistance: 0.4,
//             destroy_time: 0.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     SoulSand, "minecraft:soul_sand" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             speed_factor: 0.4,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     SoulSoil, "minecraft:soul_soil" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::DIRT)
//         }
//     },
//     Basalt, "minecraft:basalt" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         BasaltProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     PolishedBasalt, "minecraft:polished_basalt" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedBasaltProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     SoulTorch, "minecraft:soul_torch" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     SoulWallTorch, "minecraft:soul_wall_torch" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         SoulWallTorchProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     Glowstone, "minecraft:glowstone" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     NetherPortal, "minecraft:nether_portal" => {
//         BlockProperties {
//             has_collision: false,
//             destroy_time: -1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PORTAL)
//         },
//         NetherPortalProperties {
//             axis, HorizontalAxis: HorizontalAxis::X
//         }
//     },
//     CarvedPumpkin, "minecraft:carved_pumpkin" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::VEGETABLE)
//         },
//         CarvedPumpkinProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     JackOLantern, "minecraft:jack_o_lantern" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::VEGETABLE)
//         },
//         JackOLanternProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     Cake, "minecraft:cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         CakeProperties {
//             bites: ConstrainedInt(0)
//         }
//     },
//     Repeater, "minecraft:repeater" => {
//         BlockProperties {
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         RepeaterProperties {
//             delay: ConstrainedInt(1),
//             facing, HorizontalFacing: HorizontalFacing::North,
//             locked: false,
//             powered: false
//         }
//     },
//     WhiteStainedGlass, "minecraft:white_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     OrangeStainedGlass, "minecraft:orange_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     MagentaStainedGlass, "minecraft:magenta_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     LightBlueStainedGlass, "minecraft:light_blue_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     YellowStainedGlass, "minecraft:yellow_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     LimeStainedGlass, "minecraft:lime_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     PinkStainedGlass, "minecraft:pink_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     GrayStainedGlass, "minecraft:gray_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     LightGrayStainedGlass, "minecraft:light_gray_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     CyanStainedGlass, "minecraft:cyan_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     PurpleStainedGlass, "minecraft:purple_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     BlueStainedGlass, "minecraft:blue_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     BrownStainedGlass, "minecraft:brown_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     GreenStainedGlass, "minecraft:green_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     RedStainedGlass, "minecraft:red_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     BlackStainedGlass, "minecraft:black_stained_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     OakTrapdoor, "minecraft:oak_trapdoor" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         OakTrapdoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             open: false,
//             powered: false,
//             waterlogged: false
//         }
//     },
//     SpruceTrapdoor, "minecraft:spruce_trapdoor" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         SpruceTrapdoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             open: false,
//             powered: false,
//             waterlogged: false
//         }
//     },
//     BirchTrapdoor, "minecraft:birch_trapdoor" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BirchTrapdoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             open: false,
//             powered: false,
//             waterlogged: false
//         }
//     },
//     JungleTrapdoor, "minecraft:jungle_trapdoor" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         JungleTrapdoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             open: false,
//             powered: false,
//             waterlogged: false
//         }
//     },
//     AcaciaTrapdoor, "minecraft:acacia_trapdoor" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         AcaciaTrapdoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             open: false,
//             powered: false,
//             waterlogged: false
//         }
//     },
//     DarkOakTrapdoor, "minecraft:dark_oak_trapdoor" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         DarkOakTrapdoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             open: false,
//             powered: false,
//             waterlogged: false
//         }
//     },
//     MangroveTrapdoor, "minecraft:mangrove_trapdoor" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MangroveTrapdoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             open: false,
//             powered: false,
//             waterlogged: false
//         }
//     },
//     StoneBricks, "minecraft:stone_bricks" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     MossyStoneBricks, "minecraft:mossy_stone_bricks" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     CrackedStoneBricks, "minecraft:cracked_stone_bricks" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     ChiseledStoneBricks, "minecraft:chiseled_stone_bricks" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PackedMud, "minecraft:packed_mud" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::DIRT)
//         }
//     },
//     MudBricks, "minecraft:mud_bricks" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     InfestedStone, "minecraft:infested_stone" => {
//         BlockProperties {
//             explosion_resistance: 0.75,
//             destroy_time: 0.75,
//             ..BlockProperties::new(crate::materials::Materials::CLAY)
//         }
//     },
//     InfestedCobblestone, "minecraft:infested_cobblestone" => {
//         BlockProperties {
//             explosion_resistance: 0.75,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::CLAY)
//         }
//     },
//     InfestedStoneBricks, "minecraft:infested_stone_bricks" => {
//         BlockProperties {
//             explosion_resistance: 0.75,
//             destroy_time: 0.75,
//             ..BlockProperties::new(crate::materials::Materials::CLAY)
//         }
//     },
//     InfestedMossyStoneBricks, "minecraft:infested_mossy_stone_bricks" => {
//         BlockProperties {
//             explosion_resistance: 0.75,
//             destroy_time: 0.75,
//             ..BlockProperties::new(crate::materials::Materials::CLAY)
//         }
//     },
//     InfestedCrackedStoneBricks, "minecraft:infested_cracked_stone_bricks" => {
//         BlockProperties {
//             explosion_resistance: 0.75,
//             destroy_time: 0.75,
//             ..BlockProperties::new(crate::materials::Materials::CLAY)
//         }
//     },
//     InfestedChiseledStoneBricks, "minecraft:infested_chiseled_stone_bricks" => {
//         BlockProperties {
//             explosion_resistance: 0.75,
//             destroy_time: 0.75,
//             ..BlockProperties::new(crate::materials::Materials::CLAY)
//         }
//     },
//     BrownMushroomBlock, "minecraft:brown_mushroom_block" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BrownMushroomBlockProperties {
//             down: true,
//             east, bool: true,
//             north, bool: true,
//             south, bool: true,
//             up: true,
//             west, bool: true
//         }
//     },
//     RedMushroomBlock, "minecraft:red_mushroom_block" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         RedMushroomBlockProperties {
//             down: true,
//             east, bool: true,
//             north, bool: true,
//             south, bool: true,
//             up: true,
//             west, bool: true
//         }
//     },
//     MushroomStem, "minecraft:mushroom_stem" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MushroomStemProperties {
//             down: true,
//             east, bool: true,
//             north, bool: true,
//             south, bool: true,
//             up: true,
//             west, bool: true
//         }
//     },
//     IronBars, "minecraft:iron_bars" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         IronBarsProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     Chain, "minecraft:chain" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         ChainProperties {
//             axis, Axis: Axis::Y,
//             waterlogged: false
//         }
//     },
//     GlassPane, "minecraft:glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         GlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     Melon, "minecraft:melon" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::VEGETABLE)
//         }
//     },
//     AttachedPumpkinStem, "minecraft:attached_pumpkin_stem" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         AttachedPumpkinStemProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     AttachedMelonStem, "minecraft:attached_melon_stem" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         AttachedMelonStemProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     PumpkinStem, "minecraft:pumpkin_stem" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         PumpkinStemProperties {
//             age, ConstrainedInt<0, 7>: ConstrainedInt(0)
//         }
//     },
//     MelonStem, "minecraft:melon_stem" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         MelonStemProperties {
//             age, ConstrainedInt<0, 7>: ConstrainedInt(0)
//         }
//     },
//     Vine, "minecraft:vine" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_PLANT)
//         },
//         VineProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             up: false,
//             west, bool: false
//         }
//     },
//     GlowLichen, "minecraft:glow_lichen" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_PLANT)
//         },
//         GlowLichenProperties {
//             down: false,
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             up: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     OakFenceGate, "minecraft:oak_fence_gate" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         OakFenceGateProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             in_wall: false,
//             open: false,
//             powered: false
//         }
//     },
//     BrickStairs, "minecraft:brick_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         BrickStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     StoneBrickStairs, "minecraft:stone_brick_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         StoneBrickStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     MudBrickStairs, "minecraft:mud_brick_stairs" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         MudBrickStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     Mycelium, "minecraft:mycelium" => {
//         BlockProperties {
//             explosion_resistance: 0.6,
//             destroy_time: 0.6,
//             ..BlockProperties::new(crate::materials::Materials::GRASS)
//         },
//         MyceliumProperties {
//             snowy: false
//         }
//     },
//     LilyPad, "minecraft:lily_pad" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     NetherBricks, "minecraft:nether_bricks" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     NetherBrickFence, "minecraft:nether_brick_fence" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         NetherBrickFenceProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     NetherBrickStairs, "minecraft:nether_brick_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         NetherBrickStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     NetherWart, "minecraft:nether_wart" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         NetherWartProperties {
//             age, ConstrainedInt<0, 3>: ConstrainedInt(0)
//         }
//     },
//     EnchantingTable, "minecraft:enchanting_table" => {
//         BlockProperties {
//             explosion_resistance: 1200.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     BrewingStand, "minecraft:brewing_stand" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         BrewingStandProperties {
//             has_bottle_0: false,
//             has_bottle_1: false,
//             has_bottle_2: false
//         }
//     },
//     Cauldron, "minecraft:cauldron" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     WaterCauldron, "minecraft:water_cauldron" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         WaterCauldronProperties {
//             level, ConstrainedInt<1,3>: ConstrainedInt(3)
//         }
//     },
//     LavaCauldron, "minecraft:lava_cauldron" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     PowderSnowCauldron, "minecraft:powder_snow_cauldron" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         PowderSnowCauldronProperties {
//             level, ConstrainedInt<1,3>: ConstrainedInt(3)
//         }
//     },
//     EndPortal, "minecraft:end_portal" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 3600000.0,
//             destroy_time: -1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PORTAL)
//         }
//     },
//     EndPortalFrame, "minecraft:end_portal_frame" => {
//         BlockProperties {
//             explosion_resistance: 3600000.0,
//             destroy_time: -1.0,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         EndPortalFrameProperties {
//             eye: false,
//             facing, HopperFacing: HopperFacing::North
//         }
//     },
//     EndStone, "minecraft:end_stone" => {
//         BlockProperties {
//             explosion_resistance: 9.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DragonEgg, "minecraft:dragon_egg" => {
//         BlockProperties {
//             explosion_resistance: 9.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::EGG)
//         }
//     },
//     RedstoneLamp, "minecraft:redstone_lamp" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             ..BlockProperties::new(crate::materials::Materials::BUILDABLE_GLASS)
//         },
//         RedstoneLampProperties {
//             lit: false
//         }
//     },
//     Cocoa, "minecraft:cocoa" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         CocoaProperties {
//             age, ConstrainedInt<0,2>: ConstrainedInt(0),
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     SandstoneStairs, "minecraft:sandstone_stairs" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         SandstoneStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     EmeraldOre, "minecraft:emerald_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DeepslateEmeraldOre, "minecraft:deepslate_emerald_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 4.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     EnderChest, "minecraft:ender_chest" => {
//         BlockProperties {
//             explosion_resistance: 600.0,
//             destroy_time: 22.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         EnderChestProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             waterlogged: false
//         }
//     },
//     TripwireHook, "minecraft:tripwire_hook" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         TripwireHookProperties {
//             attached: false,
//             facing, HorizontalFacing: HorizontalFacing::North,
//             powered: false
//         }
//     },
//     Tripwire, "minecraft:tripwire" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         TripwireProperties {
//             attached: false,
//             disarmed: false,
//             east, bool: false,
//             north, bool: false,
//             powered: false,
//             south, bool: false,
//             west, bool: false
//         }
//     },
//     EmeraldBlock, "minecraft:emerald_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     SpruceStairs, "minecraft:spruce_stairs" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         SpruceStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     BirchStairs, "minecraft:birch_stairs" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BirchStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     JungleStairs, "minecraft:jungle_stairs" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         JungleStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     CommandBlock, "minecraft:command_block" => {
//         BlockProperties {
//             explosion_resistance: 3600000.0,
//             destroy_time: -1.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         CommandBlockProperties {
//             conditional: false,
//             facing, Facing: Facing::North
//         }
//     },
//     Beacon, "minecraft:beacon" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     CobblestoneWall, "minecraft:cobblestone_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         CobblestoneWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     MossyCobblestoneWall, "minecraft:mossy_cobblestone_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         MossyCobblestoneWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     FlowerPot, "minecraft:flower_pot" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedOakSapling, "minecraft:potted_oak_sapling" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedSpruceSapling, "minecraft:potted_spruce_sapling" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedBirchSapling, "minecraft:potted_birch_sapling" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedJungleSapling, "minecraft:potted_jungle_sapling" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedAcaciaSapling, "minecraft:potted_acacia_sapling" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedDarkOakSapling, "minecraft:potted_dark_oak_sapling" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedMangrovePropagule, "minecraft:potted_mangrove_propagule" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedFern, "minecraft:potted_fern" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedDandelion, "minecraft:potted_dandelion" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedPoppy, "minecraft:potted_poppy" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedBlueOrchid, "minecraft:potted_blue_orchid" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedAllium, "minecraft:potted_allium" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedAzureBluet, "minecraft:potted_azure_bluet" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedRedTulip, "minecraft:potted_red_tulip" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedOrangeTulip, "minecraft:potted_orange_tulip" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedWhiteTulip, "minecraft:potted_white_tulip" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedPinkTulip, "minecraft:potted_pink_tulip" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedOxeyeDaisy, "minecraft:potted_oxeye_daisy" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedCornflower, "minecraft:potted_cornflower" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedLilyOfTheValley, "minecraft:potted_lily_of_the_valley" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedWitherRose, "minecraft:potted_wither_rose" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedRedMushroom, "minecraft:potted_red_mushroom" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedBrownMushroom, "minecraft:potted_brown_mushroom" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedDeadBush, "minecraft:potted_dead_bush" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedCactus, "minecraft:potted_cactus" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     Carrots, "minecraft:carrots" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         CarrotsProperties {
//             age, ConstrainedInt<0, 7>: ConstrainedInt(0)
//         }
//     },
//     Potatoes, "minecraft:potatoes" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         PotatoesProperties {
//             age, ConstrainedInt<0, 7>: ConstrainedInt(0)
//         }
//     },
//     OakButton, "minecraft:oak_button" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         OakButtonProperties {
//             face: AttachFace::Wall,
//             facing, Facing: Facing::North,
//             powered: false
//         }
//     },
//     SpruceButton, "minecraft:spruce_button" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         SpruceButtonProperties {
//             face: AttachFace::Wall,
//             facing, Facing: Facing::North,
//             powered: false
//         }
//     },
//     BirchButton, "minecraft:birch_button" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         BirchButtonProperties {
//             face: AttachFace::Wall,
//             facing, Facing: Facing::North,
//             powered: false
//         }
//     },
//     JungleButton, "minecraft:jungle_button" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         JungleButtonProperties {
//             face: AttachFace::Wall,
//             facing, Facing: Facing::North,
//             powered: false
//         }
//     },
//     AcaciaButton, "minecraft:acacia_button" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         AcaciaButtonProperties {
//             face: AttachFace::Wall,
//             facing, Facing: Facing::North,
//             powered: false
//         }
//     },
//     DarkOakButton, "minecraft:dark_oak_button" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         DarkOakButtonProperties {
//             face: AttachFace::Wall,
//             facing, Facing: Facing::North,
//             powered: false
//         }
//     },
//     MangroveButton, "minecraft:mangrove_button" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         MangroveButtonProperties {
//             face: AttachFace::Wall,
//             facing, Facing: Facing::North,
//             powered: false
//         }
//     },
//     SkeletonSkull, "minecraft:skeleton_skull" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         SkeletonSkullProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     SkeletonWallSkull, "minecraft:skeleton_wall_skull" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         SkeletonWallSkullProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     WitherSkeletonSkull, "minecraft:wither_skeleton_skull" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         WitherSkeletonSkullProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     WitherSkeletonWallSkull, "minecraft:wither_skeleton_wall_skull" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         WitherSkeletonWallSkullProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     ZombieHead, "minecraft:zombie_head" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         ZombieHeadProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     ZombieWallHead, "minecraft:zombie_wall_head" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         ZombieWallHeadProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     PlayerHead, "minecraft:player_head" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         PlayerHeadProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     PlayerWallHead, "minecraft:player_wall_head" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         PlayerWallHeadProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     CreeperHead, "minecraft:creeper_head" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         CreeperHeadProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     CreeperWallHead, "minecraft:creeper_wall_head" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         CreeperWallHeadProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     DragonHead, "minecraft:dragon_head" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         DragonHeadProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     DragonWallHead, "minecraft:dragon_wall_head" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         DragonWallHeadProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     Anvil, "minecraft:anvil" => {
//         BlockProperties {
//             explosion_resistance: 1200.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::HEAVY_METAL)
//         },
//         AnvilProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     ChippedAnvil, "minecraft:chipped_anvil" => {
//         BlockProperties {
//             explosion_resistance: 1200.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::HEAVY_METAL)
//         },
//         ChippedAnvilProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     DamagedAnvil, "minecraft:damaged_anvil" => {
//         BlockProperties {
//             explosion_resistance: 1200.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::HEAVY_METAL)
//         },
//         DamagedAnvilProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     TrappedChest, "minecraft:trapped_chest" => {
//         BlockProperties {
//             explosion_resistance: 2.5,
//             destroy_time: 2.5,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         TrappedChestProperties {
//             facing, Facing: Facing::North,
//             r#type, ChestType: ChestType::Single,
//             waterlogged: false
//         }
//     },
//     LightWeightedPressurePlate, "minecraft:light_weighted_pressure_plate" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         LightWeightedPressurePlateProperties {
//             power: ConstrainedInt(0)
//         }
//     },
//     HeavyWeightedPressurePlate, "minecraft:heavy_weighted_pressure_plate" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         HeavyWeightedPressurePlateProperties {
//             power: ConstrainedInt(0)
//         }
//     },
//     Comparator, "minecraft:comparator" => {
//         BlockProperties {
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         ComparatorProperties {
//             facing, Facing: Facing::North,
//             mode, ComparatorMode: ComparatorMode::Compare,
//             powered: false
//         }
//     },
//     DaylightDetector, "minecraft:daylight_detector" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         DaylightDetectorProperties {
//             inverted: false,
//             power: ConstrainedInt(0)
//         }
//     },
//     RedstoneBlock, "minecraft:redstone_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     NetherQuartzOre, "minecraft:nether_quartz_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     Hopper, "minecraft:hopper" => {
//         BlockProperties {
//             explosion_resistance: 4.8,
//             destroy_time: 3.0,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         HopperProperties {
//             enabled: true,
//             facing, HopperFacing: HopperFacing::Down
//         }
//     },
//     QuartzBlock, "minecraft:quartz_block" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     ChiseledQuartzBlock, "minecraft:chiseled_quartz_block" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     QuartzPillar, "minecraft:quartz_pillar" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         QuartzPillarProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     QuartzStairs, "minecraft:quartz_stairs" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         QuartzStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     ActivatorRail, "minecraft:activator_rail" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.7,
//             destroy_time: 0.7,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         ActivatorRailProperties {
//             powered: false,
//             shape, StraightRailShape: StraightRailShape::NorthSouth,
//             waterlogged: false
//         }
//     },
//     Dropper, "minecraft:dropper" => {
//         BlockProperties {
//             explosion_resistance: 3.5,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DropperProperties {
//             facing, Facing: Facing::North,
//             triggered: false
//         }
//     },
//     WhiteTerracotta, "minecraft:white_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     OrangeTerracotta, "minecraft:orange_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     MagentaTerracotta, "minecraft:magenta_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     LightBlueTerracotta, "minecraft:light_blue_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     YellowTerracotta, "minecraft:yellow_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     LimeTerracotta, "minecraft:lime_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PinkTerracotta, "minecraft:pink_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     GrayTerracotta, "minecraft:gray_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     LightGrayTerracotta, "minecraft:light_gray_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     CyanTerracotta, "minecraft:cyan_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PurpleTerracotta, "minecraft:purple_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     BlueTerracotta, "minecraft:blue_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     BrownTerracotta, "minecraft:brown_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     GreenTerracotta, "minecraft:green_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     RedTerracotta, "minecraft:red_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     BlackTerracotta, "minecraft:black_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     WhiteStainedGlassPane, "minecraft:white_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         WhiteStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     OrangeStainedGlassPane, "minecraft:orange_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         OrangeStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     MagentaStainedGlassPane, "minecraft:magenta_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         MagentaStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     LightBlueStainedGlassPane, "minecraft:light_blue_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         LightBlueStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     YellowStainedGlassPane, "minecraft:yellow_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         YellowStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     LimeStainedGlassPane, "minecraft:lime_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         LimeStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     PinkStainedGlassPane, "minecraft:pink_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         PinkStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     GrayStainedGlassPane, "minecraft:gray_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         GrayStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     LightGrayStainedGlassPane, "minecraft:light_gray_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         LightGrayStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     CyanStainedGlassPane, "minecraft:cyan_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         CyanStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     PurpleStainedGlassPane, "minecraft:purple_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         PurpleStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     BlueStainedGlassPane, "minecraft:blue_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         BlueStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     BrownStainedGlassPane, "minecraft:brown_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         BrownStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     GreenStainedGlassPane, "minecraft:green_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         GreenStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     RedStainedGlassPane, "minecraft:red_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         RedStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     BlackStainedGlassPane, "minecraft:black_stained_glass_pane" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         BlackStainedGlassPaneProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     AcaciaStairs, "minecraft:acacia_stairs" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         AcaciaStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     DarkOakStairs, "minecraft:dark_oak_stairs" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         DarkOakStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     MangroveStairs, "minecraft:mangrove_stairs" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MangroveStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     SlimeBlock, "minecraft:slime_block" => {
//         BlockProperties {
//             friction: 0.8,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::CLAY)
//         }
//     },
//     Barrier, "minecraft:barrier" => {
//         BlockProperties {
//             explosion_resistance: 3600000.8,
//             destroy_time: -1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::BARRIER)
//         }
//     },
//     Light, "minecraft:light" => {
//         BlockProperties {
//             explosion_resistance: 3600000.8,
//             destroy_time: -1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::AIR)
//         },
//         LightProperties {
//             level, ConstrainedInt<0, 15>: ConstrainedInt(0),
//             waterlogged: false
//         }
//     },
//     IronTrapdoor, "minecraft:iron_trapdoor" => {
//         BlockProperties {
//             explosion_resistance: 5.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         IronTrapDoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             open: false,
//             powered: false,
//             waterlogged: false
//         }
//     },
//     Prismarine, "minecraft:prismarine" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PrismarineBricks, "minecraft:prismarine_bricks" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DarkPrismarine, "minecraft:dark_prismarine" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PrismarineStairs, "minecraft:prismarine_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PrismarineStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     PrismarineBrickStairs, "minecraft:prismarine_brick_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PrismarineBrickStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     DarkPrismarineStairs, "minecraft:dark_prismarine_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DarkPrismarineStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     PrismarineSlab, "minecraft:prismarine_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PrismarineSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     PrismarineBrickSlab, "minecraft:prismarine_brick_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PrismarineBrickSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     DarkPrismarineSlab, "minecraft:dark_prismarine_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DarkPrismarineSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     SeaLantern, "minecraft:sea_lantern" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     HayBlock, "minecraft:hay_block" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::GRASS)
//         },
//         HayBlockProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     WhiteCarpet, "minecraft:white_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     OrangeCarpet, "minecraft:orange_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     MagentaCarpet, "minecraft:magenta_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     LightBlueCarpet, "minecraft:light_blue_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     YellowCarpet, "minecraft:yellow_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     LimeCarpet, "minecraft:lime_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     PinkCarpet, "minecraft:pink_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     GrayCarpet, "minecraft:gray_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     LightGrayCarpet, "minecraft:light_gray_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     CyanCarpet, "minecraft:cyan_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     PurpleCarpet, "minecraft:purple_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     BlueCarpet, "minecraft:blue_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     BrownCarpet, "minecraft:brown_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     GreenCarpet, "minecraft:green_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     RedCarpet, "minecraft:red_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     BlackCarpet, "minecraft:black_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::CLOTH_DECORATION)
//         }
//     },
//     Terracotta, "minecraft:terracotta" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     CoalBlock, "minecraft:coal_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PackedIce, "minecraft:packed_ice" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             friction: 0.98,
//             ..BlockProperties::new(crate::materials::Materials::ICE_SOLID)
//         }
//     },
//     Sunflower, "minecraft:sunflower" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_PLANT)
//         },
//         SunflowerProperties {
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower
//         }
//     },
//     Lilac, "minecraft:lilac" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_PLANT)
//         },
//         LilacProperties {
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower
//         }
//     },
//     RoseBush, "minecraft:rose_bush" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_PLANT)
//         },
//         RoseBushProperties {
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower
//         }
//     },
//     Peony, "minecraft:peony" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_PLANT)
//         },
//         PeonyProperties {
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower
//         }
//     },
//     TallGrass, "minecraft:tall_grass" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_PLANT)
//         },
//         TallGrassProperties {
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower
//         }
//     },
//     LargeFern, "minecraft:large_fern" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_PLANT)
//         },
//         LargeFernProperties {
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower
//         }
//     },
//     WhiteBanner, "minecraft:white_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         WhiteBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     OrangeBanner, "minecraft:orange_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         OrangeBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     MagentaBanner, "minecraft:magenta_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MagentaBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     LightBlueBanner, "minecraft:light_blue_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         LightBlueBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     YellowBanner, "minecraft:yellow_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         YellowBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     LimeBanner, "minecraft:lime_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         LimeBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     PinkBanner, "minecraft:pink_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         PinkBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     GrayBanner, "minecraft:gray_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         GrayBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     LightGrayBanner, "minecraft:light_gray_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         LightGrayBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     CyanBanner, "minecraft:cyan_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         CyanBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     PurpleBanner, "minecraft:purple_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         PurpleBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     BlueBanner, "minecraft:blue_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BlueBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     BrownBanner, "minecraft:brown_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BrownBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     GreenBanner, "minecraft:green_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         GreenBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     RedBanner, "minecraft:red_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         RedBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     BlackBanner, "minecraft:black_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BlackBannerProperties {
//             rotation: ConstrainedInt(0)
//         }
//     },
//     WhiteWallBanner, "minecraft:white_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         WhiteWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     OrangeWallBanner, "minecraft:orange_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         OrangeWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     MagentaWallBanner, "minecraft:magenta_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MagentaWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     LightBlueWallBanner, "minecraft:light_blue_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         LightBlueWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     YellowWallBanner, "minecraft:yellow_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         YellowWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     LimeWallBanner, "minecraft:lime_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         LimeWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     PinkWallBanner, "minecraft:pink_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         PinkWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     GrayWallBanner, "minecraft:gray_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         GrayWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     LightGrayWallBanner, "minecraft:light_gray_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         LightGrayWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     CyanWallBanner, "minecraft:cyan_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         CyanWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     PurpleWallBanner, "minecraft:purple_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         PurpleWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     BlueWallBanner, "minecraft:blue_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BlueWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     BrownWallBanner, "minecraft:brown_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BrownWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     GreenWallBanner, "minecraft:green_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         GreenWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     RedWallBanner, "minecraft:red_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         RedWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     BlackWallBanner, "minecraft:black_wall_banner" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BlackWallBannerProperties {
//             facing, HorizontalFacing: HorizontalFacing::North
//         }
//     },
//     RedSandstone, "minecraft:red_sandstone" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     ChiseledRedSandstone, "minecraft:chiseled_red_sandstone" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     CutRedSandstone, "minecraft:cut_red_sandstone" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     RedSandstoneStairs, "minecraft:red_sandstone_stairs" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         RedSandstoneStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     OakSlab, "minecraft:oak_slab" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         OakSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     SpruceSlab, "minecraft:spruce_slab" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         SpruceSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     BirchSlab, "minecraft:birch_slab" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BirchSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     JungleSlab, "minecraft:jungle_slab" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         JungleSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     AcaciaSlab, "minecraft:acacia_slab" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         AcaciaSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     DarkOakSlab, "minecraft:dark_oak_slab" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         DarkOakSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     MangroveSlab, "minecraft:mangrove_slab" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MangroveSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     StoneSlab, "minecraft:stone_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         StoneSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     SmoothStoneSlab, "minecraft:smooth_stone_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         SmoothStoneSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     SandstoneSlab, "minecraft:sandstone_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         SandstoneSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     CutSandstoneSlab, "minecraft:cut_sandstone_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         CutSandstoneSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     PetrifiedOakSlab, "minecraft:petrified_oak_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PetrifiedOakSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     CobblestoneSlab, "minecraft:cobblestone_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         CobblestoneSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     BrickSlab, "minecraft:brick_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         BrickSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     StoneBrickSlab, "minecraft:stone_brick_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         StoneBrickSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     MudBrickSlab, "minecraft:mud_brick_slab" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         MudBrickSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     NetherBrickSlab, "minecraft:nether_brick_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         NetherBrickSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     QuartzSlab, "minecraft:quartz_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         QuartzSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     RedSandstoneSlab, "minecraft:red_sandstone_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         RedSandstoneSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     CutRedSandstoneSlab, "minecraft:cut_red_sandstone_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         CutRedSandstoneSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     PurpurSlab, "minecraft:purpur_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PurpurSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     SmoothStone, "minecraft:smooth_stone" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     SmoothSandstone, "minecraft:smooth_sandstone" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     SmoothQuartz, "minecraft:smooth_quartz" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     SmoothRedSandstone, "minecraft:smooth_red_sandstone" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     SpruceFenceGate, "minecraft:spruce_fence_gate" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         SpruceFenceGateProperties {
//             facing, Facing: Facing::North,
//             in_wall: false,
//             open: false,
//             powered: false
//         }
//     },
//     BirchFenceGate, "minecraft:birch_fence_gate" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BirchFenceGateProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             in_wall: false,
//             open: false,
//             powered: false
//         }
//     },
//     JungleFenceGate, "minecraft:jungle_fence_gate" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         JungleFenceGateProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             in_wall: false,
//             open: false,
//             powered: false
//         }
//     },
//     AcaciaFenceGate, "minecraft:acacia_fence_gate" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         AcaciaFenceGateProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             in_wall: false,
//             open: false,
//             powered: false
//         }
//     },
//     DarkOakFenceGate, "minecraft:dark_oak_fence_gate" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         DarkOakFenceGateProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             in_wall: false,
//             open: false,
//             powered: false
//         }
//     },
//     MangroveFenceGate, "minecraft:mangrove_fence_gate" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MangroveFenceGateProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             in_wall: false,
//             open: false,
//             powered: false
//         }
//     },
//     SpruceFence, "minecraft:spruce_fence" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         SpruceFenceProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     BirchFence, "minecraft:birch_fence" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BirchFenceProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     JungleFence, "minecraft:jungle_fence" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         JungleFenceProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     AcaciaFence, "minecraft:acacia_fence" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         AcaciaFenceProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     DarkOakFence, "minecraft:dark_oak_fence" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         DarkOakFenceProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     MangroveFence, "minecraft:mangrove_fence" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MangroveFenceProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     SpruceDoor, "minecraft:spruce_door" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         SpruceDoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower,
//             hinge: DoorHinge::Left,
//             open: false,
//             powered: false
//         }
//     },
//     BirchDoor, "minecraft:birch_door" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BirchDoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower,
//             hinge: DoorHinge::Left,
//             open: false,
//             powered: false
//         }
//     },
//     JungleDoor, "minecraft:jungle_door" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         JungleDoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower,
//             hinge: DoorHinge::Left,
//             open: false,
//             powered: false
//         }
//     },
//     AcaciaDoor, "minecraft:acacia_door" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         AcaciaDoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower,
//             hinge: DoorHinge::Left,
//             open: false,
//             powered: false
//         }
//     },
//     DarkOakDoor, "minecraft:dark_oak_door" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         DarkOakDoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower,
//             hinge: DoorHinge::Left,
//             open: false,
//             powered: false
//         }
//     },
//     MangroveDoor, "minecraft:mangrove_door" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         MangroveDoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower,
//             hinge: DoorHinge::Left,
//             open: false,
//             powered: false
//         }
//     },
//     EndRod, "minecraft:end_rod" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         EndRodProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     ChorusPlant, "minecraft:chorus_plant" => {
//         BlockProperties {
//             explosion_resistance: 0.4,
//             destroy_time: 0.4,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         ChorusPlantProperties {
//             down: false,
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             up: false,
//             west, bool: false
//         }
//     },
//     ChorusFlower, "minecraft:chorus_flower" => {
//         BlockProperties {
//             explosion_resistance: 0.4,
//             destroy_time: 0.4,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         ChorusFlowerProperties {
//             age, ConstrainedInt<0, 5>: ConstrainedInt(0)
//         }
//     },
//     PurpurBlock, "minecraft:purpur_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PurpurPillar, "minecraft:purpur_pillar" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PurpurPillarProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     PurpurStairs, "minecraft:purpur_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PurpurStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     EndStoneBricks, "minecraft:end_stone_bricks" => {
//         BlockProperties {
//             explosion_resistance: 9.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     Beetroots, "minecraft:beetroots" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         BeetrootsProperties {
//             age, ConstrainedInt<0, 3>: ConstrainedInt(0)
//         }
//     },
//     DirtPath, "minecraft:dirt_path" => {
//         BlockProperties {
//             explosion_resistance: 0.65,
//             destroy_time: 0.65,
//             ..BlockProperties::new(crate::materials::Materials::DIRT)
//         }
//     },
//     EndGateway, "minecraft:end_gateway" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 3600000.0,
//             destroy_time: -1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PORTAL)
//         }
//     },
//     RepeatingCommandBlock, "minecraft:repeating_command_block" => {
//         BlockProperties {
//             explosion_resistance: 3600000.0,
//             destroy_time: -1.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         RepeatingCommandBlockProperties {
//             conditional: false,
//             facing, Facing: Facing::North
//         }
//     },
//     ChainCommandBlock, "minecraft:chain_command_block" => {
//         BlockProperties {
//             explosion_resistance: 3600000.0,
//             destroy_time: -1.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         ChainCommandBlockProperties {
//             conditional: false,
//             facing, Facing: Facing::North
//         }
//     },
//     FrostedIce, "minecraft:frosted_ice" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             friction: 0.98,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::ICE)
//         },
//         FrostedIceProperties {
//             age, ConstrainedInt<0, 3>: ConstrainedInt(0)
//         }
//     },
//     MagmaBlock, "minecraft:magma_block" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     NetherWartBlock, "minecraft:nether_wart_block" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::GRASS)
//         }
//     },
//     RedNetherBricks, "minecraft:red_nether_bricks" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     BoneBlock, "minecraft:bone_block" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         BoneBlockProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StructureVoid, "minecraft:structure_void" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STRUCTURAL_AIR)
//         }
//     },
//     Observer, "minecraft:observer" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         ObserverProperties {
//             facing, Facing: Facing::South,
//             powered: false
//         }
//     },
//     ShulkerBox, "minecraft:shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         ShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     WhiteShulkerBox, "minecraft:white_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         WhiteShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     OrangeShulkerBox, "minecraft:orange_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         OrangeShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     MagentaShulkerBox, "minecraft:magenta_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         MagentaShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     LightBlueShulkerBox, "minecraft:light_blue_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         LightBlueShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     YellowShulkerBox, "minecraft:yellow_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         YellowShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     LimeShulkerBox, "minecraft:lime_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         LimeShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     PinkShulkerBox, "minecraft:pink_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         PinkShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     GrayShulkerBox, "minecraft:gray_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         GrayShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     LightGrayShulkerBox, "minecraft:light_gray_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         LightGrayShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     CyanShulkerBox, "minecraft:cyan_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         CyanShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     PurpleShulkerBox, "minecraft:purple_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         PurpleShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     BlueShulkerBox, "minecraft:blue_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         BlueShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     BrownShulkerBox, "minecraft:brown_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         BrownShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     GreenShulkerBox, "minecraft:green_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         GreenShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     RedShulkerBox, "minecraft:red_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         RedShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     BlackShulkerBox, "minecraft:black_shulker_box" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SHULKER_SHELL)
//         },
//         BlackShulkerBoxProperties {
//             facing, Facing: Facing::Up
//         }
//     },
//     WhiteGlazedTerracotta, "minecraft:white_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         WhiteGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     OrangeGlazedTerracotta, "minecraft:orange_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         OrangeGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     MagentaGlazedTerracotta, "minecraft:magenta_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         MagentaGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     LightBlueGlazedTerracotta, "minecraft:light_blue_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         LightBlueGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     YellowGlazedTerracotta, "minecraft:yellow_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         YellowGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     LimeGlazedTerracotta, "minecraft:lime_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         LimeGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     PinkGlazedTerracotta, "minecraft:pink_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PinkGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     GrayGlazedTerracotta, "minecraft:gray_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         GrayGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     LightGrayGlazedTerracotta, "minecraft:light_gray_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         LightGrayGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     CyanGlazedTerracotta, "minecraft:cyan_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         CyanGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     PurpleGlazedTerracotta, "minecraft:purple_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PurpleGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     BlueGlazedTerracotta, "minecraft:blue_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         BlueGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     BrownGlazedTerracotta, "minecraft:brown_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         BrownGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     GreenGlazedTerracotta, "minecraft:green_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         GreenGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     RedGlazedTerracotta, "minecraft:red_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         RedGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     BlackGlazedTerracotta, "minecraft:black_glazed_terracotta" => {
//         BlockProperties {
//             explosion_resistance: 1.4,
//             destroy_time: 1.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         BlackGlazedTerracottaProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     WhiteConcrete, "minecraft:white_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     OrangeConcrete, "minecraft:orange_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     MagentaConcrete, "minecraft:magenta_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     LightBlueConcrete, "minecraft:light_blue_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     YellowConcrete, "minecraft:yellow_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     LimeConcrete, "minecraft:lime_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PinkConcrete, "minecraft:pink_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     GrayConcrete, "minecraft:gray_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     LightGrayConcrete, "minecraft:light_gray_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     CyanConcrete, "minecraft:cyan_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PurpleConcrete, "minecraft:purple_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     BlueConcrete, "minecraft:blue_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     BrownConcrete, "minecraft:brown_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     GreenConcrete, "minecraft:green_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     RedConcrete, "minecraft:red_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     BlackConcrete, "minecraft:black_concrete" => {
//         BlockProperties {
//             explosion_resistance: 1.8,
//             destroy_time: 1.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     WhiteConcretePowder, "minecraft:white_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     OrangeConcretePowder, "minecraft:orange_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     MagentaConcretePowder, "minecraft:magenta_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     LightBlueConcretePowder, "minecraft:light_blue_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     YellowConcretePowder, "minecraft:yellow_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     LimeConcretePowder, "minecraft:lime_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     PinkConcretePowder, "minecraft:pink_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     GrayConcretePowder, "minecraft:gray_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     LightGrayConcretePowder, "minecraft:light_gray_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     CyanConcretePowder, "minecraft:cyan_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     PurpleConcretePowder, "minecraft:purple_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     BlueConcretePowder, "minecraft:blue_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     BrownConcretePowder, "minecraft:brown_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     GreenConcretePowder, "minecraft:green_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     RedConcretePowder, "minecraft:red_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     BlackConcretePowder, "minecraft:black_concrete_powder" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::SAND)
//         }
//     },
//     Kelp, "minecraft:kelp" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         KelpProperties {
//             age, ConstrainedInt<0, 25>: ConstrainedInt(0)
//         }
//     },
//     KelpPlant, "minecraft:kelp_plant" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         }
//     },
//     DriedKelpBlock, "minecraft:dried_kelp_block" => {
//         BlockProperties {
//             explosion_resistance: 2.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::GRASS)
//         }
//     },
//     TurtleEgg, "minecraft:turtle_egg" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::EGG)
//         },
//         TurtleEggProperties {
//             eggs: ConstrainedInt(0),
//             hatch: ConstrainedInt(0)
//         }
//     },
//     DeadTubeCoralBlock, "minecraft:dead_tube_coral_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DeadBrainCoralBlock, "minecraft:dead_brain_coral_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DeadBubbleCoralBlock, "minecraft:dead_bubble_coral_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DeadFireCoralBlock, "minecraft:dead_fire_coral_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DeadHornCoralBlock, "minecraft:dead_horn_coral_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     TubeCoralBlock, "minecraft:tube_coral_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     BrainCoralBlock, "minecraft:brain_coral_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     BubbleCoralBlock, "minecraft:bubble_coral_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     FireCoralBlock, "minecraft:fire_coral_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     HornCoralBlock, "minecraft:horn_coral_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DeadTubeCoral, "minecraft:dead_tube_coral" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadTubeCoralProperties {
//             waterlogged: true
//         }
//     },
//     DeadBrainCoral, "minecraft:dead_brain_coral" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadBrainCoralProperties {
//             waterlogged: true
//         }
//     },
//     DeadBubbleCoral, "minecraft:dead_bubble_coral" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadBubbleCoralProperties {
//             waterlogged: true
//         }
//     },
//     DeadFireCoral, "minecraft:dead_fire_coral" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadFireCoralProperties {
//             waterlogged: true
//         }
//     },
//     DeadHornCoral, "minecraft:dead_horn_coral" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadHornCoralProperties {
//             waterlogged: true
//         }
//     },
//     TubeCoral, "minecraft:tube_coral" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         TubeCoralProperties {
//             waterlogged: true
//         }
//     },
//     BrainCoral, "minecraft:brain_coral" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         BrainCoralProperties {
//             waterlogged: true
//         }
//     },
//     BubbleCoral, "minecraft:bubble_coral" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         BubbleCoralProperties {
//             waterlogged: true
//         }
//     },
//     FireCoral, "minecraft:fire_coral" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         FireCoralProperties {
//             waterlogged: true
//         }
//     },
//     HornCoral, "minecraft:horn_coral" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         HornCoralProperties {
//             waterlogged: true
//         }
//     },
//     DeadTubeCoralFan, "minecraft:dead_tube_coral_fan" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadTubeCoralFanProperties {
//             waterlogged: true
//         }
//     },
//     DeadBrainCoralFan, "minecraft:dead_brain_coral_fan" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadBrainCoralFanProperties {
//             waterlogged: true
//         }
//     },
//     DeadBubbleCoralFan, "minecraft:dead_bubble_coral_fan" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadBubbleCoralFanProperties {
//             waterlogged: true
//         }
//     },
//     DeadFireCoralFan, "minecraft:dead_fire_coral_fan" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadFireCoralFanProperties {
//             waterlogged: true
//         }
//     },
//     DeadHornCoralFan, "minecraft:dead_horn_coral_fan" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadHornCoralFanProperties {
//             waterlogged: true
//         }
//     },
//     TubeCoralFan, "minecraft:tube_coral_fan" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         TubeCoralFanProperties {
//             waterlogged: true
//         }
//     },
//     BrainCoralFan, "minecraft:brain_coral_fan" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         BrainCoralFanProperties {
//             waterlogged: true
//         }
//     },
//     BubbleCoralFan, "minecraft:bubble_coral_fan" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         BubbleCoralFanProperties {
//             waterlogged: true
//         }
//     },
//     FireCoralFan, "minecraft:fire_coral_fan" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         FireCoralFanProperties {
//             waterlogged: true
//         }
//     },
//     HornCoralFan, "minecraft:horn_coral_fan" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         HornCoralFanProperties {
//             waterlogged: true
//         }
//     },
//     DeadTubeCoralWallFan, "minecraft:dead_tube_coral_wall_fan" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadTubeCoralWallFanProperties {
//             facing, Facing: Facing::North,
//             waterlogged: true
//         }
//     },
//     DeadBrainCoralWallFan, "minecraft:dead_brain_coral_wall_fan" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadBrainCoralWallFanProperties {
//             facing, Facing: Facing::North,
//             waterlogged: true
//         }
//     },
//     DeadBubbleCoralWallFan, "minecraft:dead_bubble_coral_wall_fan" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadBubbleCoralWallFanProperties {
//             facing, Facing: Facing::North,
//             waterlogged: true
//         }
//     },
//     DeadFireCoralWallFan, "minecraft:dead_fire_coral_wall_fan" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadFireCoralWallFanProperties {
//             facing, Facing: Facing::North,
//             waterlogged: true
//         }
//     },
//     DeadHornCoralWallFan, "minecraft:dead_horn_coral_wall_fan" => {
//         BlockProperties {
//             has_collision: false,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeadHornCoralWallFanProperties {
//             facing, Facing: Facing::North,
//             waterlogged: true
//         }
//     },
//     TubeCoralWallFan, "minecraft:tube_coral_wall_fan" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         TubeCoralWallFanProperties {
//             facing, Facing: Facing::North,
//             waterlogged: true
//         }
//     },
//     BrainCoralWallFan, "minecraft:brain_coral_wall_fan" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         BrainCoralWallFanProperties {
//             facing, Facing: Facing::North,
//             waterlogged: true
//         }
//     },
//     BubbleCoralWallFan, "minecraft:bubble_coral_wall_fan" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         BubbleCoralWallFanProperties {
//             facing, Facing: Facing::North,
//             waterlogged: true
//         }
//     },
//     FireCoralWallFan, "minecraft:fire_coral_wall_fan" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         FireCoralWallFanProperties {
//             facing, Facing: Facing::North,
//             waterlogged: true
//         }
//     },
//     HornCoralWallFan, "minecraft:horn_coral_wall_fan" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         HornCoralWallFanProperties {
//             facing, Facing: Facing::North,
//             waterlogged: true
//         }
//     },
//     SeaPickle, "minecraft:sea_pickle" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WATER_PLANT)
//         },
//         SeaPickleProperties {
//             pickles: ConstrainedInt(1),
//             waterlogged: true
//         }
//     },
//     BlueIce, "minecraft:blue_ice" => {
//         BlockProperties {
//             explosion_resistance: 2.8,
//             destroy_time: 2.8,
//             friction: 0.989,
//             ..BlockProperties::new(crate::materials::Materials::ICE_SOLID)
//         }
//     },
//     Conduit, "minecraft:conduit" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         },
//         ConduitProperties {
//             waterlogged: true
//         }
//     },
//     BambooSapling, "minecraft:bamboo_sapling" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::BAMBOO_SAPLING)
//         }
//     },
//     Bamboo, "minecraft:bamboo" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::BAMBOO)
//         },
//         BambooProperties {
//             age, ConstrainedInt<0, 1>: ConstrainedInt(0),
//             leaves: BambooLeaves::None,
//             stage: ConstrainedInt(0)
//         }
//     },
//     PottedBamboo, "minecraft:potted_bamboo" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     VoidAir, "minecraft:void_air" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             is_air: true,
//             ..BlockProperties::new(crate::materials::Materials::AIR)
//         }
//     },
//     CaveAir, "minecraft:cave_air" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             is_air: true,
//             ..BlockProperties::new(crate::materials::Materials::AIR)
//         }
//     },
//     BubbleColumn, "minecraft:bubble_column" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::BUBBLE_COLUMN)
//         },
//         BubbleColumnProperties {
//             drag: true
//         }
//     },
//     PolishedGraniteStairs, "minecraft:polished_granite_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedGraniteStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     SmoothRedSandstoneStairs, "minecraft:smooth_red_sandstone_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         SmoothRedSandstoneStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     MossyStoneBrickStairs, "minecraft:mossy_stone_brick_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         MossyStoneBrickStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     PolishedDioriteStairs, "minecraft:polished_diorite_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedDioriteStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     MossyCobblestoneStairs, "minecraft:mossy_cobblestone_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         MossyCobblestoneStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     EndStoneBrickStairs, "minecraft:end_stone_brick_stairs" => {
//         BlockProperties {
//             explosion_resistance: 9.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         EndStoneBrickStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     StoneStairs, "minecraft:stone_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         StoneStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     SmoothSandstoneStairs, "minecraft:smooth_sandstone_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         SmoothSandstoneStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     SmoothQuartzStairs, "minecraft:smooth_quartz_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         SmoothQuartzStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     GraniteStairs, "minecraft:granite_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         GraniteStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     AndesiteStairs, "minecraft:andesite_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         AndesiteStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     RedNetherBrickStairs, "minecraft:red_nether_brick_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         RedNetherBrickStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     PolishedAndesiteStairs, "minecraft:polished_andesite_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedAndesiteStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     DioriteStairs, "minecraft:diorite_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DioriteStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     PolishedGraniteSlab, "minecraft:polished_granite_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedGraniteSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     SmoothRedSandstoneSlab, "minecraft:smooth_red_sandstone_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         SmoothRedSandstoneSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     MossyStoneBrickSlab, "minecraft:mossy_stone_brick_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         MossyStoneBrickSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     PolishedDioriteSlab, "minecraft:polished_diorite_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedDioriteSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     MossyCobblestoneSlab, "minecraft:mossy_cobblestone_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         MossyCobblestoneSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     EndStoneBrickSlab, "minecraft:end_stone_brick_slab" => {
//         BlockProperties {
//             explosion_resistance: 9.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         EndStoneBrickSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     SmoothSandstoneSlab, "minecraft:smooth_sandstone_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         SmoothSandstoneSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     SmoothQuartzSlab, "minecraft:smooth_quartz_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         SmoothQuartzSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     GraniteSlab, "minecraft:granite_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         GraniteSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     AndesiteSlab, "minecraft:andesite_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         AndesiteSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     RedNetherBrickSlab, "minecraft:red_nether_brick_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         RedNetherBrickSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     PolishedAndesiteSlab, "minecraft:polished_andesite_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedAndesiteSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     DioriteSlab, "minecraft:diorite_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DioriteSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     BrickWall, "minecraft:brick_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         BrickWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     PrismarineWall, "minecraft:prismarine_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PrismarineWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     RedSandstoneWall, "minecraft:red_sandstone_wall" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         RedSandstoneWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     MossyStoneBrickWall, "minecraft:mossy_stone_brick_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         MossyStoneBrickWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     GraniteWall, "minecraft:granite_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         GraniteWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     StoneBrickWall, "minecraft:stone_brick_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         StoneBrickWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     MudBrickWall, "minecraft:mud_brick_wall" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         MudBrickWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     NetherBrickWall, "minecraft:nether_brick_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         NetherBrickWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     AndesiteWall, "minecraft:andesite_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         AndesiteWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     RedNetherBrickWall, "minecraft:red_nether_brick_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         RedNetherBrickWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     SandstoneWall, "minecraft:sandstone_wall" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         SandstoneWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     EndStoneBrickWall, "minecraft:end_stone_brick_wall" => {
//         BlockProperties {
//             explosion_resistance: 9.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         EndStoneBrickWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     DioriteWall, "minecraft:diorite_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DioriteWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     Scaffolding, "minecraft:scaffolding" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         ScaffoldingProperties {
//             bottom: false,
//             distance, ConstrainedInt<1,7>: ConstrainedInt(0),
//             waterlogged: false
//         }
//     },
//     Loom, "minecraft:loom" => {
//         BlockProperties {
//             explosion_resistance: 2.5,
//             destroy_time: 2.5,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         LoomProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     Barrel, "minecraft:barrel" => {
//         BlockProperties {
//             explosion_resistance: 2.5,
//             destroy_time: 2.5,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BarrelProperties {
//             facing, Facing: Facing::North,
//             open: false
//         }
//     },
//     Smoker, "minecraft:smoker" => {
//         BlockProperties {
//             explosion_resistance: 3.5,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         SmokerProperties {
//             facing, Facing: Facing::North,
//             lit: false
//         }
//     },
//     BlastFurnace, "minecraft:blast_furnace" => {
//         BlockProperties {
//             explosion_resistance: 3.5,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         BlastFurnaceProperties {
//             facing, Facing: Facing::North,
//             lit: false
//         }
//     },
//     CartographyTable, "minecraft:cartography_table" => {
//         BlockProperties {
//             explosion_resistance: 2.5,
//             destroy_time: 2.5,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         }
//     },
//     FletchingTable, "minecraft:fletching_table" => {
//         BlockProperties {
//             explosion_resistance: 2.5,
//             destroy_time: 2.5,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         }
//     },
//     Grindstone, "minecraft:grindstone" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::HEAVY_METAL)
//         },
//         GrindstoneProperties {
//             face: AttachFace::Wall,
//             facing, Facing: Facing::North
//         }
//     },
//     Lectern, "minecraft:lectern" => {
//         BlockProperties {
//             explosion_resistance: 2.5,
//             destroy_time: 2.5,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         LecternProperties {
//             facing, Facing: Facing::North,
//             has_book: false,
//             powered: false
//         }
//     },
//     SmithingTable, "minecraft:smithing_table" => {
//         BlockProperties {
//             explosion_resistance: 2.5,
//             destroy_time: 2.5,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         }
//     },
//     Stonecutter, "minecraft:stonecutter" => {
//         BlockProperties {
//             explosion_resistance: 3.5,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         StonecutterProperties {
//             facing, Facing: Facing::North
//         }
//     },
//     Bell, "minecraft:bell" => {
//         BlockProperties {
//             explosion_resistance: 5.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         BellProperties {
//             attachment: BellAttach::Floor,
//             facing, Facing: Facing::North,
//             powered: false
//         }
//     },
//     Lantern, "minecraft:lantern" => {
//         BlockProperties {
//             explosion_resistance: 3.5,
//             destroy_time: 3.5,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         LanternProperties {
//             hanging: false,
//             waterlogged: false
//         }
//     },
//     SoulLantern, "minecraft:soul_lantern" => {
//         BlockProperties {
//             explosion_resistance: 3.5,
//             destroy_time: 3.5,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         SoulLanternProperties {
//             hanging: false,
//             waterlogged: false
//         }
//     },
//     Campfire, "minecraft:campfire" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         CampfireProperties {
//             facing, Facing: Facing::North,
//             lit: true,
//             signal_fire: false,
//             waterlogged: false
//         }
//     },
//     SoulCampfire, "minecraft:soul_campfire" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         SoulCampfireProperties {
//             facing, Facing: Facing::North,
//             lit: true,
//             signal_fire: false,
//             waterlogged: false
//         }
//     },
//     SweetBerryBush, "minecraft:sweet_berry_bush" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         SweetBerryBushProperties {
//             age, ConstrainedInt<0, 3>: ConstrainedInt(0)
//         }
//     },
//     WarpedStem, "minecraft:warped_stem" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         WarpedStemProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedWarpedStem, "minecraft:stripped_warped_stem" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         StrippedWarpedStemProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     WarpedHyphae, "minecraft:warped_hyphae" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         WarpedHyphaeProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedWarpedHyphae, "minecraft:stripped_warped_hyphae" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         StrippedWarpedHyphaeProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     WarpedNylium, "minecraft:warped_nylium" => {
//         BlockProperties {
//             explosion_resistance: 0.4,
//             destroy_time: 0.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     WarpedFungus, "minecraft:warped_fungus" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     WarpedWartBlock, "minecraft:warped_wart_block" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::GRASS)
//         }
//     },
//     WarpedRoots, "minecraft:warped_roots" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_FIREPROOF_PLANT)
//         }
//     },
//     NetherSprouts, "minecraft:nether_sprouts" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_FIREPROOF_PLANT)
//         }
//     },
//     CrimsonStem, "minecraft:crimson_stem" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         CrimsonStemProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedCrimsonStem, "minecraft:stripped_crimson_stem" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         StrippedCrimsonStemProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     CrimsonHyphae, "minecraft:crimson_hyphae" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         CrimsonHyphaeProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     StrippedCrimsonHyphae, "minecraft:stripped_crimson_hyphae" => {
//         BlockProperties {
//             explosion_resistance: 2.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         StrippedCrimsonHyphaeProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     CrimsonNylium, "minecraft:crimson_nylium" => {
//         BlockProperties {
//             explosion_resistance: 0.4,
//             destroy_time: 0.4,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     CrimsonFungus, "minecraft:crimson_fungus" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     Shroomlight, "minecraft:shroomlight" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             ..BlockProperties::new(crate::materials::Materials::GRASS)
//         }
//     },
//     WeepingVines, "minecraft:weeping_vines" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         WeepingVinesProperties {
//             age, ConstrainedInt<0, 25>: ConstrainedInt(0)
//         }
//     },
//     WeepingVinesPlant, "minecraft:weeping_vines_plant" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     TwistingVines, "minecraft:twisting_vines" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         TwistingVinesProperties {
//             age, ConstrainedInt<0, 25>: ConstrainedInt(0)
//         }
//     },
//     TwistingVinesPlant, "minecraft:twisting_vines_plant" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     CrimsonRoots, "minecraft:crimson_roots" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_FIREPROOF_PLANT)
//         }
//     },
//     CrimsonPlanks, "minecraft:crimson_planks" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         }
//     },
//     WarpedPlanks, "minecraft:warped_planks" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         }
//     },
//     CrimsonSlab, "minecraft:crimson_slab" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         CrimsonSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     WarpedSlab, "minecraft:warped_slab" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         WarpedSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     CrimsonPressurePlate, "minecraft:crimson_pressure_plate" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         CrimsonPressurePlateProperties {
//             powered: false
//         }
//     },
//     WarpedPressurePlate, "minecraft:warped_pressure_plate" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         WarpedPressurePlateProperties {
//             powered: false
//         }
//     },
//     CrimsonFence, "minecraft:crimson_fence" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         CrimsonFenceProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     WarpedFence, "minecraft:warped_fence" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         WarpedFenceProperties {
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     CrimsonTrapdoor, "minecraft:crimson_trapdoor" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         CrimsonTrapDoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             open: false,
//             powered: false,
//             waterlogged: false
//         }
//     },
//     WarpedTrapdoor, "minecraft:warped_trapdoor" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         WarpedTrapDoorProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             open: false,
//             powered: false,
//             waterlogged: false
//         }
//     },
//     CrimsonFenceGate, "minecraft:crimson_fence_gate" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         CrimsonFenceGateProperties {
//             facing, Facing: Facing::North,
//             in_wall: false,
//             open: false,
//             powered: false
//         }
//     },
//     WarpedFenceGate, "minecraft:warped_fence_gate" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         WarpedFenceGateProperties {
//             facing, Facing: Facing::North,
//             in_wall: false,
//             open: false,
//             powered: false
//         }
//     },
//     CrimsonStairs, "minecraft:crimson_stairs" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         CrimsonStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     WarpedStairs, "minecraft:warped_stairs" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 2.0,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         WarpedStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     CrimsonButton, "minecraft:crimson_button" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         CrimsonButtonProperties {
//             face: AttachFace::Wall,
//             facing, Facing: Facing::North,
//             powered: false
//         }
//     },
//     WarpedButton, "minecraft:warped_button" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         WarpedButtonProperties {
//             face: AttachFace::Wall,
//             facing, Facing: Facing::North,
//             powered: false
//         }
//     },
//     CrimsonDoor, "minecraft:crimson_door" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         CrimsonDoorProperties {
//             facing, Facing: Facing::North,
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower,
//             hinge: DoorHinge::Left,
//             open: false,
//             powered: false
//         }
//     },
//     WarpedDoor, "minecraft:warped_door" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         WarpedDoorProperties {
//             facing, Facing: Facing::North,
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower,
//             hinge: DoorHinge::Left,
//             open: false,
//             powered: false
//         }
//     },
//     CrimsonSign, "minecraft:crimson_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         CrimsonSignProperties {
//             rotation: ConstrainedInt(0),
//             waterlogged: false
//         }
//     },
//     WarpedSign, "minecraft:warped_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         WarpedSignProperties {
//             rotation: ConstrainedInt(0),
//             waterlogged: false
//         }
//     },
//     CrimsonWallSign, "minecraft:crimson_wall_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         CrimsonWallSignProperties {
//             facing, Facing: Facing::North,
//             waterlogged: false
//         }
//     },
//     WarpedWallSign, "minecraft:warped_wall_sign" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 1.0,
//             destroy_time: 1.0,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::NETHER_WOOD)
//         },
//         WarpedWallSignProperties {
//             facing, Facing: Facing::North,
//             waterlogged: false
//         }
//     },
//     StructureBlock, "minecraft:structure_block" => {
//         BlockProperties {
//             explosion_resistance: 3600000.0,
//             destroy_time: -1.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         StructureBlockProperties {
//             mode, StructureBlockMode: StructureBlockMode::Load
//         }
//     },
//     Jigsaw, "minecraft:jigsaw" => {
//         BlockProperties {
//             explosion_resistance: 3600000.0,
//             destroy_time: -1.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         JigsawProperties {
//             orientation: Orientation::NorthUp
//         }
//     },
//     Composter, "minecraft:composter" => {
//         BlockProperties {
//             explosion_resistance: 0.6,
//             destroy_time: 0.6,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         ComposterProperties {
//             level, ConstrainedInt<0, 8>: ConstrainedInt(0)
//         }
//     },
//     Target, "minecraft:target" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::GRASS)
//         },
//         TargetProperties {
//             power: ConstrainedInt(0)
//         }
//     },
//     BeeNest, "minecraft:bee_nest" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BeeNestProperties {
//             facing, Facing: Facing::North,
//             honey_level: ConstrainedInt(0)
//         }
//     },
//     Beehive, "minecraft:beehive" => {
//         BlockProperties {
//             explosion_resistance: 0.6,
//             destroy_time: 0.6,
//             ..BlockProperties::new(crate::materials::Materials::WOOD)
//         },
//         BeehiveProperties {
//             facing, Facing: Facing::North,
//             honey_level: ConstrainedInt(0)
//         }
//     },
//     HoneyBlock, "minecraft:honey_block" => {
//         BlockProperties {
//             speed_factor: 0.4,
//             jump_factor: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::CLAY)
//         }
//     },
//     HoneycombBlock, "minecraft:honeycomb_block" => {
//         BlockProperties {
//             explosion_resistance: 0.6,
//             destroy_time: 0.6,
//             ..BlockProperties::new(crate::materials::Materials::CLAY)
//         }
//     },
//     NetheriteBlock, "minecraft:netherite_block" => {
//         BlockProperties {
//             explosion_resistance: 1200.0,
//             destroy_time: 50.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     AncientDebris, "minecraft:ancient_debris" => {
//         BlockProperties {
//             explosion_resistance: 1200.0,
//             destroy_time: 30.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     CryingObsidian, "minecraft:crying_obsidian" => {
//         BlockProperties {
//             explosion_resistance: 1200.0,
//             destroy_time: 50.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     RespawnAnchor, "minecraft:respawn_anchor" => {
//         BlockProperties {
//             explosion_resistance: 1200.0,
//             destroy_time: 50.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         RespawnAnchorProperties {
//             charges: ConstrainedInt(0)
//         }
//     },
//     PottedCrimsonFungus, "minecraft:potted_crimson_fungus" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedWarpedFungus, "minecraft:potted_warped_fungus" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedCrimsonRoots, "minecraft:potted_crimson_roots" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedWarpedRoots, "minecraft:potted_warped_roots" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     Lodestone, "minecraft:lodestone" => {
//         BlockProperties {
//             explosion_resistance: 3.5,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::HEAVY_METAL)
//         }
//     },
//     Blackstone, "minecraft:blackstone" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     BlackstoneStairs, "minecraft:blackstone_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         BlackstoneStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     BlackstoneWall, "minecraft:blackstone_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         BlackstoneWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     BlackstoneSlab, "minecraft:blackstone_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         BlackstoneSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     PolishedBlackstone, "minecraft:polished_blackstone" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PolishedBlackstoneBricks, "minecraft:polished_blackstone_bricks" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     CrackedPolishedBlackstoneBricks, "minecraft:cracked_polished_blackstone_bricks" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     ChiseledPolishedBlackstone, "minecraft:chiseled_polished_blackstone" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PolishedBlackstoneBrickSlab, "minecraft:polished_blackstone_brick_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedBlackstoneBrickSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     PolishedBlackstoneBrickStairs, "minecraft:polished_blackstone_brick_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedBlackstoneBrickStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     PolishedBlackstoneBrickWall, "minecraft:polished_blackstone_brick_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedBlackstoneBrickWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     GildedBlackstone, "minecraft:gilded_blackstone" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PolishedBlackstoneStairs, "minecraft:polished_blackstone_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedBlackstoneStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     PolishedBlackstoneSlab, "minecraft:polished_blackstone_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedBlackstoneSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     PolishedBlackstonePressurePlate, "minecraft:polished_blackstone_pressure_plate" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedBlackstonePressurePlateProperties {
//             powered: false
//         }
//     },
//     PolishedBlackstoneButton, "minecraft:polished_blackstone_button" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         PolishedBlackstoneButtonProperties {
//             face: AttachFace::Wall,
//             facing, Facing: Facing::North,
//             powered: false
//         }
//     },
//     PolishedBlackstoneWall, "minecraft:polished_blackstone_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedBlackstoneWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     ChiseledNetherBricks, "minecraft:chiseled_nether_bricks" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     CrackedNetherBricks, "minecraft:cracked_nether_bricks" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 2.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     QuartzBricks, "minecraft:quartz_bricks" => {
//         BlockProperties {
//             explosion_resistance: 0.8,
//             destroy_time: 0.8,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     Candle, "minecraft:candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         CandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     WhiteCandle, "minecraft:white_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         WhiteCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     OrangeCandle, "minecraft:orange_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         OrangeCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     MagentaCandle, "minecraft:magenta_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         MagentaCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     LightBlueCandle, "minecraft:light_blue_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         LightBlueCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     YellowCandle, "minecraft:yellow_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         YellowCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     LimeCandle, "minecraft:lime_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         LimeCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     PinkCandle, "minecraft:pink_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         PinkCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     GrayCandle, "minecraft:gray_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         GrayCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     LightGrayCandle, "minecraft:light_gray_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         LightGrayCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     CyanCandle, "minecraft:cyan_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         CyanCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     PurpleCandle, "minecraft:purple_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         PurpleCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     BlueCandle, "minecraft:blue_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         BlueCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     BrownCandle, "minecraft:brown_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         BrownCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     GreenCandle, "minecraft:green_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         GreenCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     RedCandle, "minecraft:red_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         RedCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     BlackCandle, "minecraft:black_candle" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         },
//         BlackCandleProperties {
//             candles: ConstrainedInt(1),
//             lit: false,
//             waterlogged: false
//         }
//     },
//     CandleCake, "minecraft:candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         CandleCakeProperties {
//             lit: false
//         }
//     },
//     WhiteCandleCake, "minecraft:white_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         WhiteCandleCakeProperties {
//             lit: false
//         }
//     },
//     OrangeCandleCake, "minecraft:orange_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         OrangeCandleCakeProperties {
//             lit: false
//         }
//     },
//     MagentaCandleCake, "minecraft:magenta_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         MagentaCandleCakeProperties {
//             lit: false
//         }
//     },
//     LightBlueCandleCake, "minecraft:light_blue_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         LightBlueCandleCakeProperties {
//             lit: false
//         }
//     },
//     YellowCandleCake, "minecraft:yellow_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         YellowCandleCakeProperties {
//             lit: false
//         }
//     },
//     LimeCandleCake, "minecraft:lime_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         LimeCandleCakeProperties {
//             lit: false
//         }
//     },
//     PinkCandleCake, "minecraft:pink_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         PinkCandleCakeProperties {
//             lit: false
//         }
//     },
//     GrayCandleCake, "minecraft:gray_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         GrayCandleCakeProperties {
//             lit: false
//         }
//     },
//     LightGrayCandleCake, "minecraft:light_gray_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         LightGrayCandleCakeProperties {
//             lit: false
//         }
//     },
//     CyanCandleCake, "minecraft:cyan_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         CyanCandleCakeProperties {
//             lit: false
//         }
//     },
//     PurpleCandleCake, "minecraft:purple_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         PurpleCandleCakeProperties {
//             lit: false
//         }
//     },
//     BlueCandleCake, "minecraft:blue_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         BlueCandleCakeProperties {
//             lit: false
//         }
//     },
//     BrownCandleCake, "minecraft:brown_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         BrownCandleCakeProperties {
//             lit: false
//         }
//     },
//     GreenCandleCake, "minecraft:green_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         GreenCandleCakeProperties {
//             lit: false
//         }
//     },
//     RedCandleCake, "minecraft:red_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         RedCandleCakeProperties {
//             lit: false
//         }
//     },
//     BlackCandleCake, "minecraft:black_candle_cake" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::CAKE)
//         },
//         BlackCandleCakeProperties {
//             lit: false
//         }
//     },
//     AmethystBlock, "minecraft:amethyst_block" => {
//         BlockProperties {
//             explosion_resistance: 1.5,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::AMETHYST)
//         }
//     },
//     BuddingAmethyst, "minecraft:budding_amethyst" => {
//         BlockProperties {
//             explosion_resistance: 1.5,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::AMETHYST)
//         }
//     },
//     AmethystCluster, "minecraft:amethyst_cluster" => {
//         BlockProperties {
//             explosion_resistance: 1.5,
//             destroy_time: 1.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::AMETHYST)
//         },
//         AmethystClusterProperties {
//             facing, Facing: Facing::Up,
//             waterlogged: false
//         }
//     },
//     LargeAmethystBud, "minecraft:large_amethyst_bud" => {
//         BlockProperties {
//             explosion_resistance: 1.5,
//             destroy_time: 1.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::AMETHYST)
//         },
//         LargeAmethystBudProperties {
//             facing, Facing: Facing::Up,
//             waterlogged: false
//         }
//     },
//     MediumAmethystBud, "minecraft:medium_amethyst_bud" => {
//         BlockProperties {
//             explosion_resistance: 1.5,
//             destroy_time: 1.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::AMETHYST)
//         },
//         MediumAmethystBudProperties {
//             facing, Facing: Facing::Up,
//             waterlogged: false
//         }
//     },
//     SmallAmethystBud, "minecraft:small_amethyst_bud" => {
//         BlockProperties {
//             explosion_resistance: 1.5,
//             destroy_time: 1.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::AMETHYST)
//         },
//         SmallAmethystBudProperties {
//             facing, Facing: Facing::Up,
//             waterlogged: false
//         }
//     },
//     Tuff, "minecraft:tuff" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     Calcite, "minecraft:calcite" => {
//         BlockProperties {
//             explosion_resistance: 0.75,
//             destroy_time: 0.75,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     TintedGlass, "minecraft:tinted_glass" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::GLASS)
//         }
//     },
//     PowderSnow, "minecraft:powder_snow" => {
//         BlockProperties {
//             explosion_resistance: 0.25,
//             destroy_time: 0.25,
//             ..BlockProperties::new(crate::materials::Materials::POWDER_SNOW)
//         }
//     },
//     SculkSensor, "minecraft:sculk_sensor" => {
//         BlockProperties {
//             explosion_resistance: 1.5,
//             destroy_time: 1.5,
//             ..BlockProperties::new(crate::materials::Materials::SCULK)
//         },
//         SculkSensorProperties {
//             power: ConstrainedInt(0),
//             sculk_sensor_phase: SculkSensorPhase::Inactive,
//             waterlogged: false
//         }
//     },
//     Sculk, "minecraft:sculk" => {
//         BlockProperties {
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             ..BlockProperties::new(crate::materials::Materials::SCULK)
//         }
//     },
//     SculkVein, "minecraft:sculk_vein" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.2,
//             destroy_time: 0.2,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::SCULK)
//         },
//         SculkVeinProperties {
//             down: false,
//             east, bool: false,
//             north, bool: false,
//             south, bool: false,
//             up: false,
//             waterlogged: false,
//             west, bool: false
//         }
//     },
//     SculkCatalyst, "minecraft:sculk_catalyst" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             ..BlockProperties::new(crate::materials::Materials::SCULK)
//         },
//         SculkCatalystProperties {
//             bloom: false
//         }
//     },
//     SculkShrieker, "minecraft:sculk_shrieker" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             ..BlockProperties::new(crate::materials::Materials::SCULK)
//         },
//         SculkShriekerProperties {
//             can_summon: false,
//             shrieking: false,
//             waterlogged: false
//         }
//     },
//     OxidizedCopper, "minecraft:oxidized_copper" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     WeatheredCopper, "minecraft:weathered_copper" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     ExposedCopper, "minecraft:exposed_copper" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     CopperBlock, "minecraft:copper_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     CopperOre, "minecraft:copper_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DeepslateCopperOre, "minecraft:deepslate_copper_ore" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 4.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     OxidizedCutCopper, "minecraft:oxidized_cut_copper" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     WeatheredCutCopper, "minecraft:weathered_cut_copper" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     ExposedCutCopper, "minecraft:exposed_cut_copper" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     CutCopper, "minecraft:cut_copper" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     OxidizedCutCopperStairs, "minecraft:oxidized_cut_copper_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         OxidizedCutCopperStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     WeatheredCutCopperStairs, "minecraft:weathered_cut_copper_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         WeatheredCutCopperStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     ExposedCutCopperStairs, "minecraft:exposed_cut_copper_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         ExposedCutCopperStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     CutCopperStairs, "minecraft:cut_copper_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         CutCopperStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     OxidizedCutCopperSlab, "minecraft:oxidized_cut_copper_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         OxidizedCutCopperSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     WeatheredCutCopperSlab, "minecraft:weathered_cut_copper_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         WeatheredCutCopperSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     ExposedCutCopperSlab, "minecraft:exposed_cut_copper_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         ExposedCutCopperSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     CutCopperSlab, "minecraft:cut_copper_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         CutCopperSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     WaxedCopperBlock, "minecraft:waxed_copper_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     WaxedWeatheredCopper, "minecraft:waxed_weathered_copper" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     WaxedExposedCopper, "minecraft:waxed_exposed_copper" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     WaxedOxidizedCopper, "minecraft:waxed_oxidized_copper" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     WaxedOxidizedCutCopper, "minecraft:waxed_oxidized_cut_copper" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     WaxedWeatheredCutCopper, "minecraft:waxed_weathered_cut_copper" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     WaxedExposedCutCopper, "minecraft:waxed_exposed_cut_copper" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     WaxedCutCopper, "minecraft:waxed_cut_copper" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         }
//     },
//     WaxedOxidizedCutCopperStairs, "minecraft:waxed_oxidized_cut_copper_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         WaxedOxidizedCutCopperStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     WaxedWeatheredCutCopperStairs, "minecraft:waxed_weathered_cut_copper_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         WaxedWeatheredCutCopperStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     WaxedExposedCutCopperStairs, "minecraft:waxed_exposed_cut_copper_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         WaxedExposedCutCopperStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     WaxedCutCopperStairs, "minecraft:waxed_cut_copper_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         WaxedCutCopperStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     WaxedOxidizedCutCopperSlab, "minecraft:waxed_oxidized_cut_copper_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         WaxedOxidizedCutCopperSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     WaxedWeatheredCutCopperSlab, "minecraft:waxed_weathered_cut_copper_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         WaxedWeatheredCutCopperSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     WaxedExposedCutCopperSlab, "minecraft:waxed_exposed_cut_copper_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         WaxedExposedCutCopperSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     WaxedCutCopperSlab, "minecraft:waxed_cut_copper_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         WaxedCutCopperSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     LightningRod, "minecraft:lightning_rod" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::METAL)
//         },
//         LightningRodProperties {
//             facing, Facing: Facing::Up,
//             powered: false,
//             waterlogged: false
//         }
//     },
//     PointedDripstone, "minecraft:pointed_dripstone" => {
//         BlockProperties {
//             explosion_resistance: 3.0,
//             destroy_time: 1.5,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PointedDripstoneProperties {
//             thickness: DripStoneThickness::Tip,
//             vertical_direction: VerticalDirection::Up,
//             waterlogged: false
//         }
//     },
//     DripstoneBlock, "minecraft:dripstone_block" => {
//         BlockProperties {
//             explosion_resistance: 1.0,
//             destroy_time: 1.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     CaveVines, "minecraft:cave_vines" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         CaveVinesProperties {
//             age, ConstrainedInt<0,25>: ConstrainedInt(0),
//             berries: false
//         }
//     },
//     CaveVinesPlant, "minecraft:cave_vines_plant" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         CaveVinesPlantProperties {
//             berries: false
//         }
//     },
//     SporeBlossom, "minecraft:spore_blossom" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     Azalea, "minecraft:azalea" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     FloweringAzalea, "minecraft:flowering_azalea" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     MossCarpet, "minecraft:moss_carpet" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         }
//     },
//     MossBlock, "minecraft:moss_block" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::MOSS)
//         }
//     },
//     BigDripleaf, "minecraft:big_dripleaf" => {
//         BlockProperties {
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         BigDripleafProperties {
//             facing, Facing: Facing::North,
//             tilt: Tilt::None,
//             waterlogged: false
//         }
//     },
//     BigDripleafStem, "minecraft:big_dripleaf_stem" => {
//         BlockProperties {
//             has_collision: false,
//             explosion_resistance: 0.1,
//             destroy_time: 0.1,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         BigDripleafStemProperties {
//             facing, Facing: Facing::North,
//             waterlogged: false
//         }
//     },
//     SmallDripleaf, "minecraft:small_dripleaf" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::PLANT)
//         },
//         SmallDripleafProperties {
//             facing, Facing: Facing::North,
//             half, DoubleBlockHalf: DoubleBlockHalf::Lower,
//             waterlogged: false
//         }
//     },
//     HangingRoots, "minecraft:hanging_roots" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::REPLACEABLE_PLANT)
//         },
//         HangingRootsProperties {
//             waterlogged: false
//         }
//     },
//     RootedDirt, "minecraft:rooted_dirt" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::DIRT)
//         }
//     },
//     Mud, "minecraft:mud" => {
//         BlockProperties {
//             explosion_resistance: 0.5,
//             destroy_time: 0.5,
//             ..BlockProperties::new(crate::materials::Materials::DIRT)
//         }
//     },
//     Deepslate, "minecraft:deepslate" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeepslateProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     CobbledDeepslate, "minecraft:cobbled_deepslate" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     CobbledDeepslateStairs, "minecraft:cobbled_deepslate_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         CobbledDeepslateStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     CobbledDeepslateSlab, "minecraft:cobbled_deepslate_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         CobbledDeepslateSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     CobbledDeepslateWall, "minecraft:cobbled_deepslate_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         CobbledDeepslateWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     PolishedDeepslate, "minecraft:polished_deepslate" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PolishedDeepslateStairs, "minecraft:polished_deepslate_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedDeepslateStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     PolishedDeepslateSlab, "minecraft:polished_deepslate_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedDeepslateSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     PolishedDeepslateWall, "minecraft:polished_deepslate_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         PolishedDeepslateWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     DeepslateTiles, "minecraft:deepslate_tiles" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DeepslateTileStairs, "minecraft:deepslate_tile_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeepslateTileStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     DeepslateTileSlab, "minecraft:deepslate_tile_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeepslateTileSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     DeepslateTileWall, "minecraft:deepslate_tile_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeepslateTileWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     DeepslateBricks, "minecraft:deepslate_bricks" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     DeepslateBrickStairs, "minecraft:deepslate_brick_stairs" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeepslateBrickStairsProperties {
//             facing, HorizontalFacing: HorizontalFacing::North,
//             half, Half: Half::Bottom,
//             shape, StairsShape: StairsShape::Straight,
//             waterlogged: false
//         }
//     },
//     DeepslateBrickSlab, "minecraft:deepslate_brick_slab" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeepslateBrickSlabProperties {
//             r#type, SlabType: SlabType::Bottom,
//             waterlogged: false
//         }
//     },
//     DeepslateBrickWall, "minecraft:deepslate_brick_wall" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         },
//         DeepslateBrickWallProperties {
//             east, WallSide: WallSide::None,
//             north, WallSide: WallSide::None,
//             south, WallSide: WallSide::None,
//             up: true,
//             waterlogged: false,
//             west, WallSide: WallSide::None
//         }
//     },
//     ChiseledDeepslate, "minecraft:chiseled_deepslate" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     CrackedDeepslateBricks, "minecraft:cracked_deepslate_bricks" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     CrackedDeepslateTiles, "minecraft:cracked_deepslate_tiles" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 3.5,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     InfestedDeepslate, "minecraft:infested_deepslate" => {
//         BlockProperties {
//             explosion_resistance: 0.75,
//             destroy_time: 1.5,
//             ..BlockProperties::new(crate::materials::Materials::CLAY)
//         },
//         InfestedDeepslateProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     SmoothBasalt, "minecraft:smooth_basalt" => {
//         BlockProperties {
//             explosion_resistance: 4.2,
//             destroy_time: 1.25,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     RawIronBlock, "minecraft:raw_iron_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     RawCopperBlock, "minecraft:raw_copper_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     RawGoldBlock, "minecraft:raw_gold_block" => {
//         BlockProperties {
//             explosion_resistance: 6.0,
//             destroy_time: 5.0,
//             requires_tool: true,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     },
//     PottedAzaleaBush, "minecraft:potted_azalea_bush" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     PottedFloweringAzaleaBush, "minecraft:potted_flowering_azalea_bush" => {
//         BlockProperties {
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::DECORATION)
//         }
//     },
//     OchreFroglight, "minecraft:ochre_froglight" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             ..BlockProperties::new(crate::materials::Materials::FROGLIGHT)
//         },
//         OchreFroglightProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     VerdantFroglight, "minecraft:verdant_froglight" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             ..BlockProperties::new(crate::materials::Materials::FROGLIGHT)
//         },
//         VerdantFroglightProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     PearlescentFroglight, "minecraft:pearlescent_froglight" => {
//         BlockProperties {
//             explosion_resistance: 0.3,
//             destroy_time: 0.3,
//             ..BlockProperties::new(crate::materials::Materials::FROGLIGHT)
//         },
//         PearlescentFroglightProperties {
//             axis, Axis: Axis::Y
//         }
//     },
//     Frogspawn, "minecraft:frogspawn" => {
//         BlockProperties {
//             has_collision: false,
//             can_occlude: false,
//             ..BlockProperties::new(crate::materials::Materials::FROGSPAWN)
//         }
//     },
//     ReinforcedDeepslate, "minecraft:reinforced_deepslate" => {
//         BlockProperties {
//             explosion_resistance: 1200.0,
//             destroy_time: 55.0,
//             ..BlockProperties::new(crate::materials::Materials::STONE)
//         }
//     }
// );
