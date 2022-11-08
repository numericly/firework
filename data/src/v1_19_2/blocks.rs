macro_rules! define_visitor {
    (
        $($prop_id:literal => $field_name:ident),*
        $(,)?
    ) => {
        use serde::Deserializer;
        #[allow(non_camel_case_types)]
        pub enum __Field {
            $($field_name),*,
            __Ignored
        }
        struct __FieldVisitor;

        impl<'de> serde::de::Visitor<'de> for __FieldVisitor {
            type Value = __Field;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_str<__E>(
                self,
                __value: &str,
            ) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    $($prop_id => Ok(__Field::$field_name)),*,
                    _ => Ok(__Field::__Ignored),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_identifier(__FieldVisitor)
            }
        }

    };
}

macro_rules! define_props {
    (
        $($prop_name:ident, $real_name:ident, $field_literal:literal => $prop_type:ty),*
        $(,)?
    ) => {
        $(
            #[allow(non_camel_case_types)]
            pub type $prop_name = $prop_type;
        )*

        #[macro_export]
        macro_rules! define_block_state {
            ($state_name:ident {
                $$($prop:ident => $default:expr),*
                $$(,)?
            }) => {
                __replace_names! {
                    (
                        $state_name;
                        $$($prop: $default),*;
                    )
                    $$($prop),*
                }
            }
        }


        #[doc(hidden)]
        #[macro_export]
        macro_rules! __literal_map {
            $(($real_name) => {$field_literal});*
        }

        #[doc(hidden)]
        #[macro_export]
        macro_rules! __replace_names {
            (
                $packaged:tt
                $$(
                    $($$($prop_name $$(@$$$prop_name:tt@)?)?)*
                ),*
            ) => {
                __declare_struct!{
                    $packaged
                    $$(
                        $(
                            $$($real_name $$(@$$$prop_name@)?)?
                        )*
                    ),*
                }
            }
        }


        #[doc(hidden)]
        #[macro_export]
        macro_rules! __declare_struct {(
                (
                    $struct_name:ident;
                    $$($field_ty:ty: $field_default:expr),*;
                )
                $$($field_name:ident),*
            ) => {
                #[derive(Debug, Hash, PartialEq, Eq)]
                pub struct $struct_name {
                    $$(
                        pub $field_name: $field_ty,
                    )*
                }

                impl prop_types::PossibleValues for $struct_name {
                    type ValueIterator = std::vec::IntoIter<$struct_name>;
                    fn possible_values() -> Self::ValueIterator {
                        use loop_chain::loop_chain;

                        let mut states = std::vec::Vec::new();

                        loop_chain! {
                            $$(for $field_name in <$field_ty>::possible_values());*;
                            then {
                                states.push($struct_name {
                                    $$($field_name: $field_name.clone()),*
                                });
                            }
                        }

                        return states.into_iter();
                    }
                }

                impl<'de> Deserialize<'de> for $struct_name {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> serde::__private::Result<Self, __D::Error>
                    where
                        __D: serde::Deserializer<'de>,
                    {
                        struct __Visitor;

                        impl<'de> serde::de::Visitor<'de> for __Visitor {
                            type Value = $struct_name;
                            fn expecting(
                                &self,
                                f: &mut serde::__private::Formatter,
                            ) -> serde::__private::fmt::Result {
                                f.write_str("")
                            }
                            fn visit_map<A>(
                                self,
                                mut m: A,
                            ) -> serde::__private::Result<Self::Value, A::Error>
                            where
                                A: serde::de::MapAccess<'de>,
                            {
                                let mut v = $struct_name::default();

                                while let serde::__private::Some(k) =
                                    serde::de::MapAccess::next_key::<prop_types::__Field>(&mut m).unwrap()
                                {
                                    match k {
                                        $$(
                                            prop_types::__Field::$field_name => {
                                                v.$field_name = serde::de::MapAccess::next_value::<String>(
                                                    &mut m,
                                                ).unwrap().parse().unwrap();
                                            }
                                        ),*
                                        _ => {}
                                    }
                                }
                                Ok(v)
                            }
                        }

                        const FIELDS: &'static [&'static str] = &[$$(__literal_map! ($field_name)),*];
                        __deserializer.deserialize_struct(
                            stringify!($struct_name),
                            FIELDS,
                            __Visitor
                        )
                    }
                }

                impl Default for $struct_name {
                    fn default() -> Self {
                        Self {
                            $$(
                                $field_name: $field_default,
                            )*
                        }
                    }
                }
            }
        }

        pub use define_block_state;
        pub use __replace_names;
        pub use __declare_struct;
        pub use __literal_map;
    }
}

macro_rules! enum_property {
    ($name:ident { $($variant:ident => $val:literal),* $(,)? }) => {

        #[derive(Debug, Deserialize, /*Serialize, */Hash, PartialEq, Eq, Clone)]
        pub enum $name {
            $($variant),*
        }

        impl PossibleValues for $name {
            type ValueIterator = vec::IntoIter<Self>;
            fn possible_values() -> Self::ValueIterator {
                let values = vec![$(Self::$variant),*];
                return values.into_iter()
            }
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

macro_rules! define_block_states {
    ($($name: ident, $block_id: literal $(=> $state_name:ident {$($prop: ident => $default: expr),*})?),*) => {

        use prop_types::*;
        use serde::{Deserialize/*, Serialize*/};
        use std::hash::Hash;
        use std::collections::HashMap;
        use lazy_static::lazy_static;

        #[derive(Debug, Deserialize, /*Serialize,*/ Hash, PartialEq, Eq)]
        #[serde(tag = "Name", content = "Properties")]
        pub enum BlockStates {
            $(
                #[serde(rename = $block_id)]
                $name$(($state_name))?
            ),*
        }

        #[doc(hidden)]
        macro_rules! __insert_optional_state {
            ($pal:ident, $block_name:ident) => {
                $pal.insert(BlockStates::$block_name, $pal.len() as i32)
            };
            ($pal:ident, $block_name:ident, $_state_name:ident) => {
                for value in $_state_name::possible_values() {
                    $pal.insert(BlockStates::$block_name(value), $pal.len() as i32);
                }
            }
        }

        lazy_static! {
            pub static ref BLOCK_PALETTE: HashMap<BlockStates, i32> = {
                let mut palette: HashMap<BlockStates, i32> = HashMap::new();
                $(
                    __insert_optional_state! (palette, $name$(, $state_name)?);
                )*
                palette
            };
        }

        impl super::super::Palette for BlockStates {
            fn get_palette(state: &BlockStates) -> i32 {
                BLOCK_PALETTE.get(state).unwrap().clone()
            }
        }

        $(
            $(
                define_block_state! {
                    $state_name {
                        $($prop  => $default),*
                    }
                }
            )?
        )*
    };
}

pub mod state {
    pub mod prop_types {
        use serde::{Deserialize, Serialize};
        use std::{array, str::FromStr, vec};

        pub trait PossibleValues {
            type ValueIterator: Iterator;
            fn possible_values() -> Self::ValueIterator
            where
                Self: Sized;
        }

        impl PossibleValues for bool {
            type ValueIterator = array::IntoIter<bool, 2>;
            fn possible_values() -> Self::ValueIterator {
                const VALUES: [bool; 2] = [true, false];
                return VALUES.into_iter();
            }
        }

        #[derive(Deserialize, Serialize, Debug, Hash, PartialEq, Eq, Clone)]
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

        impl<const MIN: u32, const MAX: u32> ToString for ConstrainedInt<MIN, MAX> {
            fn to_string(&self) -> std::string::String {
                self.0.to_string()
            }
        }

        impl<const MIN: u32, const MAX: u32> PossibleValues for ConstrainedInt<MIN, MAX> {
            type ValueIterator = vec::IntoIter<ConstrainedInt<MIN, MAX>>;
            fn possible_values() -> Self::ValueIterator {
                let values: Vec<ConstrainedInt<MIN, MAX>> = (MIN..=MAX)
                    .into_iter()
                    .map(|v| ConstrainedInt::<MIN, MAX>(v))
                    .collect();
                values.into_iter()
            }
        }

        enum_property! { HorizontalAxis { X => "x", Z => "z" } }
        enum_property! { Axis { X => "x", Y => "y", Z => "z" } }
        enum_property! { Facing {
            North => "north",
            East => "east",
            South => "south",
            West => "west",
            Up => "up",
            Down => "down"
        } }
        enum_property! { FacingHopper {
            Down => "down",
            North => "north",
            South => "south",
            West => "west",
            East => "east"
        }}
        enum_property! { HorizontalFacing { North => "north", South => "south", West => "west", East => "east" } }
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
            EastUp => "east_up",
            NorthUp => "north_up",
            SouthUp => "south_up",
        } }
        enum_property! { AttachFace { Floor => "floor", Wall => "wall", Ceiling => "ceiling" } }
        enum_property! { BellAttachment { Floor => "floor", Ceiling => "ceiling", SingleWall => "single_wall", DoubleWall => "double_wall" } }
        enum_property! { WallSide { None => "none", Low => "low", Tall => "tall" } }
        enum_property! { RedstoneSide { Up => "up", Side => "side", None => "none" } }
        enum_property! { DoubleBlockHalf { Upper => "upper", Lower => "lower" } }
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
        enum_property! { RailShapeStraight {
            NorthSouth => "north_south",
            EastWest => "east_west",
            AscendingEast => "ascending_east",
            AscendingWest => "ascending_west",
            AscendingNorth => "ascending_north",
            AscendingSouth => "ascending_south"
        }}
        enum_property! { BedPart { Head => "head", Foot => "foot" } }
        enum_property! { ChestType { Single => "single", Left => "left", Right => "right" } }
        enum_property! { ModeComparator { Compare => "compare", Subtract => "subtract" } }
        enum_property! { DoorHinge { Left => "left", Right => "right" } }
        enum_property! { NoteblockInstrument {
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
        enum_property! { StructureblockMode { Save => "save", Load => "load", Corner => "corner", Data => "data" } }
        enum_property! { BambooLeaves { None => "none", Small => "small", Large => "large" } }
        enum_property! { Tilt { None => "none", Unstable => "unstable", Partial => "partial", Full => "full" } }
        enum_property! { VerticalDirection { Up => "up", Down => "down" } }
        enum_property! { DripstoneThickness {
            TipMerge => "tip_merge",
            Tip => "tip",
            Frustum => "frustum",
            Middle => "middle",
            Base => "base"
        }}
        enum_property! { SculkSensorPhase { Inactive => "inactive", Active => "active", CoolDown => "cooldown" } }

        define_visitor! {
            "attached" => attached,
            "bottom" => bottom,
            "conditional" => conditional,
            "disarmed" => disarmed,
            "drag" => drag,
            "enabled" => enabled,
            "extended" => extended,
            "eye" => eye,
            "falling" => falling,
            "hanging" => hanging,
            "has_bottle_0" => has_bottle_0,
            "has_bottle_1" => has_bottle_1,
            "has_bottle_2" => has_bottle_2,
            "has_record" => has_record,
            "has_book" => has_book,
            "inverted" => inverted,
            "in_wall" => in_wall,
            "lit" => lit,
            "locked" => locked,
            "occupied" => occupied,
            "open" => open,
            "persistent" => persistent,
            "powered" => powered,
            "short" => short,
            "signal_fire" => signal_fire,
            "snowy" => snowy,
            "triggered" => triggered,
            "unstable" => unstable,
            "waterlogged" => waterlogged,
            "vine_end" => vine_end,
            "berries" => berries,
            "bloom" => bloom,
            "shrieking" => shrieking,
            "can_summon" => can_summon,
            "axis" => axis,
            "bites" => bites,
            "delay" => delay,
            "distance" => distance,
            "eggs" => eggs,
            "hatch" => hatch,
            "layers" => layers,
            "level" => level,
            "honey_level" => honey_level,
            "moisture" => moisture,
            "up" => up,
            "down" => down,
            "north" => north,
            "east" => east,
            "south" => south,
            "west" => west,
            "facing" => facing,
            "face" => face,
            "orientation" => orientation,
            "attachment" => attachment,
            "half" => half,
            "shape" => shape,
            "age" => age,
            "candles" => candles,
            "pickles" => pickles,
            "note" => note,
            "power" => power,
            "stage" => stage,
            "charges" => charges,
            "rotation" => rotation,
            "part" => part,
            "type" => r#type,
            "mode" => mode,
            "hinge" => hinge,
            "instrument" => instrument,
            "leaves" => leaves,
            "tilt" => tilt,
            "vertical_direction" => vertical_direction,
            "thickness" => thickness,
            "sculk_sensor_phase" => sculk_sensor_phase,
        }
        define_props! {
            attached, attached, "attached" => bool,
            bottom, bottom, "bottom" => bool,
            conditional, conditional, "conditional" => bool,
            disarmed, disarmed, "disarmed" => bool,
            drag, drag, "drag" => bool,
            enabled, enabled, "enabled" => bool,
            extended, extended, "extended" => bool,
            eye, eye, "eye" => bool,
            falling, falling, "falling" => bool,
            hanging, hanging, "hanging" => bool,
            has_bottle_0, has_bottle_0, "has_bottle_0" => bool,
            has_bottle_1, has_bottle_1, "has_bottle_1" => bool,
            has_bottle_2, has_bottle_2, "has_bottle_2" => bool,
            has_record, has_record, "has_record" => bool,
            has_book, has_book, "has_book" => bool,
            inverted, inverted, "inverted" => bool,
            in_wall, in_wall, "in_wall" => bool,
            lit, lit, "lit" => bool,
            locked, locked, "locked" => bool,
            occupied, occupied, "occupied" => bool,
            open, open, "open" => bool,
            persistent, persistent, "persistent" => bool,
            powered, powered, "powered" => bool,
            short, short, "short" => bool,
            signal_fire, signal_fire, "signal_fire" => bool,
            snowy, snowy, "snowy" => bool,
            triggered, triggered, "triggered" => bool,
            unstable, unstable, "unstable" => bool,
            waterlogged, waterlogged, "waterlogged" => bool,
            vine_end, vine_end, "vine_end" => bool,
            berries, berries, "berries" => bool,
            bloom, bloom, "bloom" => bool,
            shrieking, shrieking, "shrieking" => bool,
            can_summon, can_summon, "can_summon" => bool,
            horizontal_axis, axis, "axis" => HorizontalAxis,
            axis, axis, "axis" => Axis,
            up, up, "up" => bool,
            down, down, "down" => bool,
            north, north, "north" => bool,
            east, east, "east" => bool,
            south, south, "south" => bool,
            west, west, "west" => bool,
            facing, facing, "facing" => Facing,
            facing_hopper, facing, "facing" => FacingHopper,
            horizontal_facing, facing, "facing" => HorizontalFacing,
            orientation, orientation, "orientation" => Orientation,
            attach_face, face, "face" => AttachFace,
            bell_attachment, attachment, "attachment" => BellAttachment,
            east_wall, east, "east" => WallSide,
            north_wall, north, "north" => WallSide,
            south_wall, south, "south" => WallSide,
            west_wall, west, "west" => WallSide,
            east_redstone, east, "east" => RedstoneSide,
            north_redstone, north, "north" => RedstoneSide,
            south_redstone, south, "south"=> RedstoneSide,
            west_redstone, west, "west" => RedstoneSide,
            double_block_half, half, "half" => DoubleBlockHalf,
            half, half, "half" => Half,
            rail_shape, shape, "shape" => RailShape,
            rail_shape_straight, shape, "shape" => RailShapeStraight,
            age_1, age, "age" => ConstrainedInt<0, 1>,
            age_2, age, "age" => ConstrainedInt<0, 2>,
            age_3, age, "age" => ConstrainedInt<0, 3>,
            age_4, age, "age" => ConstrainedInt<0, 4>,
            age_5, age, "age" => ConstrainedInt<0, 5>,
            age_7, age, "age" => ConstrainedInt<0, 7>,
            age_15, age, "age" => ConstrainedInt<0, 15>,
            age_25, age, "age" => ConstrainedInt<0, 25>,
            bites, bites, "bites" => ConstrainedInt<0, 6>,
            candles, candles, "candles" => ConstrainedInt<1, 4>,
            delay, delay, "delay" => ConstrainedInt<1, 4>,
            distance, distance, "distance" => ConstrainedInt<1, 7>,
            eggs, eggs, "eggs" => ConstrainedInt<1, 4>,
            hatch, hatch, "hatch" => ConstrainedInt<0, 2>,
            layers, layers, "layers" => ConstrainedInt<1, 8>,
            level_cauldron, level, "level" => ConstrainedInt<1, 3>,
            level_composter, level, "level" => ConstrainedInt<0, 8>,
            level_flowing, level, "level" => ConstrainedInt<1, 8>,
            level_honey, honey_level, "honey_level" => ConstrainedInt<0, 5>,
            level, level, "level" => ConstrainedInt<0, 15>,
            moisture, moisture, "moisture" => ConstrainedInt<0, 7>,
            pickles, pickles, "pickles" => ConstrainedInt<1, 4>,
            note, note, "note" => ConstrainedInt<0, 24>,
            power, power, "power" => ConstrainedInt<0, 15>,
            stage, stage, "stage" => ConstrainedInt<0, 1>,
            stability_distance, distance, "distance" => ConstrainedInt<0, 7>,
            respawn_anchor_charges, charges, "charges" => ConstrainedInt<0, 4>,
            rotation_16, rotation, "rotation" => ConstrainedInt<0, 15>,
            bed_part, part, "part" => BedPart,
            chest_type, r#type, "type" => ChestType,
            mode_comparator, mode, "mode" => ModeComparator,
            door_hinge, hinge, "hinge" => DoorHinge,
            noteblock_instrument, instrument, "instrument" => NoteblockInstrument,
            piston_type, r#type, "type" => PistonType,
            slab_type, r#type, "type" => SlabType,
            stairs_shape, shape, "shape" => StairsShape,
            structureblock_mode, mode, "mode" => StructureblockMode,
            bamboo_leaves, leaves, "leaves" => BambooLeaves,
            tilt, tilt, "tilt" => Tilt,
            vertical_direction, vertical_direction, "vertical_direction" => VerticalDirection,
            dripstone_thickness, thickness, "thickness" => DripstoneThickness,
            sculk_sensor_phase, sculk_sensor_phase, "sculk_sensor_phase" => SculkSensorPhase
        }
    }
    define_block_states! {
        Air, "minecraft:air",
        Stone, "minecraft:stone",
        Granite, "minecraft:granite",
        PolishedGranite, "minecraft:polished_granite",
        Diorite, "minecraft:diorite",
        PolishedDiorite, "minecraft:polished_diorite",
        Andesite, "minecraft:andesite",
        PolishedAndesite, "minecraft:polished_andesite",
        GrassBlock, "minecraft:grass_block" => GrassBlockState { snowy => false },
        Dirt, "minecraft:dirt",
        CoarseDirt, "minecraft:coarse_dirt",
        Podzol, "minecraft:podzol" => PodzolState { snowy => false },
        Cobblestone, "minecraft:cobblestone",
        OakPlanks, "minecraft:oak_planks",
        SprucePlanks, "minecraft:spruce_planks",
        BirchPlanks, "minecraft:birch_planks",
        JunglePlanks, "minecraft:jungle_planks",
        AcaciaPlanks, "minecraft:acacia_planks",
        DarkOakPlanks, "minecraft:dark_oak_planks",
        MangrovePlanks, "minecraft:mangrove_planks",
        OakSapling, "minecraft:oak_sapling" => OakSaplingState { stage => prop_types::ConstrainedInt(0) },
        SpruceSapling, "minecraft:spruce_sapling" => SpruceSaplingState { stage => prop_types::ConstrainedInt(0) },
        BirchSapling, "minecraft:birch_sapling" => BirchSaplingState { stage => prop_types::ConstrainedInt(0) },
        JungleSapling, "minecraft:jungle_sapling" => JungleSaplingState { stage => prop_types::ConstrainedInt(0) },
        AcaciaSapling, "minecraft:acacia_sapling" => AcaciaSaplingState { stage => prop_types::ConstrainedInt(0) },
        DarkOakSapling, "minecraft:dark_oak_sapling" => DarkOakSaplingState { stage => prop_types::ConstrainedInt(0) },
        MangrovePropagule, "minecraft:mangrove_propagule" => MangrovePropaguleState {
            age_4 => prop_types::ConstrainedInt(0),
            hanging => false,
            stage => prop_types::ConstrainedInt(0),
            waterlogged => false
        },
        Bedrock, "minecraft:bedrock",
        Water, "minecraft:water" => WaterState { level => prop_types::ConstrainedInt(0) },
        Lava, "minecraft:lava" => LavaState { level => prop_types::ConstrainedInt(0) },
        Sand, "minecraft:sand",
        RedSand, "minecraft:red_sand",
        Gravel, "minecraft:gravel",
        GoldOre, "minecraft:gold_ore",
        DeepslateGoldOre, "minecraft:deepslate_gold_ore",
        IronOre, "minecraft:iron_ore",
        DeepslateIronOre, "minecraft:deepslate_iron_ore",
        CoalOre, "minecraft:coal_ore",
        DeepslateCoalOre, "minecraft:deepslate_coal_ore",
        NetherGoldOre, "minecraft:nether_gold_ore",
        OakLog, "minecraft:oak_log" => OakLogState { axis => prop_types::Axis::Y },
        SpruceLog, "minecraft:spruce_log" => SpruceLogState { axis => prop_types::Axis::Y },
        BirchLog, "minecraft:birch_log" => BirchLogState { axis => prop_types::Axis::Y },
        JungleLog, "minecraft:jungle_log" => JungleLogState { axis => prop_types::Axis::Y },
        AcaciaLog, "minecraft:acacia_log" => AcaciaLogState { axis => prop_types::Axis::Y },
        DarkOakLog, "minecraft:dark_oak_log" => DarkOakLogState { axis => prop_types::Axis::Y },
        MangroveLog, "minecraft:mangrove_log" => MangroveLogState { axis => prop_types::Axis::Y },
        MangroveRoots, "minecraft:mangrove_roots" => MangroveRootsState { waterlogged => false },
        MuddyMangroveRoots, "minecraft:muddy_mangrove_roots" => MuddyMangroveRootsState { axis => prop_types::Axis::Y },
        StrippedSpruceLog, "minecraft:stripped_spruce_log" => StrippedSpruceLogState { axis => prop_types::Axis::Y },
        StrippedBirchLog, "minecraft:stripped_birch_log" => StrippedBirchLogState { axis => prop_types::Axis::Y },
        StrippedJungleLog, "minecraft:stripped_jungle_log" => StrippedJungleLogState { axis => prop_types::Axis::Y },
        StrippedAcaciaLog, "minecraft:stripped_acacia_log" => StrippedAcaciaLogState { axis => prop_types::Axis::Y },
        StrippedDarkOakLog, "minecraft:stripped_dark_oak_log" => StrippedDarkOakLogState { axis => prop_types::Axis::Y },
        StrippedOakLog, "minecraft:stripped_oak_log" => StrippedOakLogState { axis => prop_types::Axis::Y },
        StrippedMangroveLog, "minecraft:stripped_mangrove_log" => StrippedMangroveLogState { axis => prop_types::Axis::Y },
        OakWood, "minecraft:oak_wood" => OakWoodState { axis => prop_types::Axis::Y },
        SpruceWood, "minecraft:spruce_wood" => SpruceWoodState { axis => prop_types::Axis::Y },
        BirchWood, "minecraft:birch_wood" => BirchWoodState { axis => prop_types::Axis::Y },
        JungleWood, "minecraft:jungle_wood" => JungleWoodState { axis => prop_types::Axis::Y },
        AcaciaWood, "minecraft:acacia_wood" => AcaciaWoodState { axis => prop_types::Axis::Y },
        DarkOakWood, "minecraft:dark_oak_wood" => DarkOakWoodState { axis => prop_types::Axis::Y },
        MangroveWood, "minecraft:mangrove_wood" => MangroveWoodState { axis => prop_types::Axis::Y },
        StrippedOakWood, "minecraft:stripped_oak_wood" => StrippedOakWoodState { axis => prop_types::Axis::Y },
        StrippedSpruceWood, "minecraft:stripped_spruce_wood" => StrippedSpruceWoodState { axis => prop_types::Axis::Y },
        StrippedBirchWood, "minecraft:stripped_birch_wood" => StrippedBirchWoodState { axis => prop_types::Axis::Y },
        StrippedJungleWood, "minecraft:stripped_jungle_wood" => StrippedJungleWoodState { axis => prop_types::Axis::Y },
        StrippedAcaciaWood, "minecraft:stripped_acacia_wood" => StrippedAcaciaWoodState { axis => prop_types::Axis::Y },
        StrippedDarkOakWood, "minecraft:stripped_dark_oak_wood" => StrippedDarkOakWoodState { axis => prop_types::Axis::Y },
        StrippedMangroveWood, "minecraft:stripped_mangrove_wood" => StrippedMangroveWoodState { axis => prop_types::Axis::Y },
        OakLeaves, "minecraft:oak_leaves" => OakLeavesState {
            distance => prop_types::ConstrainedInt(7),
            persistent => false,
            waterlogged => false
        },
        SpruceLeaves, "minecraft:spruce_leaves" => SpruceLeavesState {
            distance => prop_types::ConstrainedInt(7),
            persistent => false,
            waterlogged => false
        },
        BirchLeaves, "minecraft:birch_leaves" => BirchLeavesState {
            distance => prop_types::ConstrainedInt(7),
            persistent => false,
            waterlogged => false
        },
        JungleLeaves, "minecraft:jungle_leaves" => JungleLeavesState {
            distance => prop_types::ConstrainedInt(7),
            persistent => false,
            waterlogged => false
        },
        AcaciaLeaves, "minecraft:acacia_leaves" => AcaciaLeavesState {
            distance => prop_types::ConstrainedInt(7),
            persistent => false,
            waterlogged => false
        },
        DarkOakLeaves, "minecraft:dark_oak_leaves" => DarkOakLeavesState {
            distance => prop_types::ConstrainedInt(7),
            persistent => false,
            waterlogged => false
        },
        MangroveLeaves, "minecraft:mangrove_leaves" => MangroveLeavesState {
            distance => prop_types::ConstrainedInt(7),
            persistent => false,
            waterlogged => false
        },
        AzaleaLeaves, "minecraft:azalea_leaves" => AzaleaLeavesState {
            distance => prop_types::ConstrainedInt(7),
            persistent => false,
            waterlogged => false
        },
        FloweringAzaleaLeaves, "minecraft:flowering_azalea_leaves" => FloweringAzaleaLeavesState {
            distance => prop_types::ConstrainedInt(7),
            persistent => false,
            waterlogged => false
        },
        Sponge, "minecraft:sponge",
        WetSponge, "minecraft:wet_sponge",
        Glass, "minecraft:glass",
        LapisOre, "minecraft:lapis_ore",
        DeepslateLapisOre, "minecraft:deepslate_lapis_ore",
        LapisBlock, "minecraft:lapis_block",
        Dispenser, "minecraft:dispenser" => DispenserState {
            facing => prop_types::Facing::North,
            triggered => false
        },
        Sandstone, "minecraft:sandstone",
        ChiseledSandstone, "minecraft:chiseled_sandstone",
        CutSandstone, "minecraft:cut_sandstone",
        NoteBlock, "minecraft:note_block" => NoteBlockState {
            noteblock_instrument => prop_types::NoteblockInstrument::Harp,
            note => prop_types::ConstrainedInt(0),
            powered => false
        },
        WhiteBed, "minecraft:white_bed" => WhiteBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        OrangeBed, "minecraft:orange_bed" => OrangeBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        MagentaBed, "minecraft:magenta_bed" => MagentaBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        LightBlueBed, "minecraft:light_blue_bed" => LightBlueBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        YellowBed, "minecraft:yellow_bed" => YellowBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        LimeBed, "minecraft:lime_bed" => LimeBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        PinkBed, "minecraft:pink_bed" => PinkBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        GrayBed, "minecraft:gray_bed" => GrayBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        LightGrayBed, "minecraft:light_gray_bed" => LightGrayBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        CyanBed, "minecraft:cyan_bed" => CyanBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        PurpleBed, "minecraft:purple_bed" => PurpleBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        BlueBed, "minecraft:blue_bed" => BlueBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        BrownBed, "minecraft:brown_bed" => BrownBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        GreenBed, "minecraft:green_bed" => GreenBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        RedBed, "minecraft:red_bed" => RedBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        BlackBed, "minecraft:black_bed" => BlackBedState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            occupied => false,
            bed_part => prop_types::BedPart::Foot
        },
        PoweredRail, "minecraft:powered_rail" => PoweredRailState {
            powered => false,
            rail_shape_straight => prop_types::RailShapeStraight::NorthSouth,
            waterlogged => false
        },
        DetectorRail, "minecraft:detector_rail" => DetectorRailState {
            powered => false,
            rail_shape_straight => prop_types::RailShapeStraight::NorthSouth,
            waterlogged => false
        },
        StickyPiston, "minecraft:sticky_piston" => StickyPistonState {
            extended => false,
            facing => prop_types::Facing::North
        },
        Cobweb, "minecraft:cobweb",
        Grass, "minecraft:grass",
        Fern, "minecraft:fern",
        DeadBush, "minecraft:dead_bush",
        Seagrass, "minecraft:seagrass",
        TallSeagrass, "minecraft:tall_seagrass" => TallSeagrassState { double_block_half => prop_types::DoubleBlockHalf::Lower },
        Piston, "minecraft:piston" => PistonState {
            extended => false,
            facing => prop_types::Facing::North
        },
        PistonHead, "minecraft:piston_head" => PistonHeadState {
            facing => prop_types::Facing::North,
            short => false,
            piston_type => prop_types::PistonType::Normal
        },
        WhiteWool, "minecraft:white_wool",
        OrangeWool, "minecraft:orange_wool",
        MagentaWool, "minecraft:magenta_wool",
        LightBlueWool, "minecraft:light_blue_wool",
        YellowWool, "minecraft:yellow_wool",
        LimeWool, "minecraft:lime_wool",
        PinkWool, "minecraft:pink_wool",
        GrayWool, "minecraft:gray_wool",
        LightGrayWool, "minecraft:light_gray_wool",
        CyanWool, "minecraft:cyan_wool",
        PurpleWool, "minecraft:purple_wool",
        BlueWool, "minecraft:blue_wool",
        BrownWool, "minecraft:brown_wool",
        GreenWool, "minecraft:green_wool",
        RedWool, "minecraft:red_wool",
        BlackWool, "minecraft:black_wool",
        MovingPiston, "minecraft:moving_piston" => MovingPistonState {
            facing => prop_types::Facing::North,
            piston_type => prop_types::PistonType::Normal
        },
        Dandelion, "minecraft:dandelion",
        Poppy, "minecraft:poppy",
        BlueOrchid, "minecraft:blue_orchid",
        Allium, "minecraft:allium",
        AzureBluet, "minecraft:azure_bluet",
        RedTulip, "minecraft:red_tulip",
        OrangeTulip, "minecraft:orange_tulip",
        WhiteTulip, "minecraft:white_tulip",
        PinkTulip, "minecraft:pink_tulip",
        OxeyeDaisy, "minecraft:oxeye_daisy",
        Cornflower, "minecraft:cornflower",
        WitherRose, "minecraft:wither_rose",
        LilyOfTheValley, "minecraft:lily_of_the_valley",
        BrownMushroom, "minecraft:brown_mushroom",
        RedMushroom, "minecraft:red_mushroom",
        GoldBlock, "minecraft:gold_block",
        IronBlock, "minecraft:iron_block",
        Bricks, "minecraft:bricks",
        Tnt, "minecraft:tnt" => TntState { unstable => false },
        Bookshelf, "minecraft:bookshelf",
        MossyCobblestone, "minecraft:mossy_cobblestone",
        Obsidian, "minecraft:obsidian",
        Torch, "minecraft:torch",
        WallTorch, "minecraft:wall_torch" => WallTorchState { horizontal_facing => prop_types::HorizontalFacing::North },
        Fire, "minecraft:fire" => FireState {
            age_15 => prop_types::ConstrainedInt(0),
            east => false,
            north => false,
            south => false,
            up => false,
            west => false
        },
        SoulFire, "minecraft:soul_fire",
        Spawner, "minecraft:spawner",
        OakStairs, "minecraft:oak_stairs" => OakStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        Chest, "minecraft:chest" => ChestState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            chest_type => prop_types::ChestType::Single,
            waterlogged => false
        },
        RedstoneWire, "minecraft:redstone_wire" => RedstoneWireState {
            east_redstone => prop_types::RedstoneSide::None,
            north_redstone => prop_types::RedstoneSide::None,
            power => prop_types::ConstrainedInt(0),
            south_redstone => prop_types::RedstoneSide::None,
            west_redstone => prop_types::RedstoneSide::None
        },
        DiamondOre, "minecraft:diamond_ore",
        DeepslateDiamondOre, "minecraft:deepslate_diamond_ore",
        DiamondBlock, "minecraft:diamond_block",
        CraftingTable, "minecraft:crafting_table",
        Wheat, "minecraft:wheat" => WheatState { age_7 => prop_types::ConstrainedInt(0) },
        Farmland, "minecraft:farmland" => FarmlandState { moisture => prop_types::ConstrainedInt(0) },
        Furnace, "minecraft:furnace" => FurnaceState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            lit => false
        },
        OakSign, "minecraft:oak_sign" => OakSignState {
            rotation_16 => prop_types::ConstrainedInt(0),
            waterlogged => false
        },
        SpruceSign, "minecraft:spruce_sign" => SpruceSignState {
            rotation_16 => prop_types::ConstrainedInt(0),
            waterlogged => false
        },
        BirchSign, "minecraft:birch_sign" => BirchSignState {
            rotation_16 => prop_types::ConstrainedInt(0),
            waterlogged => false
        },
        AcaciaSign, "minecraft:acacia_sign" => AcaciaSignState {
            rotation_16 => prop_types::ConstrainedInt(0),
            waterlogged => false
        },
        JungleSign, "minecraft:jungle_sign" => JungleSignState {
            rotation_16 => prop_types::ConstrainedInt(0),
            waterlogged => false
        },
        DarkOakSign, "minecraft:dark_oak_sign" => DarkOakSignState {
            rotation_16 => prop_types::ConstrainedInt(0),
            waterlogged => false
        },
        MangroveSign, "minecraft:mangrove_sign" => MangroveSignState {
            rotation_16 => prop_types::ConstrainedInt(0),
            waterlogged => false
        },
        OakDoor, "minecraft:oak_door" => OakDoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            double_block_half => prop_types::DoubleBlockHalf::Lower,
            door_hinge => prop_types::DoorHinge::Left,
            open => false,
            powered => false
        },
        Ladder, "minecraft:ladder" => LadderState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => false
        },
        Rail, "minecraft:rail" => RailState {
            rail_shape => prop_types::RailShape::NorthSouth,
            waterlogged => false
        },
        CobblestoneStairs, "minecraft:cobblestone_stairs" => CobblestoneStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        OakWallSign, "minecraft:oak_wall_sign" => OakWallSignState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => false
        },
        SpruceWallSign, "minecraft:spruce_wall_sign" => SpruceWallSignState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => false
        },
        BirchWallSign, "minecraft:birch_wall_sign" => BirchWallSignState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => false
        },
        AcaciaWallSign, "minecraft:acacia_wall_sign" => AcaciaWallSignState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => false
        },
        JungleWallSign, "minecraft:jungle_wall_sign" => JungleWallSignState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => false
        },
        DarkOakWallSign, "minecraft:dark_oak_wall_sign" => DarkOakWallSignState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => false
        },
        MangroveWallSign, "minecraft:mangrove_wall_sign" => MangroveWallSignState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => false
        },
        Lever, "minecraft:lever" => LeverState {
            attach_face => prop_types::AttachFace::Wall,
            horizontal_facing => prop_types::HorizontalFacing::North,
            powered => false
        },
        StonePressurePlate, "minecraft:stone_pressure_plate" => StonePressurePlateState { powered => false },
        IronDoor, "minecraft:iron_door" => IronDoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            double_block_half => prop_types::DoubleBlockHalf::Lower,
            door_hinge => prop_types::DoorHinge::Left,
            open => false,
            powered => false
        },
        OakPressurePlate, "minecraft:oak_pressure_plate" => OakPressurePlateState { powered => false },
        SprucePressurePlate, "minecraft:spruce_pressure_plate" => SprucePressurePlateState { powered => false },
        BirchPressurePlate, "minecraft:birch_pressure_plate" => BirchPressurePlateState { powered => false },
        JunglePressurePlate, "minecraft:jungle_pressure_plate" => JunglePressurePlateState { powered => false },
        AcaciaPressurePlate, "minecraft:acacia_pressure_plate" => AcaciaPressurePlateState { powered => false },
        DarkOakPressurePlate, "minecraft:dark_oak_pressure_plate" => DarkOakPressurePlateState { powered => false },
        MangrovePressurePlate, "minecraft:mangrove_pressure_plate" => MangrovePressurePlateState { powered => false },
        RedstoneOre, "minecraft:redstone_ore" => RedstoneOreState { lit => false },
        DeepslateRedstoneOre, "minecraft:deepslate_redstone_ore" => DeepslateRedstoneOreState { lit => false },
        RedstoneTorch, "minecraft:redstone_torch" => RedstoneTorchState { lit => true },
        RedstoneWallTorch, "minecraft:redstone_wall_torch" => RedstoneWallTorchState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            lit => true
        },
        StoneButton, "minecraft:stone_button" => StoneButtonState {
            attach_face => prop_types::AttachFace::Wall,
            horizontal_facing => prop_types::HorizontalFacing::North,
            powered => false
        },
        Snow, "minecraft:snow" => SnowState { layers => prop_types::ConstrainedInt(1) },
        Ice, "minecraft:ice",
        SnowBlock, "minecraft:snow_block",
        Cactus, "minecraft:cactus" => CactusState { age_15 => prop_types::ConstrainedInt(0) },
        Clay, "minecraft:clay",
        SugarCane, "minecraft:sugar_cane" => SugarCaneState { age_15 => prop_types::ConstrainedInt(0) },
        Jukebox, "minecraft:jukebox" => JukeboxState { has_record => false },
        OakFence, "minecraft:oak_fence" => OakFenceState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        Pumpkin, "minecraft:pumpkin",
        Netherrack, "minecraft:netherrack",
        SoulSand, "minecraft:soul_sand",
        SoulSoil, "minecraft:soul_soil",
        Basalt, "minecraft:basalt" => BasaltState { axis => prop_types::Axis::Y },
        PolishedBasalt, "minecraft:polished_basalt" => PolishedBasaltState { axis => prop_types::Axis::Y },
        SoulTorch, "minecraft:soul_torch",
        SoulWallTorch, "minecraft:soul_wall_torch" => SoulWallTorchState { horizontal_facing => prop_types::HorizontalFacing::North },
        Glowstone, "minecraft:glowstone",
        NetherPortal, "minecraft:nether_portal" => NetherPortalState { horizontal_axis => prop_types::HorizontalAxis::X },
        CarvedPumpkin, "minecraft:carved_pumpkin" => CarvedPumpkinState { horizontal_facing => prop_types::HorizontalFacing::North },
        JackOLantern, "minecraft:jack_o_lantern" => JackOLanternState { horizontal_facing => prop_types::HorizontalFacing::North },
        Cake, "minecraft:cake" => CakeState { bites => prop_types::ConstrainedInt(0) },
        Repeater, "minecraft:repeater" => RepeaterState {
            delay => prop_types::ConstrainedInt(1),
            horizontal_facing => prop_types::HorizontalFacing::North,
            locked => false,
            powered => false
        },
        WhiteStainedGlass, "minecraft:white_stained_glass",
        OrangeStainedGlass, "minecraft:orange_stained_glass",
        MagentaStainedGlass, "minecraft:magenta_stained_glass",
        LightBlueStainedGlass, "minecraft:light_blue_stained_glass",
        YellowStainedGlass, "minecraft:yellow_stained_glass",
        LimeStainedGlass, "minecraft:lime_stained_glass",
        PinkStainedGlass, "minecraft:pink_stained_glass",
        GrayStainedGlass, "minecraft:gray_stained_glass",
        LightGrayStainedGlass, "minecraft:light_gray_stained_glass",
        CyanStainedGlass, "minecraft:cyan_stained_glass",
        PurpleStainedGlass, "minecraft:purple_stained_glass",
        BlueStainedGlass, "minecraft:blue_stained_glass",
        BrownStainedGlass, "minecraft:brown_stained_glass",
        GreenStainedGlass, "minecraft:green_stained_glass",
        RedStainedGlass, "minecraft:red_stained_glass",
        BlackStainedGlass, "minecraft:black_stained_glass",
        OakTrapdoor, "minecraft:oak_trapdoor" => OakTrapdoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            open => false,
            powered => false,
            waterlogged => false
        },
        SpruceTrapdoor, "minecraft:spruce_trapdoor" => SpruceTrapdoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            open => false,
            powered => false,
            waterlogged => false
        },
        BirchTrapdoor, "minecraft:birch_trapdoor" => BirchTrapdoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            open => false,
            powered => false,
            waterlogged => false
        },
        JungleTrapdoor, "minecraft:jungle_trapdoor" => JungleTrapdoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            open => false,
            powered => false,
            waterlogged => false
        },
        AcaciaTrapdoor, "minecraft:acacia_trapdoor" => AcaciaTrapdoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            open => false,
            powered => false,
            waterlogged => false
        },
        DarkOakTrapdoor, "minecraft:dark_oak_trapdoor" => DarkOakTrapdoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            open => false,
            powered => false,
            waterlogged => false
        },
        MangroveTrapdoor, "minecraft:mangrove_trapdoor" => MangroveTrapdoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            open => false,
            powered => false,
            waterlogged => false
        },
        StoneBricks, "minecraft:stone_bricks",
        MossyStoneBricks, "minecraft:mossy_stone_bricks",
        CrackedStoneBricks, "minecraft:cracked_stone_bricks",
        ChiseledStoneBricks, "minecraft:chiseled_stone_bricks",
        PackedMud, "minecraft:packed_mud",
        MudBricks, "minecraft:mud_bricks",
        InfestedStone, "minecraft:infested_stone",
        InfestedCobblestone, "minecraft:infested_cobblestone",
        InfestedStoneBricks, "minecraft:infested_stone_bricks",
        InfestedMossyStoneBricks, "minecraft:infested_mossy_stone_bricks",
        InfestedCrackedStoneBricks, "minecraft:infested_cracked_stone_bricks",
        InfestedChiseledStoneBricks, "minecraft:infested_chiseled_stone_bricks",
        BrownMushroomBlock, "minecraft:brown_mushroom_block" => BrownMushroomBlockState {
            down => true,
            east => true,
            north => true,
            south => true,
            up => true,
            west => true
        },
        RedMushroomBlock, "minecraft:red_mushroom_block" => RedMushroomBlockState {
            down => true,
            east => true,
            north => true,
            south => true,
            up => true,
            west => true
        },
        MushroomStem, "minecraft:mushroom_stem" => MushroomStemState {
            down => true,
            east => true,
            north => true,
            south => true,
            up => true,
            west => true
        },
        IronBars, "minecraft:iron_bars" => IronBarsState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        Chain, "minecraft:chain" => ChainState {
            axis => prop_types::Axis::Y,
            waterlogged => false
        },
        GlassPane, "minecraft:glass_pane" => GlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        Melon, "minecraft:melon",
        AttachedPumpkinStem, "minecraft:attached_pumpkin_stem" => AttachedPumpkinStemState { horizontal_facing => prop_types::HorizontalFacing::North },
        AttachedMelonStem, "minecraft:attached_melon_stem" => AttachedMelonStemState { horizontal_facing => prop_types::HorizontalFacing::North },
        PumpkinStem, "minecraft:pumpkin_stem" => PumpkinStemState { age_7 => prop_types::ConstrainedInt(0) },
        MelonStem, "minecraft:melon_stem" => MelonStemState { age_7 => prop_types::ConstrainedInt(0) },
        Vine, "minecraft:vine" => VineState {
            east => false,
            north => false,
            south => false,
            up => false,
            west => false
        },
        GlowLichen, "minecraft:glow_lichen" => GlowLichenState {
            down => false,
            east => false,
            north => false,
            south => false,
            up => false,
            waterlogged => false,
            west => false
        },
        OakFenceGate, "minecraft:oak_fence_gate" => OakFenceGateState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            in_wall => false,
            open => false,
            powered => false
        },
        BrickStairs, "minecraft:brick_stairs" => BrickStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        StoneBrickStairs, "minecraft:stone_brick_stairs" => StoneBrickStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        MudBrickStairs, "minecraft:mud_brick_stairs" => MudBrickStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        Mycelium, "minecraft:mycelium" => MyceliumState { snowy => false },
        LilyPad, "minecraft:lily_pad",
        NetherBricks, "minecraft:nether_bricks",
        NetherBrickFence, "minecraft:nether_brick_fence" => NetherBrickFenceState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        NetherBrickStairs, "minecraft:nether_brick_stairs" => NetherBrickStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        NetherWart, "minecraft:nether_wart" => NetherWartState { age_3 => prop_types::ConstrainedInt(0) },
        EnchantingTable, "minecraft:enchanting_table",
        BrewingStand, "minecraft:brewing_stand" => BrewingStandState {
            has_bottle_0 => false,
            has_bottle_1 => false,
            has_bottle_2 => false
        },
        Cauldron, "minecraft:cauldron",
        WaterCauldron, "minecraft:water_cauldron" => WaterCauldronState { level_cauldron => prop_types::ConstrainedInt(1) },
        LavaCauldron, "minecraft:lava_cauldron",
        PowderSnowCauldron, "minecraft:powder_snow_cauldron" => PowderSnowCauldronState { level_cauldron => prop_types::ConstrainedInt(1) },
        EndPortal, "minecraft:end_portal",
        EndPortalFrame, "minecraft:end_portal_frame" => EndPortalFrameState {
            eye => false,
            horizontal_facing => prop_types::HorizontalFacing::North
        },
        EndStone, "minecraft:end_stone",
        DragonEgg, "minecraft:dragon_egg",
        RedstoneLamp, "minecraft:redstone_lamp" => RedstoneLampState { lit => false },
        Cocoa, "minecraft:cocoa" => CocoaState {
            age_2 => prop_types::ConstrainedInt(0),
            horizontal_facing => prop_types::HorizontalFacing::North
        },
        SandstoneStairs, "minecraft:sandstone_stairs" => SandstoneStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        EmeraldOre, "minecraft:emerald_ore",
        DeepslateEmeraldOre, "minecraft:deepslate_emerald_ore",
        EnderChest, "minecraft:ender_chest" => EnderChestState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => false
        },
        TripwireHook, "minecraft:tripwire_hook" => TripwireHookState {
            attached => false,
            horizontal_facing => prop_types::HorizontalFacing::North,
            powered => false
        },
        Tripwire, "minecraft:tripwire" => TripwireState {
            attached => false,
            disarmed => false,
            east => false,
            north => false,
            powered => false,
            south => false,
            west => false
        },
        EmeraldBlock, "minecraft:emerald_block",
        SpruceStairs, "minecraft:spruce_stairs" => SpruceStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        BirchStairs, "minecraft:birch_stairs" => BirchStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        JungleStairs, "minecraft:jungle_stairs" => JungleStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        CommandBlock, "minecraft:command_block" => CommandBlockState {
            conditional => false,
            facing => prop_types::Facing::North
        },
        Beacon, "minecraft:beacon",
        CobblestoneWall, "minecraft:cobblestone_wall" => CobblestoneWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        MossyCobblestoneWall, "minecraft:mossy_cobblestone_wall" => MossyCobblestoneWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        FlowerPot, "minecraft:flower_pot",
        PottedOakSapling, "minecraft:potted_oak_sapling",
        PottedSpruceSapling, "minecraft:potted_spruce_sapling",
        PottedBirchSapling, "minecraft:potted_birch_sapling",
        PottedJungleSapling, "minecraft:potted_jungle_sapling",
        PottedAcaciaSapling, "minecraft:potted_acacia_sapling",
        PottedDarkOakSapling, "minecraft:potted_dark_oak_sapling",
        PottedMangrovePropagule, "minecraft:potted_mangrove_propagule",
        PottedFern, "minecraft:potted_fern",
        PottedDandelion, "minecraft:potted_dandelion",
        PottedPoppy, "minecraft:potted_poppy",
        PottedBlueOrchid, "minecraft:potted_blue_orchid",
        PottedAllium, "minecraft:potted_allium",
        PottedAzureBluet, "minecraft:potted_azure_bluet",
        PottedRedTulip, "minecraft:potted_red_tulip",
        PottedOrangeTulip, "minecraft:potted_orange_tulip",
        PottedWhiteTulip, "minecraft:potted_white_tulip",
        PottedPinkTulip, "minecraft:potted_pink_tulip",
        PottedOxeyeDaisy, "minecraft:potted_oxeye_daisy",
        PottedCornflower, "minecraft:potted_cornflower",
        PottedLilyOfTheValley, "minecraft:potted_lily_of_the_valley",
        PottedWitherRose, "minecraft:potted_wither_rose",
        PottedRedMushroom, "minecraft:potted_red_mushroom",
        PottedBrownMushroom, "minecraft:potted_brown_mushroom",
        PottedDeadBush, "minecraft:potted_dead_bush",
        PottedCactus, "minecraft:potted_cactus",
        Carrots, "minecraft:carrots" => CarrotsState { age_7 => prop_types::ConstrainedInt(0) },
        Potatoes, "minecraft:potatoes" => PotatoesState { age_7 => prop_types::ConstrainedInt(0) },
        OakButton, "minecraft:oak_button" => OakButtonState {
            attach_face => prop_types::AttachFace::Wall,
            horizontal_facing => prop_types::HorizontalFacing::North,
            powered => false
        },
        SpruceButton, "minecraft:spruce_button" => SpruceButtonState {
            attach_face => prop_types::AttachFace::Wall,
            horizontal_facing => prop_types::HorizontalFacing::North,
            powered => false
        },
        BirchButton, "minecraft:birch_button" => BirchButtonState {
            attach_face => prop_types::AttachFace::Wall,
            horizontal_facing => prop_types::HorizontalFacing::North,
            powered => false
        },
        JungleButton, "minecraft:jungle_button" => JungleButtonState {
            attach_face => prop_types::AttachFace::Wall,
            horizontal_facing => prop_types::HorizontalFacing::North,
            powered => false
        },
        AcaciaButton, "minecraft:acacia_button" => AcaciaButtonState {
            attach_face => prop_types::AttachFace::Wall,
            horizontal_facing => prop_types::HorizontalFacing::North,
            powered => false
        },
        DarkOakButton, "minecraft:dark_oak_button" => DarkOakButtonState {
            attach_face => prop_types::AttachFace::Wall,
            horizontal_facing => prop_types::HorizontalFacing::North,
            powered => false
        },
        MangroveButton, "minecraft:mangrove_button" => MangroveButtonState {
            attach_face => prop_types::AttachFace::Wall,
            horizontal_facing => prop_types::HorizontalFacing::North,
            powered => false
        },
        SkeletonSkull, "minecraft:skeleton_skull" => SkeletonSkullState { rotation_16 => prop_types::ConstrainedInt(0) },
        SkeletonWallSkull, "minecraft:skeleton_wall_skull" => SkeletonWallSkullState { horizontal_facing => prop_types::HorizontalFacing::North },
        WitherSkeletonSkull, "minecraft:wither_skeleton_skull" => WitherSkeletonSkullState { rotation_16 => prop_types::ConstrainedInt(0) },
        WitherSkeletonWallSkull, "minecraft:wither_skeleton_wall_skull" => WitherSkeletonWallSkullState { horizontal_facing => prop_types::HorizontalFacing::North },
        ZombieHead, "minecraft:zombie_head" => ZombieHeadState { rotation_16 => prop_types::ConstrainedInt(0) },
        ZombieWallHead, "minecraft:zombie_wall_head" => ZombieWallHeadState { horizontal_facing => prop_types::HorizontalFacing::North },
        PlayerHead, "minecraft:player_head" => PlayerHeadState { rotation_16 => prop_types::ConstrainedInt(0) },
        PlayerWallHead, "minecraft:player_wall_head" => PlayerWallHeadState { horizontal_facing => prop_types::HorizontalFacing::North },
        CreeperHead, "minecraft:creeper_head" => CreeperHeadState { rotation_16 => prop_types::ConstrainedInt(0) },
        CreeperWallHead, "minecraft:creeper_wall_head" => CreeperWallHeadState { horizontal_facing => prop_types::HorizontalFacing::North },
        DragonHead, "minecraft:dragon_head" => DragonHeadState { rotation_16 => prop_types::ConstrainedInt(0) },
        DragonWallHead, "minecraft:dragon_wall_head" => DragonWallHeadState { horizontal_facing => prop_types::HorizontalFacing::North },
        Anvil, "minecraft:anvil" => AnvilState { horizontal_facing => prop_types::HorizontalFacing::North },
        ChippedAnvil, "minecraft:chipped_anvil" => ChippedAnvilState { horizontal_facing => prop_types::HorizontalFacing::North },
        DamagedAnvil, "minecraft:damaged_anvil" => DamagedAnvilState { horizontal_facing => prop_types::HorizontalFacing::North },
        TrappedChest, "minecraft:trapped_chest" => TrappedChestState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            chest_type => prop_types::ChestType::Single,
            waterlogged => false
        },
        LightWeightedPressurePlate, "minecraft:light_weighted_pressure_plate" => LightWeightedPressurePlateState { power => prop_types::ConstrainedInt(0) },
        HeavyWeightedPressurePlate, "minecraft:heavy_weighted_pressure_plate" => HeavyWeightedPressurePlateState { power => prop_types::ConstrainedInt(0) },
        Comparator, "minecraft:comparator" => ComparatorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            mode_comparator => prop_types::ModeComparator::Compare,
            powered => false
        },
        DaylightDetector, "minecraft:daylight_detector" => DaylightDetectorState {
            inverted => false,
            power => prop_types::ConstrainedInt(0)
        },
        RedstoneBlock, "minecraft:redstone_block",
        NetherQuartzOre, "minecraft:nether_quartz_ore",
        Hopper, "minecraft:hopper" => HopperState {
            enabled => true,
            facing_hopper => prop_types::FacingHopper::Down
        },
        QuartzBlock, "minecraft:quartz_block",
        ChiseledQuartzBlock, "minecraft:chiseled_quartz_block",
        QuartzPillar, "minecraft:quartz_pillar" => QuartzPillarState { axis => prop_types::Axis::Y },
        QuartzStairs, "minecraft:quartz_stairs" => QuartzStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        ActivatorRail, "minecraft:activator_rail" => ActivatorRailState {
            powered => false,
            rail_shape_straight => prop_types::RailShapeStraight::NorthSouth,
            waterlogged => false
        },
        Dropper, "minecraft:dropper" => DropperState {
            facing => prop_types::Facing::North,
            triggered => false
        },
        WhiteTerracotta, "minecraft:white_terracotta",
        OrangeTerracotta, "minecraft:orange_terracotta",
        MagentaTerracotta, "minecraft:magenta_terracotta",
        LightBlueTerracotta, "minecraft:light_blue_terracotta",
        YellowTerracotta, "minecraft:yellow_terracotta",
        LimeTerracotta, "minecraft:lime_terracotta",
        PinkTerracotta, "minecraft:pink_terracotta",
        GrayTerracotta, "minecraft:gray_terracotta",
        LightGrayTerracotta, "minecraft:light_gray_terracotta",
        CyanTerracotta, "minecraft:cyan_terracotta",
        PurpleTerracotta, "minecraft:purple_terracotta",
        BlueTerracotta, "minecraft:blue_terracotta",
        BrownTerracotta, "minecraft:brown_terracotta",
        GreenTerracotta, "minecraft:green_terracotta",
        RedTerracotta, "minecraft:red_terracotta",
        BlackTerracotta, "minecraft:black_terracotta",
        WhiteStainedGlassPane, "minecraft:white_stained_glass_pane" => WhiteStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        OrangeStainedGlassPane, "minecraft:orange_stained_glass_pane" => OrangeStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        MagentaStainedGlassPane, "minecraft:magenta_stained_glass_pane" => MagentaStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        LightBlueStainedGlassPane, "minecraft:light_blue_stained_glass_pane" => LightBlueStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        YellowStainedGlassPane, "minecraft:yellow_stained_glass_pane" => YellowStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        LimeStainedGlassPane, "minecraft:lime_stained_glass_pane" => LimeStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        PinkStainedGlassPane, "minecraft:pink_stained_glass_pane" => PinkStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        GrayStainedGlassPane, "minecraft:gray_stained_glass_pane" => GrayStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        LightGrayStainedGlassPane, "minecraft:light_gray_stained_glass_pane" => LightGrayStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        CyanStainedGlassPane, "minecraft:cyan_stained_glass_pane" => CyanStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        PurpleStainedGlassPane, "minecraft:purple_stained_glass_pane" => PurpleStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        BlueStainedGlassPane, "minecraft:blue_stained_glass_pane" => BlueStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        BrownStainedGlassPane, "minecraft:brown_stained_glass_pane" => BrownStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        GreenStainedGlassPane, "minecraft:green_stained_glass_pane" => GreenStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        RedStainedGlassPane, "minecraft:red_stained_glass_pane" => RedStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        BlackStainedGlassPane, "minecraft:black_stained_glass_pane" => BlackStainedGlassPaneState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        AcaciaStairs, "minecraft:acacia_stairs" => AcaciaStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        DarkOakStairs, "minecraft:dark_oak_stairs" => DarkOakStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        MangroveStairs, "minecraft:mangrove_stairs" => MangroveStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        SlimeBlock, "minecraft:slime_block",
        Barrier, "minecraft:barrier",
        Light, "minecraft:light" => LightState {
            level => prop_types::ConstrainedInt(15),
            waterlogged => false
        },
        IronTrapdoor, "minecraft:iron_trapdoor" => IronTrapdoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            open => false,
            powered => false,
            waterlogged => false
        },
        Prismarine, "minecraft:prismarine",
        PrismarineBricks, "minecraft:prismarine_bricks",
        DarkPrismarine, "minecraft:dark_prismarine",
        PrismarineStairs, "minecraft:prismarine_stairs" => PrismarineStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        PrismarineBrickStairs, "minecraft:prismarine_brick_stairs" => PrismarineBrickStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        DarkPrismarineStairs, "minecraft:dark_prismarine_stairs" => DarkPrismarineStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        PrismarineSlab, "minecraft:prismarine_slab" => PrismarineSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        PrismarineBrickSlab, "minecraft:prismarine_brick_slab" => PrismarineBrickSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        DarkPrismarineSlab, "minecraft:dark_prismarine_slab" => DarkPrismarineSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        SeaLantern, "minecraft:sea_lantern",
        HayBlock, "minecraft:hay_block" => HayBlockState { axis => prop_types::Axis::Y },
        WhiteCarpet, "minecraft:white_carpet",
        OrangeCarpet, "minecraft:orange_carpet",
        MagentaCarpet, "minecraft:magenta_carpet",
        LightBlueCarpet, "minecraft:light_blue_carpet",
        YellowCarpet, "minecraft:yellow_carpet",
        LimeCarpet, "minecraft:lime_carpet",
        PinkCarpet, "minecraft:pink_carpet",
        GrayCarpet, "minecraft:gray_carpet",
        LightGrayCarpet, "minecraft:light_gray_carpet",
        CyanCarpet, "minecraft:cyan_carpet",
        PurpleCarpet, "minecraft:purple_carpet",
        BlueCarpet, "minecraft:blue_carpet",
        BrownCarpet, "minecraft:brown_carpet",
        GreenCarpet, "minecraft:green_carpet",
        RedCarpet, "minecraft:red_carpet",
        BlackCarpet, "minecraft:black_carpet",
        Terracotta, "minecraft:terracotta",
        CoalBlock, "minecraft:coal_block",
        PackedIce, "minecraft:packed_ice",
        Sunflower, "minecraft:sunflower" => SunflowerState { double_block_half => prop_types::DoubleBlockHalf::Lower },
        Lilac, "minecraft:lilac" => LilacState { double_block_half => prop_types::DoubleBlockHalf::Lower },
        RoseBush, "minecraft:rose_bush" => RoseBushState { double_block_half => prop_types::DoubleBlockHalf::Lower },
        Peony, "minecraft:peony" => PeonyState { double_block_half => prop_types::DoubleBlockHalf::Lower },
        TallGrass, "minecraft:tall_grass" => TallGrassState { double_block_half => prop_types::DoubleBlockHalf::Lower },
        LargeFern, "minecraft:large_fern" => LargeFernState { double_block_half => prop_types::DoubleBlockHalf::Lower },
        WhiteBanner, "minecraft:white_banner" => WhiteBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        OrangeBanner, "minecraft:orange_banner" => OrangeBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        MagentaBanner, "minecraft:magenta_banner" => MagentaBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        LightBlueBanner, "minecraft:light_blue_banner" => LightBlueBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        YellowBanner, "minecraft:yellow_banner" => YellowBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        LimeBanner, "minecraft:lime_banner" => LimeBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        PinkBanner, "minecraft:pink_banner" => PinkBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        GrayBanner, "minecraft:gray_banner" => GrayBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        LightGrayBanner, "minecraft:light_gray_banner" => LightGrayBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        CyanBanner, "minecraft:cyan_banner" => CyanBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        PurpleBanner, "minecraft:purple_banner" => PurpleBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        BlueBanner, "minecraft:blue_banner" => BlueBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        BrownBanner, "minecraft:brown_banner" => BrownBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        GreenBanner, "minecraft:green_banner" => GreenBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        RedBanner, "minecraft:red_banner" => RedBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        BlackBanner, "minecraft:black_banner" => BlackBannerState { rotation_16 => prop_types::ConstrainedInt(0) },
        WhiteWallBanner, "minecraft:white_wall_banner" => WhiteWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        OrangeWallBanner, "minecraft:orange_wall_banner" => OrangeWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        MagentaWallBanner, "minecraft:magenta_wall_banner" => MagentaWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        LightBlueWallBanner, "minecraft:light_blue_wall_banner" => LightBlueWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        YellowWallBanner, "minecraft:yellow_wall_banner" => YellowWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        LimeWallBanner, "minecraft:lime_wall_banner" => LimeWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        PinkWallBanner, "minecraft:pink_wall_banner" => PinkWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        GrayWallBanner, "minecraft:gray_wall_banner" => GrayWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        LightGrayWallBanner, "minecraft:light_gray_wall_banner" => LightGrayWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        CyanWallBanner, "minecraft:cyan_wall_banner" => CyanWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        PurpleWallBanner, "minecraft:purple_wall_banner" => PurpleWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        BlueWallBanner, "minecraft:blue_wall_banner" => BlueWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        BrownWallBanner, "minecraft:brown_wall_banner" => BrownWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        GreenWallBanner, "minecraft:green_wall_banner" => GreenWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        RedWallBanner, "minecraft:red_wall_banner" => RedWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        BlackWallBanner, "minecraft:black_wall_banner" => BlackWallBannerState { horizontal_facing => prop_types::HorizontalFacing::North },
        RedSandstone, "minecraft:red_sandstone",
        ChiseledRedSandstone, "minecraft:chiseled_red_sandstone",
        CutRedSandstone, "minecraft:cut_red_sandstone",
        RedSandstoneStairs, "minecraft:red_sandstone_stairs" => RedSandstoneStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        OakSlab, "minecraft:oak_slab" => OakSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        SpruceSlab, "minecraft:spruce_slab" => SpruceSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        BirchSlab, "minecraft:birch_slab" => BirchSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        JungleSlab, "minecraft:jungle_slab" => JungleSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        AcaciaSlab, "minecraft:acacia_slab" => AcaciaSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        DarkOakSlab, "minecraft:dark_oak_slab" => DarkOakSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        MangroveSlab, "minecraft:mangrove_slab" => MangroveSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        StoneSlab, "minecraft:stone_slab" => StoneSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        SmoothStoneSlab, "minecraft:smooth_stone_slab" => SmoothStoneSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        SandstoneSlab, "minecraft:sandstone_slab" => SandstoneSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        CutSandstoneSlab, "minecraft:cut_sandstone_slab" => CutSandstoneSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        PetrifiedOakSlab, "minecraft:petrified_oak_slab" => PetrifiedOakSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        CobblestoneSlab, "minecraft:cobblestone_slab" => CobblestoneSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        BrickSlab, "minecraft:brick_slab" => BrickSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        StoneBrickSlab, "minecraft:stone_brick_slab" => StoneBrickSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        MudBrickSlab, "minecraft:mud_brick_slab" => MudBrickSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        NetherBrickSlab, "minecraft:nether_brick_slab" => NetherBrickSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        QuartzSlab, "minecraft:quartz_slab" => QuartzSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        RedSandstoneSlab, "minecraft:red_sandstone_slab" => RedSandstoneSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        CutRedSandstoneSlab, "minecraft:cut_red_sandstone_slab" => CutRedSandstoneSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        PurpurSlab, "minecraft:purpur_slab" => PurpurSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        SmoothStone, "minecraft:smooth_stone",
        SmoothSandstone, "minecraft:smooth_sandstone",
        SmoothQuartz, "minecraft:smooth_quartz",
        SmoothRedSandstone, "minecraft:smooth_red_sandstone",
        SpruceFenceGate, "minecraft:spruce_fence_gate" => SpruceFenceGateState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            in_wall => false,
            open => false,
            powered => false
        },
        BirchFenceGate, "minecraft:birch_fence_gate" => BirchFenceGateState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            in_wall => false,
            open => false,
            powered => false
        },
        JungleFenceGate, "minecraft:jungle_fence_gate" => JungleFenceGateState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            in_wall => false,
            open => false,
            powered => false
        },
        AcaciaFenceGate, "minecraft:acacia_fence_gate" => AcaciaFenceGateState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            in_wall => false,
            open => false,
            powered => false
        },
        DarkOakFenceGate, "minecraft:dark_oak_fence_gate" => DarkOakFenceGateState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            in_wall => false,
            open => false,
            powered => false
        },
        MangroveFenceGate, "minecraft:mangrove_fence_gate" => MangroveFenceGateState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            in_wall => false,
            open => false,
            powered => false
        },
        SpruceFence, "minecraft:spruce_fence" => SpruceFenceState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        BirchFence, "minecraft:birch_fence" => BirchFenceState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        JungleFence, "minecraft:jungle_fence" => JungleFenceState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        AcaciaFence, "minecraft:acacia_fence" => AcaciaFenceState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        DarkOakFence, "minecraft:dark_oak_fence" => DarkOakFenceState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        MangroveFence, "minecraft:mangrove_fence" => MangroveFenceState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        SpruceDoor, "minecraft:spruce_door" => SpruceDoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            double_block_half => prop_types::DoubleBlockHalf::Lower,
            door_hinge => prop_types::DoorHinge::Left,
            open => false,
            powered => false
        },
        BirchDoor, "minecraft:birch_door" => BirchDoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            double_block_half => prop_types::DoubleBlockHalf::Lower,
            door_hinge => prop_types::DoorHinge::Left,
            open => false,
            powered => false
        },
        JungleDoor, "minecraft:jungle_door" => JungleDoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            double_block_half => prop_types::DoubleBlockHalf::Lower,
            door_hinge => prop_types::DoorHinge::Left,
            open => false,
            powered => false
        },
        AcaciaDoor, "minecraft:acacia_door" => AcaciaDoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            double_block_half => prop_types::DoubleBlockHalf::Lower,
            door_hinge => prop_types::DoorHinge::Left,
            open => false,
            powered => false
        },
        DarkOakDoor, "minecraft:dark_oak_door" => DarkOakDoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            double_block_half => prop_types::DoubleBlockHalf::Lower,
            door_hinge => prop_types::DoorHinge::Left,
            open => false,
            powered => false
        },
        MangroveDoor, "minecraft:mangrove_door" => MangroveDoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            double_block_half => prop_types::DoubleBlockHalf::Lower,
            door_hinge => prop_types::DoorHinge::Left,
            open => false,
            powered => false
        },
        EndRod, "minecraft:end_rod" => EndRodState { facing => prop_types::Facing::Up },
        ChorusPlant, "minecraft:chorus_plant" => ChorusPlantState {
            down => false,
            east => false,
            north => false,
            south => false,
            up => false,
            west => false
        },
        ChorusFlower, "minecraft:chorus_flower" => ChorusFlowerState { age_5 => prop_types::ConstrainedInt(0) },
        PurpurBlock, "minecraft:purpur_block",
        PurpurPillar, "minecraft:purpur_pillar" => PurpurPillarState { axis => prop_types::Axis::Y },
        PurpurStairs, "minecraft:purpur_stairs" => PurpurStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        EndStoneBricks, "minecraft:end_stone_bricks",
        Beetroots, "minecraft:beetroots" => BeetrootsState { age_3 => prop_types::ConstrainedInt(0) },
        DirtPath, "minecraft:dirt_path",
        EndGateway, "minecraft:end_gateway",
        RepeatingCommandBlock, "minecraft:repeating_command_block" => RepeatingCommandBlockState {
            conditional => false,
            facing => prop_types::Facing::North
        },
        ChainCommandBlock, "minecraft:chain_command_block" => ChainCommandBlockState {
            conditional => false,
            facing => prop_types::Facing::North
        },
        FrostedIce, "minecraft:frosted_ice" => FrostedIceState { age_3 => prop_types::ConstrainedInt(0) },
        MagmaBlock, "minecraft:magma_block",
        NetherWartBlock, "minecraft:nether_wart_block",
        RedNetherBricks, "minecraft:red_nether_bricks",
        BoneBlock, "minecraft:bone_block" => BoneBlockState { axis => prop_types::Axis::Y },
        StructureVoid, "minecraft:structure_void",
        Observer, "minecraft:observer" => ObserverState {
            facing => prop_types::Facing::South,
            powered => false
        },
        ShulkerBox, "minecraft:shulker_box" => ShulkerBoxState { facing => prop_types::Facing::Up },
        WhiteShulkerBox, "minecraft:white_shulker_box" => WhiteShulkerBoxState { facing => prop_types::Facing::Up },
        OrangeShulkerBox, "minecraft:orange_shulker_box" => OrangeShulkerBoxState { facing => prop_types::Facing::Up },
        MagentaShulkerBox, "minecraft:magenta_shulker_box" => MagentaShulkerBoxState { facing => prop_types::Facing::Up },
        LightBlueShulkerBox, "minecraft:light_blue_shulker_box" => LightBlueShulkerBoxState { facing => prop_types::Facing::Up },
        YellowShulkerBox, "minecraft:yellow_shulker_box" => YellowShulkerBoxState { facing => prop_types::Facing::Up },
        LimeShulkerBox, "minecraft:lime_shulker_box" => LimeShulkerBoxState { facing => prop_types::Facing::Up },
        PinkShulkerBox, "minecraft:pink_shulker_box" => PinkShulkerBoxState { facing => prop_types::Facing::Up },
        GrayShulkerBox, "minecraft:gray_shulker_box" => GrayShulkerBoxState { facing => prop_types::Facing::Up },
        LightGrayShulkerBox, "minecraft:light_gray_shulker_box" => LightGrayShulkerBoxState { facing => prop_types::Facing::Up },
        CyanShulkerBox, "minecraft:cyan_shulker_box" => CyanShulkerBoxState { facing => prop_types::Facing::Up },
        PurpleShulkerBox, "minecraft:purple_shulker_box" => PurpleShulkerBoxState { facing => prop_types::Facing::Up },
        BlueShulkerBox, "minecraft:blue_shulker_box" => BlueShulkerBoxState { facing => prop_types::Facing::Up },
        BrownShulkerBox, "minecraft:brown_shulker_box" => BrownShulkerBoxState { facing => prop_types::Facing::Up },
        GreenShulkerBox, "minecraft:green_shulker_box" => GreenShulkerBoxState { facing => prop_types::Facing::Up },
        RedShulkerBox, "minecraft:red_shulker_box" => RedShulkerBoxState { facing => prop_types::Facing::Up },
        BlackShulkerBox, "minecraft:black_shulker_box" => BlackShulkerBoxState { facing => prop_types::Facing::Up },
        WhiteGlazedTerracotta, "minecraft:white_glazed_terracotta" => WhiteGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        OrangeGlazedTerracotta, "minecraft:orange_glazed_terracotta" => OrangeGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        MagentaGlazedTerracotta, "minecraft:magenta_glazed_terracotta" => MagentaGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        LightBlueGlazedTerracotta, "minecraft:light_blue_glazed_terracotta" => LightBlueGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        YellowGlazedTerracotta, "minecraft:yellow_glazed_terracotta" => YellowGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        LimeGlazedTerracotta, "minecraft:lime_glazed_terracotta" => LimeGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        PinkGlazedTerracotta, "minecraft:pink_glazed_terracotta" => PinkGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        GrayGlazedTerracotta, "minecraft:gray_glazed_terracotta" => GrayGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        LightGrayGlazedTerracotta, "minecraft:light_gray_glazed_terracotta" => LightGrayGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        CyanGlazedTerracotta, "minecraft:cyan_glazed_terracotta" => CyanGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        PurpleGlazedTerracotta, "minecraft:purple_glazed_terracotta" => PurpleGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        BlueGlazedTerracotta, "minecraft:blue_glazed_terracotta" => BlueGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        BrownGlazedTerracotta, "minecraft:brown_glazed_terracotta" => BrownGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        GreenGlazedTerracotta, "minecraft:green_glazed_terracotta" => GreenGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        RedGlazedTerracotta, "minecraft:red_glazed_terracotta" => RedGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        BlackGlazedTerracotta, "minecraft:black_glazed_terracotta" => BlackGlazedTerracottaState { horizontal_facing => prop_types::HorizontalFacing::North },
        WhiteConcrete, "minecraft:white_concrete",
        OrangeConcrete, "minecraft:orange_concrete",
        MagentaConcrete, "minecraft:magenta_concrete",
        LightBlueConcrete, "minecraft:light_blue_concrete",
        YellowConcrete, "minecraft:yellow_concrete",
        LimeConcrete, "minecraft:lime_concrete",
        PinkConcrete, "minecraft:pink_concrete",
        GrayConcrete, "minecraft:gray_concrete",
        LightGrayConcrete, "minecraft:light_gray_concrete",
        CyanConcrete, "minecraft:cyan_concrete",
        PurpleConcrete, "minecraft:purple_concrete",
        BlueConcrete, "minecraft:blue_concrete",
        BrownConcrete, "minecraft:brown_concrete",
        GreenConcrete, "minecraft:green_concrete",
        RedConcrete, "minecraft:red_concrete",
        BlackConcrete, "minecraft:black_concrete",
        WhiteConcretePowder, "minecraft:white_concrete_powder",
        OrangeConcretePowder, "minecraft:orange_concrete_powder",
        MagentaConcretePowder, "minecraft:magenta_concrete_powder",
        LightBlueConcretePowder, "minecraft:light_blue_concrete_powder",
        YellowConcretePowder, "minecraft:yellow_concrete_powder",
        LimeConcretePowder, "minecraft:lime_concrete_powder",
        PinkConcretePowder, "minecraft:pink_concrete_powder",
        GrayConcretePowder, "minecraft:gray_concrete_powder",
        LightGrayConcretePowder, "minecraft:light_gray_concrete_powder",
        CyanConcretePowder, "minecraft:cyan_concrete_powder",
        PurpleConcretePowder, "minecraft:purple_concrete_powder",
        BlueConcretePowder, "minecraft:blue_concrete_powder",
        BrownConcretePowder, "minecraft:brown_concrete_powder",
        GreenConcretePowder, "minecraft:green_concrete_powder",
        RedConcretePowder, "minecraft:red_concrete_powder",
        BlackConcretePowder, "minecraft:black_concrete_powder",
        Kelp, "minecraft:kelp" => KelpState { age_25 => prop_types::ConstrainedInt(0) },
        KelpPlant, "minecraft:kelp_plant",
        DriedKelpBlock, "minecraft:dried_kelp_block",
        TurtleEgg, "minecraft:turtle_egg" => TurtleEggState {
            eggs => prop_types::ConstrainedInt(1),
            hatch => prop_types::ConstrainedInt(0)
        },
        DeadTubeCoralBlock, "minecraft:dead_tube_coral_block",
        DeadBrainCoralBlock, "minecraft:dead_brain_coral_block",
        DeadBubbleCoralBlock, "minecraft:dead_bubble_coral_block",
        DeadFireCoralBlock, "minecraft:dead_fire_coral_block",
        DeadHornCoralBlock, "minecraft:dead_horn_coral_block",
        TubeCoralBlock, "minecraft:tube_coral_block",
        BrainCoralBlock, "minecraft:brain_coral_block",
        BubbleCoralBlock, "minecraft:bubble_coral_block",
        FireCoralBlock, "minecraft:fire_coral_block",
        HornCoralBlock, "minecraft:horn_coral_block",
        DeadTubeCoral, "minecraft:dead_tube_coral" => DeadTubeCoralState { waterlogged => true },
        DeadBrainCoral, "minecraft:dead_brain_coral" => DeadBrainCoralState { waterlogged => true },
        DeadBubbleCoral, "minecraft:dead_bubble_coral" => DeadBubbleCoralState { waterlogged => true },
        DeadFireCoral, "minecraft:dead_fire_coral" => DeadFireCoralState { waterlogged => true },
        DeadHornCoral, "minecraft:dead_horn_coral" => DeadHornCoralState { waterlogged => true },
        TubeCoral, "minecraft:tube_coral" => TubeCoralState { waterlogged => true },
        BrainCoral, "minecraft:brain_coral" => BrainCoralState { waterlogged => true },
        BubbleCoral, "minecraft:bubble_coral" => BubbleCoralState { waterlogged => true },
        FireCoral, "minecraft:fire_coral" => FireCoralState { waterlogged => true },
        HornCoral, "minecraft:horn_coral" => HornCoralState { waterlogged => true },
        DeadTubeCoralFan, "minecraft:dead_tube_coral_fan" => DeadTubeCoralFanState { waterlogged => true },
        DeadBrainCoralFan, "minecraft:dead_brain_coral_fan" => DeadBrainCoralFanState { waterlogged => true },
        DeadBubbleCoralFan, "minecraft:dead_bubble_coral_fan" => DeadBubbleCoralFanState { waterlogged => true },
        DeadFireCoralFan, "minecraft:dead_fire_coral_fan" => DeadFireCoralFanState { waterlogged => true },
        DeadHornCoralFan, "minecraft:dead_horn_coral_fan" => DeadHornCoralFanState { waterlogged => true },
        TubeCoralFan, "minecraft:tube_coral_fan" => TubeCoralFanState { waterlogged => true },
        BrainCoralFan, "minecraft:brain_coral_fan" => BrainCoralFanState { waterlogged => true },
        BubbleCoralFan, "minecraft:bubble_coral_fan" => BubbleCoralFanState { waterlogged => true },
        FireCoralFan, "minecraft:fire_coral_fan" => FireCoralFanState { waterlogged => true },
        HornCoralFan, "minecraft:horn_coral_fan" => HornCoralFanState { waterlogged => true },
        DeadTubeCoralWallFan, "minecraft:dead_tube_coral_wall_fan" => DeadTubeCoralWallFanState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => true
        },
        DeadBrainCoralWallFan, "minecraft:dead_brain_coral_wall_fan" => DeadBrainCoralWallFanState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => true
        },
        DeadBubbleCoralWallFan, "minecraft:dead_bubble_coral_wall_fan" => DeadBubbleCoralWallFanState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => true
        },
        DeadFireCoralWallFan, "minecraft:dead_fire_coral_wall_fan" => DeadFireCoralWallFanState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => true
        },
        DeadHornCoralWallFan, "minecraft:dead_horn_coral_wall_fan" => DeadHornCoralWallFanState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => true
        },
        TubeCoralWallFan, "minecraft:tube_coral_wall_fan" => TubeCoralWallFanState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => true
        },
        BrainCoralWallFan, "minecraft:brain_coral_wall_fan" => BrainCoralWallFanState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => true
        },
        BubbleCoralWallFan, "minecraft:bubble_coral_wall_fan" => BubbleCoralWallFanState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => true
        },
        FireCoralWallFan, "minecraft:fire_coral_wall_fan" => FireCoralWallFanState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => true
        },
        HornCoralWallFan, "minecraft:horn_coral_wall_fan" => HornCoralWallFanState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => true
        },
        SeaPickle, "minecraft:sea_pickle" => SeaPickleState {
            pickles => prop_types::ConstrainedInt(1),
            waterlogged => true
        },
        BlueIce, "minecraft:blue_ice",
        Conduit, "minecraft:conduit" => ConduitState { waterlogged => true },
        BambooSapling, "minecraft:bamboo_sapling",
        Bamboo, "minecraft:bamboo" => BambooState {
            age_1 => prop_types::ConstrainedInt(0),
            bamboo_leaves => prop_types::BambooLeaves::None,
            stage => prop_types::ConstrainedInt(0)
        },
        PottedBamboo, "minecraft:potted_bamboo",
        VoidAir, "minecraft:void_air",
        CaveAir, "minecraft:cave_air",
        BubbleColumn, "minecraft:bubble_column" => BubbleColumnState { drag => true },
        PolishedGraniteStairs, "minecraft:polished_granite_stairs" => PolishedGraniteStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        SmoothRedSandstoneStairs, "minecraft:smooth_red_sandstone_stairs" => SmoothRedSandstoneStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        MossyStoneBrickStairs, "minecraft:mossy_stone_brick_stairs" => MossyStoneBrickStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        PolishedDioriteStairs, "minecraft:polished_diorite_stairs" => PolishedDioriteStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        MossyCobblestoneStairs, "minecraft:mossy_cobblestone_stairs" => MossyCobblestoneStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        EndStoneBrickStairs, "minecraft:end_stone_brick_stairs" => EndStoneBrickStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        StoneStairs, "minecraft:stone_stairs" => StoneStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        SmoothSandstoneStairs, "minecraft:smooth_sandstone_stairs" => SmoothSandstoneStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        SmoothQuartzStairs, "minecraft:smooth_quartz_stairs" => SmoothQuartzStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        GraniteStairs, "minecraft:granite_stairs" => GraniteStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        AndesiteStairs, "minecraft:andesite_stairs" => AndesiteStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        RedNetherBrickStairs, "minecraft:red_nether_brick_stairs" => RedNetherBrickStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        PolishedAndesiteStairs, "minecraft:polished_andesite_stairs" => PolishedAndesiteStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        DioriteStairs, "minecraft:diorite_stairs" => DioriteStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        PolishedGraniteSlab, "minecraft:polished_granite_slab" => PolishedGraniteSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        SmoothRedSandstoneSlab, "minecraft:smooth_red_sandstone_slab" => SmoothRedSandstoneSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        MossyStoneBrickSlab, "minecraft:mossy_stone_brick_slab" => MossyStoneBrickSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        PolishedDioriteSlab, "minecraft:polished_diorite_slab" => PolishedDioriteSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        MossyCobblestoneSlab, "minecraft:mossy_cobblestone_slab" => MossyCobblestoneSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        EndStoneBrickSlab, "minecraft:end_stone_brick_slab" => EndStoneBrickSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        SmoothSandstoneSlab, "minecraft:smooth_sandstone_slab" => SmoothSandstoneSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        SmoothQuartzSlab, "minecraft:smooth_quartz_slab" => SmoothQuartzSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        GraniteSlab, "minecraft:granite_slab" => GraniteSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        AndesiteSlab, "minecraft:andesite_slab" => AndesiteSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        RedNetherBrickSlab, "minecraft:red_nether_brick_slab" => RedNetherBrickSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        PolishedAndesiteSlab, "minecraft:polished_andesite_slab" => PolishedAndesiteSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        DioriteSlab, "minecraft:diorite_slab" => DioriteSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        BrickWall, "minecraft:brick_wall" => BrickWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        PrismarineWall, "minecraft:prismarine_wall" => PrismarineWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        RedSandstoneWall, "minecraft:red_sandstone_wall" => RedSandstoneWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        MossyStoneBrickWall, "minecraft:mossy_stone_brick_wall" => MossyStoneBrickWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        GraniteWall, "minecraft:granite_wall" => GraniteWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        StoneBrickWall, "minecraft:stone_brick_wall" => StoneBrickWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        MudBrickWall, "minecraft:mud_brick_wall" => MudBrickWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        NetherBrickWall, "minecraft:nether_brick_wall" => NetherBrickWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        AndesiteWall, "minecraft:andesite_wall" => AndesiteWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        RedNetherBrickWall, "minecraft:red_nether_brick_wall" => RedNetherBrickWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        SandstoneWall, "minecraft:sandstone_wall" => SandstoneWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        EndStoneBrickWall, "minecraft:end_stone_brick_wall" => EndStoneBrickWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        DioriteWall, "minecraft:diorite_wall" => DioriteWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        Scaffolding, "minecraft:scaffolding" => ScaffoldingState {
            bottom => false,
            stability_distance => prop_types::ConstrainedInt(7),
            waterlogged => false
        },
        Loom, "minecraft:loom" => LoomState { horizontal_facing => prop_types::HorizontalFacing::North },
        Barrel, "minecraft:barrel" => BarrelState {
            facing => prop_types::Facing::North,
            open => false
        },
        Smoker, "minecraft:smoker" => SmokerState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            lit => false
        },
        BlastFurnace, "minecraft:blast_furnace" => BlastFurnaceState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            lit => false
        },
        CartographyTable, "minecraft:cartography_table",
        FletchingTable, "minecraft:fletching_table",
        Grindstone, "minecraft:grindstone" => GrindstoneState {
            attach_face => prop_types::AttachFace::Wall,
            horizontal_facing => prop_types::HorizontalFacing::North
        },
        Lectern, "minecraft:lectern" => LecternState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            has_book => false,
            powered => false
        },
        SmithingTable, "minecraft:smithing_table",
        Stonecutter, "minecraft:stonecutter" => StonecutterState { horizontal_facing => prop_types::HorizontalFacing::North },
        Bell, "minecraft:bell" => BellState {
            bell_attachment => prop_types::BellAttachment::Floor,
            horizontal_facing => prop_types::HorizontalFacing::North,
            powered => false
        },
        Lantern, "minecraft:lantern" => LanternState {
            hanging => false,
            waterlogged => false
        },
        SoulLantern, "minecraft:soul_lantern" => SoulLanternState {
            hanging => false,
            waterlogged => false
        },
        Campfire, "minecraft:campfire" => CampfireState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            lit => true,
            signal_fire => false,
            waterlogged => false
        },
        SoulCampfire, "minecraft:soul_campfire" => SoulCampfireState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            lit => true,
            signal_fire => false,
            waterlogged => false
        },
        SweetBerryBush, "minecraft:sweet_berry_bush" => SweetBerryBushState { age_3 => prop_types::ConstrainedInt(0) },
        WarpedStem, "minecraft:warped_stem" => WarpedStemState { axis => prop_types::Axis::Y },
        StrippedWarpedStem, "minecraft:stripped_warped_stem" => StrippedWarpedStemState { axis => prop_types::Axis::Y },
        WarpedHyphae, "minecraft:warped_hyphae" => WarpedHyphaeState { axis => prop_types::Axis::Y },
        StrippedWarpedHyphae, "minecraft:stripped_warped_hyphae" => StrippedWarpedHyphaeState { axis => prop_types::Axis::Y },
        WarpedNylium, "minecraft:warped_nylium",
        WarpedFungus, "minecraft:warped_fungus",
        WarpedWartBlock, "minecraft:warped_wart_block",
        WarpedRoots, "minecraft:warped_roots",
        NetherSprouts, "minecraft:nether_sprouts",
        CrimsonStem, "minecraft:crimson_stem" => CrimsonStemState { axis => prop_types::Axis::Y },
        StrippedCrimsonStem, "minecraft:stripped_crimson_stem" => StrippedCrimsonStemState { axis => prop_types::Axis::Y },
        CrimsonHyphae, "minecraft:crimson_hyphae" => CrimsonHyphaeState { axis => prop_types::Axis::Y },
        StrippedCrimsonHyphae, "minecraft:stripped_crimson_hyphae" => StrippedCrimsonHyphaeState { axis => prop_types::Axis::Y },
        CrimsonNylium, "minecraft:crimson_nylium",
        CrimsonFungus, "minecraft:crimson_fungus",
        Shroomlight, "minecraft:shroomlight",
        WeepingVines, "minecraft:weeping_vines" => WeepingVinesState { age_25 => prop_types::ConstrainedInt(0) },
        WeepingVinesPlant, "minecraft:weeping_vines_plant",
        TwistingVines, "minecraft:twisting_vines" => TwistingVinesState { age_25 => prop_types::ConstrainedInt(0) },
        TwistingVinesPlant, "minecraft:twisting_vines_plant",
        CrimsonRoots, "minecraft:crimson_roots",
        CrimsonPlanks, "minecraft:crimson_planks",
        WarpedPlanks, "minecraft:warped_planks",
        CrimsonSlab, "minecraft:crimson_slab" => CrimsonSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        WarpedSlab, "minecraft:warped_slab" => WarpedSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        CrimsonPressurePlate, "minecraft:crimson_pressure_plate" => CrimsonPressurePlateState { powered => false },
        WarpedPressurePlate, "minecraft:warped_pressure_plate" => WarpedPressurePlateState { powered => false },
        CrimsonFence, "minecraft:crimson_fence" => CrimsonFenceState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        WarpedFence, "minecraft:warped_fence" => WarpedFenceState {
            east => false,
            north => false,
            south => false,
            waterlogged => false,
            west => false
        },
        CrimsonTrapdoor, "minecraft:crimson_trapdoor" => CrimsonTrapdoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            open => false,
            powered => false,
            waterlogged => false
        },
        WarpedTrapdoor, "minecraft:warped_trapdoor" => WarpedTrapdoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            open => false,
            powered => false,
            waterlogged => false
        },
        CrimsonFenceGate, "minecraft:crimson_fence_gate" => CrimsonFenceGateState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            in_wall => false,
            open => false,
            powered => false
        },
        WarpedFenceGate, "minecraft:warped_fence_gate" => WarpedFenceGateState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            in_wall => false,
            open => false,
            powered => false
        },
        CrimsonStairs, "minecraft:crimson_stairs" => CrimsonStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        WarpedStairs, "minecraft:warped_stairs" => WarpedStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        CrimsonButton, "minecraft:crimson_button" => CrimsonButtonState {
            attach_face => prop_types::AttachFace::Wall,
            horizontal_facing => prop_types::HorizontalFacing::North,
            powered => false
        },
        WarpedButton, "minecraft:warped_button" => WarpedButtonState {
            attach_face => prop_types::AttachFace::Wall,
            horizontal_facing => prop_types::HorizontalFacing::North,
            powered => false
        },
        CrimsonDoor, "minecraft:crimson_door" => CrimsonDoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            double_block_half => prop_types::DoubleBlockHalf::Lower,
            door_hinge => prop_types::DoorHinge::Left,
            open => false,
            powered => false
        },
        WarpedDoor, "minecraft:warped_door" => WarpedDoorState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            double_block_half => prop_types::DoubleBlockHalf::Lower,
            door_hinge => prop_types::DoorHinge::Left,
            open => false,
            powered => false
        },
        CrimsonSign, "minecraft:crimson_sign" => CrimsonSignState {
            rotation_16 => prop_types::ConstrainedInt(0),
            waterlogged => false
        },
        WarpedSign, "minecraft:warped_sign" => WarpedSignState {
            rotation_16 => prop_types::ConstrainedInt(0),
            waterlogged => false
        },
        CrimsonWallSign, "minecraft:crimson_wall_sign" => CrimsonWallSignState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => false
        },
        WarpedWallSign, "minecraft:warped_wall_sign" => WarpedWallSignState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => false
        },
        StructureBlock, "minecraft:structure_block" => StructureBlockState { structureblock_mode => prop_types::StructureblockMode::Load },
        Jigsaw, "minecraft:jigsaw" => JigsawState { orientation => prop_types::Orientation::NorthUp },
        Composter, "minecraft:composter" => ComposterState { level_composter => prop_types::ConstrainedInt(0) },
        Target, "minecraft:target" => TargetState { power => prop_types::ConstrainedInt(0) },
        BeeNest, "minecraft:bee_nest" => BeeNestState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            level_honey => prop_types::ConstrainedInt(0)
        },
        Beehive, "minecraft:beehive" => BeehiveState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            level_honey => prop_types::ConstrainedInt(0)
        },
        HoneyBlock, "minecraft:honey_block",
        HoneycombBlock, "minecraft:honeycomb_block",
        NetheriteBlock, "minecraft:netherite_block",
        AncientDebris, "minecraft:ancient_debris",
        CryingObsidian, "minecraft:crying_obsidian",
        RespawnAnchor, "minecraft:respawn_anchor" => RespawnAnchorState { respawn_anchor_charges => prop_types::ConstrainedInt(0) },
        PottedCrimsonFungus, "minecraft:potted_crimson_fungus",
        PottedWarpedFungus, "minecraft:potted_warped_fungus",
        PottedCrimsonRoots, "minecraft:potted_crimson_roots",
        PottedWarpedRoots, "minecraft:potted_warped_roots",
        Lodestone, "minecraft:lodestone",
        Blackstone, "minecraft:blackstone",
        BlackstoneStairs, "minecraft:blackstone_stairs" => BlackstoneStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        BlackstoneWall, "minecraft:blackstone_wall" => BlackstoneWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        BlackstoneSlab, "minecraft:blackstone_slab" => BlackstoneSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        PolishedBlackstone, "minecraft:polished_blackstone",
        PolishedBlackstoneBricks, "minecraft:polished_blackstone_bricks",
        CrackedPolishedBlackstoneBricks, "minecraft:cracked_polished_blackstone_bricks",
        ChiseledPolishedBlackstone, "minecraft:chiseled_polished_blackstone",
        PolishedBlackstoneBrickSlab, "minecraft:polished_blackstone_brick_slab" => PolishedBlackstoneBrickSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        PolishedBlackstoneBrickStairs, "minecraft:polished_blackstone_brick_stairs" => PolishedBlackstoneBrickStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        PolishedBlackstoneBrickWall, "minecraft:polished_blackstone_brick_wall" => PolishedBlackstoneBrickWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        GildedBlackstone, "minecraft:gilded_blackstone",
        PolishedBlackstoneStairs, "minecraft:polished_blackstone_stairs" => PolishedBlackstoneStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        PolishedBlackstoneSlab, "minecraft:polished_blackstone_slab" => PolishedBlackstoneSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        PolishedBlackstonePressurePlate, "minecraft:polished_blackstone_pressure_plate" => PolishedBlackstonePressurePlateState { powered => false },
        PolishedBlackstoneButton, "minecraft:polished_blackstone_button" => PolishedBlackstoneButtonState {
            attach_face => prop_types::AttachFace::Wall,
            horizontal_facing => prop_types::HorizontalFacing::North,
            powered => false
        },
        PolishedBlackstoneWall, "minecraft:polished_blackstone_wall" => PolishedBlackstoneWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        ChiseledNetherBricks, "minecraft:chiseled_nether_bricks",
        CrackedNetherBricks, "minecraft:cracked_nether_bricks",
        QuartzBricks, "minecraft:quartz_bricks",
        Candle, "minecraft:candle" => CandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        WhiteCandle, "minecraft:white_candle" => WhiteCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        OrangeCandle, "minecraft:orange_candle" => OrangeCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        MagentaCandle, "minecraft:magenta_candle" => MagentaCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        LightBlueCandle, "minecraft:light_blue_candle" => LightBlueCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        YellowCandle, "minecraft:yellow_candle" => YellowCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        LimeCandle, "minecraft:lime_candle" => LimeCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        PinkCandle, "minecraft:pink_candle" => PinkCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        GrayCandle, "minecraft:gray_candle" => GrayCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        LightGrayCandle, "minecraft:light_gray_candle" => LightGrayCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        CyanCandle, "minecraft:cyan_candle" => CyanCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        PurpleCandle, "minecraft:purple_candle" => PurpleCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        BlueCandle, "minecraft:blue_candle" => BlueCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        BrownCandle, "minecraft:brown_candle" => BrownCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        GreenCandle, "minecraft:green_candle" => GreenCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        RedCandle, "minecraft:red_candle" => RedCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        BlackCandle, "minecraft:black_candle" => BlackCandleState {
            candles => prop_types::ConstrainedInt(1),
            lit => false,
            waterlogged => false
        },
        CandleCake, "minecraft:candle_cake" => CandleCakeState { lit => false },
        WhiteCandleCake, "minecraft:white_candle_cake" => WhiteCandleCakeState { lit => false },
        OrangeCandleCake, "minecraft:orange_candle_cake" => OrangeCandleCakeState { lit => false },
        MagentaCandleCake, "minecraft:magenta_candle_cake" => MagentaCandleCakeState { lit => false },
        LightBlueCandleCake, "minecraft:light_blue_candle_cake" => LightBlueCandleCakeState { lit => false },
        YellowCandleCake, "minecraft:yellow_candle_cake" => YellowCandleCakeState { lit => false },
        LimeCandleCake, "minecraft:lime_candle_cake" => LimeCandleCakeState { lit => false },
        PinkCandleCake, "minecraft:pink_candle_cake" => PinkCandleCakeState { lit => false },
        GrayCandleCake, "minecraft:gray_candle_cake" => GrayCandleCakeState { lit => false },
        LightGrayCandleCake, "minecraft:light_gray_candle_cake" => LightGrayCandleCakeState { lit => false },
        CyanCandleCake, "minecraft:cyan_candle_cake" => CyanCandleCakeState { lit => false },
        PurpleCandleCake, "minecraft:purple_candle_cake" => PurpleCandleCakeState { lit => false },
        BlueCandleCake, "minecraft:blue_candle_cake" => BlueCandleCakeState { lit => false },
        BrownCandleCake, "minecraft:brown_candle_cake" => BrownCandleCakeState { lit => false },
        GreenCandleCake, "minecraft:green_candle_cake" => GreenCandleCakeState { lit => false },
        RedCandleCake, "minecraft:red_candle_cake" => RedCandleCakeState { lit => false },
        BlackCandleCake, "minecraft:black_candle_cake" => BlackCandleCakeState { lit => false },
        AmethystBlock, "minecraft:amethyst_block",
        BuddingAmethyst, "minecraft:budding_amethyst",
        AmethystCluster, "minecraft:amethyst_cluster" => AmethystClusterState {
            facing => prop_types::Facing::Up,
            waterlogged => false
        },
        LargeAmethystBud, "minecraft:large_amethyst_bud" => LargeAmethystBudState {
            facing => prop_types::Facing::Up,
            waterlogged => false
        },
        MediumAmethystBud, "minecraft:medium_amethyst_bud" => MediumAmethystBudState {
            facing => prop_types::Facing::Up,
            waterlogged => false
        },
        SmallAmethystBud, "minecraft:small_amethyst_bud" => SmallAmethystBudState {
            facing => prop_types::Facing::Up,
            waterlogged => false
        },
        Tuff, "minecraft:tuff",
        Calcite, "minecraft:calcite",
        TintedGlass, "minecraft:tinted_glass",
        PowderSnow, "minecraft:powder_snow",
        SculkSensor, "minecraft:sculk_sensor" => SculkSensorState {
            power => prop_types::ConstrainedInt(0),
            sculk_sensor_phase => prop_types::SculkSensorPhase::Inactive,
            waterlogged => false
        },
        Sculk, "minecraft:sculk",
        SculkVein, "minecraft:sculk_vein" => SculkVeinState {
            down => false,
            east => false,
            north => false,
            south => false,
            up => false,
            waterlogged => false,
            west => false
        },
        SculkCatalyst, "minecraft:sculk_catalyst" => SculkCatalystState { bloom => false },
        SculkShrieker, "minecraft:sculk_shrieker" => SculkShriekerState {
            can_summon => false,
            shrieking => false,
            waterlogged => false
        },
        OxidizedCopper, "minecraft:oxidized_copper",
        WeatheredCopper, "minecraft:weathered_copper",
        ExposedCopper, "minecraft:exposed_copper",
        CopperBlock, "minecraft:copper_block",
        CopperOre, "minecraft:copper_ore",
        DeepslateCopperOre, "minecraft:deepslate_copper_ore",
        OxidizedCutCopper, "minecraft:oxidized_cut_copper",
        WeatheredCutCopper, "minecraft:weathered_cut_copper",
        ExposedCutCopper, "minecraft:exposed_cut_copper",
        CutCopper, "minecraft:cut_copper",
        OxidizedCutCopperStairs, "minecraft:oxidized_cut_copper_stairs" => OxidizedCutCopperStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        WeatheredCutCopperStairs, "minecraft:weathered_cut_copper_stairs" => WeatheredCutCopperStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        ExposedCutCopperStairs, "minecraft:exposed_cut_copper_stairs" => ExposedCutCopperStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        CutCopperStairs, "minecraft:cut_copper_stairs" => CutCopperStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        OxidizedCutCopperSlab, "minecraft:oxidized_cut_copper_slab" => OxidizedCutCopperSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        WeatheredCutCopperSlab, "minecraft:weathered_cut_copper_slab" => WeatheredCutCopperSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        ExposedCutCopperSlab, "minecraft:exposed_cut_copper_slab" => ExposedCutCopperSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        CutCopperSlab, "minecraft:cut_copper_slab" => CutCopperSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        WaxedCopperBlock, "minecraft:waxed_copper_block",
        WaxedWeatheredCopper, "minecraft:waxed_weathered_copper",
        WaxedExposedCopper, "minecraft:waxed_exposed_copper",
        WaxedOxidizedCopper, "minecraft:waxed_oxidized_copper",
        WaxedOxidizedCutCopper, "minecraft:waxed_oxidized_cut_copper",
        WaxedWeatheredCutCopper, "minecraft:waxed_weathered_cut_copper",
        WaxedExposedCutCopper, "minecraft:waxed_exposed_cut_copper",
        WaxedCutCopper, "minecraft:waxed_cut_copper",
        WaxedOxidizedCutCopperStairs, "minecraft:waxed_oxidized_cut_copper_stairs" => WaxedOxidizedCutCopperStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        WaxedWeatheredCutCopperStairs, "minecraft:waxed_weathered_cut_copper_stairs" => WaxedWeatheredCutCopperStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        WaxedExposedCutCopperStairs, "minecraft:waxed_exposed_cut_copper_stairs" => WaxedExposedCutCopperStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        WaxedCutCopperStairs, "minecraft:waxed_cut_copper_stairs" => WaxedCutCopperStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        WaxedOxidizedCutCopperSlab, "minecraft:waxed_oxidized_cut_copper_slab" => WaxedOxidizedCutCopperSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        WaxedWeatheredCutCopperSlab, "minecraft:waxed_weathered_cut_copper_slab" => WaxedWeatheredCutCopperSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        WaxedExposedCutCopperSlab, "minecraft:waxed_exposed_cut_copper_slab" => WaxedExposedCutCopperSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        WaxedCutCopperSlab, "minecraft:waxed_cut_copper_slab" => WaxedCutCopperSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        LightningRod, "minecraft:lightning_rod" => LightningRodState {
            facing => prop_types::Facing::Up,
            powered => false,
            waterlogged => false
        },
        PointedDripstone, "minecraft:pointed_dripstone" => PointedDripstoneState {
            dripstone_thickness => prop_types::DripstoneThickness::Tip,
            vertical_direction => prop_types::VerticalDirection::Up,
            waterlogged => false
        },
        DripstoneBlock, "minecraft:dripstone_block",
        CaveVines, "minecraft:cave_vines" => CaveVinesState {
            age_25 => prop_types::ConstrainedInt(0),
            berries => false
        },
        CaveVinesPlant, "minecraft:cave_vines_plant" => CaveVinesPlantState { berries => false },
        SporeBlossom, "minecraft:spore_blossom",
        Azalea, "minecraft:azalea",
        FloweringAzalea, "minecraft:flowering_azalea",
        MossCarpet, "minecraft:moss_carpet",
        MossBlock, "minecraft:moss_block",
        BigDripleaf, "minecraft:big_dripleaf" => BigDripleafState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            tilt => prop_types::Tilt::None,
            waterlogged => false
        },
        BigDripleafStem, "minecraft:big_dripleaf_stem" => BigDripleafStemState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            waterlogged => false
        },
        SmallDripleaf, "minecraft:small_dripleaf" => SmallDripleafState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            double_block_half => prop_types::DoubleBlockHalf::Lower,
            waterlogged => false
        },
        HangingRoots, "minecraft:hanging_roots" => HangingRootsState { waterlogged => false },
        RootedDirt, "minecraft:rooted_dirt",
        Mud, "minecraft:mud",
        Deepslate, "minecraft:deepslate" => DeepslateState { axis => prop_types::Axis::Y },
        CobbledDeepslate, "minecraft:cobbled_deepslate",
        CobbledDeepslateStairs, "minecraft:cobbled_deepslate_stairs" => CobbledDeepslateStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        CobbledDeepslateSlab, "minecraft:cobbled_deepslate_slab" => CobbledDeepslateSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        CobbledDeepslateWall, "minecraft:cobbled_deepslate_wall" => CobbledDeepslateWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        PolishedDeepslate, "minecraft:polished_deepslate",
        PolishedDeepslateStairs, "minecraft:polished_deepslate_stairs" => PolishedDeepslateStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        PolishedDeepslateSlab, "minecraft:polished_deepslate_slab" => PolishedDeepslateSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        PolishedDeepslateWall, "minecraft:polished_deepslate_wall" => PolishedDeepslateWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        DeepslateTiles, "minecraft:deepslate_tiles",
        DeepslateTileStairs, "minecraft:deepslate_tile_stairs" => DeepslateTileStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        DeepslateTileSlab, "minecraft:deepslate_tile_slab" => DeepslateTileSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        DeepslateTileWall, "minecraft:deepslate_tile_wall" => DeepslateTileWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        DeepslateBricks, "minecraft:deepslate_bricks",
        DeepslateBrickStairs, "minecraft:deepslate_brick_stairs" => DeepslateBrickStairsState {
            horizontal_facing => prop_types::HorizontalFacing::North,
            half => prop_types::Half::Bottom,
            stairs_shape => prop_types::StairsShape::Straight,
            waterlogged => false
        },
        DeepslateBrickSlab, "minecraft:deepslate_brick_slab" => DeepslateBrickSlabState {
            slab_type => prop_types::SlabType::Bottom,
            waterlogged => false
        },
        DeepslateBrickWall, "minecraft:deepslate_brick_wall" => DeepslateBrickWallState {
            east_wall => prop_types::WallSide::None,
            north_wall => prop_types::WallSide::None,
            south_wall => prop_types::WallSide::None,
            up => true,
            waterlogged => false,
            west_wall => prop_types::WallSide::None
        },
        ChiseledDeepslate, "minecraft:chiseled_deepslate",
        CrackedDeepslateBricks, "minecraft:cracked_deepslate_bricks",
        CrackedDeepslateTiles, "minecraft:cracked_deepslate_tiles",
        InfestedDeepslate, "minecraft:infested_deepslate" => InfestedDeepslateState { axis => prop_types::Axis::Y },
        SmoothBasalt, "minecraft:smooth_basalt",
        RawIronBlock, "minecraft:raw_iron_block",
        RawCopperBlock, "minecraft:raw_copper_block",
        RawGoldBlock, "minecraft:raw_gold_block",
        PottedAzaleaBush, "minecraft:potted_azalea_bush",
        PottedFloweringAzaleaBush, "minecraft:potted_flowering_azalea_bush",
        OchreFroglight, "minecraft:ochre_froglight" => OchreFroglightState { axis => prop_types::Axis::Y },
        VerdantFroglight, "minecraft:verdant_froglight" => VerdantFroglightState { axis => prop_types::Axis::Y },
        PearlescentFroglight, "minecraft:pearlescent_froglight" => PearlescentFroglightState { axis => prop_types::Axis::Y },
        Frogspawn, "minecraft:frogspawn",
        ReinforcedDeepslate, "minecraft:reinforced_deepslate"
    }
}
