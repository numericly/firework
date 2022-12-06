use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Biome {
	TheVoid,
	Plains,
	SunflowerPlains,
	SnowyPlains,
	IceSpikes,
	Desert,
	Swamp,
	MangroveSwamp,
	Forest,
	FlowerForest,
	BirchForest,
	DarkForest,
	OldGrowthBirchForest,
	OldGrowthPineTaiga,
	OldGrowthSpruceTaiga,
	Taiga,
	SnowyTaiga,
	Savanna,
	SavannaPlateau,
	WindsweptHills,
	WindsweptGravellyHills,
	WindsweptForest,
	WindsweptSavanna,
	Jungle,
	SparseJungle,
	BambooJungle,
	Badlands,
	ErodedBadlands,
	WoodedBadlands,
	Meadow,
	Grove,
	SnowySlopes,
	FrozenPeaks,
	JaggedPeaks,
	StonyPeaks,
	River,
	FrozenRiver,
	Beach,
	SnowyBeach,
	StonyShore,
	WarmOcean,
	LukewarmOcean,
	DeepLukewarmOcean,
	Ocean,
	DeepOcean,
	ColdOcean,
	DeepColdOcean,
	FrozenOcean,
	DeepFrozenOcean,
	MushroomFields,
	DripstoneCaves,
	LushCaves,
	DeepDark,
	NetherWastes,
	WarpedForest,
	CrimsonForest,
	SoulSandValley,
	BasaltDeltas,
	TheEnd,
	EndHighlands,
	EndMidlands,
	SmallEndIslands,
	EndBarrens,
}

pub fn create_biome_map() -> HashMap<Biome, usize> {
	let mut map = HashMap::new();
	map.insert(Biome::TheVoid, 0);
	map.insert(Biome::Plains, 1);
	map.insert(Biome::SunflowerPlains, 2);
	map.insert(Biome::SnowyPlains, 3);
	map.insert(Biome::IceSpikes, 4);
	map.insert(Biome::Desert, 5);
	map.insert(Biome::Swamp, 6);
	map.insert(Biome::MangroveSwamp, 7);
	map.insert(Biome::Forest, 8);
	map.insert(Biome::FlowerForest, 9);
	map.insert(Biome::BirchForest, 10);
	map.insert(Biome::DarkForest, 11);
	map.insert(Biome::OldGrowthBirchForest, 12);
	map.insert(Biome::OldGrowthPineTaiga, 13);
	map.insert(Biome::OldGrowthSpruceTaiga, 14);
	map.insert(Biome::Taiga, 15);
	map.insert(Biome::SnowyTaiga, 16);
	map.insert(Biome::Savanna, 17);
	map.insert(Biome::SavannaPlateau, 18);
	map.insert(Biome::WindsweptHills, 19);
	map.insert(Biome::WindsweptGravellyHills, 20);
	map.insert(Biome::WindsweptForest, 21);
	map.insert(Biome::WindsweptSavanna, 22);
	map.insert(Biome::Jungle, 23);
	map.insert(Biome::SparseJungle, 24);
	map.insert(Biome::BambooJungle, 25);
	map.insert(Biome::Badlands, 26);
	map.insert(Biome::ErodedBadlands, 27);
	map.insert(Biome::WoodedBadlands, 28);
	map.insert(Biome::Meadow, 29);
	map.insert(Biome::Grove, 30);
	map.insert(Biome::SnowySlopes, 31);
	map.insert(Biome::FrozenPeaks, 32);
	map.insert(Biome::JaggedPeaks, 33);
	map.insert(Biome::StonyPeaks, 34);
	map.insert(Biome::River, 35);
	map.insert(Biome::FrozenRiver, 36);
	map.insert(Biome::Beach, 37);
	map.insert(Biome::SnowyBeach, 38);
	map.insert(Biome::StonyShore, 39);
	map.insert(Biome::WarmOcean, 40);
	map.insert(Biome::LukewarmOcean, 41);
	map.insert(Biome::DeepLukewarmOcean, 42);
	map.insert(Biome::Ocean, 43);
	map.insert(Biome::DeepOcean, 44);
	map.insert(Biome::ColdOcean, 45);
	map.insert(Biome::DeepColdOcean, 46);
	map.insert(Biome::FrozenOcean, 47);
	map.insert(Biome::DeepFrozenOcean, 48);
	map.insert(Biome::MushroomFields, 49);
	map.insert(Biome::DripstoneCaves, 50);
	map.insert(Biome::LushCaves, 51);
	map.insert(Biome::DeepDark, 52);
	map.insert(Biome::NetherWastes, 53);
	map.insert(Biome::WarpedForest, 54);
	map.insert(Biome::CrimsonForest, 55);
	map.insert(Biome::SoulSandValley, 56);
	map.insert(Biome::BasaltDeltas, 57);
	map.insert(Biome::TheEnd, 58);
	map.insert(Biome::EndHighlands, 59);
	map.insert(Biome::EndMidlands, 60);
	map.insert(Biome::SmallEndIslands, 61);
	map.insert(Biome::EndBarrens, 62);
	map
}
pub fn visit_str<Err>(__value: &str) -> serde::__private::Result<Biome, Err>
    where
        Err: serde::de::Error,
    {
        const FIELDS: &'static [&'static str] = &[];
        match __value {
			"minecraft:the_void" => Ok(Biome::TheVoid),
			"minecraft:plains" => Ok(Biome::Plains),
			"minecraft:sunflower_plains" => Ok(Biome::SunflowerPlains),
			"minecraft:snowy_plains" => Ok(Biome::SnowyPlains),
			"minecraft:ice_spikes" => Ok(Biome::IceSpikes),
			"minecraft:desert" => Ok(Biome::Desert),
			"minecraft:swamp" => Ok(Biome::Swamp),
			"minecraft:mangrove_swamp" => Ok(Biome::MangroveSwamp),
			"minecraft:forest" => Ok(Biome::Forest),
			"minecraft:flower_forest" => Ok(Biome::FlowerForest),
			"minecraft:birch_forest" => Ok(Biome::BirchForest),
			"minecraft:dark_forest" => Ok(Biome::DarkForest),
			"minecraft:old_growth_birch_forest" => Ok(Biome::OldGrowthBirchForest),
			"minecraft:old_growth_pine_taiga" => Ok(Biome::OldGrowthPineTaiga),
			"minecraft:old_growth_spruce_taiga" => Ok(Biome::OldGrowthSpruceTaiga),
			"minecraft:taiga" => Ok(Biome::Taiga),
			"minecraft:snowy_taiga" => Ok(Biome::SnowyTaiga),
			"minecraft:savanna" => Ok(Biome::Savanna),
			"minecraft:savanna_plateau" => Ok(Biome::SavannaPlateau),
			"minecraft:windswept_hills" => Ok(Biome::WindsweptHills),
			"minecraft:windswept_gravelly_hills" => Ok(Biome::WindsweptGravellyHills),
			"minecraft:windswept_forest" => Ok(Biome::WindsweptForest),
			"minecraft:windswept_savanna" => Ok(Biome::WindsweptSavanna),
			"minecraft:jungle" => Ok(Biome::Jungle),
			"minecraft:sparse_jungle" => Ok(Biome::SparseJungle),
			"minecraft:bamboo_jungle" => Ok(Biome::BambooJungle),
			"minecraft:badlands" => Ok(Biome::Badlands),
			"minecraft:eroded_badlands" => Ok(Biome::ErodedBadlands),
			"minecraft:wooded_badlands" => Ok(Biome::WoodedBadlands),
			"minecraft:meadow" => Ok(Biome::Meadow),
			"minecraft:grove" => Ok(Biome::Grove),
			"minecraft:snowy_slopes" => Ok(Biome::SnowySlopes),
			"minecraft:frozen_peaks" => Ok(Biome::FrozenPeaks),
			"minecraft:jagged_peaks" => Ok(Biome::JaggedPeaks),
			"minecraft:stony_peaks" => Ok(Biome::StonyPeaks),
			"minecraft:river" => Ok(Biome::River),
			"minecraft:frozen_river" => Ok(Biome::FrozenRiver),
			"minecraft:beach" => Ok(Biome::Beach),
			"minecraft:snowy_beach" => Ok(Biome::SnowyBeach),
			"minecraft:stony_shore" => Ok(Biome::StonyShore),
			"minecraft:warm_ocean" => Ok(Biome::WarmOcean),
			"minecraft:lukewarm_ocean" => Ok(Biome::LukewarmOcean),
			"minecraft:deep_lukewarm_ocean" => Ok(Biome::DeepLukewarmOcean),
			"minecraft:ocean" => Ok(Biome::Ocean),
			"minecraft:deep_ocean" => Ok(Biome::DeepOcean),
			"minecraft:cold_ocean" => Ok(Biome::ColdOcean),
			"minecraft:deep_cold_ocean" => Ok(Biome::DeepColdOcean),
			"minecraft:frozen_ocean" => Ok(Biome::FrozenOcean),
			"minecraft:deep_frozen_ocean" => Ok(Biome::DeepFrozenOcean),
			"minecraft:mushroom_fields" => Ok(Biome::MushroomFields),
			"minecraft:dripstone_caves" => Ok(Biome::DripstoneCaves),
			"minecraft:lush_caves" => Ok(Biome::LushCaves),
			"minecraft:deep_dark" => Ok(Biome::DeepDark),
			"minecraft:nether_wastes" => Ok(Biome::NetherWastes),
			"minecraft:warped_forest" => Ok(Biome::WarpedForest),
			"minecraft:crimson_forest" => Ok(Biome::CrimsonForest),
			"minecraft:soul_sand_valley" => Ok(Biome::SoulSandValley),
			"minecraft:basalt_deltas" => Ok(Biome::BasaltDeltas),
			"minecraft:the_end" => Ok(Biome::TheEnd),
			"minecraft:end_highlands" => Ok(Biome::EndHighlands),
			"minecraft:end_midlands" => Ok(Biome::EndMidlands),
			"minecraft:small_end_islands" => Ok(Biome::SmallEndIslands),
			"minecraft:end_barrens" => Ok(Biome::EndBarrens),
			f => Err(serde::de::Error::unknown_field(f, FIELDS)),
        }
    }