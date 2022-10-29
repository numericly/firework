macro_rules! define_biomes {
    (
        $($biome:ident => $id:literal),*
        $(,)?
    ) => {
        use serde::Deserialize;
        use lazy_static::lazy_static;
        use std::collections::HashMap;

        #[derive(Debug, Hash, PartialEq, Eq)]
        pub enum Biomes {
            $(
                $biome
            ),*
        }

        impl<'de> Deserialize<'de> for Biomes {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                struct __Visitor;

                impl<'de> serde::de::Visitor<'de> for __Visitor {
                    type Value = Biomes;
                    fn expecting(
                        &self,
                        f: &mut serde::__private::Formatter,
                    ) -> serde::__private::fmt::Result {
                        f.write_str("biome string")
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> serde::__private::Result<Self::Value, __E>
                    where
                        __E: serde::de::Error,
                    {
                        const FIELDS: &'static [&'static str] = &[
                            $($id),*
                        ];
                        match __value {
                            $($id => Ok(Biomes::$biome)),*,
                            f => Err(serde::de::Error::unknown_field(f, FIELDS))
                        }
                    }
                }

                __deserializer.deserialize_str(__Visitor)
            }
        }

        lazy_static! {
            pub static ref BIOME_PALETTE: HashMap<Biomes, i32> = {
                let mut palette: HashMap<Biomes, i32> = HashMap::new();
                $(
                    palette.insert(Biomes::$biome, palette.len() as i32);
                )*
                palette
            };
        }

        impl super::Palette for Biomes {
            fn get_palette(state: &Biomes) -> i32 {
                BIOME_PALETTE.get(state).unwrap().clone()
            }
        }
    };
}

define_biomes! {
    TheVoid => "minecraft:the_void",
    Plains => "minecraft:plains",
    SunflowerPlains => "minecraft:sunflower_plains",
    SnowyPlains => "minecraft:snowy_plains",
    IceSpikes => "minecraft:ice_spikes",
    Desert => "minecraft:desert",
    Swamp => "minecraft:swamp",
    MangroveSwamp => "minecraft:mangrove_swamp",
    Forest => "minecraft:forest",
    FlowerForest => "minecraft:flower_forest",
    BirchForest => "minecraft:birch_forest",
    DarkForest => "minecraft:dark_forest",
    OldGrowthBirchForest => "minecraft:old_growth_birch_forest",
    OldGrowthPineTaiga => "minecraft:old_growth_pine_taiga",
    OldGrowthSpruceTaiga => "minecraft:old_growth_spruce_taiga",
    Taiga => "minecraft:taiga",
    SnowyTaiga => "minecraft:snowy_taiga",
    Savanna => "minecraft:savanna",
    SavannaPlateau => "minecraft:savanna_plateau",
    WindsweptHills => "minecraft:windswept_hills",
    WindsweptGravellyHills => "minecraft:windswept_gravelly_hills",
    WindsweptForest => "minecraft:windswept_forest",
    WindsweptSavanna => "minecraft:windswept_savanna",
    Jungle => "minecraft:jungle",
    SparseJungle => "minecraft:sparse_jungle",
    BambooJungle => "minecraft:bamboo_jungle",
    Badlands => "minecraft:badlands",
    ErodedBadlands => "minecraft:eroded_badlands",
    WoodedBadlands => "minecraft:wooded_badlands",
    Meadow => "minecraft:meadow",
    Grove => "minecraft:grove",
    SnowySlopes => "minecraft:snowy_slopes",
    FrozenPeaks => "minecraft:frozen_peaks",
    JaggedPeaks => "minecraft:jagged_peaks",
    StonyPeaks => "minecraft:stony_peaks",
    River => "minecraft:river",
    FrozenRiver => "minecraft:frozen_river",
    Beach => "minecraft:beach",
    SnowyBeach => "minecraft:snowy_beach",
    StonyShore => "minecraft:stony_shore",
    WarmOcean => "minecraft:warm_ocean",
    LukewarmOcean => "minecraft:lukewarm_ocean",
    DeepLukewarmOcean => "minecraft:deep_lukewarm_ocean",
    Ocean => "minecraft:ocean",
    DeepOcean => "minecraft:deep_ocean",
    ColdOcean => "minecraft:cold_ocean",
    DeepColdOcean => "minecraft:deep_cold_ocean",
    FrozenOcean => "minecraft:frozen_ocean",
    DeepFrozenOcean => "minecraft:deep_frozen_ocean",
    MushroomFields => "minecraft:mushroom_fields",
    DripstoneCaves => "minecraft:dripstone_caves",
    LushCaves => "minecraft:lush_caves",
    DeepDark => "minecraft:deep_dark",
    NetherWastes => "minecraft:nether_wastes",
    WarpedForest => "minecraft:warped_forest",
    CrimsonForest => "minecraft:crimson_forest",
    SoulSandValley => "minecraft:soul_sand_valley",
    BasaltDeltas => "minecraft:basalt_deltas",
    TheEnd => "minecraft:the_end",
    EndHighlands => "minecraft:end_highlands",
    EndMidlands => "minecraft:end_midlands",
    SmallEndIslands => "minecraft:small_end_islands",
    EndBarrens => "minecraft:end_barrens"
}
