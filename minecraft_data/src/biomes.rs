use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Biome {
	Badlands,
	BambooJungle,
	BasaltDeltas,
	Beach,
	BirchForest,
	ColdOcean,
	CrimsonForest,
	DarkForest,
	DeepColdOcean,
	DeepDark,
	DeepFrozenOcean,
	DeepLukewarmOcean,
	DeepOcean,
	Desert,
	DripstoneCaves,
	EndBarrens,
	EndHighlands,
	EndMidlands,
	ErodedBadlands,
	FlowerForest,
	Forest,
	FrozenOcean,
	FrozenPeaks,
	FrozenRiver,
	Grove,
	IceSpikes,
	JaggedPeaks,
	Jungle,
	LukewarmOcean,
	LushCaves,
	MangroveSwamp,
	Meadow,
	MushroomFields,
	NetherWastes,
	Ocean,
	OldGrowthBirchForest,
	OldGrowthPineTaiga,
	OldGrowthSpruceTaiga,
	Plains,
	River,
	Savanna,
	SavannaPlateau,
	SmallEndIslands,
	SnowyBeach,
	SnowyPlains,
	SnowySlopes,
	SnowyTaiga,
	SoulSandValley,
	SparseJungle,
	StonyPeaks,
	StonyShore,
	SunflowerPlains,
	Swamp,
	Taiga,
	TheEnd,
	TheVoid,
	WarmOcean,
	WarpedForest,
	WindsweptForest,
	WindsweptGravellyHills,
	WindsweptHills,
	WindsweptSavanna,
	WoodedBadlands,
}

pub fn create_biome_map() -> HashMap<Biome, usize> {
	let mut map = HashMap::new();
	map.insert(Biome::Badlands, 0);
	map.insert(Biome::BambooJungle, 1);
	map.insert(Biome::BasaltDeltas, 2);
	map.insert(Biome::Beach, 3);
	map.insert(Biome::BirchForest, 4);
	map.insert(Biome::ColdOcean, 5);
	map.insert(Biome::CrimsonForest, 6);
	map.insert(Biome::DarkForest, 7);
	map.insert(Biome::DeepColdOcean, 8);
	map.insert(Biome::DeepDark, 9);
	map.insert(Biome::DeepFrozenOcean, 10);
	map.insert(Biome::DeepLukewarmOcean, 11);
	map.insert(Biome::DeepOcean, 12);
	map.insert(Biome::Desert, 13);
	map.insert(Biome::DripstoneCaves, 14);
	map.insert(Biome::EndBarrens, 15);
	map.insert(Biome::EndHighlands, 16);
	map.insert(Biome::EndMidlands, 17);
	map.insert(Biome::ErodedBadlands, 18);
	map.insert(Biome::FlowerForest, 19);
	map.insert(Biome::Forest, 20);
	map.insert(Biome::FrozenOcean, 21);
	map.insert(Biome::FrozenPeaks, 22);
	map.insert(Biome::FrozenRiver, 23);
	map.insert(Biome::Grove, 24);
	map.insert(Biome::IceSpikes, 25);
	map.insert(Biome::JaggedPeaks, 26);
	map.insert(Biome::Jungle, 27);
	map.insert(Biome::LukewarmOcean, 28);
	map.insert(Biome::LushCaves, 29);
	map.insert(Biome::MangroveSwamp, 30);
	map.insert(Biome::Meadow, 31);
	map.insert(Biome::MushroomFields, 32);
	map.insert(Biome::NetherWastes, 33);
	map.insert(Biome::Ocean, 34);
	map.insert(Biome::OldGrowthBirchForest, 35);
	map.insert(Biome::OldGrowthPineTaiga, 36);
	map.insert(Biome::OldGrowthSpruceTaiga, 37);
	map.insert(Biome::Plains, 38);
	map.insert(Biome::River, 39);
	map.insert(Biome::Savanna, 40);
	map.insert(Biome::SavannaPlateau, 41);
	map.insert(Biome::SmallEndIslands, 42);
	map.insert(Biome::SnowyBeach, 43);
	map.insert(Biome::SnowyPlains, 44);
	map.insert(Biome::SnowySlopes, 45);
	map.insert(Biome::SnowyTaiga, 46);
	map.insert(Biome::SoulSandValley, 47);
	map.insert(Biome::SparseJungle, 48);
	map.insert(Biome::StonyPeaks, 49);
	map.insert(Biome::StonyShore, 50);
	map.insert(Biome::SunflowerPlains, 51);
	map.insert(Biome::Swamp, 52);
	map.insert(Biome::Taiga, 53);
	map.insert(Biome::TheEnd, 54);
	map.insert(Biome::TheVoid, 55);
	map.insert(Biome::WarmOcean, 56);
	map.insert(Biome::WarpedForest, 57);
	map.insert(Biome::WindsweptForest, 58);
	map.insert(Biome::WindsweptGravellyHills, 59);
	map.insert(Biome::WindsweptHills, 60);
	map.insert(Biome::WindsweptSavanna, 61);
	map.insert(Biome::WoodedBadlands, 62);
	map
}
pub fn visit_str<Err>(__value: &str) -> serde::__private::Result<Biome, Err>
    where
        Err: serde::de::Error,
    {
        const FIELDS: &'static [&'static str] = &[];
        match __value {
			"minecraft:badlands" => Ok(Biome::Badlands),
			"minecraft:bamboo_jungle" => Ok(Biome::BambooJungle),
			"minecraft:basalt_deltas" => Ok(Biome::BasaltDeltas),
			"minecraft:beach" => Ok(Biome::Beach),
			"minecraft:birch_forest" => Ok(Biome::BirchForest),
			"minecraft:cold_ocean" => Ok(Biome::ColdOcean),
			"minecraft:crimson_forest" => Ok(Biome::CrimsonForest),
			"minecraft:dark_forest" => Ok(Biome::DarkForest),
			"minecraft:deep_cold_ocean" => Ok(Biome::DeepColdOcean),
			"minecraft:deep_dark" => Ok(Biome::DeepDark),
			"minecraft:deep_frozen_ocean" => Ok(Biome::DeepFrozenOcean),
			"minecraft:deep_lukewarm_ocean" => Ok(Biome::DeepLukewarmOcean),
			"minecraft:deep_ocean" => Ok(Biome::DeepOcean),
			"minecraft:desert" => Ok(Biome::Desert),
			"minecraft:dripstone_caves" => Ok(Biome::DripstoneCaves),
			"minecraft:end_barrens" => Ok(Biome::EndBarrens),
			"minecraft:end_highlands" => Ok(Biome::EndHighlands),
			"minecraft:end_midlands" => Ok(Biome::EndMidlands),
			"minecraft:eroded_badlands" => Ok(Biome::ErodedBadlands),
			"minecraft:flower_forest" => Ok(Biome::FlowerForest),
			"minecraft:forest" => Ok(Biome::Forest),
			"minecraft:frozen_ocean" => Ok(Biome::FrozenOcean),
			"minecraft:frozen_peaks" => Ok(Biome::FrozenPeaks),
			"minecraft:frozen_river" => Ok(Biome::FrozenRiver),
			"minecraft:grove" => Ok(Biome::Grove),
			"minecraft:ice_spikes" => Ok(Biome::IceSpikes),
			"minecraft:jagged_peaks" => Ok(Biome::JaggedPeaks),
			"minecraft:jungle" => Ok(Biome::Jungle),
			"minecraft:lukewarm_ocean" => Ok(Biome::LukewarmOcean),
			"minecraft:lush_caves" => Ok(Biome::LushCaves),
			"minecraft:mangrove_swamp" => Ok(Biome::MangroveSwamp),
			"minecraft:meadow" => Ok(Biome::Meadow),
			"minecraft:mushroom_fields" => Ok(Biome::MushroomFields),
			"minecraft:nether_wastes" => Ok(Biome::NetherWastes),
			"minecraft:ocean" => Ok(Biome::Ocean),
			"minecraft:old_growth_birch_forest" => Ok(Biome::OldGrowthBirchForest),
			"minecraft:old_growth_pine_taiga" => Ok(Biome::OldGrowthPineTaiga),
			"minecraft:old_growth_spruce_taiga" => Ok(Biome::OldGrowthSpruceTaiga),
			"minecraft:plains" => Ok(Biome::Plains),
			"minecraft:river" => Ok(Biome::River),
			"minecraft:savanna" => Ok(Biome::Savanna),
			"minecraft:savanna_plateau" => Ok(Biome::SavannaPlateau),
			"minecraft:small_end_islands" => Ok(Biome::SmallEndIslands),
			"minecraft:snowy_beach" => Ok(Biome::SnowyBeach),
			"minecraft:snowy_plains" => Ok(Biome::SnowyPlains),
			"minecraft:snowy_slopes" => Ok(Biome::SnowySlopes),
			"minecraft:snowy_taiga" => Ok(Biome::SnowyTaiga),
			"minecraft:soul_sand_valley" => Ok(Biome::SoulSandValley),
			"minecraft:sparse_jungle" => Ok(Biome::SparseJungle),
			"minecraft:stony_peaks" => Ok(Biome::StonyPeaks),
			"minecraft:stony_shore" => Ok(Biome::StonyShore),
			"minecraft:sunflower_plains" => Ok(Biome::SunflowerPlains),
			"minecraft:swamp" => Ok(Biome::Swamp),
			"minecraft:taiga" => Ok(Biome::Taiga),
			"minecraft:the_end" => Ok(Biome::TheEnd),
			"minecraft:the_void" => Ok(Biome::TheVoid),
			"minecraft:warm_ocean" => Ok(Biome::WarmOcean),
			"minecraft:warped_forest" => Ok(Biome::WarpedForest),
			"minecraft:windswept_forest" => Ok(Biome::WindsweptForest),
			"minecraft:windswept_gravelly_hills" => Ok(Biome::WindsweptGravellyHills),
			"minecraft:windswept_hills" => Ok(Biome::WindsweptHills),
			"minecraft:windswept_savanna" => Ok(Biome::WindsweptSavanna),
			"minecraft:wooded_badlands" => Ok(Biome::WoodedBadlands),
			f => Err(serde::de::Error::unknown_field(f, FIELDS)),
        }
    }