pub trait Item {
    const ID: i32;
    const NAME: &'static str;
    const DISPLAY_NAME: &'static str;
    const STACK_SIZE: i32 = 64i32;
}
pub struct r#Air;
impl Item for r#Air {
    const ID: i32 = 0i32;
    const NAME: &'static str = "air";
    const DISPLAY_NAME: &'static str = "Air";
}
pub struct r#Stone;
impl Item for r#Stone {
    const ID: i32 = 1i32;
    const NAME: &'static str = "stone";
    const DISPLAY_NAME: &'static str = "Stone";
}
pub struct r#Granite;
impl Item for r#Granite {
    const ID: i32 = 2i32;
    const NAME: &'static str = "granite";
    const DISPLAY_NAME: &'static str = "Granite";
}
pub struct r#PolishedGranite;
impl Item for r#PolishedGranite {
    const ID: i32 = 3i32;
    const NAME: &'static str = "polished_granite";
    const DISPLAY_NAME: &'static str = "Polished Granite";
}
pub struct r#Diorite;
impl Item for r#Diorite {
    const ID: i32 = 4i32;
    const NAME: &'static str = "diorite";
    const DISPLAY_NAME: &'static str = "Diorite";
}
pub struct r#PolishedDiorite;
impl Item for r#PolishedDiorite {
    const ID: i32 = 5i32;
    const NAME: &'static str = "polished_diorite";
    const DISPLAY_NAME: &'static str = "Polished Diorite";
}
pub struct r#Andesite;
impl Item for r#Andesite {
    const ID: i32 = 6i32;
    const NAME: &'static str = "andesite";
    const DISPLAY_NAME: &'static str = "Andesite";
}
pub struct r#PolishedAndesite;
impl Item for r#PolishedAndesite {
    const ID: i32 = 7i32;
    const NAME: &'static str = "polished_andesite";
    const DISPLAY_NAME: &'static str = "Polished Andesite";
}
pub struct r#Deepslate;
impl Item for r#Deepslate {
    const ID: i32 = 8i32;
    const NAME: &'static str = "deepslate";
    const DISPLAY_NAME: &'static str = "Deepslate";
}
pub struct r#CobbledDeepslate;
impl Item for r#CobbledDeepslate {
    const ID: i32 = 9i32;
    const NAME: &'static str = "cobbled_deepslate";
    const DISPLAY_NAME: &'static str = "Cobbled Deepslate";
}
pub struct r#PolishedDeepslate;
impl Item for r#PolishedDeepslate {
    const ID: i32 = 10i32;
    const NAME: &'static str = "polished_deepslate";
    const DISPLAY_NAME: &'static str = "Polished Deepslate";
}
pub struct r#Calcite;
impl Item for r#Calcite {
    const ID: i32 = 11i32;
    const NAME: &'static str = "calcite";
    const DISPLAY_NAME: &'static str = "Calcite";
}
pub struct r#Tuff;
impl Item for r#Tuff {
    const ID: i32 = 12i32;
    const NAME: &'static str = "tuff";
    const DISPLAY_NAME: &'static str = "Tuff";
}
pub struct r#DripstoneBlock;
impl Item for r#DripstoneBlock {
    const ID: i32 = 13i32;
    const NAME: &'static str = "dripstone_block";
    const DISPLAY_NAME: &'static str = "Dripstone Block";
}
pub struct r#GrassBlock;
impl Item for r#GrassBlock {
    const ID: i32 = 14i32;
    const NAME: &'static str = "grass_block";
    const DISPLAY_NAME: &'static str = "Grass Block";
}
pub struct r#Dirt;
impl Item for r#Dirt {
    const ID: i32 = 15i32;
    const NAME: &'static str = "dirt";
    const DISPLAY_NAME: &'static str = "Dirt";
}
pub struct r#CoarseDirt;
impl Item for r#CoarseDirt {
    const ID: i32 = 16i32;
    const NAME: &'static str = "coarse_dirt";
    const DISPLAY_NAME: &'static str = "Coarse Dirt";
}
pub struct r#Podzol;
impl Item for r#Podzol {
    const ID: i32 = 17i32;
    const NAME: &'static str = "podzol";
    const DISPLAY_NAME: &'static str = "Podzol";
}
pub struct r#RootedDirt;
impl Item for r#RootedDirt {
    const ID: i32 = 18i32;
    const NAME: &'static str = "rooted_dirt";
    const DISPLAY_NAME: &'static str = "Rooted Dirt";
}
pub struct r#Mud;
impl Item for r#Mud {
    const ID: i32 = 19i32;
    const NAME: &'static str = "mud";
    const DISPLAY_NAME: &'static str = "Mud";
}
pub struct r#CrimsonNylium;
impl Item for r#CrimsonNylium {
    const ID: i32 = 20i32;
    const NAME: &'static str = "crimson_nylium";
    const DISPLAY_NAME: &'static str = "Crimson Nylium";
}
pub struct r#WarpedNylium;
impl Item for r#WarpedNylium {
    const ID: i32 = 21i32;
    const NAME: &'static str = "warped_nylium";
    const DISPLAY_NAME: &'static str = "Warped Nylium";
}
pub struct r#Cobblestone;
impl Item for r#Cobblestone {
    const ID: i32 = 22i32;
    const NAME: &'static str = "cobblestone";
    const DISPLAY_NAME: &'static str = "Cobblestone";
}
pub struct r#OakPlanks;
impl Item for r#OakPlanks {
    const ID: i32 = 23i32;
    const NAME: &'static str = "oak_planks";
    const DISPLAY_NAME: &'static str = "Oak Planks";
}
pub struct r#SprucePlanks;
impl Item for r#SprucePlanks {
    const ID: i32 = 24i32;
    const NAME: &'static str = "spruce_planks";
    const DISPLAY_NAME: &'static str = "Spruce Planks";
}
pub struct r#BirchPlanks;
impl Item for r#BirchPlanks {
    const ID: i32 = 25i32;
    const NAME: &'static str = "birch_planks";
    const DISPLAY_NAME: &'static str = "Birch Planks";
}
pub struct r#JunglePlanks;
impl Item for r#JunglePlanks {
    const ID: i32 = 26i32;
    const NAME: &'static str = "jungle_planks";
    const DISPLAY_NAME: &'static str = "Jungle Planks";
}
pub struct r#AcaciaPlanks;
impl Item for r#AcaciaPlanks {
    const ID: i32 = 27i32;
    const NAME: &'static str = "acacia_planks";
    const DISPLAY_NAME: &'static str = "Acacia Planks";
}
pub struct r#DarkOakPlanks;
impl Item for r#DarkOakPlanks {
    const ID: i32 = 28i32;
    const NAME: &'static str = "dark_oak_planks";
    const DISPLAY_NAME: &'static str = "Dark Oak Planks";
}
pub struct r#MangrovePlanks;
impl Item for r#MangrovePlanks {
    const ID: i32 = 29i32;
    const NAME: &'static str = "mangrove_planks";
    const DISPLAY_NAME: &'static str = "Mangrove Planks";
}
pub struct r#BambooPlanks;
impl Item for r#BambooPlanks {
    const ID: i32 = 30i32;
    const NAME: &'static str = "bamboo_planks";
    const DISPLAY_NAME: &'static str = "Bamboo Planks";
}
pub struct r#CrimsonPlanks;
impl Item for r#CrimsonPlanks {
    const ID: i32 = 31i32;
    const NAME: &'static str = "crimson_planks";
    const DISPLAY_NAME: &'static str = "Crimson Planks";
}
pub struct r#WarpedPlanks;
impl Item for r#WarpedPlanks {
    const ID: i32 = 32i32;
    const NAME: &'static str = "warped_planks";
    const DISPLAY_NAME: &'static str = "Warped Planks";
}
pub struct r#BambooMosaic;
impl Item for r#BambooMosaic {
    const ID: i32 = 33i32;
    const NAME: &'static str = "bamboo_mosaic";
    const DISPLAY_NAME: &'static str = "Bamboo Mosaic";
}
pub struct r#OakSapling;
impl Item for r#OakSapling {
    const ID: i32 = 34i32;
    const NAME: &'static str = "oak_sapling";
    const DISPLAY_NAME: &'static str = "Oak Sapling";
}
pub struct r#SpruceSapling;
impl Item for r#SpruceSapling {
    const ID: i32 = 35i32;
    const NAME: &'static str = "spruce_sapling";
    const DISPLAY_NAME: &'static str = "Spruce Sapling";
}
pub struct r#BirchSapling;
impl Item for r#BirchSapling {
    const ID: i32 = 36i32;
    const NAME: &'static str = "birch_sapling";
    const DISPLAY_NAME: &'static str = "Birch Sapling";
}
pub struct r#JungleSapling;
impl Item for r#JungleSapling {
    const ID: i32 = 37i32;
    const NAME: &'static str = "jungle_sapling";
    const DISPLAY_NAME: &'static str = "Jungle Sapling";
}
pub struct r#AcaciaSapling;
impl Item for r#AcaciaSapling {
    const ID: i32 = 38i32;
    const NAME: &'static str = "acacia_sapling";
    const DISPLAY_NAME: &'static str = "Acacia Sapling";
}
pub struct r#DarkOakSapling;
impl Item for r#DarkOakSapling {
    const ID: i32 = 39i32;
    const NAME: &'static str = "dark_oak_sapling";
    const DISPLAY_NAME: &'static str = "Dark Oak Sapling";
}
pub struct r#MangrovePropagule;
impl Item for r#MangrovePropagule {
    const ID: i32 = 40i32;
    const NAME: &'static str = "mangrove_propagule";
    const DISPLAY_NAME: &'static str = "Mangrove Propagule";
}
pub struct r#Bedrock;
impl Item for r#Bedrock {
    const ID: i32 = 41i32;
    const NAME: &'static str = "bedrock";
    const DISPLAY_NAME: &'static str = "Bedrock";
}
pub struct r#Sand;
impl Item for r#Sand {
    const ID: i32 = 42i32;
    const NAME: &'static str = "sand";
    const DISPLAY_NAME: &'static str = "Sand";
}
pub struct r#RedSand;
impl Item for r#RedSand {
    const ID: i32 = 43i32;
    const NAME: &'static str = "red_sand";
    const DISPLAY_NAME: &'static str = "Red Sand";
}
pub struct r#Gravel;
impl Item for r#Gravel {
    const ID: i32 = 44i32;
    const NAME: &'static str = "gravel";
    const DISPLAY_NAME: &'static str = "Gravel";
}
pub struct r#CoalOre;
impl Item for r#CoalOre {
    const ID: i32 = 45i32;
    const NAME: &'static str = "coal_ore";
    const DISPLAY_NAME: &'static str = "Coal Ore";
}
pub struct r#DeepslateCoalOre;
impl Item for r#DeepslateCoalOre {
    const ID: i32 = 46i32;
    const NAME: &'static str = "deepslate_coal_ore";
    const DISPLAY_NAME: &'static str = "Deepslate Coal Ore";
}
pub struct r#IronOre;
impl Item for r#IronOre {
    const ID: i32 = 47i32;
    const NAME: &'static str = "iron_ore";
    const DISPLAY_NAME: &'static str = "Iron Ore";
}
pub struct r#DeepslateIronOre;
impl Item for r#DeepslateIronOre {
    const ID: i32 = 48i32;
    const NAME: &'static str = "deepslate_iron_ore";
    const DISPLAY_NAME: &'static str = "Deepslate Iron Ore";
}
pub struct r#CopperOre;
impl Item for r#CopperOre {
    const ID: i32 = 49i32;
    const NAME: &'static str = "copper_ore";
    const DISPLAY_NAME: &'static str = "Copper Ore";
}
pub struct r#DeepslateCopperOre;
impl Item for r#DeepslateCopperOre {
    const ID: i32 = 50i32;
    const NAME: &'static str = "deepslate_copper_ore";
    const DISPLAY_NAME: &'static str = "Deepslate Copper Ore";
}
pub struct r#GoldOre;
impl Item for r#GoldOre {
    const ID: i32 = 51i32;
    const NAME: &'static str = "gold_ore";
    const DISPLAY_NAME: &'static str = "Gold Ore";
}
pub struct r#DeepslateGoldOre;
impl Item for r#DeepslateGoldOre {
    const ID: i32 = 52i32;
    const NAME: &'static str = "deepslate_gold_ore";
    const DISPLAY_NAME: &'static str = "Deepslate Gold Ore";
}
pub struct r#RedstoneOre;
impl Item for r#RedstoneOre {
    const ID: i32 = 53i32;
    const NAME: &'static str = "redstone_ore";
    const DISPLAY_NAME: &'static str = "Redstone Ore";
}
pub struct r#DeepslateRedstoneOre;
impl Item for r#DeepslateRedstoneOre {
    const ID: i32 = 54i32;
    const NAME: &'static str = "deepslate_redstone_ore";
    const DISPLAY_NAME: &'static str = "Deepslate Redstone Ore";
}
pub struct r#EmeraldOre;
impl Item for r#EmeraldOre {
    const ID: i32 = 55i32;
    const NAME: &'static str = "emerald_ore";
    const DISPLAY_NAME: &'static str = "Emerald Ore";
}
pub struct r#DeepslateEmeraldOre;
impl Item for r#DeepslateEmeraldOre {
    const ID: i32 = 56i32;
    const NAME: &'static str = "deepslate_emerald_ore";
    const DISPLAY_NAME: &'static str = "Deepslate Emerald Ore";
}
pub struct r#LapisOre;
impl Item for r#LapisOre {
    const ID: i32 = 57i32;
    const NAME: &'static str = "lapis_ore";
    const DISPLAY_NAME: &'static str = "Lapis Lazuli Ore";
}
pub struct r#DeepslateLapisOre;
impl Item for r#DeepslateLapisOre {
    const ID: i32 = 58i32;
    const NAME: &'static str = "deepslate_lapis_ore";
    const DISPLAY_NAME: &'static str = "Deepslate Lapis Lazuli Ore";
}
pub struct r#DiamondOre;
impl Item for r#DiamondOre {
    const ID: i32 = 59i32;
    const NAME: &'static str = "diamond_ore";
    const DISPLAY_NAME: &'static str = "Diamond Ore";
}
pub struct r#DeepslateDiamondOre;
impl Item for r#DeepslateDiamondOre {
    const ID: i32 = 60i32;
    const NAME: &'static str = "deepslate_diamond_ore";
    const DISPLAY_NAME: &'static str = "Deepslate Diamond Ore";
}
pub struct r#NetherGoldOre;
impl Item for r#NetherGoldOre {
    const ID: i32 = 61i32;
    const NAME: &'static str = "nether_gold_ore";
    const DISPLAY_NAME: &'static str = "Nether Gold Ore";
}
pub struct r#NetherQuartzOre;
impl Item for r#NetherQuartzOre {
    const ID: i32 = 62i32;
    const NAME: &'static str = "nether_quartz_ore";
    const DISPLAY_NAME: &'static str = "Nether Quartz Ore";
}
pub struct r#AncientDebris;
impl Item for r#AncientDebris {
    const ID: i32 = 63i32;
    const NAME: &'static str = "ancient_debris";
    const DISPLAY_NAME: &'static str = "Ancient Debris";
}
pub struct r#CoalBlock;
impl Item for r#CoalBlock {
    const ID: i32 = 64i32;
    const NAME: &'static str = "coal_block";
    const DISPLAY_NAME: &'static str = "Block of Coal";
}
pub struct r#RawIronBlock;
impl Item for r#RawIronBlock {
    const ID: i32 = 65i32;
    const NAME: &'static str = "raw_iron_block";
    const DISPLAY_NAME: &'static str = "Block of Raw Iron";
}
pub struct r#RawCopperBlock;
impl Item for r#RawCopperBlock {
    const ID: i32 = 66i32;
    const NAME: &'static str = "raw_copper_block";
    const DISPLAY_NAME: &'static str = "Block of Raw Copper";
}
pub struct r#RawGoldBlock;
impl Item for r#RawGoldBlock {
    const ID: i32 = 67i32;
    const NAME: &'static str = "raw_gold_block";
    const DISPLAY_NAME: &'static str = "Block of Raw Gold";
}
pub struct r#AmethystBlock;
impl Item for r#AmethystBlock {
    const ID: i32 = 68i32;
    const NAME: &'static str = "amethyst_block";
    const DISPLAY_NAME: &'static str = "Block of Amethyst";
}
pub struct r#BuddingAmethyst;
impl Item for r#BuddingAmethyst {
    const ID: i32 = 69i32;
    const NAME: &'static str = "budding_amethyst";
    const DISPLAY_NAME: &'static str = "Budding Amethyst";
}
pub struct r#IronBlock;
impl Item for r#IronBlock {
    const ID: i32 = 70i32;
    const NAME: &'static str = "iron_block";
    const DISPLAY_NAME: &'static str = "Block of Iron";
}
pub struct r#CopperBlock;
impl Item for r#CopperBlock {
    const ID: i32 = 71i32;
    const NAME: &'static str = "copper_block";
    const DISPLAY_NAME: &'static str = "Block of Copper";
}
pub struct r#GoldBlock;
impl Item for r#GoldBlock {
    const ID: i32 = 72i32;
    const NAME: &'static str = "gold_block";
    const DISPLAY_NAME: &'static str = "Block of Gold";
}
pub struct r#DiamondBlock;
impl Item for r#DiamondBlock {
    const ID: i32 = 73i32;
    const NAME: &'static str = "diamond_block";
    const DISPLAY_NAME: &'static str = "Block of Diamond";
}
pub struct r#NetheriteBlock;
impl Item for r#NetheriteBlock {
    const ID: i32 = 74i32;
    const NAME: &'static str = "netherite_block";
    const DISPLAY_NAME: &'static str = "Block of Netherite";
}
pub struct r#ExposedCopper;
impl Item for r#ExposedCopper {
    const ID: i32 = 75i32;
    const NAME: &'static str = "exposed_copper";
    const DISPLAY_NAME: &'static str = "Exposed Copper";
}
pub struct r#WeatheredCopper;
impl Item for r#WeatheredCopper {
    const ID: i32 = 76i32;
    const NAME: &'static str = "weathered_copper";
    const DISPLAY_NAME: &'static str = "Weathered Copper";
}
pub struct r#OxidizedCopper;
impl Item for r#OxidizedCopper {
    const ID: i32 = 77i32;
    const NAME: &'static str = "oxidized_copper";
    const DISPLAY_NAME: &'static str = "Oxidized Copper";
}
pub struct r#CutCopper;
impl Item for r#CutCopper {
    const ID: i32 = 78i32;
    const NAME: &'static str = "cut_copper";
    const DISPLAY_NAME: &'static str = "Cut Copper";
}
pub struct r#ExposedCutCopper;
impl Item for r#ExposedCutCopper {
    const ID: i32 = 79i32;
    const NAME: &'static str = "exposed_cut_copper";
    const DISPLAY_NAME: &'static str = "Exposed Cut Copper";
}
pub struct r#WeatheredCutCopper;
impl Item for r#WeatheredCutCopper {
    const ID: i32 = 80i32;
    const NAME: &'static str = "weathered_cut_copper";
    const DISPLAY_NAME: &'static str = "Weathered Cut Copper";
}
pub struct r#OxidizedCutCopper;
impl Item for r#OxidizedCutCopper {
    const ID: i32 = 81i32;
    const NAME: &'static str = "oxidized_cut_copper";
    const DISPLAY_NAME: &'static str = "Oxidized Cut Copper";
}
pub struct r#CutCopperStairs;
impl Item for r#CutCopperStairs {
    const ID: i32 = 82i32;
    const NAME: &'static str = "cut_copper_stairs";
    const DISPLAY_NAME: &'static str = "Cut Copper Stairs";
}
pub struct r#ExposedCutCopperStairs;
impl Item for r#ExposedCutCopperStairs {
    const ID: i32 = 83i32;
    const NAME: &'static str = "exposed_cut_copper_stairs";
    const DISPLAY_NAME: &'static str = "Exposed Cut Copper Stairs";
}
pub struct r#WeatheredCutCopperStairs;
impl Item for r#WeatheredCutCopperStairs {
    const ID: i32 = 84i32;
    const NAME: &'static str = "weathered_cut_copper_stairs";
    const DISPLAY_NAME: &'static str = "Weathered Cut Copper Stairs";
}
pub struct r#OxidizedCutCopperStairs;
impl Item for r#OxidizedCutCopperStairs {
    const ID: i32 = 85i32;
    const NAME: &'static str = "oxidized_cut_copper_stairs";
    const DISPLAY_NAME: &'static str = "Oxidized Cut Copper Stairs";
}
pub struct r#CutCopperSlab;
impl Item for r#CutCopperSlab {
    const ID: i32 = 86i32;
    const NAME: &'static str = "cut_copper_slab";
    const DISPLAY_NAME: &'static str = "Cut Copper Slab";
}
pub struct r#ExposedCutCopperSlab;
impl Item for r#ExposedCutCopperSlab {
    const ID: i32 = 87i32;
    const NAME: &'static str = "exposed_cut_copper_slab";
    const DISPLAY_NAME: &'static str = "Exposed Cut Copper Slab";
}
pub struct r#WeatheredCutCopperSlab;
impl Item for r#WeatheredCutCopperSlab {
    const ID: i32 = 88i32;
    const NAME: &'static str = "weathered_cut_copper_slab";
    const DISPLAY_NAME: &'static str = "Weathered Cut Copper Slab";
}
pub struct r#OxidizedCutCopperSlab;
impl Item for r#OxidizedCutCopperSlab {
    const ID: i32 = 89i32;
    const NAME: &'static str = "oxidized_cut_copper_slab";
    const DISPLAY_NAME: &'static str = "Oxidized Cut Copper Slab";
}
pub struct r#WaxedCopperBlock;
impl Item for r#WaxedCopperBlock {
    const ID: i32 = 90i32;
    const NAME: &'static str = "waxed_copper_block";
    const DISPLAY_NAME: &'static str = "Waxed Block of Copper";
}
pub struct r#WaxedExposedCopper;
impl Item for r#WaxedExposedCopper {
    const ID: i32 = 91i32;
    const NAME: &'static str = "waxed_exposed_copper";
    const DISPLAY_NAME: &'static str = "Waxed Exposed Copper";
}
pub struct r#WaxedWeatheredCopper;
impl Item for r#WaxedWeatheredCopper {
    const ID: i32 = 92i32;
    const NAME: &'static str = "waxed_weathered_copper";
    const DISPLAY_NAME: &'static str = "Waxed Weathered Copper";
}
pub struct r#WaxedOxidizedCopper;
impl Item for r#WaxedOxidizedCopper {
    const ID: i32 = 93i32;
    const NAME: &'static str = "waxed_oxidized_copper";
    const DISPLAY_NAME: &'static str = "Waxed Oxidized Copper";
}
pub struct r#WaxedCutCopper;
impl Item for r#WaxedCutCopper {
    const ID: i32 = 94i32;
    const NAME: &'static str = "waxed_cut_copper";
    const DISPLAY_NAME: &'static str = "Waxed Cut Copper";
}
pub struct r#WaxedExposedCutCopper;
impl Item for r#WaxedExposedCutCopper {
    const ID: i32 = 95i32;
    const NAME: &'static str = "waxed_exposed_cut_copper";
    const DISPLAY_NAME: &'static str = "Waxed Exposed Cut Copper";
}
pub struct r#WaxedWeatheredCutCopper;
impl Item for r#WaxedWeatheredCutCopper {
    const ID: i32 = 96i32;
    const NAME: &'static str = "waxed_weathered_cut_copper";
    const DISPLAY_NAME: &'static str = "Waxed Weathered Cut Copper";
}
pub struct r#WaxedOxidizedCutCopper;
impl Item for r#WaxedOxidizedCutCopper {
    const ID: i32 = 97i32;
    const NAME: &'static str = "waxed_oxidized_cut_copper";
    const DISPLAY_NAME: &'static str = "Waxed Oxidized Cut Copper";
}
pub struct r#WaxedCutCopperStairs;
impl Item for r#WaxedCutCopperStairs {
    const ID: i32 = 98i32;
    const NAME: &'static str = "waxed_cut_copper_stairs";
    const DISPLAY_NAME: &'static str = "Waxed Cut Copper Stairs";
}
pub struct r#WaxedExposedCutCopperStairs;
impl Item for r#WaxedExposedCutCopperStairs {
    const ID: i32 = 99i32;
    const NAME: &'static str = "waxed_exposed_cut_copper_stairs";
    const DISPLAY_NAME: &'static str = "Waxed Exposed Cut Copper Stairs";
}
pub struct r#WaxedWeatheredCutCopperStairs;
impl Item for r#WaxedWeatheredCutCopperStairs {
    const ID: i32 = 100i32;
    const NAME: &'static str = "waxed_weathered_cut_copper_stairs";
    const DISPLAY_NAME: &'static str = "Waxed Weathered Cut Copper Stairs";
}
pub struct r#WaxedOxidizedCutCopperStairs;
impl Item for r#WaxedOxidizedCutCopperStairs {
    const ID: i32 = 101i32;
    const NAME: &'static str = "waxed_oxidized_cut_copper_stairs";
    const DISPLAY_NAME: &'static str = "Waxed Oxidized Cut Copper Stairs";
}
pub struct r#WaxedCutCopperSlab;
impl Item for r#WaxedCutCopperSlab {
    const ID: i32 = 102i32;
    const NAME: &'static str = "waxed_cut_copper_slab";
    const DISPLAY_NAME: &'static str = "Waxed Cut Copper Slab";
}
pub struct r#WaxedExposedCutCopperSlab;
impl Item for r#WaxedExposedCutCopperSlab {
    const ID: i32 = 103i32;
    const NAME: &'static str = "waxed_exposed_cut_copper_slab";
    const DISPLAY_NAME: &'static str = "Waxed Exposed Cut Copper Slab";
}
pub struct r#WaxedWeatheredCutCopperSlab;
impl Item for r#WaxedWeatheredCutCopperSlab {
    const ID: i32 = 104i32;
    const NAME: &'static str = "waxed_weathered_cut_copper_slab";
    const DISPLAY_NAME: &'static str = "Waxed Weathered Cut Copper Slab";
}
pub struct r#WaxedOxidizedCutCopperSlab;
impl Item for r#WaxedOxidizedCutCopperSlab {
    const ID: i32 = 105i32;
    const NAME: &'static str = "waxed_oxidized_cut_copper_slab";
    const DISPLAY_NAME: &'static str = "Waxed Oxidized Cut Copper Slab";
}
pub struct r#OakLog;
impl Item for r#OakLog {
    const ID: i32 = 106i32;
    const NAME: &'static str = "oak_log";
    const DISPLAY_NAME: &'static str = "Oak Log";
}
pub struct r#SpruceLog;
impl Item for r#SpruceLog {
    const ID: i32 = 107i32;
    const NAME: &'static str = "spruce_log";
    const DISPLAY_NAME: &'static str = "Spruce Log";
}
pub struct r#BirchLog;
impl Item for r#BirchLog {
    const ID: i32 = 108i32;
    const NAME: &'static str = "birch_log";
    const DISPLAY_NAME: &'static str = "Birch Log";
}
pub struct r#JungleLog;
impl Item for r#JungleLog {
    const ID: i32 = 109i32;
    const NAME: &'static str = "jungle_log";
    const DISPLAY_NAME: &'static str = "Jungle Log";
}
pub struct r#AcaciaLog;
impl Item for r#AcaciaLog {
    const ID: i32 = 110i32;
    const NAME: &'static str = "acacia_log";
    const DISPLAY_NAME: &'static str = "Acacia Log";
}
pub struct r#DarkOakLog;
impl Item for r#DarkOakLog {
    const ID: i32 = 111i32;
    const NAME: &'static str = "dark_oak_log";
    const DISPLAY_NAME: &'static str = "Dark Oak Log";
}
pub struct r#MangroveLog;
impl Item for r#MangroveLog {
    const ID: i32 = 112i32;
    const NAME: &'static str = "mangrove_log";
    const DISPLAY_NAME: &'static str = "Mangrove Log";
}
pub struct r#MangroveRoots;
impl Item for r#MangroveRoots {
    const ID: i32 = 113i32;
    const NAME: &'static str = "mangrove_roots";
    const DISPLAY_NAME: &'static str = "Mangrove Roots";
}
pub struct r#MuddyMangroveRoots;
impl Item for r#MuddyMangroveRoots {
    const ID: i32 = 114i32;
    const NAME: &'static str = "muddy_mangrove_roots";
    const DISPLAY_NAME: &'static str = "Muddy Mangrove Roots";
}
pub struct r#CrimsonStem;
impl Item for r#CrimsonStem {
    const ID: i32 = 115i32;
    const NAME: &'static str = "crimson_stem";
    const DISPLAY_NAME: &'static str = "Crimson Stem";
}
pub struct r#WarpedStem;
impl Item for r#WarpedStem {
    const ID: i32 = 116i32;
    const NAME: &'static str = "warped_stem";
    const DISPLAY_NAME: &'static str = "Warped Stem";
}
pub struct r#BambooBlock;
impl Item for r#BambooBlock {
    const ID: i32 = 117i32;
    const NAME: &'static str = "bamboo_block";
    const DISPLAY_NAME: &'static str = "Block of Bamboo";
}
pub struct r#StrippedOakLog;
impl Item for r#StrippedOakLog {
    const ID: i32 = 118i32;
    const NAME: &'static str = "stripped_oak_log";
    const DISPLAY_NAME: &'static str = "Stripped Oak Log";
}
pub struct r#StrippedSpruceLog;
impl Item for r#StrippedSpruceLog {
    const ID: i32 = 119i32;
    const NAME: &'static str = "stripped_spruce_log";
    const DISPLAY_NAME: &'static str = "Stripped Spruce Log";
}
pub struct r#StrippedBirchLog;
impl Item for r#StrippedBirchLog {
    const ID: i32 = 120i32;
    const NAME: &'static str = "stripped_birch_log";
    const DISPLAY_NAME: &'static str = "Stripped Birch Log";
}
pub struct r#StrippedJungleLog;
impl Item for r#StrippedJungleLog {
    const ID: i32 = 121i32;
    const NAME: &'static str = "stripped_jungle_log";
    const DISPLAY_NAME: &'static str = "Stripped Jungle Log";
}
pub struct r#StrippedAcaciaLog;
impl Item for r#StrippedAcaciaLog {
    const ID: i32 = 122i32;
    const NAME: &'static str = "stripped_acacia_log";
    const DISPLAY_NAME: &'static str = "Stripped Acacia Log";
}
pub struct r#StrippedDarkOakLog;
impl Item for r#StrippedDarkOakLog {
    const ID: i32 = 123i32;
    const NAME: &'static str = "stripped_dark_oak_log";
    const DISPLAY_NAME: &'static str = "Stripped Dark Oak Log";
}
pub struct r#StrippedMangroveLog;
impl Item for r#StrippedMangroveLog {
    const ID: i32 = 124i32;
    const NAME: &'static str = "stripped_mangrove_log";
    const DISPLAY_NAME: &'static str = "Stripped Mangrove Log";
}
pub struct r#StrippedCrimsonStem;
impl Item for r#StrippedCrimsonStem {
    const ID: i32 = 125i32;
    const NAME: &'static str = "stripped_crimson_stem";
    const DISPLAY_NAME: &'static str = "Stripped Crimson Stem";
}
pub struct r#StrippedWarpedStem;
impl Item for r#StrippedWarpedStem {
    const ID: i32 = 126i32;
    const NAME: &'static str = "stripped_warped_stem";
    const DISPLAY_NAME: &'static str = "Stripped Warped Stem";
}
pub struct r#StrippedOakWood;
impl Item for r#StrippedOakWood {
    const ID: i32 = 127i32;
    const NAME: &'static str = "stripped_oak_wood";
    const DISPLAY_NAME: &'static str = "Stripped Oak Wood";
}
pub struct r#StrippedSpruceWood;
impl Item for r#StrippedSpruceWood {
    const ID: i32 = 128i32;
    const NAME: &'static str = "stripped_spruce_wood";
    const DISPLAY_NAME: &'static str = "Stripped Spruce Wood";
}
pub struct r#StrippedBirchWood;
impl Item for r#StrippedBirchWood {
    const ID: i32 = 129i32;
    const NAME: &'static str = "stripped_birch_wood";
    const DISPLAY_NAME: &'static str = "Stripped Birch Wood";
}
pub struct r#StrippedJungleWood;
impl Item for r#StrippedJungleWood {
    const ID: i32 = 130i32;
    const NAME: &'static str = "stripped_jungle_wood";
    const DISPLAY_NAME: &'static str = "Stripped Jungle Wood";
}
pub struct r#StrippedAcaciaWood;
impl Item for r#StrippedAcaciaWood {
    const ID: i32 = 131i32;
    const NAME: &'static str = "stripped_acacia_wood";
    const DISPLAY_NAME: &'static str = "Stripped Acacia Wood";
}
pub struct r#StrippedDarkOakWood;
impl Item for r#StrippedDarkOakWood {
    const ID: i32 = 132i32;
    const NAME: &'static str = "stripped_dark_oak_wood";
    const DISPLAY_NAME: &'static str = "Stripped Dark Oak Wood";
}
pub struct r#StrippedMangroveWood;
impl Item for r#StrippedMangroveWood {
    const ID: i32 = 133i32;
    const NAME: &'static str = "stripped_mangrove_wood";
    const DISPLAY_NAME: &'static str = "Stripped Mangrove Wood";
}
pub struct r#StrippedCrimsonHyphae;
impl Item for r#StrippedCrimsonHyphae {
    const ID: i32 = 134i32;
    const NAME: &'static str = "stripped_crimson_hyphae";
    const DISPLAY_NAME: &'static str = "Stripped Crimson Hyphae";
}
pub struct r#StrippedWarpedHyphae;
impl Item for r#StrippedWarpedHyphae {
    const ID: i32 = 135i32;
    const NAME: &'static str = "stripped_warped_hyphae";
    const DISPLAY_NAME: &'static str = "Stripped Warped Hyphae";
}
pub struct r#StrippedBambooBlock;
impl Item for r#StrippedBambooBlock {
    const ID: i32 = 136i32;
    const NAME: &'static str = "stripped_bamboo_block";
    const DISPLAY_NAME: &'static str = "Block of Stripped Bamboo";
}
pub struct r#OakWood;
impl Item for r#OakWood {
    const ID: i32 = 137i32;
    const NAME: &'static str = "oak_wood";
    const DISPLAY_NAME: &'static str = "Oak Wood";
}
pub struct r#SpruceWood;
impl Item for r#SpruceWood {
    const ID: i32 = 138i32;
    const NAME: &'static str = "spruce_wood";
    const DISPLAY_NAME: &'static str = "Spruce Wood";
}
pub struct r#BirchWood;
impl Item for r#BirchWood {
    const ID: i32 = 139i32;
    const NAME: &'static str = "birch_wood";
    const DISPLAY_NAME: &'static str = "Birch Wood";
}
pub struct r#JungleWood;
impl Item for r#JungleWood {
    const ID: i32 = 140i32;
    const NAME: &'static str = "jungle_wood";
    const DISPLAY_NAME: &'static str = "Jungle Wood";
}
pub struct r#AcaciaWood;
impl Item for r#AcaciaWood {
    const ID: i32 = 141i32;
    const NAME: &'static str = "acacia_wood";
    const DISPLAY_NAME: &'static str = "Acacia Wood";
}
pub struct r#DarkOakWood;
impl Item for r#DarkOakWood {
    const ID: i32 = 142i32;
    const NAME: &'static str = "dark_oak_wood";
    const DISPLAY_NAME: &'static str = "Dark Oak Wood";
}
pub struct r#MangroveWood;
impl Item for r#MangroveWood {
    const ID: i32 = 143i32;
    const NAME: &'static str = "mangrove_wood";
    const DISPLAY_NAME: &'static str = "Mangrove Wood";
}
pub struct r#CrimsonHyphae;
impl Item for r#CrimsonHyphae {
    const ID: i32 = 144i32;
    const NAME: &'static str = "crimson_hyphae";
    const DISPLAY_NAME: &'static str = "Crimson Hyphae";
}
pub struct r#WarpedHyphae;
impl Item for r#WarpedHyphae {
    const ID: i32 = 145i32;
    const NAME: &'static str = "warped_hyphae";
    const DISPLAY_NAME: &'static str = "Warped Hyphae";
}
pub struct r#OakLeaves;
impl Item for r#OakLeaves {
    const ID: i32 = 146i32;
    const NAME: &'static str = "oak_leaves";
    const DISPLAY_NAME: &'static str = "Oak Leaves";
}
pub struct r#SpruceLeaves;
impl Item for r#SpruceLeaves {
    const ID: i32 = 147i32;
    const NAME: &'static str = "spruce_leaves";
    const DISPLAY_NAME: &'static str = "Spruce Leaves";
}
pub struct r#BirchLeaves;
impl Item for r#BirchLeaves {
    const ID: i32 = 148i32;
    const NAME: &'static str = "birch_leaves";
    const DISPLAY_NAME: &'static str = "Birch Leaves";
}
pub struct r#JungleLeaves;
impl Item for r#JungleLeaves {
    const ID: i32 = 149i32;
    const NAME: &'static str = "jungle_leaves";
    const DISPLAY_NAME: &'static str = "Jungle Leaves";
}
pub struct r#AcaciaLeaves;
impl Item for r#AcaciaLeaves {
    const ID: i32 = 150i32;
    const NAME: &'static str = "acacia_leaves";
    const DISPLAY_NAME: &'static str = "Acacia Leaves";
}
pub struct r#DarkOakLeaves;
impl Item for r#DarkOakLeaves {
    const ID: i32 = 151i32;
    const NAME: &'static str = "dark_oak_leaves";
    const DISPLAY_NAME: &'static str = "Dark Oak Leaves";
}
pub struct r#MangroveLeaves;
impl Item for r#MangroveLeaves {
    const ID: i32 = 152i32;
    const NAME: &'static str = "mangrove_leaves";
    const DISPLAY_NAME: &'static str = "Mangrove Leaves";
}
pub struct r#AzaleaLeaves;
impl Item for r#AzaleaLeaves {
    const ID: i32 = 153i32;
    const NAME: &'static str = "azalea_leaves";
    const DISPLAY_NAME: &'static str = "Azalea Leaves";
}
pub struct r#FloweringAzaleaLeaves;
impl Item for r#FloweringAzaleaLeaves {
    const ID: i32 = 154i32;
    const NAME: &'static str = "flowering_azalea_leaves";
    const DISPLAY_NAME: &'static str = "Flowering Azalea Leaves";
}
pub struct r#Sponge;
impl Item for r#Sponge {
    const ID: i32 = 155i32;
    const NAME: &'static str = "sponge";
    const DISPLAY_NAME: &'static str = "Sponge";
}
pub struct r#WetSponge;
impl Item for r#WetSponge {
    const ID: i32 = 156i32;
    const NAME: &'static str = "wet_sponge";
    const DISPLAY_NAME: &'static str = "Wet Sponge";
}
pub struct r#Glass;
impl Item for r#Glass {
    const ID: i32 = 157i32;
    const NAME: &'static str = "glass";
    const DISPLAY_NAME: &'static str = "Glass";
}
pub struct r#TintedGlass;
impl Item for r#TintedGlass {
    const ID: i32 = 158i32;
    const NAME: &'static str = "tinted_glass";
    const DISPLAY_NAME: &'static str = "Tinted Glass";
}
pub struct r#LapisBlock;
impl Item for r#LapisBlock {
    const ID: i32 = 159i32;
    const NAME: &'static str = "lapis_block";
    const DISPLAY_NAME: &'static str = "Block of Lapis Lazuli";
}
pub struct r#Sandstone;
impl Item for r#Sandstone {
    const ID: i32 = 160i32;
    const NAME: &'static str = "sandstone";
    const DISPLAY_NAME: &'static str = "Sandstone";
}
pub struct r#ChiseledSandstone;
impl Item for r#ChiseledSandstone {
    const ID: i32 = 161i32;
    const NAME: &'static str = "chiseled_sandstone";
    const DISPLAY_NAME: &'static str = "Chiseled Sandstone";
}
pub struct r#CutSandstone;
impl Item for r#CutSandstone {
    const ID: i32 = 162i32;
    const NAME: &'static str = "cut_sandstone";
    const DISPLAY_NAME: &'static str = "Cut Sandstone";
}
pub struct r#Cobweb;
impl Item for r#Cobweb {
    const ID: i32 = 163i32;
    const NAME: &'static str = "cobweb";
    const DISPLAY_NAME: &'static str = "Cobweb";
}
pub struct r#Grass;
impl Item for r#Grass {
    const ID: i32 = 164i32;
    const NAME: &'static str = "grass";
    const DISPLAY_NAME: &'static str = "Grass";
}
pub struct r#Fern;
impl Item for r#Fern {
    const ID: i32 = 165i32;
    const NAME: &'static str = "fern";
    const DISPLAY_NAME: &'static str = "Fern";
}
pub struct r#Azalea;
impl Item for r#Azalea {
    const ID: i32 = 166i32;
    const NAME: &'static str = "azalea";
    const DISPLAY_NAME: &'static str = "Azalea";
}
pub struct r#FloweringAzalea;
impl Item for r#FloweringAzalea {
    const ID: i32 = 167i32;
    const NAME: &'static str = "flowering_azalea";
    const DISPLAY_NAME: &'static str = "Flowering Azalea";
}
pub struct r#DeadBush;
impl Item for r#DeadBush {
    const ID: i32 = 168i32;
    const NAME: &'static str = "dead_bush";
    const DISPLAY_NAME: &'static str = "Dead Bush";
}
pub struct r#Seagrass;
impl Item for r#Seagrass {
    const ID: i32 = 169i32;
    const NAME: &'static str = "seagrass";
    const DISPLAY_NAME: &'static str = "Seagrass";
}
pub struct r#SeaPickle;
impl Item for r#SeaPickle {
    const ID: i32 = 170i32;
    const NAME: &'static str = "sea_pickle";
    const DISPLAY_NAME: &'static str = "Sea Pickle";
}
pub struct r#WhiteWool;
impl Item for r#WhiteWool {
    const ID: i32 = 171i32;
    const NAME: &'static str = "white_wool";
    const DISPLAY_NAME: &'static str = "White Wool";
}
pub struct r#OrangeWool;
impl Item for r#OrangeWool {
    const ID: i32 = 172i32;
    const NAME: &'static str = "orange_wool";
    const DISPLAY_NAME: &'static str = "Orange Wool";
}
pub struct r#MagentaWool;
impl Item for r#MagentaWool {
    const ID: i32 = 173i32;
    const NAME: &'static str = "magenta_wool";
    const DISPLAY_NAME: &'static str = "Magenta Wool";
}
pub struct r#LightBlueWool;
impl Item for r#LightBlueWool {
    const ID: i32 = 174i32;
    const NAME: &'static str = "light_blue_wool";
    const DISPLAY_NAME: &'static str = "Light Blue Wool";
}
pub struct r#YellowWool;
impl Item for r#YellowWool {
    const ID: i32 = 175i32;
    const NAME: &'static str = "yellow_wool";
    const DISPLAY_NAME: &'static str = "Yellow Wool";
}
pub struct r#LimeWool;
impl Item for r#LimeWool {
    const ID: i32 = 176i32;
    const NAME: &'static str = "lime_wool";
    const DISPLAY_NAME: &'static str = "Lime Wool";
}
pub struct r#PinkWool;
impl Item for r#PinkWool {
    const ID: i32 = 177i32;
    const NAME: &'static str = "pink_wool";
    const DISPLAY_NAME: &'static str = "Pink Wool";
}
pub struct r#GrayWool;
impl Item for r#GrayWool {
    const ID: i32 = 178i32;
    const NAME: &'static str = "gray_wool";
    const DISPLAY_NAME: &'static str = "Gray Wool";
}
pub struct r#LightGrayWool;
impl Item for r#LightGrayWool {
    const ID: i32 = 179i32;
    const NAME: &'static str = "light_gray_wool";
    const DISPLAY_NAME: &'static str = "Light Gray Wool";
}
pub struct r#CyanWool;
impl Item for r#CyanWool {
    const ID: i32 = 180i32;
    const NAME: &'static str = "cyan_wool";
    const DISPLAY_NAME: &'static str = "Cyan Wool";
}
pub struct r#PurpleWool;
impl Item for r#PurpleWool {
    const ID: i32 = 181i32;
    const NAME: &'static str = "purple_wool";
    const DISPLAY_NAME: &'static str = "Purple Wool";
}
pub struct r#BlueWool;
impl Item for r#BlueWool {
    const ID: i32 = 182i32;
    const NAME: &'static str = "blue_wool";
    const DISPLAY_NAME: &'static str = "Blue Wool";
}
pub struct r#BrownWool;
impl Item for r#BrownWool {
    const ID: i32 = 183i32;
    const NAME: &'static str = "brown_wool";
    const DISPLAY_NAME: &'static str = "Brown Wool";
}
pub struct r#GreenWool;
impl Item for r#GreenWool {
    const ID: i32 = 184i32;
    const NAME: &'static str = "green_wool";
    const DISPLAY_NAME: &'static str = "Green Wool";
}
pub struct r#RedWool;
impl Item for r#RedWool {
    const ID: i32 = 185i32;
    const NAME: &'static str = "red_wool";
    const DISPLAY_NAME: &'static str = "Red Wool";
}
pub struct r#BlackWool;
impl Item for r#BlackWool {
    const ID: i32 = 186i32;
    const NAME: &'static str = "black_wool";
    const DISPLAY_NAME: &'static str = "Black Wool";
}
pub struct r#Dandelion;
impl Item for r#Dandelion {
    const ID: i32 = 187i32;
    const NAME: &'static str = "dandelion";
    const DISPLAY_NAME: &'static str = "Dandelion";
}
pub struct r#Poppy;
impl Item for r#Poppy {
    const ID: i32 = 188i32;
    const NAME: &'static str = "poppy";
    const DISPLAY_NAME: &'static str = "Poppy";
}
pub struct r#BlueOrchid;
impl Item for r#BlueOrchid {
    const ID: i32 = 189i32;
    const NAME: &'static str = "blue_orchid";
    const DISPLAY_NAME: &'static str = "Blue Orchid";
}
pub struct r#Allium;
impl Item for r#Allium {
    const ID: i32 = 190i32;
    const NAME: &'static str = "allium";
    const DISPLAY_NAME: &'static str = "Allium";
}
pub struct r#AzureBluet;
impl Item for r#AzureBluet {
    const ID: i32 = 191i32;
    const NAME: &'static str = "azure_bluet";
    const DISPLAY_NAME: &'static str = "Azure Bluet";
}
pub struct r#RedTulip;
impl Item for r#RedTulip {
    const ID: i32 = 192i32;
    const NAME: &'static str = "red_tulip";
    const DISPLAY_NAME: &'static str = "Red Tulip";
}
pub struct r#OrangeTulip;
impl Item for r#OrangeTulip {
    const ID: i32 = 193i32;
    const NAME: &'static str = "orange_tulip";
    const DISPLAY_NAME: &'static str = "Orange Tulip";
}
pub struct r#WhiteTulip;
impl Item for r#WhiteTulip {
    const ID: i32 = 194i32;
    const NAME: &'static str = "white_tulip";
    const DISPLAY_NAME: &'static str = "White Tulip";
}
pub struct r#PinkTulip;
impl Item for r#PinkTulip {
    const ID: i32 = 195i32;
    const NAME: &'static str = "pink_tulip";
    const DISPLAY_NAME: &'static str = "Pink Tulip";
}
pub struct r#OxeyeDaisy;
impl Item for r#OxeyeDaisy {
    const ID: i32 = 196i32;
    const NAME: &'static str = "oxeye_daisy";
    const DISPLAY_NAME: &'static str = "Oxeye Daisy";
}
pub struct r#Cornflower;
impl Item for r#Cornflower {
    const ID: i32 = 197i32;
    const NAME: &'static str = "cornflower";
    const DISPLAY_NAME: &'static str = "Cornflower";
}
pub struct r#LilyOfTheValley;
impl Item for r#LilyOfTheValley {
    const ID: i32 = 198i32;
    const NAME: &'static str = "lily_of_the_valley";
    const DISPLAY_NAME: &'static str = "Lily of the Valley";
}
pub struct r#WitherRose;
impl Item for r#WitherRose {
    const ID: i32 = 199i32;
    const NAME: &'static str = "wither_rose";
    const DISPLAY_NAME: &'static str = "Wither Rose";
}
pub struct r#SporeBlossom;
impl Item for r#SporeBlossom {
    const ID: i32 = 200i32;
    const NAME: &'static str = "spore_blossom";
    const DISPLAY_NAME: &'static str = "Spore Blossom";
}
pub struct r#BrownMushroom;
impl Item for r#BrownMushroom {
    const ID: i32 = 201i32;
    const NAME: &'static str = "brown_mushroom";
    const DISPLAY_NAME: &'static str = "Brown Mushroom";
}
pub struct r#RedMushroom;
impl Item for r#RedMushroom {
    const ID: i32 = 202i32;
    const NAME: &'static str = "red_mushroom";
    const DISPLAY_NAME: &'static str = "Red Mushroom";
}
pub struct r#CrimsonFungus;
impl Item for r#CrimsonFungus {
    const ID: i32 = 203i32;
    const NAME: &'static str = "crimson_fungus";
    const DISPLAY_NAME: &'static str = "Crimson Fungus";
}
pub struct r#WarpedFungus;
impl Item for r#WarpedFungus {
    const ID: i32 = 204i32;
    const NAME: &'static str = "warped_fungus";
    const DISPLAY_NAME: &'static str = "Warped Fungus";
}
pub struct r#CrimsonRoots;
impl Item for r#CrimsonRoots {
    const ID: i32 = 205i32;
    const NAME: &'static str = "crimson_roots";
    const DISPLAY_NAME: &'static str = "Crimson Roots";
}
pub struct r#WarpedRoots;
impl Item for r#WarpedRoots {
    const ID: i32 = 206i32;
    const NAME: &'static str = "warped_roots";
    const DISPLAY_NAME: &'static str = "Warped Roots";
}
pub struct r#NetherSprouts;
impl Item for r#NetherSprouts {
    const ID: i32 = 207i32;
    const NAME: &'static str = "nether_sprouts";
    const DISPLAY_NAME: &'static str = "Nether Sprouts";
}
pub struct r#WeepingVines;
impl Item for r#WeepingVines {
    const ID: i32 = 208i32;
    const NAME: &'static str = "weeping_vines";
    const DISPLAY_NAME: &'static str = "Weeping Vines";
}
pub struct r#TwistingVines;
impl Item for r#TwistingVines {
    const ID: i32 = 209i32;
    const NAME: &'static str = "twisting_vines";
    const DISPLAY_NAME: &'static str = "Twisting Vines";
}
pub struct r#SugarCane;
impl Item for r#SugarCane {
    const ID: i32 = 210i32;
    const NAME: &'static str = "sugar_cane";
    const DISPLAY_NAME: &'static str = "Sugar Cane";
}
pub struct r#Kelp;
impl Item for r#Kelp {
    const ID: i32 = 211i32;
    const NAME: &'static str = "kelp";
    const DISPLAY_NAME: &'static str = "Kelp";
}
pub struct r#MossCarpet;
impl Item for r#MossCarpet {
    const ID: i32 = 212i32;
    const NAME: &'static str = "moss_carpet";
    const DISPLAY_NAME: &'static str = "Moss Carpet";
}
pub struct r#MossBlock;
impl Item for r#MossBlock {
    const ID: i32 = 213i32;
    const NAME: &'static str = "moss_block";
    const DISPLAY_NAME: &'static str = "Moss Block";
}
pub struct r#HangingRoots;
impl Item for r#HangingRoots {
    const ID: i32 = 214i32;
    const NAME: &'static str = "hanging_roots";
    const DISPLAY_NAME: &'static str = "Hanging Roots";
}
pub struct r#BigDripleaf;
impl Item for r#BigDripleaf {
    const ID: i32 = 215i32;
    const NAME: &'static str = "big_dripleaf";
    const DISPLAY_NAME: &'static str = "Big Dripleaf";
}
pub struct r#SmallDripleaf;
impl Item for r#SmallDripleaf {
    const ID: i32 = 216i32;
    const NAME: &'static str = "small_dripleaf";
    const DISPLAY_NAME: &'static str = "Small Dripleaf";
}
pub struct r#Bamboo;
impl Item for r#Bamboo {
    const ID: i32 = 217i32;
    const NAME: &'static str = "bamboo";
    const DISPLAY_NAME: &'static str = "Bamboo";
}
pub struct r#OakSlab;
impl Item for r#OakSlab {
    const ID: i32 = 218i32;
    const NAME: &'static str = "oak_slab";
    const DISPLAY_NAME: &'static str = "Oak Slab";
}
pub struct r#SpruceSlab;
impl Item for r#SpruceSlab {
    const ID: i32 = 219i32;
    const NAME: &'static str = "spruce_slab";
    const DISPLAY_NAME: &'static str = "Spruce Slab";
}
pub struct r#BirchSlab;
impl Item for r#BirchSlab {
    const ID: i32 = 220i32;
    const NAME: &'static str = "birch_slab";
    const DISPLAY_NAME: &'static str = "Birch Slab";
}
pub struct r#JungleSlab;
impl Item for r#JungleSlab {
    const ID: i32 = 221i32;
    const NAME: &'static str = "jungle_slab";
    const DISPLAY_NAME: &'static str = "Jungle Slab";
}
pub struct r#AcaciaSlab;
impl Item for r#AcaciaSlab {
    const ID: i32 = 222i32;
    const NAME: &'static str = "acacia_slab";
    const DISPLAY_NAME: &'static str = "Acacia Slab";
}
pub struct r#DarkOakSlab;
impl Item for r#DarkOakSlab {
    const ID: i32 = 223i32;
    const NAME: &'static str = "dark_oak_slab";
    const DISPLAY_NAME: &'static str = "Dark Oak Slab";
}
pub struct r#MangroveSlab;
impl Item for r#MangroveSlab {
    const ID: i32 = 224i32;
    const NAME: &'static str = "mangrove_slab";
    const DISPLAY_NAME: &'static str = "Mangrove Slab";
}
pub struct r#BambooSlab;
impl Item for r#BambooSlab {
    const ID: i32 = 225i32;
    const NAME: &'static str = "bamboo_slab";
    const DISPLAY_NAME: &'static str = "Bamboo Slab";
}
pub struct r#BambooMosaicSlab;
impl Item for r#BambooMosaicSlab {
    const ID: i32 = 226i32;
    const NAME: &'static str = "bamboo_mosaic_slab";
    const DISPLAY_NAME: &'static str = "Bamboo Mosaic Slab";
}
pub struct r#CrimsonSlab;
impl Item for r#CrimsonSlab {
    const ID: i32 = 227i32;
    const NAME: &'static str = "crimson_slab";
    const DISPLAY_NAME: &'static str = "Crimson Slab";
}
pub struct r#WarpedSlab;
impl Item for r#WarpedSlab {
    const ID: i32 = 228i32;
    const NAME: &'static str = "warped_slab";
    const DISPLAY_NAME: &'static str = "Warped Slab";
}
pub struct r#StoneSlab;
impl Item for r#StoneSlab {
    const ID: i32 = 229i32;
    const NAME: &'static str = "stone_slab";
    const DISPLAY_NAME: &'static str = "Stone Slab";
}
pub struct r#SmoothStoneSlab;
impl Item for r#SmoothStoneSlab {
    const ID: i32 = 230i32;
    const NAME: &'static str = "smooth_stone_slab";
    const DISPLAY_NAME: &'static str = "Smooth Stone Slab";
}
pub struct r#SandstoneSlab;
impl Item for r#SandstoneSlab {
    const ID: i32 = 231i32;
    const NAME: &'static str = "sandstone_slab";
    const DISPLAY_NAME: &'static str = "Sandstone Slab";
}
pub struct r#CutSandstoneSlab;
impl Item for r#CutSandstoneSlab {
    const ID: i32 = 232i32;
    const NAME: &'static str = "cut_sandstone_slab";
    const DISPLAY_NAME: &'static str = "Cut Sandstone Slab";
}
pub struct r#PetrifiedOakSlab;
impl Item for r#PetrifiedOakSlab {
    const ID: i32 = 233i32;
    const NAME: &'static str = "petrified_oak_slab";
    const DISPLAY_NAME: &'static str = "Petrified Oak Slab";
}
pub struct r#CobblestoneSlab;
impl Item for r#CobblestoneSlab {
    const ID: i32 = 234i32;
    const NAME: &'static str = "cobblestone_slab";
    const DISPLAY_NAME: &'static str = "Cobblestone Slab";
}
pub struct r#BrickSlab;
impl Item for r#BrickSlab {
    const ID: i32 = 235i32;
    const NAME: &'static str = "brick_slab";
    const DISPLAY_NAME: &'static str = "Brick Slab";
}
pub struct r#StoneBrickSlab;
impl Item for r#StoneBrickSlab {
    const ID: i32 = 236i32;
    const NAME: &'static str = "stone_brick_slab";
    const DISPLAY_NAME: &'static str = "Stone Brick Slab";
}
pub struct r#MudBrickSlab;
impl Item for r#MudBrickSlab {
    const ID: i32 = 237i32;
    const NAME: &'static str = "mud_brick_slab";
    const DISPLAY_NAME: &'static str = "Mud Brick Slab";
}
pub struct r#NetherBrickSlab;
impl Item for r#NetherBrickSlab {
    const ID: i32 = 238i32;
    const NAME: &'static str = "nether_brick_slab";
    const DISPLAY_NAME: &'static str = "Nether Brick Slab";
}
pub struct r#QuartzSlab;
impl Item for r#QuartzSlab {
    const ID: i32 = 239i32;
    const NAME: &'static str = "quartz_slab";
    const DISPLAY_NAME: &'static str = "Quartz Slab";
}
pub struct r#RedSandstoneSlab;
impl Item for r#RedSandstoneSlab {
    const ID: i32 = 240i32;
    const NAME: &'static str = "red_sandstone_slab";
    const DISPLAY_NAME: &'static str = "Red Sandstone Slab";
}
pub struct r#CutRedSandstoneSlab;
impl Item for r#CutRedSandstoneSlab {
    const ID: i32 = 241i32;
    const NAME: &'static str = "cut_red_sandstone_slab";
    const DISPLAY_NAME: &'static str = "Cut Red Sandstone Slab";
}
pub struct r#PurpurSlab;
impl Item for r#PurpurSlab {
    const ID: i32 = 242i32;
    const NAME: &'static str = "purpur_slab";
    const DISPLAY_NAME: &'static str = "Purpur Slab";
}
pub struct r#PrismarineSlab;
impl Item for r#PrismarineSlab {
    const ID: i32 = 243i32;
    const NAME: &'static str = "prismarine_slab";
    const DISPLAY_NAME: &'static str = "Prismarine Slab";
}
pub struct r#PrismarineBrickSlab;
impl Item for r#PrismarineBrickSlab {
    const ID: i32 = 244i32;
    const NAME: &'static str = "prismarine_brick_slab";
    const DISPLAY_NAME: &'static str = "Prismarine Brick Slab";
}
pub struct r#DarkPrismarineSlab;
impl Item for r#DarkPrismarineSlab {
    const ID: i32 = 245i32;
    const NAME: &'static str = "dark_prismarine_slab";
    const DISPLAY_NAME: &'static str = "Dark Prismarine Slab";
}
pub struct r#SmoothQuartz;
impl Item for r#SmoothQuartz {
    const ID: i32 = 246i32;
    const NAME: &'static str = "smooth_quartz";
    const DISPLAY_NAME: &'static str = "Smooth Quartz Block";
}
pub struct r#SmoothRedSandstone;
impl Item for r#SmoothRedSandstone {
    const ID: i32 = 247i32;
    const NAME: &'static str = "smooth_red_sandstone";
    const DISPLAY_NAME: &'static str = "Smooth Red Sandstone";
}
pub struct r#SmoothSandstone;
impl Item for r#SmoothSandstone {
    const ID: i32 = 248i32;
    const NAME: &'static str = "smooth_sandstone";
    const DISPLAY_NAME: &'static str = "Smooth Sandstone";
}
pub struct r#SmoothStone;
impl Item for r#SmoothStone {
    const ID: i32 = 249i32;
    const NAME: &'static str = "smooth_stone";
    const DISPLAY_NAME: &'static str = "Smooth Stone";
}
pub struct r#Bricks;
impl Item for r#Bricks {
    const ID: i32 = 250i32;
    const NAME: &'static str = "bricks";
    const DISPLAY_NAME: &'static str = "Bricks";
}
pub struct r#Bookshelf;
impl Item for r#Bookshelf {
    const ID: i32 = 251i32;
    const NAME: &'static str = "bookshelf";
    const DISPLAY_NAME: &'static str = "Bookshelf";
}
pub struct r#ChiseledBookshelf;
impl Item for r#ChiseledBookshelf {
    const ID: i32 = 252i32;
    const NAME: &'static str = "chiseled_bookshelf";
    const DISPLAY_NAME: &'static str = "Chiseled Bookshelf";
}
pub struct r#MossyCobblestone;
impl Item for r#MossyCobblestone {
    const ID: i32 = 253i32;
    const NAME: &'static str = "mossy_cobblestone";
    const DISPLAY_NAME: &'static str = "Mossy Cobblestone";
}
pub struct r#Obsidian;
impl Item for r#Obsidian {
    const ID: i32 = 254i32;
    const NAME: &'static str = "obsidian";
    const DISPLAY_NAME: &'static str = "Obsidian";
}
pub struct r#Torch;
impl Item for r#Torch {
    const ID: i32 = 255i32;
    const NAME: &'static str = "torch";
    const DISPLAY_NAME: &'static str = "Torch";
}
pub struct r#EndRod;
impl Item for r#EndRod {
    const ID: i32 = 256i32;
    const NAME: &'static str = "end_rod";
    const DISPLAY_NAME: &'static str = "End Rod";
}
pub struct r#ChorusPlant;
impl Item for r#ChorusPlant {
    const ID: i32 = 257i32;
    const NAME: &'static str = "chorus_plant";
    const DISPLAY_NAME: &'static str = "Chorus Plant";
}
pub struct r#ChorusFlower;
impl Item for r#ChorusFlower {
    const ID: i32 = 258i32;
    const NAME: &'static str = "chorus_flower";
    const DISPLAY_NAME: &'static str = "Chorus Flower";
}
pub struct r#PurpurBlock;
impl Item for r#PurpurBlock {
    const ID: i32 = 259i32;
    const NAME: &'static str = "purpur_block";
    const DISPLAY_NAME: &'static str = "Purpur Block";
}
pub struct r#PurpurPillar;
impl Item for r#PurpurPillar {
    const ID: i32 = 260i32;
    const NAME: &'static str = "purpur_pillar";
    const DISPLAY_NAME: &'static str = "Purpur Pillar";
}
pub struct r#PurpurStairs;
impl Item for r#PurpurStairs {
    const ID: i32 = 261i32;
    const NAME: &'static str = "purpur_stairs";
    const DISPLAY_NAME: &'static str = "Purpur Stairs";
}
pub struct r#Spawner;
impl Item for r#Spawner {
    const ID: i32 = 262i32;
    const NAME: &'static str = "spawner";
    const DISPLAY_NAME: &'static str = "Monster Spawner";
}
pub struct r#Chest;
impl Item for r#Chest {
    const ID: i32 = 263i32;
    const NAME: &'static str = "chest";
    const DISPLAY_NAME: &'static str = "Chest";
}
pub struct r#CraftingTable;
impl Item for r#CraftingTable {
    const ID: i32 = 264i32;
    const NAME: &'static str = "crafting_table";
    const DISPLAY_NAME: &'static str = "Crafting Table";
}
pub struct r#Farmland;
impl Item for r#Farmland {
    const ID: i32 = 265i32;
    const NAME: &'static str = "farmland";
    const DISPLAY_NAME: &'static str = "Farmland";
}
pub struct r#Furnace;
impl Item for r#Furnace {
    const ID: i32 = 266i32;
    const NAME: &'static str = "furnace";
    const DISPLAY_NAME: &'static str = "Furnace";
}
pub struct r#Ladder;
impl Item for r#Ladder {
    const ID: i32 = 267i32;
    const NAME: &'static str = "ladder";
    const DISPLAY_NAME: &'static str = "Ladder";
}
pub struct r#CobblestoneStairs;
impl Item for r#CobblestoneStairs {
    const ID: i32 = 268i32;
    const NAME: &'static str = "cobblestone_stairs";
    const DISPLAY_NAME: &'static str = "Cobblestone Stairs";
}
pub struct r#Snow;
impl Item for r#Snow {
    const ID: i32 = 269i32;
    const NAME: &'static str = "snow";
    const DISPLAY_NAME: &'static str = "Snow";
}
pub struct r#Ice;
impl Item for r#Ice {
    const ID: i32 = 270i32;
    const NAME: &'static str = "ice";
    const DISPLAY_NAME: &'static str = "Ice";
}
pub struct r#SnowBlock;
impl Item for r#SnowBlock {
    const ID: i32 = 271i32;
    const NAME: &'static str = "snow_block";
    const DISPLAY_NAME: &'static str = "Snow Block";
}
pub struct r#Cactus;
impl Item for r#Cactus {
    const ID: i32 = 272i32;
    const NAME: &'static str = "cactus";
    const DISPLAY_NAME: &'static str = "Cactus";
}
pub struct r#Clay;
impl Item for r#Clay {
    const ID: i32 = 273i32;
    const NAME: &'static str = "clay";
    const DISPLAY_NAME: &'static str = "Clay";
}
pub struct r#Jukebox;
impl Item for r#Jukebox {
    const ID: i32 = 274i32;
    const NAME: &'static str = "jukebox";
    const DISPLAY_NAME: &'static str = "Jukebox";
}
pub struct r#OakFence;
impl Item for r#OakFence {
    const ID: i32 = 275i32;
    const NAME: &'static str = "oak_fence";
    const DISPLAY_NAME: &'static str = "Oak Fence";
}
pub struct r#SpruceFence;
impl Item for r#SpruceFence {
    const ID: i32 = 276i32;
    const NAME: &'static str = "spruce_fence";
    const DISPLAY_NAME: &'static str = "Spruce Fence";
}
pub struct r#BirchFence;
impl Item for r#BirchFence {
    const ID: i32 = 277i32;
    const NAME: &'static str = "birch_fence";
    const DISPLAY_NAME: &'static str = "Birch Fence";
}
pub struct r#JungleFence;
impl Item for r#JungleFence {
    const ID: i32 = 278i32;
    const NAME: &'static str = "jungle_fence";
    const DISPLAY_NAME: &'static str = "Jungle Fence";
}
pub struct r#AcaciaFence;
impl Item for r#AcaciaFence {
    const ID: i32 = 279i32;
    const NAME: &'static str = "acacia_fence";
    const DISPLAY_NAME: &'static str = "Acacia Fence";
}
pub struct r#DarkOakFence;
impl Item for r#DarkOakFence {
    const ID: i32 = 280i32;
    const NAME: &'static str = "dark_oak_fence";
    const DISPLAY_NAME: &'static str = "Dark Oak Fence";
}
pub struct r#MangroveFence;
impl Item for r#MangroveFence {
    const ID: i32 = 281i32;
    const NAME: &'static str = "mangrove_fence";
    const DISPLAY_NAME: &'static str = "Mangrove Fence";
}
pub struct r#BambooFence;
impl Item for r#BambooFence {
    const ID: i32 = 282i32;
    const NAME: &'static str = "bamboo_fence";
    const DISPLAY_NAME: &'static str = "Bamboo Fence";
}
pub struct r#CrimsonFence;
impl Item for r#CrimsonFence {
    const ID: i32 = 283i32;
    const NAME: &'static str = "crimson_fence";
    const DISPLAY_NAME: &'static str = "Crimson Fence";
}
pub struct r#WarpedFence;
impl Item for r#WarpedFence {
    const ID: i32 = 284i32;
    const NAME: &'static str = "warped_fence";
    const DISPLAY_NAME: &'static str = "Warped Fence";
}
pub struct r#Pumpkin;
impl Item for r#Pumpkin {
    const ID: i32 = 285i32;
    const NAME: &'static str = "pumpkin";
    const DISPLAY_NAME: &'static str = "Pumpkin";
}
pub struct r#CarvedPumpkin;
impl Item for r#CarvedPumpkin {
    const ID: i32 = 286i32;
    const NAME: &'static str = "carved_pumpkin";
    const DISPLAY_NAME: &'static str = "Carved Pumpkin";
}
pub struct r#JackOLantern;
impl Item for r#JackOLantern {
    const ID: i32 = 287i32;
    const NAME: &'static str = "jack_o_lantern";
    const DISPLAY_NAME: &'static str = "Jack o'Lantern";
}
pub struct r#Netherrack;
impl Item for r#Netherrack {
    const ID: i32 = 288i32;
    const NAME: &'static str = "netherrack";
    const DISPLAY_NAME: &'static str = "Netherrack";
}
pub struct r#SoulSand;
impl Item for r#SoulSand {
    const ID: i32 = 289i32;
    const NAME: &'static str = "soul_sand";
    const DISPLAY_NAME: &'static str = "Soul Sand";
}
pub struct r#SoulSoil;
impl Item for r#SoulSoil {
    const ID: i32 = 290i32;
    const NAME: &'static str = "soul_soil";
    const DISPLAY_NAME: &'static str = "Soul Soil";
}
pub struct r#Basalt;
impl Item for r#Basalt {
    const ID: i32 = 291i32;
    const NAME: &'static str = "basalt";
    const DISPLAY_NAME: &'static str = "Basalt";
}
pub struct r#PolishedBasalt;
impl Item for r#PolishedBasalt {
    const ID: i32 = 292i32;
    const NAME: &'static str = "polished_basalt";
    const DISPLAY_NAME: &'static str = "Polished Basalt";
}
pub struct r#SmoothBasalt;
impl Item for r#SmoothBasalt {
    const ID: i32 = 293i32;
    const NAME: &'static str = "smooth_basalt";
    const DISPLAY_NAME: &'static str = "Smooth Basalt";
}
pub struct r#SoulTorch;
impl Item for r#SoulTorch {
    const ID: i32 = 294i32;
    const NAME: &'static str = "soul_torch";
    const DISPLAY_NAME: &'static str = "Soul Torch";
}
pub struct r#Glowstone;
impl Item for r#Glowstone {
    const ID: i32 = 295i32;
    const NAME: &'static str = "glowstone";
    const DISPLAY_NAME: &'static str = "Glowstone";
}
pub struct r#InfestedStone;
impl Item for r#InfestedStone {
    const ID: i32 = 296i32;
    const NAME: &'static str = "infested_stone";
    const DISPLAY_NAME: &'static str = "Infested Stone";
}
pub struct r#InfestedCobblestone;
impl Item for r#InfestedCobblestone {
    const ID: i32 = 297i32;
    const NAME: &'static str = "infested_cobblestone";
    const DISPLAY_NAME: &'static str = "Infested Cobblestone";
}
pub struct r#InfestedStoneBricks;
impl Item for r#InfestedStoneBricks {
    const ID: i32 = 298i32;
    const NAME: &'static str = "infested_stone_bricks";
    const DISPLAY_NAME: &'static str = "Infested Stone Bricks";
}
pub struct r#InfestedMossyStoneBricks;
impl Item for r#InfestedMossyStoneBricks {
    const ID: i32 = 299i32;
    const NAME: &'static str = "infested_mossy_stone_bricks";
    const DISPLAY_NAME: &'static str = "Infested Mossy Stone Bricks";
}
pub struct r#InfestedCrackedStoneBricks;
impl Item for r#InfestedCrackedStoneBricks {
    const ID: i32 = 300i32;
    const NAME: &'static str = "infested_cracked_stone_bricks";
    const DISPLAY_NAME: &'static str = "Infested Cracked Stone Bricks";
}
pub struct r#InfestedChiseledStoneBricks;
impl Item for r#InfestedChiseledStoneBricks {
    const ID: i32 = 301i32;
    const NAME: &'static str = "infested_chiseled_stone_bricks";
    const DISPLAY_NAME: &'static str = "Infested Chiseled Stone Bricks";
}
pub struct r#InfestedDeepslate;
impl Item for r#InfestedDeepslate {
    const ID: i32 = 302i32;
    const NAME: &'static str = "infested_deepslate";
    const DISPLAY_NAME: &'static str = "Infested Deepslate";
}
pub struct r#StoneBricks;
impl Item for r#StoneBricks {
    const ID: i32 = 303i32;
    const NAME: &'static str = "stone_bricks";
    const DISPLAY_NAME: &'static str = "Stone Bricks";
}
pub struct r#MossyStoneBricks;
impl Item for r#MossyStoneBricks {
    const ID: i32 = 304i32;
    const NAME: &'static str = "mossy_stone_bricks";
    const DISPLAY_NAME: &'static str = "Mossy Stone Bricks";
}
pub struct r#CrackedStoneBricks;
impl Item for r#CrackedStoneBricks {
    const ID: i32 = 305i32;
    const NAME: &'static str = "cracked_stone_bricks";
    const DISPLAY_NAME: &'static str = "Cracked Stone Bricks";
}
pub struct r#ChiseledStoneBricks;
impl Item for r#ChiseledStoneBricks {
    const ID: i32 = 306i32;
    const NAME: &'static str = "chiseled_stone_bricks";
    const DISPLAY_NAME: &'static str = "Chiseled Stone Bricks";
}
pub struct r#PackedMud;
impl Item for r#PackedMud {
    const ID: i32 = 307i32;
    const NAME: &'static str = "packed_mud";
    const DISPLAY_NAME: &'static str = "Packed Mud";
}
pub struct r#MudBricks;
impl Item for r#MudBricks {
    const ID: i32 = 308i32;
    const NAME: &'static str = "mud_bricks";
    const DISPLAY_NAME: &'static str = "Mud Bricks";
}
pub struct r#DeepslateBricks;
impl Item for r#DeepslateBricks {
    const ID: i32 = 309i32;
    const NAME: &'static str = "deepslate_bricks";
    const DISPLAY_NAME: &'static str = "Deepslate Bricks";
}
pub struct r#CrackedDeepslateBricks;
impl Item for r#CrackedDeepslateBricks {
    const ID: i32 = 310i32;
    const NAME: &'static str = "cracked_deepslate_bricks";
    const DISPLAY_NAME: &'static str = "Cracked Deepslate Bricks";
}
pub struct r#DeepslateTiles;
impl Item for r#DeepslateTiles {
    const ID: i32 = 311i32;
    const NAME: &'static str = "deepslate_tiles";
    const DISPLAY_NAME: &'static str = "Deepslate Tiles";
}
pub struct r#CrackedDeepslateTiles;
impl Item for r#CrackedDeepslateTiles {
    const ID: i32 = 312i32;
    const NAME: &'static str = "cracked_deepslate_tiles";
    const DISPLAY_NAME: &'static str = "Cracked Deepslate Tiles";
}
pub struct r#ChiseledDeepslate;
impl Item for r#ChiseledDeepslate {
    const ID: i32 = 313i32;
    const NAME: &'static str = "chiseled_deepslate";
    const DISPLAY_NAME: &'static str = "Chiseled Deepslate";
}
pub struct r#ReinforcedDeepslate;
impl Item for r#ReinforcedDeepslate {
    const ID: i32 = 314i32;
    const NAME: &'static str = "reinforced_deepslate";
    const DISPLAY_NAME: &'static str = "Reinforced Deepslate";
}
pub struct r#BrownMushroomBlock;
impl Item for r#BrownMushroomBlock {
    const ID: i32 = 315i32;
    const NAME: &'static str = "brown_mushroom_block";
    const DISPLAY_NAME: &'static str = "Brown Mushroom Block";
}
pub struct r#RedMushroomBlock;
impl Item for r#RedMushroomBlock {
    const ID: i32 = 316i32;
    const NAME: &'static str = "red_mushroom_block";
    const DISPLAY_NAME: &'static str = "Red Mushroom Block";
}
pub struct r#MushroomStem;
impl Item for r#MushroomStem {
    const ID: i32 = 317i32;
    const NAME: &'static str = "mushroom_stem";
    const DISPLAY_NAME: &'static str = "Mushroom Stem";
}
pub struct r#IronBars;
impl Item for r#IronBars {
    const ID: i32 = 318i32;
    const NAME: &'static str = "iron_bars";
    const DISPLAY_NAME: &'static str = "Iron Bars";
}
pub struct r#Chain;
impl Item for r#Chain {
    const ID: i32 = 319i32;
    const NAME: &'static str = "chain";
    const DISPLAY_NAME: &'static str = "Chain";
}
pub struct r#GlassPane;
impl Item for r#GlassPane {
    const ID: i32 = 320i32;
    const NAME: &'static str = "glass_pane";
    const DISPLAY_NAME: &'static str = "Glass Pane";
}
pub struct r#Melon;
impl Item for r#Melon {
    const ID: i32 = 321i32;
    const NAME: &'static str = "melon";
    const DISPLAY_NAME: &'static str = "Melon";
}
pub struct r#Vine;
impl Item for r#Vine {
    const ID: i32 = 322i32;
    const NAME: &'static str = "vine";
    const DISPLAY_NAME: &'static str = "Vines";
}
pub struct r#GlowLichen;
impl Item for r#GlowLichen {
    const ID: i32 = 323i32;
    const NAME: &'static str = "glow_lichen";
    const DISPLAY_NAME: &'static str = "Glow Lichen";
}
pub struct r#BrickStairs;
impl Item for r#BrickStairs {
    const ID: i32 = 324i32;
    const NAME: &'static str = "brick_stairs";
    const DISPLAY_NAME: &'static str = "Brick Stairs";
}
pub struct r#StoneBrickStairs;
impl Item for r#StoneBrickStairs {
    const ID: i32 = 325i32;
    const NAME: &'static str = "stone_brick_stairs";
    const DISPLAY_NAME: &'static str = "Stone Brick Stairs";
}
pub struct r#MudBrickStairs;
impl Item for r#MudBrickStairs {
    const ID: i32 = 326i32;
    const NAME: &'static str = "mud_brick_stairs";
    const DISPLAY_NAME: &'static str = "Mud Brick Stairs";
}
pub struct r#Mycelium;
impl Item for r#Mycelium {
    const ID: i32 = 327i32;
    const NAME: &'static str = "mycelium";
    const DISPLAY_NAME: &'static str = "Mycelium";
}
pub struct r#LilyPad;
impl Item for r#LilyPad {
    const ID: i32 = 328i32;
    const NAME: &'static str = "lily_pad";
    const DISPLAY_NAME: &'static str = "Lily Pad";
}
pub struct r#NetherBricks;
impl Item for r#NetherBricks {
    const ID: i32 = 329i32;
    const NAME: &'static str = "nether_bricks";
    const DISPLAY_NAME: &'static str = "Nether Bricks";
}
pub struct r#CrackedNetherBricks;
impl Item for r#CrackedNetherBricks {
    const ID: i32 = 330i32;
    const NAME: &'static str = "cracked_nether_bricks";
    const DISPLAY_NAME: &'static str = "Cracked Nether Bricks";
}
pub struct r#ChiseledNetherBricks;
impl Item for r#ChiseledNetherBricks {
    const ID: i32 = 331i32;
    const NAME: &'static str = "chiseled_nether_bricks";
    const DISPLAY_NAME: &'static str = "Chiseled Nether Bricks";
}
pub struct r#NetherBrickFence;
impl Item for r#NetherBrickFence {
    const ID: i32 = 332i32;
    const NAME: &'static str = "nether_brick_fence";
    const DISPLAY_NAME: &'static str = "Nether Brick Fence";
}
pub struct r#NetherBrickStairs;
impl Item for r#NetherBrickStairs {
    const ID: i32 = 333i32;
    const NAME: &'static str = "nether_brick_stairs";
    const DISPLAY_NAME: &'static str = "Nether Brick Stairs";
}
pub struct r#Sculk;
impl Item for r#Sculk {
    const ID: i32 = 334i32;
    const NAME: &'static str = "sculk";
    const DISPLAY_NAME: &'static str = "Sculk";
}
pub struct r#SculkVein;
impl Item for r#SculkVein {
    const ID: i32 = 335i32;
    const NAME: &'static str = "sculk_vein";
    const DISPLAY_NAME: &'static str = "Sculk Vein";
}
pub struct r#SculkCatalyst;
impl Item for r#SculkCatalyst {
    const ID: i32 = 336i32;
    const NAME: &'static str = "sculk_catalyst";
    const DISPLAY_NAME: &'static str = "Sculk Catalyst";
}
pub struct r#SculkShrieker;
impl Item for r#SculkShrieker {
    const ID: i32 = 337i32;
    const NAME: &'static str = "sculk_shrieker";
    const DISPLAY_NAME: &'static str = "Sculk Shrieker";
}
pub struct r#EnchantingTable;
impl Item for r#EnchantingTable {
    const ID: i32 = 338i32;
    const NAME: &'static str = "enchanting_table";
    const DISPLAY_NAME: &'static str = "Enchanting Table";
}
pub struct r#EndPortalFrame;
impl Item for r#EndPortalFrame {
    const ID: i32 = 339i32;
    const NAME: &'static str = "end_portal_frame";
    const DISPLAY_NAME: &'static str = "End Portal Frame";
}
pub struct r#EndStone;
impl Item for r#EndStone {
    const ID: i32 = 340i32;
    const NAME: &'static str = "end_stone";
    const DISPLAY_NAME: &'static str = "End Stone";
}
pub struct r#EndStoneBricks;
impl Item for r#EndStoneBricks {
    const ID: i32 = 341i32;
    const NAME: &'static str = "end_stone_bricks";
    const DISPLAY_NAME: &'static str = "End Stone Bricks";
}
pub struct r#DragonEgg;
impl Item for r#DragonEgg {
    const ID: i32 = 342i32;
    const NAME: &'static str = "dragon_egg";
    const DISPLAY_NAME: &'static str = "Dragon Egg";
}
pub struct r#SandstoneStairs;
impl Item for r#SandstoneStairs {
    const ID: i32 = 343i32;
    const NAME: &'static str = "sandstone_stairs";
    const DISPLAY_NAME: &'static str = "Sandstone Stairs";
}
pub struct r#EnderChest;
impl Item for r#EnderChest {
    const ID: i32 = 344i32;
    const NAME: &'static str = "ender_chest";
    const DISPLAY_NAME: &'static str = "Ender Chest";
}
pub struct r#EmeraldBlock;
impl Item for r#EmeraldBlock {
    const ID: i32 = 345i32;
    const NAME: &'static str = "emerald_block";
    const DISPLAY_NAME: &'static str = "Block of Emerald";
}
pub struct r#OakStairs;
impl Item for r#OakStairs {
    const ID: i32 = 346i32;
    const NAME: &'static str = "oak_stairs";
    const DISPLAY_NAME: &'static str = "Oak Stairs";
}
pub struct r#SpruceStairs;
impl Item for r#SpruceStairs {
    const ID: i32 = 347i32;
    const NAME: &'static str = "spruce_stairs";
    const DISPLAY_NAME: &'static str = "Spruce Stairs";
}
pub struct r#BirchStairs;
impl Item for r#BirchStairs {
    const ID: i32 = 348i32;
    const NAME: &'static str = "birch_stairs";
    const DISPLAY_NAME: &'static str = "Birch Stairs";
}
pub struct r#JungleStairs;
impl Item for r#JungleStairs {
    const ID: i32 = 349i32;
    const NAME: &'static str = "jungle_stairs";
    const DISPLAY_NAME: &'static str = "Jungle Stairs";
}
pub struct r#AcaciaStairs;
impl Item for r#AcaciaStairs {
    const ID: i32 = 350i32;
    const NAME: &'static str = "acacia_stairs";
    const DISPLAY_NAME: &'static str = "Acacia Stairs";
}
pub struct r#DarkOakStairs;
impl Item for r#DarkOakStairs {
    const ID: i32 = 351i32;
    const NAME: &'static str = "dark_oak_stairs";
    const DISPLAY_NAME: &'static str = "Dark Oak Stairs";
}
pub struct r#MangroveStairs;
impl Item for r#MangroveStairs {
    const ID: i32 = 352i32;
    const NAME: &'static str = "mangrove_stairs";
    const DISPLAY_NAME: &'static str = "Mangrove Stairs";
}
pub struct r#BambooStairs;
impl Item for r#BambooStairs {
    const ID: i32 = 353i32;
    const NAME: &'static str = "bamboo_stairs";
    const DISPLAY_NAME: &'static str = "Bamboo Stairs";
}
pub struct r#BambooMosaicStairs;
impl Item for r#BambooMosaicStairs {
    const ID: i32 = 354i32;
    const NAME: &'static str = "bamboo_mosaic_stairs";
    const DISPLAY_NAME: &'static str = "Bamboo Mosaic Stairs";
}
pub struct r#CrimsonStairs;
impl Item for r#CrimsonStairs {
    const ID: i32 = 355i32;
    const NAME: &'static str = "crimson_stairs";
    const DISPLAY_NAME: &'static str = "Crimson Stairs";
}
pub struct r#WarpedStairs;
impl Item for r#WarpedStairs {
    const ID: i32 = 356i32;
    const NAME: &'static str = "warped_stairs";
    const DISPLAY_NAME: &'static str = "Warped Stairs";
}
pub struct r#CommandBlock;
impl Item for r#CommandBlock {
    const ID: i32 = 357i32;
    const NAME: &'static str = "command_block";
    const DISPLAY_NAME: &'static str = "Command Block";
}
pub struct r#Beacon;
impl Item for r#Beacon {
    const ID: i32 = 358i32;
    const NAME: &'static str = "beacon";
    const DISPLAY_NAME: &'static str = "Beacon";
}
pub struct r#CobblestoneWall;
impl Item for r#CobblestoneWall {
    const ID: i32 = 359i32;
    const NAME: &'static str = "cobblestone_wall";
    const DISPLAY_NAME: &'static str = "Cobblestone Wall";
}
pub struct r#MossyCobblestoneWall;
impl Item for r#MossyCobblestoneWall {
    const ID: i32 = 360i32;
    const NAME: &'static str = "mossy_cobblestone_wall";
    const DISPLAY_NAME: &'static str = "Mossy Cobblestone Wall";
}
pub struct r#BrickWall;
impl Item for r#BrickWall {
    const ID: i32 = 361i32;
    const NAME: &'static str = "brick_wall";
    const DISPLAY_NAME: &'static str = "Brick Wall";
}
pub struct r#PrismarineWall;
impl Item for r#PrismarineWall {
    const ID: i32 = 362i32;
    const NAME: &'static str = "prismarine_wall";
    const DISPLAY_NAME: &'static str = "Prismarine Wall";
}
pub struct r#RedSandstoneWall;
impl Item for r#RedSandstoneWall {
    const ID: i32 = 363i32;
    const NAME: &'static str = "red_sandstone_wall";
    const DISPLAY_NAME: &'static str = "Red Sandstone Wall";
}
pub struct r#MossyStoneBrickWall;
impl Item for r#MossyStoneBrickWall {
    const ID: i32 = 364i32;
    const NAME: &'static str = "mossy_stone_brick_wall";
    const DISPLAY_NAME: &'static str = "Mossy Stone Brick Wall";
}
pub struct r#GraniteWall;
impl Item for r#GraniteWall {
    const ID: i32 = 365i32;
    const NAME: &'static str = "granite_wall";
    const DISPLAY_NAME: &'static str = "Granite Wall";
}
pub struct r#StoneBrickWall;
impl Item for r#StoneBrickWall {
    const ID: i32 = 366i32;
    const NAME: &'static str = "stone_brick_wall";
    const DISPLAY_NAME: &'static str = "Stone Brick Wall";
}
pub struct r#MudBrickWall;
impl Item for r#MudBrickWall {
    const ID: i32 = 367i32;
    const NAME: &'static str = "mud_brick_wall";
    const DISPLAY_NAME: &'static str = "Mud Brick Wall";
}
pub struct r#NetherBrickWall;
impl Item for r#NetherBrickWall {
    const ID: i32 = 368i32;
    const NAME: &'static str = "nether_brick_wall";
    const DISPLAY_NAME: &'static str = "Nether Brick Wall";
}
pub struct r#AndesiteWall;
impl Item for r#AndesiteWall {
    const ID: i32 = 369i32;
    const NAME: &'static str = "andesite_wall";
    const DISPLAY_NAME: &'static str = "Andesite Wall";
}
pub struct r#RedNetherBrickWall;
impl Item for r#RedNetherBrickWall {
    const ID: i32 = 370i32;
    const NAME: &'static str = "red_nether_brick_wall";
    const DISPLAY_NAME: &'static str = "Red Nether Brick Wall";
}
pub struct r#SandstoneWall;
impl Item for r#SandstoneWall {
    const ID: i32 = 371i32;
    const NAME: &'static str = "sandstone_wall";
    const DISPLAY_NAME: &'static str = "Sandstone Wall";
}
pub struct r#EndStoneBrickWall;
impl Item for r#EndStoneBrickWall {
    const ID: i32 = 372i32;
    const NAME: &'static str = "end_stone_brick_wall";
    const DISPLAY_NAME: &'static str = "End Stone Brick Wall";
}
pub struct r#DioriteWall;
impl Item for r#DioriteWall {
    const ID: i32 = 373i32;
    const NAME: &'static str = "diorite_wall";
    const DISPLAY_NAME: &'static str = "Diorite Wall";
}
pub struct r#BlackstoneWall;
impl Item for r#BlackstoneWall {
    const ID: i32 = 374i32;
    const NAME: &'static str = "blackstone_wall";
    const DISPLAY_NAME: &'static str = "Blackstone Wall";
}
pub struct r#PolishedBlackstoneWall;
impl Item for r#PolishedBlackstoneWall {
    const ID: i32 = 375i32;
    const NAME: &'static str = "polished_blackstone_wall";
    const DISPLAY_NAME: &'static str = "Polished Blackstone Wall";
}
pub struct r#PolishedBlackstoneBrickWall;
impl Item for r#PolishedBlackstoneBrickWall {
    const ID: i32 = 376i32;
    const NAME: &'static str = "polished_blackstone_brick_wall";
    const DISPLAY_NAME: &'static str = "Polished Blackstone Brick Wall";
}
pub struct r#CobbledDeepslateWall;
impl Item for r#CobbledDeepslateWall {
    const ID: i32 = 377i32;
    const NAME: &'static str = "cobbled_deepslate_wall";
    const DISPLAY_NAME: &'static str = "Cobbled Deepslate Wall";
}
pub struct r#PolishedDeepslateWall;
impl Item for r#PolishedDeepslateWall {
    const ID: i32 = 378i32;
    const NAME: &'static str = "polished_deepslate_wall";
    const DISPLAY_NAME: &'static str = "Polished Deepslate Wall";
}
pub struct r#DeepslateBrickWall;
impl Item for r#DeepslateBrickWall {
    const ID: i32 = 379i32;
    const NAME: &'static str = "deepslate_brick_wall";
    const DISPLAY_NAME: &'static str = "Deepslate Brick Wall";
}
pub struct r#DeepslateTileWall;
impl Item for r#DeepslateTileWall {
    const ID: i32 = 380i32;
    const NAME: &'static str = "deepslate_tile_wall";
    const DISPLAY_NAME: &'static str = "Deepslate Tile Wall";
}
pub struct r#Anvil;
impl Item for r#Anvil {
    const ID: i32 = 381i32;
    const NAME: &'static str = "anvil";
    const DISPLAY_NAME: &'static str = "Anvil";
}
pub struct r#ChippedAnvil;
impl Item for r#ChippedAnvil {
    const ID: i32 = 382i32;
    const NAME: &'static str = "chipped_anvil";
    const DISPLAY_NAME: &'static str = "Chipped Anvil";
}
pub struct r#DamagedAnvil;
impl Item for r#DamagedAnvil {
    const ID: i32 = 383i32;
    const NAME: &'static str = "damaged_anvil";
    const DISPLAY_NAME: &'static str = "Damaged Anvil";
}
pub struct r#ChiseledQuartzBlock;
impl Item for r#ChiseledQuartzBlock {
    const ID: i32 = 384i32;
    const NAME: &'static str = "chiseled_quartz_block";
    const DISPLAY_NAME: &'static str = "Chiseled Quartz Block";
}
pub struct r#QuartzBlock;
impl Item for r#QuartzBlock {
    const ID: i32 = 385i32;
    const NAME: &'static str = "quartz_block";
    const DISPLAY_NAME: &'static str = "Block of Quartz";
}
pub struct r#QuartzBricks;
impl Item for r#QuartzBricks {
    const ID: i32 = 386i32;
    const NAME: &'static str = "quartz_bricks";
    const DISPLAY_NAME: &'static str = "Quartz Bricks";
}
pub struct r#QuartzPillar;
impl Item for r#QuartzPillar {
    const ID: i32 = 387i32;
    const NAME: &'static str = "quartz_pillar";
    const DISPLAY_NAME: &'static str = "Quartz Pillar";
}
pub struct r#QuartzStairs;
impl Item for r#QuartzStairs {
    const ID: i32 = 388i32;
    const NAME: &'static str = "quartz_stairs";
    const DISPLAY_NAME: &'static str = "Quartz Stairs";
}
pub struct r#WhiteTerracotta;
impl Item for r#WhiteTerracotta {
    const ID: i32 = 389i32;
    const NAME: &'static str = "white_terracotta";
    const DISPLAY_NAME: &'static str = "White Terracotta";
}
pub struct r#OrangeTerracotta;
impl Item for r#OrangeTerracotta {
    const ID: i32 = 390i32;
    const NAME: &'static str = "orange_terracotta";
    const DISPLAY_NAME: &'static str = "Orange Terracotta";
}
pub struct r#MagentaTerracotta;
impl Item for r#MagentaTerracotta {
    const ID: i32 = 391i32;
    const NAME: &'static str = "magenta_terracotta";
    const DISPLAY_NAME: &'static str = "Magenta Terracotta";
}
pub struct r#LightBlueTerracotta;
impl Item for r#LightBlueTerracotta {
    const ID: i32 = 392i32;
    const NAME: &'static str = "light_blue_terracotta";
    const DISPLAY_NAME: &'static str = "Light Blue Terracotta";
}
pub struct r#YellowTerracotta;
impl Item for r#YellowTerracotta {
    const ID: i32 = 393i32;
    const NAME: &'static str = "yellow_terracotta";
    const DISPLAY_NAME: &'static str = "Yellow Terracotta";
}
pub struct r#LimeTerracotta;
impl Item for r#LimeTerracotta {
    const ID: i32 = 394i32;
    const NAME: &'static str = "lime_terracotta";
    const DISPLAY_NAME: &'static str = "Lime Terracotta";
}
pub struct r#PinkTerracotta;
impl Item for r#PinkTerracotta {
    const ID: i32 = 395i32;
    const NAME: &'static str = "pink_terracotta";
    const DISPLAY_NAME: &'static str = "Pink Terracotta";
}
pub struct r#GrayTerracotta;
impl Item for r#GrayTerracotta {
    const ID: i32 = 396i32;
    const NAME: &'static str = "gray_terracotta";
    const DISPLAY_NAME: &'static str = "Gray Terracotta";
}
pub struct r#LightGrayTerracotta;
impl Item for r#LightGrayTerracotta {
    const ID: i32 = 397i32;
    const NAME: &'static str = "light_gray_terracotta";
    const DISPLAY_NAME: &'static str = "Light Gray Terracotta";
}
pub struct r#CyanTerracotta;
impl Item for r#CyanTerracotta {
    const ID: i32 = 398i32;
    const NAME: &'static str = "cyan_terracotta";
    const DISPLAY_NAME: &'static str = "Cyan Terracotta";
}
pub struct r#PurpleTerracotta;
impl Item for r#PurpleTerracotta {
    const ID: i32 = 399i32;
    const NAME: &'static str = "purple_terracotta";
    const DISPLAY_NAME: &'static str = "Purple Terracotta";
}
pub struct r#BlueTerracotta;
impl Item for r#BlueTerracotta {
    const ID: i32 = 400i32;
    const NAME: &'static str = "blue_terracotta";
    const DISPLAY_NAME: &'static str = "Blue Terracotta";
}
pub struct r#BrownTerracotta;
impl Item for r#BrownTerracotta {
    const ID: i32 = 401i32;
    const NAME: &'static str = "brown_terracotta";
    const DISPLAY_NAME: &'static str = "Brown Terracotta";
}
pub struct r#GreenTerracotta;
impl Item for r#GreenTerracotta {
    const ID: i32 = 402i32;
    const NAME: &'static str = "green_terracotta";
    const DISPLAY_NAME: &'static str = "Green Terracotta";
}
pub struct r#RedTerracotta;
impl Item for r#RedTerracotta {
    const ID: i32 = 403i32;
    const NAME: &'static str = "red_terracotta";
    const DISPLAY_NAME: &'static str = "Red Terracotta";
}
pub struct r#BlackTerracotta;
impl Item for r#BlackTerracotta {
    const ID: i32 = 404i32;
    const NAME: &'static str = "black_terracotta";
    const DISPLAY_NAME: &'static str = "Black Terracotta";
}
pub struct r#Barrier;
impl Item for r#Barrier {
    const ID: i32 = 405i32;
    const NAME: &'static str = "barrier";
    const DISPLAY_NAME: &'static str = "Barrier";
}
pub struct r#Light;
impl Item for r#Light {
    const ID: i32 = 406i32;
    const NAME: &'static str = "light";
    const DISPLAY_NAME: &'static str = "Light";
}
pub struct r#HayBlock;
impl Item for r#HayBlock {
    const ID: i32 = 407i32;
    const NAME: &'static str = "hay_block";
    const DISPLAY_NAME: &'static str = "Hay Bale";
}
pub struct r#WhiteCarpet;
impl Item for r#WhiteCarpet {
    const ID: i32 = 408i32;
    const NAME: &'static str = "white_carpet";
    const DISPLAY_NAME: &'static str = "White Carpet";
}
pub struct r#OrangeCarpet;
impl Item for r#OrangeCarpet {
    const ID: i32 = 409i32;
    const NAME: &'static str = "orange_carpet";
    const DISPLAY_NAME: &'static str = "Orange Carpet";
}
pub struct r#MagentaCarpet;
impl Item for r#MagentaCarpet {
    const ID: i32 = 410i32;
    const NAME: &'static str = "magenta_carpet";
    const DISPLAY_NAME: &'static str = "Magenta Carpet";
}
pub struct r#LightBlueCarpet;
impl Item for r#LightBlueCarpet {
    const ID: i32 = 411i32;
    const NAME: &'static str = "light_blue_carpet";
    const DISPLAY_NAME: &'static str = "Light Blue Carpet";
}
pub struct r#YellowCarpet;
impl Item for r#YellowCarpet {
    const ID: i32 = 412i32;
    const NAME: &'static str = "yellow_carpet";
    const DISPLAY_NAME: &'static str = "Yellow Carpet";
}
pub struct r#LimeCarpet;
impl Item for r#LimeCarpet {
    const ID: i32 = 413i32;
    const NAME: &'static str = "lime_carpet";
    const DISPLAY_NAME: &'static str = "Lime Carpet";
}
pub struct r#PinkCarpet;
impl Item for r#PinkCarpet {
    const ID: i32 = 414i32;
    const NAME: &'static str = "pink_carpet";
    const DISPLAY_NAME: &'static str = "Pink Carpet";
}
pub struct r#GrayCarpet;
impl Item for r#GrayCarpet {
    const ID: i32 = 415i32;
    const NAME: &'static str = "gray_carpet";
    const DISPLAY_NAME: &'static str = "Gray Carpet";
}
pub struct r#LightGrayCarpet;
impl Item for r#LightGrayCarpet {
    const ID: i32 = 416i32;
    const NAME: &'static str = "light_gray_carpet";
    const DISPLAY_NAME: &'static str = "Light Gray Carpet";
}
pub struct r#CyanCarpet;
impl Item for r#CyanCarpet {
    const ID: i32 = 417i32;
    const NAME: &'static str = "cyan_carpet";
    const DISPLAY_NAME: &'static str = "Cyan Carpet";
}
pub struct r#PurpleCarpet;
impl Item for r#PurpleCarpet {
    const ID: i32 = 418i32;
    const NAME: &'static str = "purple_carpet";
    const DISPLAY_NAME: &'static str = "Purple Carpet";
}
pub struct r#BlueCarpet;
impl Item for r#BlueCarpet {
    const ID: i32 = 419i32;
    const NAME: &'static str = "blue_carpet";
    const DISPLAY_NAME: &'static str = "Blue Carpet";
}
pub struct r#BrownCarpet;
impl Item for r#BrownCarpet {
    const ID: i32 = 420i32;
    const NAME: &'static str = "brown_carpet";
    const DISPLAY_NAME: &'static str = "Brown Carpet";
}
pub struct r#GreenCarpet;
impl Item for r#GreenCarpet {
    const ID: i32 = 421i32;
    const NAME: &'static str = "green_carpet";
    const DISPLAY_NAME: &'static str = "Green Carpet";
}
pub struct r#RedCarpet;
impl Item for r#RedCarpet {
    const ID: i32 = 422i32;
    const NAME: &'static str = "red_carpet";
    const DISPLAY_NAME: &'static str = "Red Carpet";
}
pub struct r#BlackCarpet;
impl Item for r#BlackCarpet {
    const ID: i32 = 423i32;
    const NAME: &'static str = "black_carpet";
    const DISPLAY_NAME: &'static str = "Black Carpet";
}
pub struct r#Terracotta;
impl Item for r#Terracotta {
    const ID: i32 = 424i32;
    const NAME: &'static str = "terracotta";
    const DISPLAY_NAME: &'static str = "Terracotta";
}
pub struct r#PackedIce;
impl Item for r#PackedIce {
    const ID: i32 = 425i32;
    const NAME: &'static str = "packed_ice";
    const DISPLAY_NAME: &'static str = "Packed Ice";
}
pub struct r#DirtPath;
impl Item for r#DirtPath {
    const ID: i32 = 426i32;
    const NAME: &'static str = "dirt_path";
    const DISPLAY_NAME: &'static str = "Dirt Path";
}
pub struct r#Sunflower;
impl Item for r#Sunflower {
    const ID: i32 = 427i32;
    const NAME: &'static str = "sunflower";
    const DISPLAY_NAME: &'static str = "Sunflower";
}
pub struct r#Lilac;
impl Item for r#Lilac {
    const ID: i32 = 428i32;
    const NAME: &'static str = "lilac";
    const DISPLAY_NAME: &'static str = "Lilac";
}
pub struct r#RoseBush;
impl Item for r#RoseBush {
    const ID: i32 = 429i32;
    const NAME: &'static str = "rose_bush";
    const DISPLAY_NAME: &'static str = "Rose Bush";
}
pub struct r#Peony;
impl Item for r#Peony {
    const ID: i32 = 430i32;
    const NAME: &'static str = "peony";
    const DISPLAY_NAME: &'static str = "Peony";
}
pub struct r#TallGrass;
impl Item for r#TallGrass {
    const ID: i32 = 431i32;
    const NAME: &'static str = "tall_grass";
    const DISPLAY_NAME: &'static str = "Tall Grass";
}
pub struct r#LargeFern;
impl Item for r#LargeFern {
    const ID: i32 = 432i32;
    const NAME: &'static str = "large_fern";
    const DISPLAY_NAME: &'static str = "Large Fern";
}
pub struct r#WhiteStainedGlass;
impl Item for r#WhiteStainedGlass {
    const ID: i32 = 433i32;
    const NAME: &'static str = "white_stained_glass";
    const DISPLAY_NAME: &'static str = "White Stained Glass";
}
pub struct r#OrangeStainedGlass;
impl Item for r#OrangeStainedGlass {
    const ID: i32 = 434i32;
    const NAME: &'static str = "orange_stained_glass";
    const DISPLAY_NAME: &'static str = "Orange Stained Glass";
}
pub struct r#MagentaStainedGlass;
impl Item for r#MagentaStainedGlass {
    const ID: i32 = 435i32;
    const NAME: &'static str = "magenta_stained_glass";
    const DISPLAY_NAME: &'static str = "Magenta Stained Glass";
}
pub struct r#LightBlueStainedGlass;
impl Item for r#LightBlueStainedGlass {
    const ID: i32 = 436i32;
    const NAME: &'static str = "light_blue_stained_glass";
    const DISPLAY_NAME: &'static str = "Light Blue Stained Glass";
}
pub struct r#YellowStainedGlass;
impl Item for r#YellowStainedGlass {
    const ID: i32 = 437i32;
    const NAME: &'static str = "yellow_stained_glass";
    const DISPLAY_NAME: &'static str = "Yellow Stained Glass";
}
pub struct r#LimeStainedGlass;
impl Item for r#LimeStainedGlass {
    const ID: i32 = 438i32;
    const NAME: &'static str = "lime_stained_glass";
    const DISPLAY_NAME: &'static str = "Lime Stained Glass";
}
pub struct r#PinkStainedGlass;
impl Item for r#PinkStainedGlass {
    const ID: i32 = 439i32;
    const NAME: &'static str = "pink_stained_glass";
    const DISPLAY_NAME: &'static str = "Pink Stained Glass";
}
pub struct r#GrayStainedGlass;
impl Item for r#GrayStainedGlass {
    const ID: i32 = 440i32;
    const NAME: &'static str = "gray_stained_glass";
    const DISPLAY_NAME: &'static str = "Gray Stained Glass";
}
pub struct r#LightGrayStainedGlass;
impl Item for r#LightGrayStainedGlass {
    const ID: i32 = 441i32;
    const NAME: &'static str = "light_gray_stained_glass";
    const DISPLAY_NAME: &'static str = "Light Gray Stained Glass";
}
pub struct r#CyanStainedGlass;
impl Item for r#CyanStainedGlass {
    const ID: i32 = 442i32;
    const NAME: &'static str = "cyan_stained_glass";
    const DISPLAY_NAME: &'static str = "Cyan Stained Glass";
}
pub struct r#PurpleStainedGlass;
impl Item for r#PurpleStainedGlass {
    const ID: i32 = 443i32;
    const NAME: &'static str = "purple_stained_glass";
    const DISPLAY_NAME: &'static str = "Purple Stained Glass";
}
pub struct r#BlueStainedGlass;
impl Item for r#BlueStainedGlass {
    const ID: i32 = 444i32;
    const NAME: &'static str = "blue_stained_glass";
    const DISPLAY_NAME: &'static str = "Blue Stained Glass";
}
pub struct r#BrownStainedGlass;
impl Item for r#BrownStainedGlass {
    const ID: i32 = 445i32;
    const NAME: &'static str = "brown_stained_glass";
    const DISPLAY_NAME: &'static str = "Brown Stained Glass";
}
pub struct r#GreenStainedGlass;
impl Item for r#GreenStainedGlass {
    const ID: i32 = 446i32;
    const NAME: &'static str = "green_stained_glass";
    const DISPLAY_NAME: &'static str = "Green Stained Glass";
}
pub struct r#RedStainedGlass;
impl Item for r#RedStainedGlass {
    const ID: i32 = 447i32;
    const NAME: &'static str = "red_stained_glass";
    const DISPLAY_NAME: &'static str = "Red Stained Glass";
}
pub struct r#BlackStainedGlass;
impl Item for r#BlackStainedGlass {
    const ID: i32 = 448i32;
    const NAME: &'static str = "black_stained_glass";
    const DISPLAY_NAME: &'static str = "Black Stained Glass";
}
pub struct r#WhiteStainedGlassPane;
impl Item for r#WhiteStainedGlassPane {
    const ID: i32 = 449i32;
    const NAME: &'static str = "white_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "White Stained Glass Pane";
}
pub struct r#OrangeStainedGlassPane;
impl Item for r#OrangeStainedGlassPane {
    const ID: i32 = 450i32;
    const NAME: &'static str = "orange_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Orange Stained Glass Pane";
}
pub struct r#MagentaStainedGlassPane;
impl Item for r#MagentaStainedGlassPane {
    const ID: i32 = 451i32;
    const NAME: &'static str = "magenta_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Magenta Stained Glass Pane";
}
pub struct r#LightBlueStainedGlassPane;
impl Item for r#LightBlueStainedGlassPane {
    const ID: i32 = 452i32;
    const NAME: &'static str = "light_blue_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Light Blue Stained Glass Pane";
}
pub struct r#YellowStainedGlassPane;
impl Item for r#YellowStainedGlassPane {
    const ID: i32 = 453i32;
    const NAME: &'static str = "yellow_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Yellow Stained Glass Pane";
}
pub struct r#LimeStainedGlassPane;
impl Item for r#LimeStainedGlassPane {
    const ID: i32 = 454i32;
    const NAME: &'static str = "lime_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Lime Stained Glass Pane";
}
pub struct r#PinkStainedGlassPane;
impl Item for r#PinkStainedGlassPane {
    const ID: i32 = 455i32;
    const NAME: &'static str = "pink_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Pink Stained Glass Pane";
}
pub struct r#GrayStainedGlassPane;
impl Item for r#GrayStainedGlassPane {
    const ID: i32 = 456i32;
    const NAME: &'static str = "gray_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Gray Stained Glass Pane";
}
pub struct r#LightGrayStainedGlassPane;
impl Item for r#LightGrayStainedGlassPane {
    const ID: i32 = 457i32;
    const NAME: &'static str = "light_gray_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Light Gray Stained Glass Pane";
}
pub struct r#CyanStainedGlassPane;
impl Item for r#CyanStainedGlassPane {
    const ID: i32 = 458i32;
    const NAME: &'static str = "cyan_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Cyan Stained Glass Pane";
}
pub struct r#PurpleStainedGlassPane;
impl Item for r#PurpleStainedGlassPane {
    const ID: i32 = 459i32;
    const NAME: &'static str = "purple_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Purple Stained Glass Pane";
}
pub struct r#BlueStainedGlassPane;
impl Item for r#BlueStainedGlassPane {
    const ID: i32 = 460i32;
    const NAME: &'static str = "blue_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Blue Stained Glass Pane";
}
pub struct r#BrownStainedGlassPane;
impl Item for r#BrownStainedGlassPane {
    const ID: i32 = 461i32;
    const NAME: &'static str = "brown_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Brown Stained Glass Pane";
}
pub struct r#GreenStainedGlassPane;
impl Item for r#GreenStainedGlassPane {
    const ID: i32 = 462i32;
    const NAME: &'static str = "green_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Green Stained Glass Pane";
}
pub struct r#RedStainedGlassPane;
impl Item for r#RedStainedGlassPane {
    const ID: i32 = 463i32;
    const NAME: &'static str = "red_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Red Stained Glass Pane";
}
pub struct r#BlackStainedGlassPane;
impl Item for r#BlackStainedGlassPane {
    const ID: i32 = 464i32;
    const NAME: &'static str = "black_stained_glass_pane";
    const DISPLAY_NAME: &'static str = "Black Stained Glass Pane";
}
pub struct r#Prismarine;
impl Item for r#Prismarine {
    const ID: i32 = 465i32;
    const NAME: &'static str = "prismarine";
    const DISPLAY_NAME: &'static str = "Prismarine";
}
pub struct r#PrismarineBricks;
impl Item for r#PrismarineBricks {
    const ID: i32 = 466i32;
    const NAME: &'static str = "prismarine_bricks";
    const DISPLAY_NAME: &'static str = "Prismarine Bricks";
}
pub struct r#DarkPrismarine;
impl Item for r#DarkPrismarine {
    const ID: i32 = 467i32;
    const NAME: &'static str = "dark_prismarine";
    const DISPLAY_NAME: &'static str = "Dark Prismarine";
}
pub struct r#PrismarineStairs;
impl Item for r#PrismarineStairs {
    const ID: i32 = 468i32;
    const NAME: &'static str = "prismarine_stairs";
    const DISPLAY_NAME: &'static str = "Prismarine Stairs";
}
pub struct r#PrismarineBrickStairs;
impl Item for r#PrismarineBrickStairs {
    const ID: i32 = 469i32;
    const NAME: &'static str = "prismarine_brick_stairs";
    const DISPLAY_NAME: &'static str = "Prismarine Brick Stairs";
}
pub struct r#DarkPrismarineStairs;
impl Item for r#DarkPrismarineStairs {
    const ID: i32 = 470i32;
    const NAME: &'static str = "dark_prismarine_stairs";
    const DISPLAY_NAME: &'static str = "Dark Prismarine Stairs";
}
pub struct r#SeaLantern;
impl Item for r#SeaLantern {
    const ID: i32 = 471i32;
    const NAME: &'static str = "sea_lantern";
    const DISPLAY_NAME: &'static str = "Sea Lantern";
}
pub struct r#RedSandstone;
impl Item for r#RedSandstone {
    const ID: i32 = 472i32;
    const NAME: &'static str = "red_sandstone";
    const DISPLAY_NAME: &'static str = "Red Sandstone";
}
pub struct r#ChiseledRedSandstone;
impl Item for r#ChiseledRedSandstone {
    const ID: i32 = 473i32;
    const NAME: &'static str = "chiseled_red_sandstone";
    const DISPLAY_NAME: &'static str = "Chiseled Red Sandstone";
}
pub struct r#CutRedSandstone;
impl Item for r#CutRedSandstone {
    const ID: i32 = 474i32;
    const NAME: &'static str = "cut_red_sandstone";
    const DISPLAY_NAME: &'static str = "Cut Red Sandstone";
}
pub struct r#RedSandstoneStairs;
impl Item for r#RedSandstoneStairs {
    const ID: i32 = 475i32;
    const NAME: &'static str = "red_sandstone_stairs";
    const DISPLAY_NAME: &'static str = "Red Sandstone Stairs";
}
pub struct r#RepeatingCommandBlock;
impl Item for r#RepeatingCommandBlock {
    const ID: i32 = 476i32;
    const NAME: &'static str = "repeating_command_block";
    const DISPLAY_NAME: &'static str = "Repeating Command Block";
}
pub struct r#ChainCommandBlock;
impl Item for r#ChainCommandBlock {
    const ID: i32 = 477i32;
    const NAME: &'static str = "chain_command_block";
    const DISPLAY_NAME: &'static str = "Chain Command Block";
}
pub struct r#MagmaBlock;
impl Item for r#MagmaBlock {
    const ID: i32 = 478i32;
    const NAME: &'static str = "magma_block";
    const DISPLAY_NAME: &'static str = "Magma Block";
}
pub struct r#NetherWartBlock;
impl Item for r#NetherWartBlock {
    const ID: i32 = 479i32;
    const NAME: &'static str = "nether_wart_block";
    const DISPLAY_NAME: &'static str = "Nether Wart Block";
}
pub struct r#WarpedWartBlock;
impl Item for r#WarpedWartBlock {
    const ID: i32 = 480i32;
    const NAME: &'static str = "warped_wart_block";
    const DISPLAY_NAME: &'static str = "Warped Wart Block";
}
pub struct r#RedNetherBricks;
impl Item for r#RedNetherBricks {
    const ID: i32 = 481i32;
    const NAME: &'static str = "red_nether_bricks";
    const DISPLAY_NAME: &'static str = "Red Nether Bricks";
}
pub struct r#BoneBlock;
impl Item for r#BoneBlock {
    const ID: i32 = 482i32;
    const NAME: &'static str = "bone_block";
    const DISPLAY_NAME: &'static str = "Bone Block";
}
pub struct r#StructureVoid;
impl Item for r#StructureVoid {
    const ID: i32 = 483i32;
    const NAME: &'static str = "structure_void";
    const DISPLAY_NAME: &'static str = "Structure Void";
}
pub struct r#ShulkerBox;
impl Item for r#ShulkerBox {
    const ID: i32 = 484i32;
    const NAME: &'static str = "shulker_box";
    const DISPLAY_NAME: &'static str = "Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#WhiteShulkerBox;
impl Item for r#WhiteShulkerBox {
    const ID: i32 = 485i32;
    const NAME: &'static str = "white_shulker_box";
    const DISPLAY_NAME: &'static str = "White Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#OrangeShulkerBox;
impl Item for r#OrangeShulkerBox {
    const ID: i32 = 486i32;
    const NAME: &'static str = "orange_shulker_box";
    const DISPLAY_NAME: &'static str = "Orange Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MagentaShulkerBox;
impl Item for r#MagentaShulkerBox {
    const ID: i32 = 487i32;
    const NAME: &'static str = "magenta_shulker_box";
    const DISPLAY_NAME: &'static str = "Magenta Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#LightBlueShulkerBox;
impl Item for r#LightBlueShulkerBox {
    const ID: i32 = 488i32;
    const NAME: &'static str = "light_blue_shulker_box";
    const DISPLAY_NAME: &'static str = "Light Blue Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#YellowShulkerBox;
impl Item for r#YellowShulkerBox {
    const ID: i32 = 489i32;
    const NAME: &'static str = "yellow_shulker_box";
    const DISPLAY_NAME: &'static str = "Yellow Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#LimeShulkerBox;
impl Item for r#LimeShulkerBox {
    const ID: i32 = 490i32;
    const NAME: &'static str = "lime_shulker_box";
    const DISPLAY_NAME: &'static str = "Lime Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#PinkShulkerBox;
impl Item for r#PinkShulkerBox {
    const ID: i32 = 491i32;
    const NAME: &'static str = "pink_shulker_box";
    const DISPLAY_NAME: &'static str = "Pink Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GrayShulkerBox;
impl Item for r#GrayShulkerBox {
    const ID: i32 = 492i32;
    const NAME: &'static str = "gray_shulker_box";
    const DISPLAY_NAME: &'static str = "Gray Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#LightGrayShulkerBox;
impl Item for r#LightGrayShulkerBox {
    const ID: i32 = 493i32;
    const NAME: &'static str = "light_gray_shulker_box";
    const DISPLAY_NAME: &'static str = "Light Gray Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#CyanShulkerBox;
impl Item for r#CyanShulkerBox {
    const ID: i32 = 494i32;
    const NAME: &'static str = "cyan_shulker_box";
    const DISPLAY_NAME: &'static str = "Cyan Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#PurpleShulkerBox;
impl Item for r#PurpleShulkerBox {
    const ID: i32 = 495i32;
    const NAME: &'static str = "purple_shulker_box";
    const DISPLAY_NAME: &'static str = "Purple Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#BlueShulkerBox;
impl Item for r#BlueShulkerBox {
    const ID: i32 = 496i32;
    const NAME: &'static str = "blue_shulker_box";
    const DISPLAY_NAME: &'static str = "Blue Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#BrownShulkerBox;
impl Item for r#BrownShulkerBox {
    const ID: i32 = 497i32;
    const NAME: &'static str = "brown_shulker_box";
    const DISPLAY_NAME: &'static str = "Brown Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GreenShulkerBox;
impl Item for r#GreenShulkerBox {
    const ID: i32 = 498i32;
    const NAME: &'static str = "green_shulker_box";
    const DISPLAY_NAME: &'static str = "Green Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#RedShulkerBox;
impl Item for r#RedShulkerBox {
    const ID: i32 = 499i32;
    const NAME: &'static str = "red_shulker_box";
    const DISPLAY_NAME: &'static str = "Red Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#BlackShulkerBox;
impl Item for r#BlackShulkerBox {
    const ID: i32 = 500i32;
    const NAME: &'static str = "black_shulker_box";
    const DISPLAY_NAME: &'static str = "Black Shulker Box";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#WhiteGlazedTerracotta;
impl Item for r#WhiteGlazedTerracotta {
    const ID: i32 = 501i32;
    const NAME: &'static str = "white_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "White Glazed Terracotta";
}
pub struct r#OrangeGlazedTerracotta;
impl Item for r#OrangeGlazedTerracotta {
    const ID: i32 = 502i32;
    const NAME: &'static str = "orange_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Orange Glazed Terracotta";
}
pub struct r#MagentaGlazedTerracotta;
impl Item for r#MagentaGlazedTerracotta {
    const ID: i32 = 503i32;
    const NAME: &'static str = "magenta_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Magenta Glazed Terracotta";
}
pub struct r#LightBlueGlazedTerracotta;
impl Item for r#LightBlueGlazedTerracotta {
    const ID: i32 = 504i32;
    const NAME: &'static str = "light_blue_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Light Blue Glazed Terracotta";
}
pub struct r#YellowGlazedTerracotta;
impl Item for r#YellowGlazedTerracotta {
    const ID: i32 = 505i32;
    const NAME: &'static str = "yellow_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Yellow Glazed Terracotta";
}
pub struct r#LimeGlazedTerracotta;
impl Item for r#LimeGlazedTerracotta {
    const ID: i32 = 506i32;
    const NAME: &'static str = "lime_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Lime Glazed Terracotta";
}
pub struct r#PinkGlazedTerracotta;
impl Item for r#PinkGlazedTerracotta {
    const ID: i32 = 507i32;
    const NAME: &'static str = "pink_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Pink Glazed Terracotta";
}
pub struct r#GrayGlazedTerracotta;
impl Item for r#GrayGlazedTerracotta {
    const ID: i32 = 508i32;
    const NAME: &'static str = "gray_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Gray Glazed Terracotta";
}
pub struct r#LightGrayGlazedTerracotta;
impl Item for r#LightGrayGlazedTerracotta {
    const ID: i32 = 509i32;
    const NAME: &'static str = "light_gray_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Light Gray Glazed Terracotta";
}
pub struct r#CyanGlazedTerracotta;
impl Item for r#CyanGlazedTerracotta {
    const ID: i32 = 510i32;
    const NAME: &'static str = "cyan_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Cyan Glazed Terracotta";
}
pub struct r#PurpleGlazedTerracotta;
impl Item for r#PurpleGlazedTerracotta {
    const ID: i32 = 511i32;
    const NAME: &'static str = "purple_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Purple Glazed Terracotta";
}
pub struct r#BlueGlazedTerracotta;
impl Item for r#BlueGlazedTerracotta {
    const ID: i32 = 512i32;
    const NAME: &'static str = "blue_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Blue Glazed Terracotta";
}
pub struct r#BrownGlazedTerracotta;
impl Item for r#BrownGlazedTerracotta {
    const ID: i32 = 513i32;
    const NAME: &'static str = "brown_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Brown Glazed Terracotta";
}
pub struct r#GreenGlazedTerracotta;
impl Item for r#GreenGlazedTerracotta {
    const ID: i32 = 514i32;
    const NAME: &'static str = "green_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Green Glazed Terracotta";
}
pub struct r#RedGlazedTerracotta;
impl Item for r#RedGlazedTerracotta {
    const ID: i32 = 515i32;
    const NAME: &'static str = "red_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Red Glazed Terracotta";
}
pub struct r#BlackGlazedTerracotta;
impl Item for r#BlackGlazedTerracotta {
    const ID: i32 = 516i32;
    const NAME: &'static str = "black_glazed_terracotta";
    const DISPLAY_NAME: &'static str = "Black Glazed Terracotta";
}
pub struct r#WhiteConcrete;
impl Item for r#WhiteConcrete {
    const ID: i32 = 517i32;
    const NAME: &'static str = "white_concrete";
    const DISPLAY_NAME: &'static str = "White Concrete";
}
pub struct r#OrangeConcrete;
impl Item for r#OrangeConcrete {
    const ID: i32 = 518i32;
    const NAME: &'static str = "orange_concrete";
    const DISPLAY_NAME: &'static str = "Orange Concrete";
}
pub struct r#MagentaConcrete;
impl Item for r#MagentaConcrete {
    const ID: i32 = 519i32;
    const NAME: &'static str = "magenta_concrete";
    const DISPLAY_NAME: &'static str = "Magenta Concrete";
}
pub struct r#LightBlueConcrete;
impl Item for r#LightBlueConcrete {
    const ID: i32 = 520i32;
    const NAME: &'static str = "light_blue_concrete";
    const DISPLAY_NAME: &'static str = "Light Blue Concrete";
}
pub struct r#YellowConcrete;
impl Item for r#YellowConcrete {
    const ID: i32 = 521i32;
    const NAME: &'static str = "yellow_concrete";
    const DISPLAY_NAME: &'static str = "Yellow Concrete";
}
pub struct r#LimeConcrete;
impl Item for r#LimeConcrete {
    const ID: i32 = 522i32;
    const NAME: &'static str = "lime_concrete";
    const DISPLAY_NAME: &'static str = "Lime Concrete";
}
pub struct r#PinkConcrete;
impl Item for r#PinkConcrete {
    const ID: i32 = 523i32;
    const NAME: &'static str = "pink_concrete";
    const DISPLAY_NAME: &'static str = "Pink Concrete";
}
pub struct r#GrayConcrete;
impl Item for r#GrayConcrete {
    const ID: i32 = 524i32;
    const NAME: &'static str = "gray_concrete";
    const DISPLAY_NAME: &'static str = "Gray Concrete";
}
pub struct r#LightGrayConcrete;
impl Item for r#LightGrayConcrete {
    const ID: i32 = 525i32;
    const NAME: &'static str = "light_gray_concrete";
    const DISPLAY_NAME: &'static str = "Light Gray Concrete";
}
pub struct r#CyanConcrete;
impl Item for r#CyanConcrete {
    const ID: i32 = 526i32;
    const NAME: &'static str = "cyan_concrete";
    const DISPLAY_NAME: &'static str = "Cyan Concrete";
}
pub struct r#PurpleConcrete;
impl Item for r#PurpleConcrete {
    const ID: i32 = 527i32;
    const NAME: &'static str = "purple_concrete";
    const DISPLAY_NAME: &'static str = "Purple Concrete";
}
pub struct r#BlueConcrete;
impl Item for r#BlueConcrete {
    const ID: i32 = 528i32;
    const NAME: &'static str = "blue_concrete";
    const DISPLAY_NAME: &'static str = "Blue Concrete";
}
pub struct r#BrownConcrete;
impl Item for r#BrownConcrete {
    const ID: i32 = 529i32;
    const NAME: &'static str = "brown_concrete";
    const DISPLAY_NAME: &'static str = "Brown Concrete";
}
pub struct r#GreenConcrete;
impl Item for r#GreenConcrete {
    const ID: i32 = 530i32;
    const NAME: &'static str = "green_concrete";
    const DISPLAY_NAME: &'static str = "Green Concrete";
}
pub struct r#RedConcrete;
impl Item for r#RedConcrete {
    const ID: i32 = 531i32;
    const NAME: &'static str = "red_concrete";
    const DISPLAY_NAME: &'static str = "Red Concrete";
}
pub struct r#BlackConcrete;
impl Item for r#BlackConcrete {
    const ID: i32 = 532i32;
    const NAME: &'static str = "black_concrete";
    const DISPLAY_NAME: &'static str = "Black Concrete";
}
pub struct r#WhiteConcretePowder;
impl Item for r#WhiteConcretePowder {
    const ID: i32 = 533i32;
    const NAME: &'static str = "white_concrete_powder";
    const DISPLAY_NAME: &'static str = "White Concrete Powder";
}
pub struct r#OrangeConcretePowder;
impl Item for r#OrangeConcretePowder {
    const ID: i32 = 534i32;
    const NAME: &'static str = "orange_concrete_powder";
    const DISPLAY_NAME: &'static str = "Orange Concrete Powder";
}
pub struct r#MagentaConcretePowder;
impl Item for r#MagentaConcretePowder {
    const ID: i32 = 535i32;
    const NAME: &'static str = "magenta_concrete_powder";
    const DISPLAY_NAME: &'static str = "Magenta Concrete Powder";
}
pub struct r#LightBlueConcretePowder;
impl Item for r#LightBlueConcretePowder {
    const ID: i32 = 536i32;
    const NAME: &'static str = "light_blue_concrete_powder";
    const DISPLAY_NAME: &'static str = "Light Blue Concrete Powder";
}
pub struct r#YellowConcretePowder;
impl Item for r#YellowConcretePowder {
    const ID: i32 = 537i32;
    const NAME: &'static str = "yellow_concrete_powder";
    const DISPLAY_NAME: &'static str = "Yellow Concrete Powder";
}
pub struct r#LimeConcretePowder;
impl Item for r#LimeConcretePowder {
    const ID: i32 = 538i32;
    const NAME: &'static str = "lime_concrete_powder";
    const DISPLAY_NAME: &'static str = "Lime Concrete Powder";
}
pub struct r#PinkConcretePowder;
impl Item for r#PinkConcretePowder {
    const ID: i32 = 539i32;
    const NAME: &'static str = "pink_concrete_powder";
    const DISPLAY_NAME: &'static str = "Pink Concrete Powder";
}
pub struct r#GrayConcretePowder;
impl Item for r#GrayConcretePowder {
    const ID: i32 = 540i32;
    const NAME: &'static str = "gray_concrete_powder";
    const DISPLAY_NAME: &'static str = "Gray Concrete Powder";
}
pub struct r#LightGrayConcretePowder;
impl Item for r#LightGrayConcretePowder {
    const ID: i32 = 541i32;
    const NAME: &'static str = "light_gray_concrete_powder";
    const DISPLAY_NAME: &'static str = "Light Gray Concrete Powder";
}
pub struct r#CyanConcretePowder;
impl Item for r#CyanConcretePowder {
    const ID: i32 = 542i32;
    const NAME: &'static str = "cyan_concrete_powder";
    const DISPLAY_NAME: &'static str = "Cyan Concrete Powder";
}
pub struct r#PurpleConcretePowder;
impl Item for r#PurpleConcretePowder {
    const ID: i32 = 543i32;
    const NAME: &'static str = "purple_concrete_powder";
    const DISPLAY_NAME: &'static str = "Purple Concrete Powder";
}
pub struct r#BlueConcretePowder;
impl Item for r#BlueConcretePowder {
    const ID: i32 = 544i32;
    const NAME: &'static str = "blue_concrete_powder";
    const DISPLAY_NAME: &'static str = "Blue Concrete Powder";
}
pub struct r#BrownConcretePowder;
impl Item for r#BrownConcretePowder {
    const ID: i32 = 545i32;
    const NAME: &'static str = "brown_concrete_powder";
    const DISPLAY_NAME: &'static str = "Brown Concrete Powder";
}
pub struct r#GreenConcretePowder;
impl Item for r#GreenConcretePowder {
    const ID: i32 = 546i32;
    const NAME: &'static str = "green_concrete_powder";
    const DISPLAY_NAME: &'static str = "Green Concrete Powder";
}
pub struct r#RedConcretePowder;
impl Item for r#RedConcretePowder {
    const ID: i32 = 547i32;
    const NAME: &'static str = "red_concrete_powder";
    const DISPLAY_NAME: &'static str = "Red Concrete Powder";
}
pub struct r#BlackConcretePowder;
impl Item for r#BlackConcretePowder {
    const ID: i32 = 548i32;
    const NAME: &'static str = "black_concrete_powder";
    const DISPLAY_NAME: &'static str = "Black Concrete Powder";
}
pub struct r#TurtleEgg;
impl Item for r#TurtleEgg {
    const ID: i32 = 549i32;
    const NAME: &'static str = "turtle_egg";
    const DISPLAY_NAME: &'static str = "Turtle Egg";
}
pub struct r#DeadTubeCoralBlock;
impl Item for r#DeadTubeCoralBlock {
    const ID: i32 = 550i32;
    const NAME: &'static str = "dead_tube_coral_block";
    const DISPLAY_NAME: &'static str = "Dead Tube Coral Block";
}
pub struct r#DeadBrainCoralBlock;
impl Item for r#DeadBrainCoralBlock {
    const ID: i32 = 551i32;
    const NAME: &'static str = "dead_brain_coral_block";
    const DISPLAY_NAME: &'static str = "Dead Brain Coral Block";
}
pub struct r#DeadBubbleCoralBlock;
impl Item for r#DeadBubbleCoralBlock {
    const ID: i32 = 552i32;
    const NAME: &'static str = "dead_bubble_coral_block";
    const DISPLAY_NAME: &'static str = "Dead Bubble Coral Block";
}
pub struct r#DeadFireCoralBlock;
impl Item for r#DeadFireCoralBlock {
    const ID: i32 = 553i32;
    const NAME: &'static str = "dead_fire_coral_block";
    const DISPLAY_NAME: &'static str = "Dead Fire Coral Block";
}
pub struct r#DeadHornCoralBlock;
impl Item for r#DeadHornCoralBlock {
    const ID: i32 = 554i32;
    const NAME: &'static str = "dead_horn_coral_block";
    const DISPLAY_NAME: &'static str = "Dead Horn Coral Block";
}
pub struct r#TubeCoralBlock;
impl Item for r#TubeCoralBlock {
    const ID: i32 = 555i32;
    const NAME: &'static str = "tube_coral_block";
    const DISPLAY_NAME: &'static str = "Tube Coral Block";
}
pub struct r#BrainCoralBlock;
impl Item for r#BrainCoralBlock {
    const ID: i32 = 556i32;
    const NAME: &'static str = "brain_coral_block";
    const DISPLAY_NAME: &'static str = "Brain Coral Block";
}
pub struct r#BubbleCoralBlock;
impl Item for r#BubbleCoralBlock {
    const ID: i32 = 557i32;
    const NAME: &'static str = "bubble_coral_block";
    const DISPLAY_NAME: &'static str = "Bubble Coral Block";
}
pub struct r#FireCoralBlock;
impl Item for r#FireCoralBlock {
    const ID: i32 = 558i32;
    const NAME: &'static str = "fire_coral_block";
    const DISPLAY_NAME: &'static str = "Fire Coral Block";
}
pub struct r#HornCoralBlock;
impl Item for r#HornCoralBlock {
    const ID: i32 = 559i32;
    const NAME: &'static str = "horn_coral_block";
    const DISPLAY_NAME: &'static str = "Horn Coral Block";
}
pub struct r#TubeCoral;
impl Item for r#TubeCoral {
    const ID: i32 = 560i32;
    const NAME: &'static str = "tube_coral";
    const DISPLAY_NAME: &'static str = "Tube Coral";
}
pub struct r#BrainCoral;
impl Item for r#BrainCoral {
    const ID: i32 = 561i32;
    const NAME: &'static str = "brain_coral";
    const DISPLAY_NAME: &'static str = "Brain Coral";
}
pub struct r#BubbleCoral;
impl Item for r#BubbleCoral {
    const ID: i32 = 562i32;
    const NAME: &'static str = "bubble_coral";
    const DISPLAY_NAME: &'static str = "Bubble Coral";
}
pub struct r#FireCoral;
impl Item for r#FireCoral {
    const ID: i32 = 563i32;
    const NAME: &'static str = "fire_coral";
    const DISPLAY_NAME: &'static str = "Fire Coral";
}
pub struct r#HornCoral;
impl Item for r#HornCoral {
    const ID: i32 = 564i32;
    const NAME: &'static str = "horn_coral";
    const DISPLAY_NAME: &'static str = "Horn Coral";
}
pub struct r#DeadBrainCoral;
impl Item for r#DeadBrainCoral {
    const ID: i32 = 565i32;
    const NAME: &'static str = "dead_brain_coral";
    const DISPLAY_NAME: &'static str = "Dead Brain Coral";
}
pub struct r#DeadBubbleCoral;
impl Item for r#DeadBubbleCoral {
    const ID: i32 = 566i32;
    const NAME: &'static str = "dead_bubble_coral";
    const DISPLAY_NAME: &'static str = "Dead Bubble Coral";
}
pub struct r#DeadFireCoral;
impl Item for r#DeadFireCoral {
    const ID: i32 = 567i32;
    const NAME: &'static str = "dead_fire_coral";
    const DISPLAY_NAME: &'static str = "Dead Fire Coral";
}
pub struct r#DeadHornCoral;
impl Item for r#DeadHornCoral {
    const ID: i32 = 568i32;
    const NAME: &'static str = "dead_horn_coral";
    const DISPLAY_NAME: &'static str = "Dead Horn Coral";
}
pub struct r#DeadTubeCoral;
impl Item for r#DeadTubeCoral {
    const ID: i32 = 569i32;
    const NAME: &'static str = "dead_tube_coral";
    const DISPLAY_NAME: &'static str = "Dead Tube Coral";
}
pub struct r#TubeCoralFan;
impl Item for r#TubeCoralFan {
    const ID: i32 = 570i32;
    const NAME: &'static str = "tube_coral_fan";
    const DISPLAY_NAME: &'static str = "Tube Coral Fan";
}
pub struct r#BrainCoralFan;
impl Item for r#BrainCoralFan {
    const ID: i32 = 571i32;
    const NAME: &'static str = "brain_coral_fan";
    const DISPLAY_NAME: &'static str = "Brain Coral Fan";
}
pub struct r#BubbleCoralFan;
impl Item for r#BubbleCoralFan {
    const ID: i32 = 572i32;
    const NAME: &'static str = "bubble_coral_fan";
    const DISPLAY_NAME: &'static str = "Bubble Coral Fan";
}
pub struct r#FireCoralFan;
impl Item for r#FireCoralFan {
    const ID: i32 = 573i32;
    const NAME: &'static str = "fire_coral_fan";
    const DISPLAY_NAME: &'static str = "Fire Coral Fan";
}
pub struct r#HornCoralFan;
impl Item for r#HornCoralFan {
    const ID: i32 = 574i32;
    const NAME: &'static str = "horn_coral_fan";
    const DISPLAY_NAME: &'static str = "Horn Coral Fan";
}
pub struct r#DeadTubeCoralFan;
impl Item for r#DeadTubeCoralFan {
    const ID: i32 = 575i32;
    const NAME: &'static str = "dead_tube_coral_fan";
    const DISPLAY_NAME: &'static str = "Dead Tube Coral Fan";
}
pub struct r#DeadBrainCoralFan;
impl Item for r#DeadBrainCoralFan {
    const ID: i32 = 576i32;
    const NAME: &'static str = "dead_brain_coral_fan";
    const DISPLAY_NAME: &'static str = "Dead Brain Coral Fan";
}
pub struct r#DeadBubbleCoralFan;
impl Item for r#DeadBubbleCoralFan {
    const ID: i32 = 577i32;
    const NAME: &'static str = "dead_bubble_coral_fan";
    const DISPLAY_NAME: &'static str = "Dead Bubble Coral Fan";
}
pub struct r#DeadFireCoralFan;
impl Item for r#DeadFireCoralFan {
    const ID: i32 = 578i32;
    const NAME: &'static str = "dead_fire_coral_fan";
    const DISPLAY_NAME: &'static str = "Dead Fire Coral Fan";
}
pub struct r#DeadHornCoralFan;
impl Item for r#DeadHornCoralFan {
    const ID: i32 = 579i32;
    const NAME: &'static str = "dead_horn_coral_fan";
    const DISPLAY_NAME: &'static str = "Dead Horn Coral Fan";
}
pub struct r#BlueIce;
impl Item for r#BlueIce {
    const ID: i32 = 580i32;
    const NAME: &'static str = "blue_ice";
    const DISPLAY_NAME: &'static str = "Blue Ice";
}
pub struct r#Conduit;
impl Item for r#Conduit {
    const ID: i32 = 581i32;
    const NAME: &'static str = "conduit";
    const DISPLAY_NAME: &'static str = "Conduit";
}
pub struct r#PolishedGraniteStairs;
impl Item for r#PolishedGraniteStairs {
    const ID: i32 = 582i32;
    const NAME: &'static str = "polished_granite_stairs";
    const DISPLAY_NAME: &'static str = "Polished Granite Stairs";
}
pub struct r#SmoothRedSandstoneStairs;
impl Item for r#SmoothRedSandstoneStairs {
    const ID: i32 = 583i32;
    const NAME: &'static str = "smooth_red_sandstone_stairs";
    const DISPLAY_NAME: &'static str = "Smooth Red Sandstone Stairs";
}
pub struct r#MossyStoneBrickStairs;
impl Item for r#MossyStoneBrickStairs {
    const ID: i32 = 584i32;
    const NAME: &'static str = "mossy_stone_brick_stairs";
    const DISPLAY_NAME: &'static str = "Mossy Stone Brick Stairs";
}
pub struct r#PolishedDioriteStairs;
impl Item for r#PolishedDioriteStairs {
    const ID: i32 = 585i32;
    const NAME: &'static str = "polished_diorite_stairs";
    const DISPLAY_NAME: &'static str = "Polished Diorite Stairs";
}
pub struct r#MossyCobblestoneStairs;
impl Item for r#MossyCobblestoneStairs {
    const ID: i32 = 586i32;
    const NAME: &'static str = "mossy_cobblestone_stairs";
    const DISPLAY_NAME: &'static str = "Mossy Cobblestone Stairs";
}
pub struct r#EndStoneBrickStairs;
impl Item for r#EndStoneBrickStairs {
    const ID: i32 = 587i32;
    const NAME: &'static str = "end_stone_brick_stairs";
    const DISPLAY_NAME: &'static str = "End Stone Brick Stairs";
}
pub struct r#StoneStairs;
impl Item for r#StoneStairs {
    const ID: i32 = 588i32;
    const NAME: &'static str = "stone_stairs";
    const DISPLAY_NAME: &'static str = "Stone Stairs";
}
pub struct r#SmoothSandstoneStairs;
impl Item for r#SmoothSandstoneStairs {
    const ID: i32 = 589i32;
    const NAME: &'static str = "smooth_sandstone_stairs";
    const DISPLAY_NAME: &'static str = "Smooth Sandstone Stairs";
}
pub struct r#SmoothQuartzStairs;
impl Item for r#SmoothQuartzStairs {
    const ID: i32 = 590i32;
    const NAME: &'static str = "smooth_quartz_stairs";
    const DISPLAY_NAME: &'static str = "Smooth Quartz Stairs";
}
pub struct r#GraniteStairs;
impl Item for r#GraniteStairs {
    const ID: i32 = 591i32;
    const NAME: &'static str = "granite_stairs";
    const DISPLAY_NAME: &'static str = "Granite Stairs";
}
pub struct r#AndesiteStairs;
impl Item for r#AndesiteStairs {
    const ID: i32 = 592i32;
    const NAME: &'static str = "andesite_stairs";
    const DISPLAY_NAME: &'static str = "Andesite Stairs";
}
pub struct r#RedNetherBrickStairs;
impl Item for r#RedNetherBrickStairs {
    const ID: i32 = 593i32;
    const NAME: &'static str = "red_nether_brick_stairs";
    const DISPLAY_NAME: &'static str = "Red Nether Brick Stairs";
}
pub struct r#PolishedAndesiteStairs;
impl Item for r#PolishedAndesiteStairs {
    const ID: i32 = 594i32;
    const NAME: &'static str = "polished_andesite_stairs";
    const DISPLAY_NAME: &'static str = "Polished Andesite Stairs";
}
pub struct r#DioriteStairs;
impl Item for r#DioriteStairs {
    const ID: i32 = 595i32;
    const NAME: &'static str = "diorite_stairs";
    const DISPLAY_NAME: &'static str = "Diorite Stairs";
}
pub struct r#CobbledDeepslateStairs;
impl Item for r#CobbledDeepslateStairs {
    const ID: i32 = 596i32;
    const NAME: &'static str = "cobbled_deepslate_stairs";
    const DISPLAY_NAME: &'static str = "Cobbled Deepslate Stairs";
}
pub struct r#PolishedDeepslateStairs;
impl Item for r#PolishedDeepslateStairs {
    const ID: i32 = 597i32;
    const NAME: &'static str = "polished_deepslate_stairs";
    const DISPLAY_NAME: &'static str = "Polished Deepslate Stairs";
}
pub struct r#DeepslateBrickStairs;
impl Item for r#DeepslateBrickStairs {
    const ID: i32 = 598i32;
    const NAME: &'static str = "deepslate_brick_stairs";
    const DISPLAY_NAME: &'static str = "Deepslate Brick Stairs";
}
pub struct r#DeepslateTileStairs;
impl Item for r#DeepslateTileStairs {
    const ID: i32 = 599i32;
    const NAME: &'static str = "deepslate_tile_stairs";
    const DISPLAY_NAME: &'static str = "Deepslate Tile Stairs";
}
pub struct r#PolishedGraniteSlab;
impl Item for r#PolishedGraniteSlab {
    const ID: i32 = 600i32;
    const NAME: &'static str = "polished_granite_slab";
    const DISPLAY_NAME: &'static str = "Polished Granite Slab";
}
pub struct r#SmoothRedSandstoneSlab;
impl Item for r#SmoothRedSandstoneSlab {
    const ID: i32 = 601i32;
    const NAME: &'static str = "smooth_red_sandstone_slab";
    const DISPLAY_NAME: &'static str = "Smooth Red Sandstone Slab";
}
pub struct r#MossyStoneBrickSlab;
impl Item for r#MossyStoneBrickSlab {
    const ID: i32 = 602i32;
    const NAME: &'static str = "mossy_stone_brick_slab";
    const DISPLAY_NAME: &'static str = "Mossy Stone Brick Slab";
}
pub struct r#PolishedDioriteSlab;
impl Item for r#PolishedDioriteSlab {
    const ID: i32 = 603i32;
    const NAME: &'static str = "polished_diorite_slab";
    const DISPLAY_NAME: &'static str = "Polished Diorite Slab";
}
pub struct r#MossyCobblestoneSlab;
impl Item for r#MossyCobblestoneSlab {
    const ID: i32 = 604i32;
    const NAME: &'static str = "mossy_cobblestone_slab";
    const DISPLAY_NAME: &'static str = "Mossy Cobblestone Slab";
}
pub struct r#EndStoneBrickSlab;
impl Item for r#EndStoneBrickSlab {
    const ID: i32 = 605i32;
    const NAME: &'static str = "end_stone_brick_slab";
    const DISPLAY_NAME: &'static str = "End Stone Brick Slab";
}
pub struct r#SmoothSandstoneSlab;
impl Item for r#SmoothSandstoneSlab {
    const ID: i32 = 606i32;
    const NAME: &'static str = "smooth_sandstone_slab";
    const DISPLAY_NAME: &'static str = "Smooth Sandstone Slab";
}
pub struct r#SmoothQuartzSlab;
impl Item for r#SmoothQuartzSlab {
    const ID: i32 = 607i32;
    const NAME: &'static str = "smooth_quartz_slab";
    const DISPLAY_NAME: &'static str = "Smooth Quartz Slab";
}
pub struct r#GraniteSlab;
impl Item for r#GraniteSlab {
    const ID: i32 = 608i32;
    const NAME: &'static str = "granite_slab";
    const DISPLAY_NAME: &'static str = "Granite Slab";
}
pub struct r#AndesiteSlab;
impl Item for r#AndesiteSlab {
    const ID: i32 = 609i32;
    const NAME: &'static str = "andesite_slab";
    const DISPLAY_NAME: &'static str = "Andesite Slab";
}
pub struct r#RedNetherBrickSlab;
impl Item for r#RedNetherBrickSlab {
    const ID: i32 = 610i32;
    const NAME: &'static str = "red_nether_brick_slab";
    const DISPLAY_NAME: &'static str = "Red Nether Brick Slab";
}
pub struct r#PolishedAndesiteSlab;
impl Item for r#PolishedAndesiteSlab {
    const ID: i32 = 611i32;
    const NAME: &'static str = "polished_andesite_slab";
    const DISPLAY_NAME: &'static str = "Polished Andesite Slab";
}
pub struct r#DioriteSlab;
impl Item for r#DioriteSlab {
    const ID: i32 = 612i32;
    const NAME: &'static str = "diorite_slab";
    const DISPLAY_NAME: &'static str = "Diorite Slab";
}
pub struct r#CobbledDeepslateSlab;
impl Item for r#CobbledDeepslateSlab {
    const ID: i32 = 613i32;
    const NAME: &'static str = "cobbled_deepslate_slab";
    const DISPLAY_NAME: &'static str = "Cobbled Deepslate Slab";
}
pub struct r#PolishedDeepslateSlab;
impl Item for r#PolishedDeepslateSlab {
    const ID: i32 = 614i32;
    const NAME: &'static str = "polished_deepslate_slab";
    const DISPLAY_NAME: &'static str = "Polished Deepslate Slab";
}
pub struct r#DeepslateBrickSlab;
impl Item for r#DeepslateBrickSlab {
    const ID: i32 = 615i32;
    const NAME: &'static str = "deepslate_brick_slab";
    const DISPLAY_NAME: &'static str = "Deepslate Brick Slab";
}
pub struct r#DeepslateTileSlab;
impl Item for r#DeepslateTileSlab {
    const ID: i32 = 616i32;
    const NAME: &'static str = "deepslate_tile_slab";
    const DISPLAY_NAME: &'static str = "Deepslate Tile Slab";
}
pub struct r#Scaffolding;
impl Item for r#Scaffolding {
    const ID: i32 = 617i32;
    const NAME: &'static str = "scaffolding";
    const DISPLAY_NAME: &'static str = "Scaffolding";
}
pub struct r#Redstone;
impl Item for r#Redstone {
    const ID: i32 = 618i32;
    const NAME: &'static str = "redstone";
    const DISPLAY_NAME: &'static str = "Redstone Dust";
}
pub struct r#RedstoneTorch;
impl Item for r#RedstoneTorch {
    const ID: i32 = 619i32;
    const NAME: &'static str = "redstone_torch";
    const DISPLAY_NAME: &'static str = "Redstone Torch";
}
pub struct r#RedstoneBlock;
impl Item for r#RedstoneBlock {
    const ID: i32 = 620i32;
    const NAME: &'static str = "redstone_block";
    const DISPLAY_NAME: &'static str = "Block of Redstone";
}
pub struct r#Repeater;
impl Item for r#Repeater {
    const ID: i32 = 621i32;
    const NAME: &'static str = "repeater";
    const DISPLAY_NAME: &'static str = "Redstone Repeater";
}
pub struct r#Comparator;
impl Item for r#Comparator {
    const ID: i32 = 622i32;
    const NAME: &'static str = "comparator";
    const DISPLAY_NAME: &'static str = "Redstone Comparator";
}
pub struct r#Piston;
impl Item for r#Piston {
    const ID: i32 = 623i32;
    const NAME: &'static str = "piston";
    const DISPLAY_NAME: &'static str = "Piston";
}
pub struct r#StickyPiston;
impl Item for r#StickyPiston {
    const ID: i32 = 624i32;
    const NAME: &'static str = "sticky_piston";
    const DISPLAY_NAME: &'static str = "Sticky Piston";
}
pub struct r#SlimeBlock;
impl Item for r#SlimeBlock {
    const ID: i32 = 625i32;
    const NAME: &'static str = "slime_block";
    const DISPLAY_NAME: &'static str = "Slime Block";
}
pub struct r#HoneyBlock;
impl Item for r#HoneyBlock {
    const ID: i32 = 626i32;
    const NAME: &'static str = "honey_block";
    const DISPLAY_NAME: &'static str = "Honey Block";
}
pub struct r#Observer;
impl Item for r#Observer {
    const ID: i32 = 627i32;
    const NAME: &'static str = "observer";
    const DISPLAY_NAME: &'static str = "Observer";
}
pub struct r#Hopper;
impl Item for r#Hopper {
    const ID: i32 = 628i32;
    const NAME: &'static str = "hopper";
    const DISPLAY_NAME: &'static str = "Hopper";
}
pub struct r#Dispenser;
impl Item for r#Dispenser {
    const ID: i32 = 629i32;
    const NAME: &'static str = "dispenser";
    const DISPLAY_NAME: &'static str = "Dispenser";
}
pub struct r#Dropper;
impl Item for r#Dropper {
    const ID: i32 = 630i32;
    const NAME: &'static str = "dropper";
    const DISPLAY_NAME: &'static str = "Dropper";
}
pub struct r#Lectern;
impl Item for r#Lectern {
    const ID: i32 = 631i32;
    const NAME: &'static str = "lectern";
    const DISPLAY_NAME: &'static str = "Lectern";
}
pub struct r#Target;
impl Item for r#Target {
    const ID: i32 = 632i32;
    const NAME: &'static str = "target";
    const DISPLAY_NAME: &'static str = "Target";
}
pub struct r#Lever;
impl Item for r#Lever {
    const ID: i32 = 633i32;
    const NAME: &'static str = "lever";
    const DISPLAY_NAME: &'static str = "Lever";
}
pub struct r#LightningRod;
impl Item for r#LightningRod {
    const ID: i32 = 634i32;
    const NAME: &'static str = "lightning_rod";
    const DISPLAY_NAME: &'static str = "Lightning Rod";
}
pub struct r#DaylightDetector;
impl Item for r#DaylightDetector {
    const ID: i32 = 635i32;
    const NAME: &'static str = "daylight_detector";
    const DISPLAY_NAME: &'static str = "Daylight Detector";
}
pub struct r#SculkSensor;
impl Item for r#SculkSensor {
    const ID: i32 = 636i32;
    const NAME: &'static str = "sculk_sensor";
    const DISPLAY_NAME: &'static str = "Sculk Sensor";
}
pub struct r#TripwireHook;
impl Item for r#TripwireHook {
    const ID: i32 = 637i32;
    const NAME: &'static str = "tripwire_hook";
    const DISPLAY_NAME: &'static str = "Tripwire Hook";
}
pub struct r#TrappedChest;
impl Item for r#TrappedChest {
    const ID: i32 = 638i32;
    const NAME: &'static str = "trapped_chest";
    const DISPLAY_NAME: &'static str = "Trapped Chest";
}
pub struct r#Tnt;
impl Item for r#Tnt {
    const ID: i32 = 639i32;
    const NAME: &'static str = "tnt";
    const DISPLAY_NAME: &'static str = "TNT";
}
pub struct r#RedstoneLamp;
impl Item for r#RedstoneLamp {
    const ID: i32 = 640i32;
    const NAME: &'static str = "redstone_lamp";
    const DISPLAY_NAME: &'static str = "Redstone Lamp";
}
pub struct r#NoteBlock;
impl Item for r#NoteBlock {
    const ID: i32 = 641i32;
    const NAME: &'static str = "note_block";
    const DISPLAY_NAME: &'static str = "Note Block";
}
pub struct r#StoneButton;
impl Item for r#StoneButton {
    const ID: i32 = 642i32;
    const NAME: &'static str = "stone_button";
    const DISPLAY_NAME: &'static str = "Stone Button";
}
pub struct r#PolishedBlackstoneButton;
impl Item for r#PolishedBlackstoneButton {
    const ID: i32 = 643i32;
    const NAME: &'static str = "polished_blackstone_button";
    const DISPLAY_NAME: &'static str = "Polished Blackstone Button";
}
pub struct r#OakButton;
impl Item for r#OakButton {
    const ID: i32 = 644i32;
    const NAME: &'static str = "oak_button";
    const DISPLAY_NAME: &'static str = "Oak Button";
}
pub struct r#SpruceButton;
impl Item for r#SpruceButton {
    const ID: i32 = 645i32;
    const NAME: &'static str = "spruce_button";
    const DISPLAY_NAME: &'static str = "Spruce Button";
}
pub struct r#BirchButton;
impl Item for r#BirchButton {
    const ID: i32 = 646i32;
    const NAME: &'static str = "birch_button";
    const DISPLAY_NAME: &'static str = "Birch Button";
}
pub struct r#JungleButton;
impl Item for r#JungleButton {
    const ID: i32 = 647i32;
    const NAME: &'static str = "jungle_button";
    const DISPLAY_NAME: &'static str = "Jungle Button";
}
pub struct r#AcaciaButton;
impl Item for r#AcaciaButton {
    const ID: i32 = 648i32;
    const NAME: &'static str = "acacia_button";
    const DISPLAY_NAME: &'static str = "Acacia Button";
}
pub struct r#DarkOakButton;
impl Item for r#DarkOakButton {
    const ID: i32 = 649i32;
    const NAME: &'static str = "dark_oak_button";
    const DISPLAY_NAME: &'static str = "Dark Oak Button";
}
pub struct r#MangroveButton;
impl Item for r#MangroveButton {
    const ID: i32 = 650i32;
    const NAME: &'static str = "mangrove_button";
    const DISPLAY_NAME: &'static str = "Mangrove Button";
}
pub struct r#BambooButton;
impl Item for r#BambooButton {
    const ID: i32 = 651i32;
    const NAME: &'static str = "bamboo_button";
    const DISPLAY_NAME: &'static str = "Bamboo Button";
}
pub struct r#CrimsonButton;
impl Item for r#CrimsonButton {
    const ID: i32 = 652i32;
    const NAME: &'static str = "crimson_button";
    const DISPLAY_NAME: &'static str = "Crimson Button";
}
pub struct r#WarpedButton;
impl Item for r#WarpedButton {
    const ID: i32 = 653i32;
    const NAME: &'static str = "warped_button";
    const DISPLAY_NAME: &'static str = "Warped Button";
}
pub struct r#StonePressurePlate;
impl Item for r#StonePressurePlate {
    const ID: i32 = 654i32;
    const NAME: &'static str = "stone_pressure_plate";
    const DISPLAY_NAME: &'static str = "Stone Pressure Plate";
}
pub struct r#PolishedBlackstonePressurePlate;
impl Item for r#PolishedBlackstonePressurePlate {
    const ID: i32 = 655i32;
    const NAME: &'static str = "polished_blackstone_pressure_plate";
    const DISPLAY_NAME: &'static str = "Polished Blackstone Pressure Plate";
}
pub struct r#LightWeightedPressurePlate;
impl Item for r#LightWeightedPressurePlate {
    const ID: i32 = 656i32;
    const NAME: &'static str = "light_weighted_pressure_plate";
    const DISPLAY_NAME: &'static str = "Light Weighted Pressure Plate";
}
pub struct r#HeavyWeightedPressurePlate;
impl Item for r#HeavyWeightedPressurePlate {
    const ID: i32 = 657i32;
    const NAME: &'static str = "heavy_weighted_pressure_plate";
    const DISPLAY_NAME: &'static str = "Heavy Weighted Pressure Plate";
}
pub struct r#OakPressurePlate;
impl Item for r#OakPressurePlate {
    const ID: i32 = 658i32;
    const NAME: &'static str = "oak_pressure_plate";
    const DISPLAY_NAME: &'static str = "Oak Pressure Plate";
}
pub struct r#SprucePressurePlate;
impl Item for r#SprucePressurePlate {
    const ID: i32 = 659i32;
    const NAME: &'static str = "spruce_pressure_plate";
    const DISPLAY_NAME: &'static str = "Spruce Pressure Plate";
}
pub struct r#BirchPressurePlate;
impl Item for r#BirchPressurePlate {
    const ID: i32 = 660i32;
    const NAME: &'static str = "birch_pressure_plate";
    const DISPLAY_NAME: &'static str = "Birch Pressure Plate";
}
pub struct r#JunglePressurePlate;
impl Item for r#JunglePressurePlate {
    const ID: i32 = 661i32;
    const NAME: &'static str = "jungle_pressure_plate";
    const DISPLAY_NAME: &'static str = "Jungle Pressure Plate";
}
pub struct r#AcaciaPressurePlate;
impl Item for r#AcaciaPressurePlate {
    const ID: i32 = 662i32;
    const NAME: &'static str = "acacia_pressure_plate";
    const DISPLAY_NAME: &'static str = "Acacia Pressure Plate";
}
pub struct r#DarkOakPressurePlate;
impl Item for r#DarkOakPressurePlate {
    const ID: i32 = 663i32;
    const NAME: &'static str = "dark_oak_pressure_plate";
    const DISPLAY_NAME: &'static str = "Dark Oak Pressure Plate";
}
pub struct r#MangrovePressurePlate;
impl Item for r#MangrovePressurePlate {
    const ID: i32 = 664i32;
    const NAME: &'static str = "mangrove_pressure_plate";
    const DISPLAY_NAME: &'static str = "Mangrove Pressure Plate";
}
pub struct r#BambooPressurePlate;
impl Item for r#BambooPressurePlate {
    const ID: i32 = 665i32;
    const NAME: &'static str = "bamboo_pressure_plate";
    const DISPLAY_NAME: &'static str = "Bamboo Pressure Plate";
}
pub struct r#CrimsonPressurePlate;
impl Item for r#CrimsonPressurePlate {
    const ID: i32 = 666i32;
    const NAME: &'static str = "crimson_pressure_plate";
    const DISPLAY_NAME: &'static str = "Crimson Pressure Plate";
}
pub struct r#WarpedPressurePlate;
impl Item for r#WarpedPressurePlate {
    const ID: i32 = 667i32;
    const NAME: &'static str = "warped_pressure_plate";
    const DISPLAY_NAME: &'static str = "Warped Pressure Plate";
}
pub struct r#IronDoor;
impl Item for r#IronDoor {
    const ID: i32 = 668i32;
    const NAME: &'static str = "iron_door";
    const DISPLAY_NAME: &'static str = "Iron Door";
}
pub struct r#OakDoor;
impl Item for r#OakDoor {
    const ID: i32 = 669i32;
    const NAME: &'static str = "oak_door";
    const DISPLAY_NAME: &'static str = "Oak Door";
}
pub struct r#SpruceDoor;
impl Item for r#SpruceDoor {
    const ID: i32 = 670i32;
    const NAME: &'static str = "spruce_door";
    const DISPLAY_NAME: &'static str = "Spruce Door";
}
pub struct r#BirchDoor;
impl Item for r#BirchDoor {
    const ID: i32 = 671i32;
    const NAME: &'static str = "birch_door";
    const DISPLAY_NAME: &'static str = "Birch Door";
}
pub struct r#JungleDoor;
impl Item for r#JungleDoor {
    const ID: i32 = 672i32;
    const NAME: &'static str = "jungle_door";
    const DISPLAY_NAME: &'static str = "Jungle Door";
}
pub struct r#AcaciaDoor;
impl Item for r#AcaciaDoor {
    const ID: i32 = 673i32;
    const NAME: &'static str = "acacia_door";
    const DISPLAY_NAME: &'static str = "Acacia Door";
}
pub struct r#DarkOakDoor;
impl Item for r#DarkOakDoor {
    const ID: i32 = 674i32;
    const NAME: &'static str = "dark_oak_door";
    const DISPLAY_NAME: &'static str = "Dark Oak Door";
}
pub struct r#MangroveDoor;
impl Item for r#MangroveDoor {
    const ID: i32 = 675i32;
    const NAME: &'static str = "mangrove_door";
    const DISPLAY_NAME: &'static str = "Mangrove Door";
}
pub struct r#BambooDoor;
impl Item for r#BambooDoor {
    const ID: i32 = 676i32;
    const NAME: &'static str = "bamboo_door";
    const DISPLAY_NAME: &'static str = "Bamboo Door";
}
pub struct r#CrimsonDoor;
impl Item for r#CrimsonDoor {
    const ID: i32 = 677i32;
    const NAME: &'static str = "crimson_door";
    const DISPLAY_NAME: &'static str = "Crimson Door";
}
pub struct r#WarpedDoor;
impl Item for r#WarpedDoor {
    const ID: i32 = 678i32;
    const NAME: &'static str = "warped_door";
    const DISPLAY_NAME: &'static str = "Warped Door";
}
pub struct r#IronTrapdoor;
impl Item for r#IronTrapdoor {
    const ID: i32 = 679i32;
    const NAME: &'static str = "iron_trapdoor";
    const DISPLAY_NAME: &'static str = "Iron Trapdoor";
}
pub struct r#OakTrapdoor;
impl Item for r#OakTrapdoor {
    const ID: i32 = 680i32;
    const NAME: &'static str = "oak_trapdoor";
    const DISPLAY_NAME: &'static str = "Oak Trapdoor";
}
pub struct r#SpruceTrapdoor;
impl Item for r#SpruceTrapdoor {
    const ID: i32 = 681i32;
    const NAME: &'static str = "spruce_trapdoor";
    const DISPLAY_NAME: &'static str = "Spruce Trapdoor";
}
pub struct r#BirchTrapdoor;
impl Item for r#BirchTrapdoor {
    const ID: i32 = 682i32;
    const NAME: &'static str = "birch_trapdoor";
    const DISPLAY_NAME: &'static str = "Birch Trapdoor";
}
pub struct r#JungleTrapdoor;
impl Item for r#JungleTrapdoor {
    const ID: i32 = 683i32;
    const NAME: &'static str = "jungle_trapdoor";
    const DISPLAY_NAME: &'static str = "Jungle Trapdoor";
}
pub struct r#AcaciaTrapdoor;
impl Item for r#AcaciaTrapdoor {
    const ID: i32 = 684i32;
    const NAME: &'static str = "acacia_trapdoor";
    const DISPLAY_NAME: &'static str = "Acacia Trapdoor";
}
pub struct r#DarkOakTrapdoor;
impl Item for r#DarkOakTrapdoor {
    const ID: i32 = 685i32;
    const NAME: &'static str = "dark_oak_trapdoor";
    const DISPLAY_NAME: &'static str = "Dark Oak Trapdoor";
}
pub struct r#MangroveTrapdoor;
impl Item for r#MangroveTrapdoor {
    const ID: i32 = 686i32;
    const NAME: &'static str = "mangrove_trapdoor";
    const DISPLAY_NAME: &'static str = "Mangrove Trapdoor";
}
pub struct r#BambooTrapdoor;
impl Item for r#BambooTrapdoor {
    const ID: i32 = 687i32;
    const NAME: &'static str = "bamboo_trapdoor";
    const DISPLAY_NAME: &'static str = "Bamboo Trapdoor";
}
pub struct r#CrimsonTrapdoor;
impl Item for r#CrimsonTrapdoor {
    const ID: i32 = 688i32;
    const NAME: &'static str = "crimson_trapdoor";
    const DISPLAY_NAME: &'static str = "Crimson Trapdoor";
}
pub struct r#WarpedTrapdoor;
impl Item for r#WarpedTrapdoor {
    const ID: i32 = 689i32;
    const NAME: &'static str = "warped_trapdoor";
    const DISPLAY_NAME: &'static str = "Warped Trapdoor";
}
pub struct r#OakFenceGate;
impl Item for r#OakFenceGate {
    const ID: i32 = 690i32;
    const NAME: &'static str = "oak_fence_gate";
    const DISPLAY_NAME: &'static str = "Oak Fence Gate";
}
pub struct r#SpruceFenceGate;
impl Item for r#SpruceFenceGate {
    const ID: i32 = 691i32;
    const NAME: &'static str = "spruce_fence_gate";
    const DISPLAY_NAME: &'static str = "Spruce Fence Gate";
}
pub struct r#BirchFenceGate;
impl Item for r#BirchFenceGate {
    const ID: i32 = 692i32;
    const NAME: &'static str = "birch_fence_gate";
    const DISPLAY_NAME: &'static str = "Birch Fence Gate";
}
pub struct r#JungleFenceGate;
impl Item for r#JungleFenceGate {
    const ID: i32 = 693i32;
    const NAME: &'static str = "jungle_fence_gate";
    const DISPLAY_NAME: &'static str = "Jungle Fence Gate";
}
pub struct r#AcaciaFenceGate;
impl Item for r#AcaciaFenceGate {
    const ID: i32 = 694i32;
    const NAME: &'static str = "acacia_fence_gate";
    const DISPLAY_NAME: &'static str = "Acacia Fence Gate";
}
pub struct r#DarkOakFenceGate;
impl Item for r#DarkOakFenceGate {
    const ID: i32 = 695i32;
    const NAME: &'static str = "dark_oak_fence_gate";
    const DISPLAY_NAME: &'static str = "Dark Oak Fence Gate";
}
pub struct r#MangroveFenceGate;
impl Item for r#MangroveFenceGate {
    const ID: i32 = 696i32;
    const NAME: &'static str = "mangrove_fence_gate";
    const DISPLAY_NAME: &'static str = "Mangrove Fence Gate";
}
pub struct r#BambooFenceGate;
impl Item for r#BambooFenceGate {
    const ID: i32 = 697i32;
    const NAME: &'static str = "bamboo_fence_gate";
    const DISPLAY_NAME: &'static str = "Bamboo Fence Gate";
}
pub struct r#CrimsonFenceGate;
impl Item for r#CrimsonFenceGate {
    const ID: i32 = 698i32;
    const NAME: &'static str = "crimson_fence_gate";
    const DISPLAY_NAME: &'static str = "Crimson Fence Gate";
}
pub struct r#WarpedFenceGate;
impl Item for r#WarpedFenceGate {
    const ID: i32 = 699i32;
    const NAME: &'static str = "warped_fence_gate";
    const DISPLAY_NAME: &'static str = "Warped Fence Gate";
}
pub struct r#PoweredRail;
impl Item for r#PoweredRail {
    const ID: i32 = 700i32;
    const NAME: &'static str = "powered_rail";
    const DISPLAY_NAME: &'static str = "Powered Rail";
}
pub struct r#DetectorRail;
impl Item for r#DetectorRail {
    const ID: i32 = 701i32;
    const NAME: &'static str = "detector_rail";
    const DISPLAY_NAME: &'static str = "Detector Rail";
}
pub struct r#Rail;
impl Item for r#Rail {
    const ID: i32 = 702i32;
    const NAME: &'static str = "rail";
    const DISPLAY_NAME: &'static str = "Rail";
}
pub struct r#ActivatorRail;
impl Item for r#ActivatorRail {
    const ID: i32 = 703i32;
    const NAME: &'static str = "activator_rail";
    const DISPLAY_NAME: &'static str = "Activator Rail";
}
pub struct r#Saddle;
impl Item for r#Saddle {
    const ID: i32 = 704i32;
    const NAME: &'static str = "saddle";
    const DISPLAY_NAME: &'static str = "Saddle";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Minecart;
impl Item for r#Minecart {
    const ID: i32 = 705i32;
    const NAME: &'static str = "minecart";
    const DISPLAY_NAME: &'static str = "Minecart";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#ChestMinecart;
impl Item for r#ChestMinecart {
    const ID: i32 = 706i32;
    const NAME: &'static str = "chest_minecart";
    const DISPLAY_NAME: &'static str = "Minecart with Chest";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#FurnaceMinecart;
impl Item for r#FurnaceMinecart {
    const ID: i32 = 707i32;
    const NAME: &'static str = "furnace_minecart";
    const DISPLAY_NAME: &'static str = "Minecart with Furnace";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#TntMinecart;
impl Item for r#TntMinecart {
    const ID: i32 = 708i32;
    const NAME: &'static str = "tnt_minecart";
    const DISPLAY_NAME: &'static str = "Minecart with TNT";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#HopperMinecart;
impl Item for r#HopperMinecart {
    const ID: i32 = 709i32;
    const NAME: &'static str = "hopper_minecart";
    const DISPLAY_NAME: &'static str = "Minecart with Hopper";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#CarrotOnAStick;
impl Item for r#CarrotOnAStick {
    const ID: i32 = 710i32;
    const NAME: &'static str = "carrot_on_a_stick";
    const DISPLAY_NAME: &'static str = "Carrot on a Stick";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#WarpedFungusOnAStick;
impl Item for r#WarpedFungusOnAStick {
    const ID: i32 = 711i32;
    const NAME: &'static str = "warped_fungus_on_a_stick";
    const DISPLAY_NAME: &'static str = "Warped Fungus on a Stick";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Elytra;
impl Item for r#Elytra {
    const ID: i32 = 712i32;
    const NAME: &'static str = "elytra";
    const DISPLAY_NAME: &'static str = "Elytra";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#OakBoat;
impl Item for r#OakBoat {
    const ID: i32 = 713i32;
    const NAME: &'static str = "oak_boat";
    const DISPLAY_NAME: &'static str = "Oak Boat";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#OakChestBoat;
impl Item for r#OakChestBoat {
    const ID: i32 = 714i32;
    const NAME: &'static str = "oak_chest_boat";
    const DISPLAY_NAME: &'static str = "Oak Boat with Chest";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#SpruceBoat;
impl Item for r#SpruceBoat {
    const ID: i32 = 715i32;
    const NAME: &'static str = "spruce_boat";
    const DISPLAY_NAME: &'static str = "Spruce Boat";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#SpruceChestBoat;
impl Item for r#SpruceChestBoat {
    const ID: i32 = 716i32;
    const NAME: &'static str = "spruce_chest_boat";
    const DISPLAY_NAME: &'static str = "Spruce Boat with Chest";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#BirchBoat;
impl Item for r#BirchBoat {
    const ID: i32 = 717i32;
    const NAME: &'static str = "birch_boat";
    const DISPLAY_NAME: &'static str = "Birch Boat";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#BirchChestBoat;
impl Item for r#BirchChestBoat {
    const ID: i32 = 718i32;
    const NAME: &'static str = "birch_chest_boat";
    const DISPLAY_NAME: &'static str = "Birch Boat with Chest";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#JungleBoat;
impl Item for r#JungleBoat {
    const ID: i32 = 719i32;
    const NAME: &'static str = "jungle_boat";
    const DISPLAY_NAME: &'static str = "Jungle Boat";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#JungleChestBoat;
impl Item for r#JungleChestBoat {
    const ID: i32 = 720i32;
    const NAME: &'static str = "jungle_chest_boat";
    const DISPLAY_NAME: &'static str = "Jungle Boat with Chest";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#AcaciaBoat;
impl Item for r#AcaciaBoat {
    const ID: i32 = 721i32;
    const NAME: &'static str = "acacia_boat";
    const DISPLAY_NAME: &'static str = "Acacia Boat";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#AcaciaChestBoat;
impl Item for r#AcaciaChestBoat {
    const ID: i32 = 722i32;
    const NAME: &'static str = "acacia_chest_boat";
    const DISPLAY_NAME: &'static str = "Acacia Boat with Chest";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DarkOakBoat;
impl Item for r#DarkOakBoat {
    const ID: i32 = 723i32;
    const NAME: &'static str = "dark_oak_boat";
    const DISPLAY_NAME: &'static str = "Dark Oak Boat";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DarkOakChestBoat;
impl Item for r#DarkOakChestBoat {
    const ID: i32 = 724i32;
    const NAME: &'static str = "dark_oak_chest_boat";
    const DISPLAY_NAME: &'static str = "Dark Oak Boat with Chest";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MangroveBoat;
impl Item for r#MangroveBoat {
    const ID: i32 = 725i32;
    const NAME: &'static str = "mangrove_boat";
    const DISPLAY_NAME: &'static str = "Mangrove Boat";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MangroveChestBoat;
impl Item for r#MangroveChestBoat {
    const ID: i32 = 726i32;
    const NAME: &'static str = "mangrove_chest_boat";
    const DISPLAY_NAME: &'static str = "Mangrove Boat with Chest";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#BambooRaft;
impl Item for r#BambooRaft {
    const ID: i32 = 727i32;
    const NAME: &'static str = "bamboo_raft";
    const DISPLAY_NAME: &'static str = "Bamboo Raft";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#BambooChestRaft;
impl Item for r#BambooChestRaft {
    const ID: i32 = 728i32;
    const NAME: &'static str = "bamboo_chest_raft";
    const DISPLAY_NAME: &'static str = "Bamboo Raft with Chest";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#StructureBlock;
impl Item for r#StructureBlock {
    const ID: i32 = 729i32;
    const NAME: &'static str = "structure_block";
    const DISPLAY_NAME: &'static str = "Structure Block";
}
pub struct r#Jigsaw;
impl Item for r#Jigsaw {
    const ID: i32 = 730i32;
    const NAME: &'static str = "jigsaw";
    const DISPLAY_NAME: &'static str = "Jigsaw Block";
}
pub struct r#TurtleHelmet;
impl Item for r#TurtleHelmet {
    const ID: i32 = 731i32;
    const NAME: &'static str = "turtle_helmet";
    const DISPLAY_NAME: &'static str = "Turtle Shell";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Scute;
impl Item for r#Scute {
    const ID: i32 = 732i32;
    const NAME: &'static str = "scute";
    const DISPLAY_NAME: &'static str = "Scute";
}
pub struct r#FlintAndSteel;
impl Item for r#FlintAndSteel {
    const ID: i32 = 733i32;
    const NAME: &'static str = "flint_and_steel";
    const DISPLAY_NAME: &'static str = "Flint and Steel";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Apple;
impl Item for r#Apple {
    const ID: i32 = 734i32;
    const NAME: &'static str = "apple";
    const DISPLAY_NAME: &'static str = "Apple";
}
pub struct r#Bow;
impl Item for r#Bow {
    const ID: i32 = 735i32;
    const NAME: &'static str = "bow";
    const DISPLAY_NAME: &'static str = "Bow";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Arrow;
impl Item for r#Arrow {
    const ID: i32 = 736i32;
    const NAME: &'static str = "arrow";
    const DISPLAY_NAME: &'static str = "Arrow";
}
pub struct r#Coal;
impl Item for r#Coal {
    const ID: i32 = 737i32;
    const NAME: &'static str = "coal";
    const DISPLAY_NAME: &'static str = "Coal";
}
pub struct r#Charcoal;
impl Item for r#Charcoal {
    const ID: i32 = 738i32;
    const NAME: &'static str = "charcoal";
    const DISPLAY_NAME: &'static str = "Charcoal";
}
pub struct r#Diamond;
impl Item for r#Diamond {
    const ID: i32 = 739i32;
    const NAME: &'static str = "diamond";
    const DISPLAY_NAME: &'static str = "Diamond";
}
pub struct r#Emerald;
impl Item for r#Emerald {
    const ID: i32 = 740i32;
    const NAME: &'static str = "emerald";
    const DISPLAY_NAME: &'static str = "Emerald";
}
pub struct r#LapisLazuli;
impl Item for r#LapisLazuli {
    const ID: i32 = 741i32;
    const NAME: &'static str = "lapis_lazuli";
    const DISPLAY_NAME: &'static str = "Lapis Lazuli";
}
pub struct r#Quartz;
impl Item for r#Quartz {
    const ID: i32 = 742i32;
    const NAME: &'static str = "quartz";
    const DISPLAY_NAME: &'static str = "Nether Quartz";
}
pub struct r#AmethystShard;
impl Item for r#AmethystShard {
    const ID: i32 = 743i32;
    const NAME: &'static str = "amethyst_shard";
    const DISPLAY_NAME: &'static str = "Amethyst Shard";
}
pub struct r#RawIron;
impl Item for r#RawIron {
    const ID: i32 = 744i32;
    const NAME: &'static str = "raw_iron";
    const DISPLAY_NAME: &'static str = "Raw Iron";
}
pub struct r#IronIngot;
impl Item for r#IronIngot {
    const ID: i32 = 745i32;
    const NAME: &'static str = "iron_ingot";
    const DISPLAY_NAME: &'static str = "Iron Ingot";
}
pub struct r#RawCopper;
impl Item for r#RawCopper {
    const ID: i32 = 746i32;
    const NAME: &'static str = "raw_copper";
    const DISPLAY_NAME: &'static str = "Raw Copper";
}
pub struct r#CopperIngot;
impl Item for r#CopperIngot {
    const ID: i32 = 747i32;
    const NAME: &'static str = "copper_ingot";
    const DISPLAY_NAME: &'static str = "Copper Ingot";
}
pub struct r#RawGold;
impl Item for r#RawGold {
    const ID: i32 = 748i32;
    const NAME: &'static str = "raw_gold";
    const DISPLAY_NAME: &'static str = "Raw Gold";
}
pub struct r#GoldIngot;
impl Item for r#GoldIngot {
    const ID: i32 = 749i32;
    const NAME: &'static str = "gold_ingot";
    const DISPLAY_NAME: &'static str = "Gold Ingot";
}
pub struct r#NetheriteIngot;
impl Item for r#NetheriteIngot {
    const ID: i32 = 750i32;
    const NAME: &'static str = "netherite_ingot";
    const DISPLAY_NAME: &'static str = "Netherite Ingot";
}
pub struct r#NetheriteScrap;
impl Item for r#NetheriteScrap {
    const ID: i32 = 751i32;
    const NAME: &'static str = "netherite_scrap";
    const DISPLAY_NAME: &'static str = "Netherite Scrap";
}
pub struct r#WoodenSword;
impl Item for r#WoodenSword {
    const ID: i32 = 752i32;
    const NAME: &'static str = "wooden_sword";
    const DISPLAY_NAME: &'static str = "Wooden Sword";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#WoodenShovel;
impl Item for r#WoodenShovel {
    const ID: i32 = 753i32;
    const NAME: &'static str = "wooden_shovel";
    const DISPLAY_NAME: &'static str = "Wooden Shovel";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#WoodenPickaxe;
impl Item for r#WoodenPickaxe {
    const ID: i32 = 754i32;
    const NAME: &'static str = "wooden_pickaxe";
    const DISPLAY_NAME: &'static str = "Wooden Pickaxe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#WoodenAxe;
impl Item for r#WoodenAxe {
    const ID: i32 = 755i32;
    const NAME: &'static str = "wooden_axe";
    const DISPLAY_NAME: &'static str = "Wooden Axe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#WoodenHoe;
impl Item for r#WoodenHoe {
    const ID: i32 = 756i32;
    const NAME: &'static str = "wooden_hoe";
    const DISPLAY_NAME: &'static str = "Wooden Hoe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#StoneSword;
impl Item for r#StoneSword {
    const ID: i32 = 757i32;
    const NAME: &'static str = "stone_sword";
    const DISPLAY_NAME: &'static str = "Stone Sword";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#StoneShovel;
impl Item for r#StoneShovel {
    const ID: i32 = 758i32;
    const NAME: &'static str = "stone_shovel";
    const DISPLAY_NAME: &'static str = "Stone Shovel";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#StonePickaxe;
impl Item for r#StonePickaxe {
    const ID: i32 = 759i32;
    const NAME: &'static str = "stone_pickaxe";
    const DISPLAY_NAME: &'static str = "Stone Pickaxe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#StoneAxe;
impl Item for r#StoneAxe {
    const ID: i32 = 760i32;
    const NAME: &'static str = "stone_axe";
    const DISPLAY_NAME: &'static str = "Stone Axe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#StoneHoe;
impl Item for r#StoneHoe {
    const ID: i32 = 761i32;
    const NAME: &'static str = "stone_hoe";
    const DISPLAY_NAME: &'static str = "Stone Hoe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GoldenSword;
impl Item for r#GoldenSword {
    const ID: i32 = 762i32;
    const NAME: &'static str = "golden_sword";
    const DISPLAY_NAME: &'static str = "Golden Sword";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GoldenShovel;
impl Item for r#GoldenShovel {
    const ID: i32 = 763i32;
    const NAME: &'static str = "golden_shovel";
    const DISPLAY_NAME: &'static str = "Golden Shovel";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GoldenPickaxe;
impl Item for r#GoldenPickaxe {
    const ID: i32 = 764i32;
    const NAME: &'static str = "golden_pickaxe";
    const DISPLAY_NAME: &'static str = "Golden Pickaxe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GoldenAxe;
impl Item for r#GoldenAxe {
    const ID: i32 = 765i32;
    const NAME: &'static str = "golden_axe";
    const DISPLAY_NAME: &'static str = "Golden Axe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GoldenHoe;
impl Item for r#GoldenHoe {
    const ID: i32 = 766i32;
    const NAME: &'static str = "golden_hoe";
    const DISPLAY_NAME: &'static str = "Golden Hoe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#IronSword;
impl Item for r#IronSword {
    const ID: i32 = 767i32;
    const NAME: &'static str = "iron_sword";
    const DISPLAY_NAME: &'static str = "Iron Sword";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#IronShovel;
impl Item for r#IronShovel {
    const ID: i32 = 768i32;
    const NAME: &'static str = "iron_shovel";
    const DISPLAY_NAME: &'static str = "Iron Shovel";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#IronPickaxe;
impl Item for r#IronPickaxe {
    const ID: i32 = 769i32;
    const NAME: &'static str = "iron_pickaxe";
    const DISPLAY_NAME: &'static str = "Iron Pickaxe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#IronAxe;
impl Item for r#IronAxe {
    const ID: i32 = 770i32;
    const NAME: &'static str = "iron_axe";
    const DISPLAY_NAME: &'static str = "Iron Axe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#IronHoe;
impl Item for r#IronHoe {
    const ID: i32 = 771i32;
    const NAME: &'static str = "iron_hoe";
    const DISPLAY_NAME: &'static str = "Iron Hoe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DiamondSword;
impl Item for r#DiamondSword {
    const ID: i32 = 772i32;
    const NAME: &'static str = "diamond_sword";
    const DISPLAY_NAME: &'static str = "Diamond Sword";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DiamondShovel;
impl Item for r#DiamondShovel {
    const ID: i32 = 773i32;
    const NAME: &'static str = "diamond_shovel";
    const DISPLAY_NAME: &'static str = "Diamond Shovel";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DiamondPickaxe;
impl Item for r#DiamondPickaxe {
    const ID: i32 = 774i32;
    const NAME: &'static str = "diamond_pickaxe";
    const DISPLAY_NAME: &'static str = "Diamond Pickaxe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DiamondAxe;
impl Item for r#DiamondAxe {
    const ID: i32 = 775i32;
    const NAME: &'static str = "diamond_axe";
    const DISPLAY_NAME: &'static str = "Diamond Axe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DiamondHoe;
impl Item for r#DiamondHoe {
    const ID: i32 = 776i32;
    const NAME: &'static str = "diamond_hoe";
    const DISPLAY_NAME: &'static str = "Diamond Hoe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#NetheriteSword;
impl Item for r#NetheriteSword {
    const ID: i32 = 777i32;
    const NAME: &'static str = "netherite_sword";
    const DISPLAY_NAME: &'static str = "Netherite Sword";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#NetheriteShovel;
impl Item for r#NetheriteShovel {
    const ID: i32 = 778i32;
    const NAME: &'static str = "netherite_shovel";
    const DISPLAY_NAME: &'static str = "Netherite Shovel";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#NetheritePickaxe;
impl Item for r#NetheritePickaxe {
    const ID: i32 = 779i32;
    const NAME: &'static str = "netherite_pickaxe";
    const DISPLAY_NAME: &'static str = "Netherite Pickaxe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#NetheriteAxe;
impl Item for r#NetheriteAxe {
    const ID: i32 = 780i32;
    const NAME: &'static str = "netherite_axe";
    const DISPLAY_NAME: &'static str = "Netherite Axe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#NetheriteHoe;
impl Item for r#NetheriteHoe {
    const ID: i32 = 781i32;
    const NAME: &'static str = "netherite_hoe";
    const DISPLAY_NAME: &'static str = "Netherite Hoe";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Stick;
impl Item for r#Stick {
    const ID: i32 = 782i32;
    const NAME: &'static str = "stick";
    const DISPLAY_NAME: &'static str = "Stick";
}
pub struct r#Bowl;
impl Item for r#Bowl {
    const ID: i32 = 783i32;
    const NAME: &'static str = "bowl";
    const DISPLAY_NAME: &'static str = "Bowl";
}
pub struct r#MushroomStew;
impl Item for r#MushroomStew {
    const ID: i32 = 784i32;
    const NAME: &'static str = "mushroom_stew";
    const DISPLAY_NAME: &'static str = "Mushroom Stew";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#String;
impl Item for r#String {
    const ID: i32 = 785i32;
    const NAME: &'static str = "string";
    const DISPLAY_NAME: &'static str = "String";
}
pub struct r#Feather;
impl Item for r#Feather {
    const ID: i32 = 786i32;
    const NAME: &'static str = "feather";
    const DISPLAY_NAME: &'static str = "Feather";
}
pub struct r#Gunpowder;
impl Item for r#Gunpowder {
    const ID: i32 = 787i32;
    const NAME: &'static str = "gunpowder";
    const DISPLAY_NAME: &'static str = "Gunpowder";
}
pub struct r#WheatSeeds;
impl Item for r#WheatSeeds {
    const ID: i32 = 788i32;
    const NAME: &'static str = "wheat_seeds";
    const DISPLAY_NAME: &'static str = "Wheat Seeds";
}
pub struct r#Wheat;
impl Item for r#Wheat {
    const ID: i32 = 789i32;
    const NAME: &'static str = "wheat";
    const DISPLAY_NAME: &'static str = "Wheat";
}
pub struct r#Bread;
impl Item for r#Bread {
    const ID: i32 = 790i32;
    const NAME: &'static str = "bread";
    const DISPLAY_NAME: &'static str = "Bread";
}
pub struct r#LeatherHelmet;
impl Item for r#LeatherHelmet {
    const ID: i32 = 791i32;
    const NAME: &'static str = "leather_helmet";
    const DISPLAY_NAME: &'static str = "Leather Cap";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#LeatherChestplate;
impl Item for r#LeatherChestplate {
    const ID: i32 = 792i32;
    const NAME: &'static str = "leather_chestplate";
    const DISPLAY_NAME: &'static str = "Leather Tunic";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#LeatherLeggings;
impl Item for r#LeatherLeggings {
    const ID: i32 = 793i32;
    const NAME: &'static str = "leather_leggings";
    const DISPLAY_NAME: &'static str = "Leather Pants";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#LeatherBoots;
impl Item for r#LeatherBoots {
    const ID: i32 = 794i32;
    const NAME: &'static str = "leather_boots";
    const DISPLAY_NAME: &'static str = "Leather Boots";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#ChainmailHelmet;
impl Item for r#ChainmailHelmet {
    const ID: i32 = 795i32;
    const NAME: &'static str = "chainmail_helmet";
    const DISPLAY_NAME: &'static str = "Chainmail Helmet";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#ChainmailChestplate;
impl Item for r#ChainmailChestplate {
    const ID: i32 = 796i32;
    const NAME: &'static str = "chainmail_chestplate";
    const DISPLAY_NAME: &'static str = "Chainmail Chestplate";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#ChainmailLeggings;
impl Item for r#ChainmailLeggings {
    const ID: i32 = 797i32;
    const NAME: &'static str = "chainmail_leggings";
    const DISPLAY_NAME: &'static str = "Chainmail Leggings";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#ChainmailBoots;
impl Item for r#ChainmailBoots {
    const ID: i32 = 798i32;
    const NAME: &'static str = "chainmail_boots";
    const DISPLAY_NAME: &'static str = "Chainmail Boots";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#IronHelmet;
impl Item for r#IronHelmet {
    const ID: i32 = 799i32;
    const NAME: &'static str = "iron_helmet";
    const DISPLAY_NAME: &'static str = "Iron Helmet";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#IronChestplate;
impl Item for r#IronChestplate {
    const ID: i32 = 800i32;
    const NAME: &'static str = "iron_chestplate";
    const DISPLAY_NAME: &'static str = "Iron Chestplate";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#IronLeggings;
impl Item for r#IronLeggings {
    const ID: i32 = 801i32;
    const NAME: &'static str = "iron_leggings";
    const DISPLAY_NAME: &'static str = "Iron Leggings";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#IronBoots;
impl Item for r#IronBoots {
    const ID: i32 = 802i32;
    const NAME: &'static str = "iron_boots";
    const DISPLAY_NAME: &'static str = "Iron Boots";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DiamondHelmet;
impl Item for r#DiamondHelmet {
    const ID: i32 = 803i32;
    const NAME: &'static str = "diamond_helmet";
    const DISPLAY_NAME: &'static str = "Diamond Helmet";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DiamondChestplate;
impl Item for r#DiamondChestplate {
    const ID: i32 = 804i32;
    const NAME: &'static str = "diamond_chestplate";
    const DISPLAY_NAME: &'static str = "Diamond Chestplate";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DiamondLeggings;
impl Item for r#DiamondLeggings {
    const ID: i32 = 805i32;
    const NAME: &'static str = "diamond_leggings";
    const DISPLAY_NAME: &'static str = "Diamond Leggings";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DiamondBoots;
impl Item for r#DiamondBoots {
    const ID: i32 = 806i32;
    const NAME: &'static str = "diamond_boots";
    const DISPLAY_NAME: &'static str = "Diamond Boots";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GoldenHelmet;
impl Item for r#GoldenHelmet {
    const ID: i32 = 807i32;
    const NAME: &'static str = "golden_helmet";
    const DISPLAY_NAME: &'static str = "Golden Helmet";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GoldenChestplate;
impl Item for r#GoldenChestplate {
    const ID: i32 = 808i32;
    const NAME: &'static str = "golden_chestplate";
    const DISPLAY_NAME: &'static str = "Golden Chestplate";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GoldenLeggings;
impl Item for r#GoldenLeggings {
    const ID: i32 = 809i32;
    const NAME: &'static str = "golden_leggings";
    const DISPLAY_NAME: &'static str = "Golden Leggings";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GoldenBoots;
impl Item for r#GoldenBoots {
    const ID: i32 = 810i32;
    const NAME: &'static str = "golden_boots";
    const DISPLAY_NAME: &'static str = "Golden Boots";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#NetheriteHelmet;
impl Item for r#NetheriteHelmet {
    const ID: i32 = 811i32;
    const NAME: &'static str = "netherite_helmet";
    const DISPLAY_NAME: &'static str = "Netherite Helmet";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#NetheriteChestplate;
impl Item for r#NetheriteChestplate {
    const ID: i32 = 812i32;
    const NAME: &'static str = "netherite_chestplate";
    const DISPLAY_NAME: &'static str = "Netherite Chestplate";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#NetheriteLeggings;
impl Item for r#NetheriteLeggings {
    const ID: i32 = 813i32;
    const NAME: &'static str = "netherite_leggings";
    const DISPLAY_NAME: &'static str = "Netherite Leggings";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#NetheriteBoots;
impl Item for r#NetheriteBoots {
    const ID: i32 = 814i32;
    const NAME: &'static str = "netherite_boots";
    const DISPLAY_NAME: &'static str = "Netherite Boots";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Flint;
impl Item for r#Flint {
    const ID: i32 = 815i32;
    const NAME: &'static str = "flint";
    const DISPLAY_NAME: &'static str = "Flint";
}
pub struct r#Porkchop;
impl Item for r#Porkchop {
    const ID: i32 = 816i32;
    const NAME: &'static str = "porkchop";
    const DISPLAY_NAME: &'static str = "Raw Porkchop";
}
pub struct r#CookedPorkchop;
impl Item for r#CookedPorkchop {
    const ID: i32 = 817i32;
    const NAME: &'static str = "cooked_porkchop";
    const DISPLAY_NAME: &'static str = "Cooked Porkchop";
}
pub struct r#Painting;
impl Item for r#Painting {
    const ID: i32 = 818i32;
    const NAME: &'static str = "painting";
    const DISPLAY_NAME: &'static str = "Painting";
}
pub struct r#GoldenApple;
impl Item for r#GoldenApple {
    const ID: i32 = 819i32;
    const NAME: &'static str = "golden_apple";
    const DISPLAY_NAME: &'static str = "Golden Apple";
}
pub struct r#EnchantedGoldenApple;
impl Item for r#EnchantedGoldenApple {
    const ID: i32 = 820i32;
    const NAME: &'static str = "enchanted_golden_apple";
    const DISPLAY_NAME: &'static str = "Enchanted Golden Apple";
}
pub struct r#OakSign;
impl Item for r#OakSign {
    const ID: i32 = 821i32;
    const NAME: &'static str = "oak_sign";
    const DISPLAY_NAME: &'static str = "Oak Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#SpruceSign;
impl Item for r#SpruceSign {
    const ID: i32 = 822i32;
    const NAME: &'static str = "spruce_sign";
    const DISPLAY_NAME: &'static str = "Spruce Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#BirchSign;
impl Item for r#BirchSign {
    const ID: i32 = 823i32;
    const NAME: &'static str = "birch_sign";
    const DISPLAY_NAME: &'static str = "Birch Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#JungleSign;
impl Item for r#JungleSign {
    const ID: i32 = 824i32;
    const NAME: &'static str = "jungle_sign";
    const DISPLAY_NAME: &'static str = "Jungle Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#AcaciaSign;
impl Item for r#AcaciaSign {
    const ID: i32 = 825i32;
    const NAME: &'static str = "acacia_sign";
    const DISPLAY_NAME: &'static str = "Acacia Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#DarkOakSign;
impl Item for r#DarkOakSign {
    const ID: i32 = 826i32;
    const NAME: &'static str = "dark_oak_sign";
    const DISPLAY_NAME: &'static str = "Dark Oak Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#MangroveSign;
impl Item for r#MangroveSign {
    const ID: i32 = 827i32;
    const NAME: &'static str = "mangrove_sign";
    const DISPLAY_NAME: &'static str = "Mangrove Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#BambooSign;
impl Item for r#BambooSign {
    const ID: i32 = 828i32;
    const NAME: &'static str = "bamboo_sign";
    const DISPLAY_NAME: &'static str = "Bamboo Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#CrimsonSign;
impl Item for r#CrimsonSign {
    const ID: i32 = 829i32;
    const NAME: &'static str = "crimson_sign";
    const DISPLAY_NAME: &'static str = "Crimson Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#WarpedSign;
impl Item for r#WarpedSign {
    const ID: i32 = 830i32;
    const NAME: &'static str = "warped_sign";
    const DISPLAY_NAME: &'static str = "Warped Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#OakHangingSign;
impl Item for r#OakHangingSign {
    const ID: i32 = 831i32;
    const NAME: &'static str = "oak_hanging_sign";
    const DISPLAY_NAME: &'static str = "Oak Hanging Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#SpruceHangingSign;
impl Item for r#SpruceHangingSign {
    const ID: i32 = 832i32;
    const NAME: &'static str = "spruce_hanging_sign";
    const DISPLAY_NAME: &'static str = "Spruce Hanging Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#BirchHangingSign;
impl Item for r#BirchHangingSign {
    const ID: i32 = 833i32;
    const NAME: &'static str = "birch_hanging_sign";
    const DISPLAY_NAME: &'static str = "Birch Hanging Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#JungleHangingSign;
impl Item for r#JungleHangingSign {
    const ID: i32 = 834i32;
    const NAME: &'static str = "jungle_hanging_sign";
    const DISPLAY_NAME: &'static str = "Jungle Hanging Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#AcaciaHangingSign;
impl Item for r#AcaciaHangingSign {
    const ID: i32 = 835i32;
    const NAME: &'static str = "acacia_hanging_sign";
    const DISPLAY_NAME: &'static str = "Acacia Hanging Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#DarkOakHangingSign;
impl Item for r#DarkOakHangingSign {
    const ID: i32 = 836i32;
    const NAME: &'static str = "dark_oak_hanging_sign";
    const DISPLAY_NAME: &'static str = "Dark Oak Hanging Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#MangroveHangingSign;
impl Item for r#MangroveHangingSign {
    const ID: i32 = 837i32;
    const NAME: &'static str = "mangrove_hanging_sign";
    const DISPLAY_NAME: &'static str = "Mangrove Hanging Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#BambooHangingSign;
impl Item for r#BambooHangingSign {
    const ID: i32 = 838i32;
    const NAME: &'static str = "bamboo_hanging_sign";
    const DISPLAY_NAME: &'static str = "Bamboo Hanging Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#CrimsonHangingSign;
impl Item for r#CrimsonHangingSign {
    const ID: i32 = 839i32;
    const NAME: &'static str = "crimson_hanging_sign";
    const DISPLAY_NAME: &'static str = "Crimson Hanging Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#WarpedHangingSign;
impl Item for r#WarpedHangingSign {
    const ID: i32 = 840i32;
    const NAME: &'static str = "warped_hanging_sign";
    const DISPLAY_NAME: &'static str = "Warped Hanging Sign";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#Bucket;
impl Item for r#Bucket {
    const ID: i32 = 841i32;
    const NAME: &'static str = "bucket";
    const DISPLAY_NAME: &'static str = "Bucket";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#WaterBucket;
impl Item for r#WaterBucket {
    const ID: i32 = 842i32;
    const NAME: &'static str = "water_bucket";
    const DISPLAY_NAME: &'static str = "Water Bucket";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#LavaBucket;
impl Item for r#LavaBucket {
    const ID: i32 = 843i32;
    const NAME: &'static str = "lava_bucket";
    const DISPLAY_NAME: &'static str = "Lava Bucket";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#PowderSnowBucket;
impl Item for r#PowderSnowBucket {
    const ID: i32 = 844i32;
    const NAME: &'static str = "powder_snow_bucket";
    const DISPLAY_NAME: &'static str = "Powder Snow Bucket";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Snowball;
impl Item for r#Snowball {
    const ID: i32 = 845i32;
    const NAME: &'static str = "snowball";
    const DISPLAY_NAME: &'static str = "Snowball";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#Leather;
impl Item for r#Leather {
    const ID: i32 = 846i32;
    const NAME: &'static str = "leather";
    const DISPLAY_NAME: &'static str = "Leather";
}
pub struct r#MilkBucket;
impl Item for r#MilkBucket {
    const ID: i32 = 847i32;
    const NAME: &'static str = "milk_bucket";
    const DISPLAY_NAME: &'static str = "Milk Bucket";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#PufferfishBucket;
impl Item for r#PufferfishBucket {
    const ID: i32 = 848i32;
    const NAME: &'static str = "pufferfish_bucket";
    const DISPLAY_NAME: &'static str = "Bucket of Pufferfish";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#SalmonBucket;
impl Item for r#SalmonBucket {
    const ID: i32 = 849i32;
    const NAME: &'static str = "salmon_bucket";
    const DISPLAY_NAME: &'static str = "Bucket of Salmon";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#CodBucket;
impl Item for r#CodBucket {
    const ID: i32 = 850i32;
    const NAME: &'static str = "cod_bucket";
    const DISPLAY_NAME: &'static str = "Bucket of Cod";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#TropicalFishBucket;
impl Item for r#TropicalFishBucket {
    const ID: i32 = 851i32;
    const NAME: &'static str = "tropical_fish_bucket";
    const DISPLAY_NAME: &'static str = "Bucket of Tropical Fish";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#AxolotlBucket;
impl Item for r#AxolotlBucket {
    const ID: i32 = 852i32;
    const NAME: &'static str = "axolotl_bucket";
    const DISPLAY_NAME: &'static str = "Bucket of Axolotl";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#TadpoleBucket;
impl Item for r#TadpoleBucket {
    const ID: i32 = 853i32;
    const NAME: &'static str = "tadpole_bucket";
    const DISPLAY_NAME: &'static str = "Bucket of Tadpole";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Brick;
impl Item for r#Brick {
    const ID: i32 = 854i32;
    const NAME: &'static str = "brick";
    const DISPLAY_NAME: &'static str = "Brick";
}
pub struct r#ClayBall;
impl Item for r#ClayBall {
    const ID: i32 = 855i32;
    const NAME: &'static str = "clay_ball";
    const DISPLAY_NAME: &'static str = "Clay Ball";
}
pub struct r#DriedKelpBlock;
impl Item for r#DriedKelpBlock {
    const ID: i32 = 856i32;
    const NAME: &'static str = "dried_kelp_block";
    const DISPLAY_NAME: &'static str = "Dried Kelp Block";
}
pub struct r#Paper;
impl Item for r#Paper {
    const ID: i32 = 857i32;
    const NAME: &'static str = "paper";
    const DISPLAY_NAME: &'static str = "Paper";
}
pub struct r#Book;
impl Item for r#Book {
    const ID: i32 = 858i32;
    const NAME: &'static str = "book";
    const DISPLAY_NAME: &'static str = "Book";
}
pub struct r#SlimeBall;
impl Item for r#SlimeBall {
    const ID: i32 = 859i32;
    const NAME: &'static str = "slime_ball";
    const DISPLAY_NAME: &'static str = "Slimeball";
}
pub struct r#Egg;
impl Item for r#Egg {
    const ID: i32 = 860i32;
    const NAME: &'static str = "egg";
    const DISPLAY_NAME: &'static str = "Egg";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#Compass;
impl Item for r#Compass {
    const ID: i32 = 861i32;
    const NAME: &'static str = "compass";
    const DISPLAY_NAME: &'static str = "Compass";
}
pub struct r#RecoveryCompass;
impl Item for r#RecoveryCompass {
    const ID: i32 = 862i32;
    const NAME: &'static str = "recovery_compass";
    const DISPLAY_NAME: &'static str = "Recovery Compass";
}
pub struct r#Bundle;
impl Item for r#Bundle {
    const ID: i32 = 863i32;
    const NAME: &'static str = "bundle";
    const DISPLAY_NAME: &'static str = "Bundle";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#FishingRod;
impl Item for r#FishingRod {
    const ID: i32 = 864i32;
    const NAME: &'static str = "fishing_rod";
    const DISPLAY_NAME: &'static str = "Fishing Rod";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Clock;
impl Item for r#Clock {
    const ID: i32 = 865i32;
    const NAME: &'static str = "clock";
    const DISPLAY_NAME: &'static str = "Clock";
}
pub struct r#Spyglass;
impl Item for r#Spyglass {
    const ID: i32 = 866i32;
    const NAME: &'static str = "spyglass";
    const DISPLAY_NAME: &'static str = "Spyglass";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GlowstoneDust;
impl Item for r#GlowstoneDust {
    const ID: i32 = 867i32;
    const NAME: &'static str = "glowstone_dust";
    const DISPLAY_NAME: &'static str = "Glowstone Dust";
}
pub struct r#Cod;
impl Item for r#Cod {
    const ID: i32 = 868i32;
    const NAME: &'static str = "cod";
    const DISPLAY_NAME: &'static str = "Raw Cod";
}
pub struct r#Salmon;
impl Item for r#Salmon {
    const ID: i32 = 869i32;
    const NAME: &'static str = "salmon";
    const DISPLAY_NAME: &'static str = "Raw Salmon";
}
pub struct r#TropicalFish;
impl Item for r#TropicalFish {
    const ID: i32 = 870i32;
    const NAME: &'static str = "tropical_fish";
    const DISPLAY_NAME: &'static str = "Tropical Fish";
}
pub struct r#Pufferfish;
impl Item for r#Pufferfish {
    const ID: i32 = 871i32;
    const NAME: &'static str = "pufferfish";
    const DISPLAY_NAME: &'static str = "Pufferfish";
}
pub struct r#CookedCod;
impl Item for r#CookedCod {
    const ID: i32 = 872i32;
    const NAME: &'static str = "cooked_cod";
    const DISPLAY_NAME: &'static str = "Cooked Cod";
}
pub struct r#CookedSalmon;
impl Item for r#CookedSalmon {
    const ID: i32 = 873i32;
    const NAME: &'static str = "cooked_salmon";
    const DISPLAY_NAME: &'static str = "Cooked Salmon";
}
pub struct r#InkSac;
impl Item for r#InkSac {
    const ID: i32 = 874i32;
    const NAME: &'static str = "ink_sac";
    const DISPLAY_NAME: &'static str = "Ink Sac";
}
pub struct r#GlowInkSac;
impl Item for r#GlowInkSac {
    const ID: i32 = 875i32;
    const NAME: &'static str = "glow_ink_sac";
    const DISPLAY_NAME: &'static str = "Glow Ink Sac";
}
pub struct r#CocoaBeans;
impl Item for r#CocoaBeans {
    const ID: i32 = 876i32;
    const NAME: &'static str = "cocoa_beans";
    const DISPLAY_NAME: &'static str = "Cocoa Beans";
}
pub struct r#WhiteDye;
impl Item for r#WhiteDye {
    const ID: i32 = 877i32;
    const NAME: &'static str = "white_dye";
    const DISPLAY_NAME: &'static str = "White Dye";
}
pub struct r#OrangeDye;
impl Item for r#OrangeDye {
    const ID: i32 = 878i32;
    const NAME: &'static str = "orange_dye";
    const DISPLAY_NAME: &'static str = "Orange Dye";
}
pub struct r#MagentaDye;
impl Item for r#MagentaDye {
    const ID: i32 = 879i32;
    const NAME: &'static str = "magenta_dye";
    const DISPLAY_NAME: &'static str = "Magenta Dye";
}
pub struct r#LightBlueDye;
impl Item for r#LightBlueDye {
    const ID: i32 = 880i32;
    const NAME: &'static str = "light_blue_dye";
    const DISPLAY_NAME: &'static str = "Light Blue Dye";
}
pub struct r#YellowDye;
impl Item for r#YellowDye {
    const ID: i32 = 881i32;
    const NAME: &'static str = "yellow_dye";
    const DISPLAY_NAME: &'static str = "Yellow Dye";
}
pub struct r#LimeDye;
impl Item for r#LimeDye {
    const ID: i32 = 882i32;
    const NAME: &'static str = "lime_dye";
    const DISPLAY_NAME: &'static str = "Lime Dye";
}
pub struct r#PinkDye;
impl Item for r#PinkDye {
    const ID: i32 = 883i32;
    const NAME: &'static str = "pink_dye";
    const DISPLAY_NAME: &'static str = "Pink Dye";
}
pub struct r#GrayDye;
impl Item for r#GrayDye {
    const ID: i32 = 884i32;
    const NAME: &'static str = "gray_dye";
    const DISPLAY_NAME: &'static str = "Gray Dye";
}
pub struct r#LightGrayDye;
impl Item for r#LightGrayDye {
    const ID: i32 = 885i32;
    const NAME: &'static str = "light_gray_dye";
    const DISPLAY_NAME: &'static str = "Light Gray Dye";
}
pub struct r#CyanDye;
impl Item for r#CyanDye {
    const ID: i32 = 886i32;
    const NAME: &'static str = "cyan_dye";
    const DISPLAY_NAME: &'static str = "Cyan Dye";
}
pub struct r#PurpleDye;
impl Item for r#PurpleDye {
    const ID: i32 = 887i32;
    const NAME: &'static str = "purple_dye";
    const DISPLAY_NAME: &'static str = "Purple Dye";
}
pub struct r#BlueDye;
impl Item for r#BlueDye {
    const ID: i32 = 888i32;
    const NAME: &'static str = "blue_dye";
    const DISPLAY_NAME: &'static str = "Blue Dye";
}
pub struct r#BrownDye;
impl Item for r#BrownDye {
    const ID: i32 = 889i32;
    const NAME: &'static str = "brown_dye";
    const DISPLAY_NAME: &'static str = "Brown Dye";
}
pub struct r#GreenDye;
impl Item for r#GreenDye {
    const ID: i32 = 890i32;
    const NAME: &'static str = "green_dye";
    const DISPLAY_NAME: &'static str = "Green Dye";
}
pub struct r#RedDye;
impl Item for r#RedDye {
    const ID: i32 = 891i32;
    const NAME: &'static str = "red_dye";
    const DISPLAY_NAME: &'static str = "Red Dye";
}
pub struct r#BlackDye;
impl Item for r#BlackDye {
    const ID: i32 = 892i32;
    const NAME: &'static str = "black_dye";
    const DISPLAY_NAME: &'static str = "Black Dye";
}
pub struct r#BoneMeal;
impl Item for r#BoneMeal {
    const ID: i32 = 893i32;
    const NAME: &'static str = "bone_meal";
    const DISPLAY_NAME: &'static str = "Bone Meal";
}
pub struct r#Bone;
impl Item for r#Bone {
    const ID: i32 = 894i32;
    const NAME: &'static str = "bone";
    const DISPLAY_NAME: &'static str = "Bone";
}
pub struct r#Sugar;
impl Item for r#Sugar {
    const ID: i32 = 895i32;
    const NAME: &'static str = "sugar";
    const DISPLAY_NAME: &'static str = "Sugar";
}
pub struct r#Cake;
impl Item for r#Cake {
    const ID: i32 = 896i32;
    const NAME: &'static str = "cake";
    const DISPLAY_NAME: &'static str = "Cake";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#WhiteBed;
impl Item for r#WhiteBed {
    const ID: i32 = 897i32;
    const NAME: &'static str = "white_bed";
    const DISPLAY_NAME: &'static str = "White Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#OrangeBed;
impl Item for r#OrangeBed {
    const ID: i32 = 898i32;
    const NAME: &'static str = "orange_bed";
    const DISPLAY_NAME: &'static str = "Orange Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MagentaBed;
impl Item for r#MagentaBed {
    const ID: i32 = 899i32;
    const NAME: &'static str = "magenta_bed";
    const DISPLAY_NAME: &'static str = "Magenta Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#LightBlueBed;
impl Item for r#LightBlueBed {
    const ID: i32 = 900i32;
    const NAME: &'static str = "light_blue_bed";
    const DISPLAY_NAME: &'static str = "Light Blue Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#YellowBed;
impl Item for r#YellowBed {
    const ID: i32 = 901i32;
    const NAME: &'static str = "yellow_bed";
    const DISPLAY_NAME: &'static str = "Yellow Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#LimeBed;
impl Item for r#LimeBed {
    const ID: i32 = 902i32;
    const NAME: &'static str = "lime_bed";
    const DISPLAY_NAME: &'static str = "Lime Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#PinkBed;
impl Item for r#PinkBed {
    const ID: i32 = 903i32;
    const NAME: &'static str = "pink_bed";
    const DISPLAY_NAME: &'static str = "Pink Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GrayBed;
impl Item for r#GrayBed {
    const ID: i32 = 904i32;
    const NAME: &'static str = "gray_bed";
    const DISPLAY_NAME: &'static str = "Gray Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#LightGrayBed;
impl Item for r#LightGrayBed {
    const ID: i32 = 905i32;
    const NAME: &'static str = "light_gray_bed";
    const DISPLAY_NAME: &'static str = "Light Gray Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#CyanBed;
impl Item for r#CyanBed {
    const ID: i32 = 906i32;
    const NAME: &'static str = "cyan_bed";
    const DISPLAY_NAME: &'static str = "Cyan Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#PurpleBed;
impl Item for r#PurpleBed {
    const ID: i32 = 907i32;
    const NAME: &'static str = "purple_bed";
    const DISPLAY_NAME: &'static str = "Purple Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#BlueBed;
impl Item for r#BlueBed {
    const ID: i32 = 908i32;
    const NAME: &'static str = "blue_bed";
    const DISPLAY_NAME: &'static str = "Blue Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#BrownBed;
impl Item for r#BrownBed {
    const ID: i32 = 909i32;
    const NAME: &'static str = "brown_bed";
    const DISPLAY_NAME: &'static str = "Brown Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GreenBed;
impl Item for r#GreenBed {
    const ID: i32 = 910i32;
    const NAME: &'static str = "green_bed";
    const DISPLAY_NAME: &'static str = "Green Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#RedBed;
impl Item for r#RedBed {
    const ID: i32 = 911i32;
    const NAME: &'static str = "red_bed";
    const DISPLAY_NAME: &'static str = "Red Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#BlackBed;
impl Item for r#BlackBed {
    const ID: i32 = 912i32;
    const NAME: &'static str = "black_bed";
    const DISPLAY_NAME: &'static str = "Black Bed";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Cookie;
impl Item for r#Cookie {
    const ID: i32 = 913i32;
    const NAME: &'static str = "cookie";
    const DISPLAY_NAME: &'static str = "Cookie";
}
pub struct r#FilledMap;
impl Item for r#FilledMap {
    const ID: i32 = 914i32;
    const NAME: &'static str = "filled_map";
    const DISPLAY_NAME: &'static str = "Map";
}
pub struct r#Shears;
impl Item for r#Shears {
    const ID: i32 = 915i32;
    const NAME: &'static str = "shears";
    const DISPLAY_NAME: &'static str = "Shears";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MelonSlice;
impl Item for r#MelonSlice {
    const ID: i32 = 916i32;
    const NAME: &'static str = "melon_slice";
    const DISPLAY_NAME: &'static str = "Melon Slice";
}
pub struct r#DriedKelp;
impl Item for r#DriedKelp {
    const ID: i32 = 917i32;
    const NAME: &'static str = "dried_kelp";
    const DISPLAY_NAME: &'static str = "Dried Kelp";
}
pub struct r#PumpkinSeeds;
impl Item for r#PumpkinSeeds {
    const ID: i32 = 918i32;
    const NAME: &'static str = "pumpkin_seeds";
    const DISPLAY_NAME: &'static str = "Pumpkin Seeds";
}
pub struct r#MelonSeeds;
impl Item for r#MelonSeeds {
    const ID: i32 = 919i32;
    const NAME: &'static str = "melon_seeds";
    const DISPLAY_NAME: &'static str = "Melon Seeds";
}
pub struct r#Beef;
impl Item for r#Beef {
    const ID: i32 = 920i32;
    const NAME: &'static str = "beef";
    const DISPLAY_NAME: &'static str = "Raw Beef";
}
pub struct r#CookedBeef;
impl Item for r#CookedBeef {
    const ID: i32 = 921i32;
    const NAME: &'static str = "cooked_beef";
    const DISPLAY_NAME: &'static str = "Steak";
}
pub struct r#Chicken;
impl Item for r#Chicken {
    const ID: i32 = 922i32;
    const NAME: &'static str = "chicken";
    const DISPLAY_NAME: &'static str = "Raw Chicken";
}
pub struct r#CookedChicken;
impl Item for r#CookedChicken {
    const ID: i32 = 923i32;
    const NAME: &'static str = "cooked_chicken";
    const DISPLAY_NAME: &'static str = "Cooked Chicken";
}
pub struct r#RottenFlesh;
impl Item for r#RottenFlesh {
    const ID: i32 = 924i32;
    const NAME: &'static str = "rotten_flesh";
    const DISPLAY_NAME: &'static str = "Rotten Flesh";
}
pub struct r#EnderPearl;
impl Item for r#EnderPearl {
    const ID: i32 = 925i32;
    const NAME: &'static str = "ender_pearl";
    const DISPLAY_NAME: &'static str = "Ender Pearl";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#BlazeRod;
impl Item for r#BlazeRod {
    const ID: i32 = 926i32;
    const NAME: &'static str = "blaze_rod";
    const DISPLAY_NAME: &'static str = "Blaze Rod";
}
pub struct r#GhastTear;
impl Item for r#GhastTear {
    const ID: i32 = 927i32;
    const NAME: &'static str = "ghast_tear";
    const DISPLAY_NAME: &'static str = "Ghast Tear";
}
pub struct r#GoldNugget;
impl Item for r#GoldNugget {
    const ID: i32 = 928i32;
    const NAME: &'static str = "gold_nugget";
    const DISPLAY_NAME: &'static str = "Gold Nugget";
}
pub struct r#NetherWart;
impl Item for r#NetherWart {
    const ID: i32 = 929i32;
    const NAME: &'static str = "nether_wart";
    const DISPLAY_NAME: &'static str = "Nether Wart";
}
pub struct r#Potion;
impl Item for r#Potion {
    const ID: i32 = 930i32;
    const NAME: &'static str = "potion";
    const DISPLAY_NAME: &'static str = "Potion";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GlassBottle;
impl Item for r#GlassBottle {
    const ID: i32 = 931i32;
    const NAME: &'static str = "glass_bottle";
    const DISPLAY_NAME: &'static str = "Glass Bottle";
}
pub struct r#SpiderEye;
impl Item for r#SpiderEye {
    const ID: i32 = 932i32;
    const NAME: &'static str = "spider_eye";
    const DISPLAY_NAME: &'static str = "Spider Eye";
}
pub struct r#FermentedSpiderEye;
impl Item for r#FermentedSpiderEye {
    const ID: i32 = 933i32;
    const NAME: &'static str = "fermented_spider_eye";
    const DISPLAY_NAME: &'static str = "Fermented Spider Eye";
}
pub struct r#BlazePowder;
impl Item for r#BlazePowder {
    const ID: i32 = 934i32;
    const NAME: &'static str = "blaze_powder";
    const DISPLAY_NAME: &'static str = "Blaze Powder";
}
pub struct r#MagmaCream;
impl Item for r#MagmaCream {
    const ID: i32 = 935i32;
    const NAME: &'static str = "magma_cream";
    const DISPLAY_NAME: &'static str = "Magma Cream";
}
pub struct r#BrewingStand;
impl Item for r#BrewingStand {
    const ID: i32 = 936i32;
    const NAME: &'static str = "brewing_stand";
    const DISPLAY_NAME: &'static str = "Brewing Stand";
}
pub struct r#Cauldron;
impl Item for r#Cauldron {
    const ID: i32 = 937i32;
    const NAME: &'static str = "cauldron";
    const DISPLAY_NAME: &'static str = "Cauldron";
}
pub struct r#EnderEye;
impl Item for r#EnderEye {
    const ID: i32 = 938i32;
    const NAME: &'static str = "ender_eye";
    const DISPLAY_NAME: &'static str = "Eye of Ender";
}
pub struct r#GlisteringMelonSlice;
impl Item for r#GlisteringMelonSlice {
    const ID: i32 = 939i32;
    const NAME: &'static str = "glistering_melon_slice";
    const DISPLAY_NAME: &'static str = "Glistering Melon Slice";
}
pub struct r#AllaySpawnEgg;
impl Item for r#AllaySpawnEgg {
    const ID: i32 = 940i32;
    const NAME: &'static str = "allay_spawn_egg";
    const DISPLAY_NAME: &'static str = "Allay Spawn Egg";
}
pub struct r#AxolotlSpawnEgg;
impl Item for r#AxolotlSpawnEgg {
    const ID: i32 = 941i32;
    const NAME: &'static str = "axolotl_spawn_egg";
    const DISPLAY_NAME: &'static str = "Axolotl Spawn Egg";
}
pub struct r#BatSpawnEgg;
impl Item for r#BatSpawnEgg {
    const ID: i32 = 942i32;
    const NAME: &'static str = "bat_spawn_egg";
    const DISPLAY_NAME: &'static str = "Bat Spawn Egg";
}
pub struct r#BeeSpawnEgg;
impl Item for r#BeeSpawnEgg {
    const ID: i32 = 943i32;
    const NAME: &'static str = "bee_spawn_egg";
    const DISPLAY_NAME: &'static str = "Bee Spawn Egg";
}
pub struct r#BlazeSpawnEgg;
impl Item for r#BlazeSpawnEgg {
    const ID: i32 = 944i32;
    const NAME: &'static str = "blaze_spawn_egg";
    const DISPLAY_NAME: &'static str = "Blaze Spawn Egg";
}
pub struct r#CatSpawnEgg;
impl Item for r#CatSpawnEgg {
    const ID: i32 = 945i32;
    const NAME: &'static str = "cat_spawn_egg";
    const DISPLAY_NAME: &'static str = "Cat Spawn Egg";
}
pub struct r#CamelSpawnEgg;
impl Item for r#CamelSpawnEgg {
    const ID: i32 = 946i32;
    const NAME: &'static str = "camel_spawn_egg";
    const DISPLAY_NAME: &'static str = "Camel Spawn Egg";
}
pub struct r#CaveSpiderSpawnEgg;
impl Item for r#CaveSpiderSpawnEgg {
    const ID: i32 = 947i32;
    const NAME: &'static str = "cave_spider_spawn_egg";
    const DISPLAY_NAME: &'static str = "Cave Spider Spawn Egg";
}
pub struct r#ChickenSpawnEgg;
impl Item for r#ChickenSpawnEgg {
    const ID: i32 = 948i32;
    const NAME: &'static str = "chicken_spawn_egg";
    const DISPLAY_NAME: &'static str = "Chicken Spawn Egg";
}
pub struct r#CodSpawnEgg;
impl Item for r#CodSpawnEgg {
    const ID: i32 = 949i32;
    const NAME: &'static str = "cod_spawn_egg";
    const DISPLAY_NAME: &'static str = "Cod Spawn Egg";
}
pub struct r#CowSpawnEgg;
impl Item for r#CowSpawnEgg {
    const ID: i32 = 950i32;
    const NAME: &'static str = "cow_spawn_egg";
    const DISPLAY_NAME: &'static str = "Cow Spawn Egg";
}
pub struct r#CreeperSpawnEgg;
impl Item for r#CreeperSpawnEgg {
    const ID: i32 = 951i32;
    const NAME: &'static str = "creeper_spawn_egg";
    const DISPLAY_NAME: &'static str = "Creeper Spawn Egg";
}
pub struct r#DolphinSpawnEgg;
impl Item for r#DolphinSpawnEgg {
    const ID: i32 = 952i32;
    const NAME: &'static str = "dolphin_spawn_egg";
    const DISPLAY_NAME: &'static str = "Dolphin Spawn Egg";
}
pub struct r#DonkeySpawnEgg;
impl Item for r#DonkeySpawnEgg {
    const ID: i32 = 953i32;
    const NAME: &'static str = "donkey_spawn_egg";
    const DISPLAY_NAME: &'static str = "Donkey Spawn Egg";
}
pub struct r#DrownedSpawnEgg;
impl Item for r#DrownedSpawnEgg {
    const ID: i32 = 954i32;
    const NAME: &'static str = "drowned_spawn_egg";
    const DISPLAY_NAME: &'static str = "Drowned Spawn Egg";
}
pub struct r#ElderGuardianSpawnEgg;
impl Item for r#ElderGuardianSpawnEgg {
    const ID: i32 = 955i32;
    const NAME: &'static str = "elder_guardian_spawn_egg";
    const DISPLAY_NAME: &'static str = "Elder Guardian Spawn Egg";
}
pub struct r#EnderDragonSpawnEgg;
impl Item for r#EnderDragonSpawnEgg {
    const ID: i32 = 956i32;
    const NAME: &'static str = "ender_dragon_spawn_egg";
    const DISPLAY_NAME: &'static str = "Ender Dragon Spawn Egg";
}
pub struct r#EndermanSpawnEgg;
impl Item for r#EndermanSpawnEgg {
    const ID: i32 = 957i32;
    const NAME: &'static str = "enderman_spawn_egg";
    const DISPLAY_NAME: &'static str = "Enderman Spawn Egg";
}
pub struct r#EndermiteSpawnEgg;
impl Item for r#EndermiteSpawnEgg {
    const ID: i32 = 958i32;
    const NAME: &'static str = "endermite_spawn_egg";
    const DISPLAY_NAME: &'static str = "Endermite Spawn Egg";
}
pub struct r#EvokerSpawnEgg;
impl Item for r#EvokerSpawnEgg {
    const ID: i32 = 959i32;
    const NAME: &'static str = "evoker_spawn_egg";
    const DISPLAY_NAME: &'static str = "Evoker Spawn Egg";
}
pub struct r#FoxSpawnEgg;
impl Item for r#FoxSpawnEgg {
    const ID: i32 = 960i32;
    const NAME: &'static str = "fox_spawn_egg";
    const DISPLAY_NAME: &'static str = "Fox Spawn Egg";
}
pub struct r#FrogSpawnEgg;
impl Item for r#FrogSpawnEgg {
    const ID: i32 = 961i32;
    const NAME: &'static str = "frog_spawn_egg";
    const DISPLAY_NAME: &'static str = "Frog Spawn Egg";
}
pub struct r#GhastSpawnEgg;
impl Item for r#GhastSpawnEgg {
    const ID: i32 = 962i32;
    const NAME: &'static str = "ghast_spawn_egg";
    const DISPLAY_NAME: &'static str = "Ghast Spawn Egg";
}
pub struct r#GlowSquidSpawnEgg;
impl Item for r#GlowSquidSpawnEgg {
    const ID: i32 = 963i32;
    const NAME: &'static str = "glow_squid_spawn_egg";
    const DISPLAY_NAME: &'static str = "Glow Squid Spawn Egg";
}
pub struct r#GoatSpawnEgg;
impl Item for r#GoatSpawnEgg {
    const ID: i32 = 964i32;
    const NAME: &'static str = "goat_spawn_egg";
    const DISPLAY_NAME: &'static str = "Goat Spawn Egg";
}
pub struct r#GuardianSpawnEgg;
impl Item for r#GuardianSpawnEgg {
    const ID: i32 = 965i32;
    const NAME: &'static str = "guardian_spawn_egg";
    const DISPLAY_NAME: &'static str = "Guardian Spawn Egg";
}
pub struct r#HoglinSpawnEgg;
impl Item for r#HoglinSpawnEgg {
    const ID: i32 = 966i32;
    const NAME: &'static str = "hoglin_spawn_egg";
    const DISPLAY_NAME: &'static str = "Hoglin Spawn Egg";
}
pub struct r#HorseSpawnEgg;
impl Item for r#HorseSpawnEgg {
    const ID: i32 = 967i32;
    const NAME: &'static str = "horse_spawn_egg";
    const DISPLAY_NAME: &'static str = "Horse Spawn Egg";
}
pub struct r#HuskSpawnEgg;
impl Item for r#HuskSpawnEgg {
    const ID: i32 = 968i32;
    const NAME: &'static str = "husk_spawn_egg";
    const DISPLAY_NAME: &'static str = "Husk Spawn Egg";
}
pub struct r#IronGolemSpawnEgg;
impl Item for r#IronGolemSpawnEgg {
    const ID: i32 = 969i32;
    const NAME: &'static str = "iron_golem_spawn_egg";
    const DISPLAY_NAME: &'static str = "Iron Golem Spawn Egg";
}
pub struct r#LlamaSpawnEgg;
impl Item for r#LlamaSpawnEgg {
    const ID: i32 = 970i32;
    const NAME: &'static str = "llama_spawn_egg";
    const DISPLAY_NAME: &'static str = "Llama Spawn Egg";
}
pub struct r#MagmaCubeSpawnEgg;
impl Item for r#MagmaCubeSpawnEgg {
    const ID: i32 = 971i32;
    const NAME: &'static str = "magma_cube_spawn_egg";
    const DISPLAY_NAME: &'static str = "Magma Cube Spawn Egg";
}
pub struct r#MooshroomSpawnEgg;
impl Item for r#MooshroomSpawnEgg {
    const ID: i32 = 972i32;
    const NAME: &'static str = "mooshroom_spawn_egg";
    const DISPLAY_NAME: &'static str = "Mooshroom Spawn Egg";
}
pub struct r#MuleSpawnEgg;
impl Item for r#MuleSpawnEgg {
    const ID: i32 = 973i32;
    const NAME: &'static str = "mule_spawn_egg";
    const DISPLAY_NAME: &'static str = "Mule Spawn Egg";
}
pub struct r#OcelotSpawnEgg;
impl Item for r#OcelotSpawnEgg {
    const ID: i32 = 974i32;
    const NAME: &'static str = "ocelot_spawn_egg";
    const DISPLAY_NAME: &'static str = "Ocelot Spawn Egg";
}
pub struct r#PandaSpawnEgg;
impl Item for r#PandaSpawnEgg {
    const ID: i32 = 975i32;
    const NAME: &'static str = "panda_spawn_egg";
    const DISPLAY_NAME: &'static str = "Panda Spawn Egg";
}
pub struct r#ParrotSpawnEgg;
impl Item for r#ParrotSpawnEgg {
    const ID: i32 = 976i32;
    const NAME: &'static str = "parrot_spawn_egg";
    const DISPLAY_NAME: &'static str = "Parrot Spawn Egg";
}
pub struct r#PhantomSpawnEgg;
impl Item for r#PhantomSpawnEgg {
    const ID: i32 = 977i32;
    const NAME: &'static str = "phantom_spawn_egg";
    const DISPLAY_NAME: &'static str = "Phantom Spawn Egg";
}
pub struct r#PigSpawnEgg;
impl Item for r#PigSpawnEgg {
    const ID: i32 = 978i32;
    const NAME: &'static str = "pig_spawn_egg";
    const DISPLAY_NAME: &'static str = "Pig Spawn Egg";
}
pub struct r#PiglinSpawnEgg;
impl Item for r#PiglinSpawnEgg {
    const ID: i32 = 979i32;
    const NAME: &'static str = "piglin_spawn_egg";
    const DISPLAY_NAME: &'static str = "Piglin Spawn Egg";
}
pub struct r#PiglinBruteSpawnEgg;
impl Item for r#PiglinBruteSpawnEgg {
    const ID: i32 = 980i32;
    const NAME: &'static str = "piglin_brute_spawn_egg";
    const DISPLAY_NAME: &'static str = "Piglin Brute Spawn Egg";
}
pub struct r#PillagerSpawnEgg;
impl Item for r#PillagerSpawnEgg {
    const ID: i32 = 981i32;
    const NAME: &'static str = "pillager_spawn_egg";
    const DISPLAY_NAME: &'static str = "Pillager Spawn Egg";
}
pub struct r#PolarBearSpawnEgg;
impl Item for r#PolarBearSpawnEgg {
    const ID: i32 = 982i32;
    const NAME: &'static str = "polar_bear_spawn_egg";
    const DISPLAY_NAME: &'static str = "Polar Bear Spawn Egg";
}
pub struct r#PufferfishSpawnEgg;
impl Item for r#PufferfishSpawnEgg {
    const ID: i32 = 983i32;
    const NAME: &'static str = "pufferfish_spawn_egg";
    const DISPLAY_NAME: &'static str = "Pufferfish Spawn Egg";
}
pub struct r#RabbitSpawnEgg;
impl Item for r#RabbitSpawnEgg {
    const ID: i32 = 984i32;
    const NAME: &'static str = "rabbit_spawn_egg";
    const DISPLAY_NAME: &'static str = "Rabbit Spawn Egg";
}
pub struct r#RavagerSpawnEgg;
impl Item for r#RavagerSpawnEgg {
    const ID: i32 = 985i32;
    const NAME: &'static str = "ravager_spawn_egg";
    const DISPLAY_NAME: &'static str = "Ravager Spawn Egg";
}
pub struct r#SalmonSpawnEgg;
impl Item for r#SalmonSpawnEgg {
    const ID: i32 = 986i32;
    const NAME: &'static str = "salmon_spawn_egg";
    const DISPLAY_NAME: &'static str = "Salmon Spawn Egg";
}
pub struct r#SheepSpawnEgg;
impl Item for r#SheepSpawnEgg {
    const ID: i32 = 987i32;
    const NAME: &'static str = "sheep_spawn_egg";
    const DISPLAY_NAME: &'static str = "Sheep Spawn Egg";
}
pub struct r#ShulkerSpawnEgg;
impl Item for r#ShulkerSpawnEgg {
    const ID: i32 = 988i32;
    const NAME: &'static str = "shulker_spawn_egg";
    const DISPLAY_NAME: &'static str = "Shulker Spawn Egg";
}
pub struct r#SilverfishSpawnEgg;
impl Item for r#SilverfishSpawnEgg {
    const ID: i32 = 989i32;
    const NAME: &'static str = "silverfish_spawn_egg";
    const DISPLAY_NAME: &'static str = "Silverfish Spawn Egg";
}
pub struct r#SkeletonSpawnEgg;
impl Item for r#SkeletonSpawnEgg {
    const ID: i32 = 990i32;
    const NAME: &'static str = "skeleton_spawn_egg";
    const DISPLAY_NAME: &'static str = "Skeleton Spawn Egg";
}
pub struct r#SkeletonHorseSpawnEgg;
impl Item for r#SkeletonHorseSpawnEgg {
    const ID: i32 = 991i32;
    const NAME: &'static str = "skeleton_horse_spawn_egg";
    const DISPLAY_NAME: &'static str = "Skeleton Horse Spawn Egg";
}
pub struct r#SlimeSpawnEgg;
impl Item for r#SlimeSpawnEgg {
    const ID: i32 = 992i32;
    const NAME: &'static str = "slime_spawn_egg";
    const DISPLAY_NAME: &'static str = "Slime Spawn Egg";
}
pub struct r#SnowGolemSpawnEgg;
impl Item for r#SnowGolemSpawnEgg {
    const ID: i32 = 993i32;
    const NAME: &'static str = "snow_golem_spawn_egg";
    const DISPLAY_NAME: &'static str = "Snow Golem Spawn Egg";
}
pub struct r#SpiderSpawnEgg;
impl Item for r#SpiderSpawnEgg {
    const ID: i32 = 994i32;
    const NAME: &'static str = "spider_spawn_egg";
    const DISPLAY_NAME: &'static str = "Spider Spawn Egg";
}
pub struct r#SquidSpawnEgg;
impl Item for r#SquidSpawnEgg {
    const ID: i32 = 995i32;
    const NAME: &'static str = "squid_spawn_egg";
    const DISPLAY_NAME: &'static str = "Squid Spawn Egg";
}
pub struct r#StraySpawnEgg;
impl Item for r#StraySpawnEgg {
    const ID: i32 = 996i32;
    const NAME: &'static str = "stray_spawn_egg";
    const DISPLAY_NAME: &'static str = "Stray Spawn Egg";
}
pub struct r#StriderSpawnEgg;
impl Item for r#StriderSpawnEgg {
    const ID: i32 = 997i32;
    const NAME: &'static str = "strider_spawn_egg";
    const DISPLAY_NAME: &'static str = "Strider Spawn Egg";
}
pub struct r#TadpoleSpawnEgg;
impl Item for r#TadpoleSpawnEgg {
    const ID: i32 = 998i32;
    const NAME: &'static str = "tadpole_spawn_egg";
    const DISPLAY_NAME: &'static str = "Tadpole Spawn Egg";
}
pub struct r#TraderLlamaSpawnEgg;
impl Item for r#TraderLlamaSpawnEgg {
    const ID: i32 = 999i32;
    const NAME: &'static str = "trader_llama_spawn_egg";
    const DISPLAY_NAME: &'static str = "Trader Llama Spawn Egg";
}
pub struct r#TropicalFishSpawnEgg;
impl Item for r#TropicalFishSpawnEgg {
    const ID: i32 = 1000i32;
    const NAME: &'static str = "tropical_fish_spawn_egg";
    const DISPLAY_NAME: &'static str = "Tropical Fish Spawn Egg";
}
pub struct r#TurtleSpawnEgg;
impl Item for r#TurtleSpawnEgg {
    const ID: i32 = 1001i32;
    const NAME: &'static str = "turtle_spawn_egg";
    const DISPLAY_NAME: &'static str = "Turtle Spawn Egg";
}
pub struct r#VexSpawnEgg;
impl Item for r#VexSpawnEgg {
    const ID: i32 = 1002i32;
    const NAME: &'static str = "vex_spawn_egg";
    const DISPLAY_NAME: &'static str = "Vex Spawn Egg";
}
pub struct r#VillagerSpawnEgg;
impl Item for r#VillagerSpawnEgg {
    const ID: i32 = 1003i32;
    const NAME: &'static str = "villager_spawn_egg";
    const DISPLAY_NAME: &'static str = "Villager Spawn Egg";
}
pub struct r#VindicatorSpawnEgg;
impl Item for r#VindicatorSpawnEgg {
    const ID: i32 = 1004i32;
    const NAME: &'static str = "vindicator_spawn_egg";
    const DISPLAY_NAME: &'static str = "Vindicator Spawn Egg";
}
pub struct r#WanderingTraderSpawnEgg;
impl Item for r#WanderingTraderSpawnEgg {
    const ID: i32 = 1005i32;
    const NAME: &'static str = "wandering_trader_spawn_egg";
    const DISPLAY_NAME: &'static str = "Wandering Trader Spawn Egg";
}
pub struct r#WardenSpawnEgg;
impl Item for r#WardenSpawnEgg {
    const ID: i32 = 1006i32;
    const NAME: &'static str = "warden_spawn_egg";
    const DISPLAY_NAME: &'static str = "Warden Spawn Egg";
}
pub struct r#WitchSpawnEgg;
impl Item for r#WitchSpawnEgg {
    const ID: i32 = 1007i32;
    const NAME: &'static str = "witch_spawn_egg";
    const DISPLAY_NAME: &'static str = "Witch Spawn Egg";
}
pub struct r#WitherSpawnEgg;
impl Item for r#WitherSpawnEgg {
    const ID: i32 = 1008i32;
    const NAME: &'static str = "wither_spawn_egg";
    const DISPLAY_NAME: &'static str = "Wither Spawn Egg";
}
pub struct r#WitherSkeletonSpawnEgg;
impl Item for r#WitherSkeletonSpawnEgg {
    const ID: i32 = 1009i32;
    const NAME: &'static str = "wither_skeleton_spawn_egg";
    const DISPLAY_NAME: &'static str = "Wither Skeleton Spawn Egg";
}
pub struct r#WolfSpawnEgg;
impl Item for r#WolfSpawnEgg {
    const ID: i32 = 1010i32;
    const NAME: &'static str = "wolf_spawn_egg";
    const DISPLAY_NAME: &'static str = "Wolf Spawn Egg";
}
pub struct r#ZoglinSpawnEgg;
impl Item for r#ZoglinSpawnEgg {
    const ID: i32 = 1011i32;
    const NAME: &'static str = "zoglin_spawn_egg";
    const DISPLAY_NAME: &'static str = "Zoglin Spawn Egg";
}
pub struct r#ZombieSpawnEgg;
impl Item for r#ZombieSpawnEgg {
    const ID: i32 = 1012i32;
    const NAME: &'static str = "zombie_spawn_egg";
    const DISPLAY_NAME: &'static str = "Zombie Spawn Egg";
}
pub struct r#ZombieHorseSpawnEgg;
impl Item for r#ZombieHorseSpawnEgg {
    const ID: i32 = 1013i32;
    const NAME: &'static str = "zombie_horse_spawn_egg";
    const DISPLAY_NAME: &'static str = "Zombie Horse Spawn Egg";
}
pub struct r#ZombieVillagerSpawnEgg;
impl Item for r#ZombieVillagerSpawnEgg {
    const ID: i32 = 1014i32;
    const NAME: &'static str = "zombie_villager_spawn_egg";
    const DISPLAY_NAME: &'static str = "Zombie Villager Spawn Egg";
}
pub struct r#ZombifiedPiglinSpawnEgg;
impl Item for r#ZombifiedPiglinSpawnEgg {
    const ID: i32 = 1015i32;
    const NAME: &'static str = "zombified_piglin_spawn_egg";
    const DISPLAY_NAME: &'static str = "Zombified Piglin Spawn Egg";
}
pub struct r#ExperienceBottle;
impl Item for r#ExperienceBottle {
    const ID: i32 = 1016i32;
    const NAME: &'static str = "experience_bottle";
    const DISPLAY_NAME: &'static str = "Bottle o' Enchanting";
}
pub struct r#FireCharge;
impl Item for r#FireCharge {
    const ID: i32 = 1017i32;
    const NAME: &'static str = "fire_charge";
    const DISPLAY_NAME: &'static str = "Fire Charge";
}
pub struct r#WritableBook;
impl Item for r#WritableBook {
    const ID: i32 = 1018i32;
    const NAME: &'static str = "writable_book";
    const DISPLAY_NAME: &'static str = "Book and Quill";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#WrittenBook;
impl Item for r#WrittenBook {
    const ID: i32 = 1019i32;
    const NAME: &'static str = "written_book";
    const DISPLAY_NAME: &'static str = "Written Book";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#ItemFrame;
impl Item for r#ItemFrame {
    const ID: i32 = 1020i32;
    const NAME: &'static str = "item_frame";
    const DISPLAY_NAME: &'static str = "Item Frame";
}
pub struct r#GlowItemFrame;
impl Item for r#GlowItemFrame {
    const ID: i32 = 1021i32;
    const NAME: &'static str = "glow_item_frame";
    const DISPLAY_NAME: &'static str = "Glow Item Frame";
}
pub struct r#FlowerPot;
impl Item for r#FlowerPot {
    const ID: i32 = 1022i32;
    const NAME: &'static str = "flower_pot";
    const DISPLAY_NAME: &'static str = "Flower Pot";
}
pub struct r#Carrot;
impl Item for r#Carrot {
    const ID: i32 = 1023i32;
    const NAME: &'static str = "carrot";
    const DISPLAY_NAME: &'static str = "Carrot";
}
pub struct r#Potato;
impl Item for r#Potato {
    const ID: i32 = 1024i32;
    const NAME: &'static str = "potato";
    const DISPLAY_NAME: &'static str = "Potato";
}
pub struct r#BakedPotato;
impl Item for r#BakedPotato {
    const ID: i32 = 1025i32;
    const NAME: &'static str = "baked_potato";
    const DISPLAY_NAME: &'static str = "Baked Potato";
}
pub struct r#PoisonousPotato;
impl Item for r#PoisonousPotato {
    const ID: i32 = 1026i32;
    const NAME: &'static str = "poisonous_potato";
    const DISPLAY_NAME: &'static str = "Poisonous Potato";
}
pub struct r#Map;
impl Item for r#Map {
    const ID: i32 = 1027i32;
    const NAME: &'static str = "map";
    const DISPLAY_NAME: &'static str = "Empty Map";
}
pub struct r#GoldenCarrot;
impl Item for r#GoldenCarrot {
    const ID: i32 = 1028i32;
    const NAME: &'static str = "golden_carrot";
    const DISPLAY_NAME: &'static str = "Golden Carrot";
}
pub struct r#SkeletonSkull;
impl Item for r#SkeletonSkull {
    const ID: i32 = 1029i32;
    const NAME: &'static str = "skeleton_skull";
    const DISPLAY_NAME: &'static str = "Skeleton Skull";
}
pub struct r#WitherSkeletonSkull;
impl Item for r#WitherSkeletonSkull {
    const ID: i32 = 1030i32;
    const NAME: &'static str = "wither_skeleton_skull";
    const DISPLAY_NAME: &'static str = "Wither Skeleton Skull";
}
pub struct r#PlayerHead;
impl Item for r#PlayerHead {
    const ID: i32 = 1031i32;
    const NAME: &'static str = "player_head";
    const DISPLAY_NAME: &'static str = "Player Head";
}
pub struct r#ZombieHead;
impl Item for r#ZombieHead {
    const ID: i32 = 1032i32;
    const NAME: &'static str = "zombie_head";
    const DISPLAY_NAME: &'static str = "Zombie Head";
}
pub struct r#CreeperHead;
impl Item for r#CreeperHead {
    const ID: i32 = 1033i32;
    const NAME: &'static str = "creeper_head";
    const DISPLAY_NAME: &'static str = "Creeper Head";
}
pub struct r#DragonHead;
impl Item for r#DragonHead {
    const ID: i32 = 1034i32;
    const NAME: &'static str = "dragon_head";
    const DISPLAY_NAME: &'static str = "Dragon Head";
}
pub struct r#PiglinHead;
impl Item for r#PiglinHead {
    const ID: i32 = 1035i32;
    const NAME: &'static str = "piglin_head";
    const DISPLAY_NAME: &'static str = "Piglin Head";
}
pub struct r#NetherStar;
impl Item for r#NetherStar {
    const ID: i32 = 1036i32;
    const NAME: &'static str = "nether_star";
    const DISPLAY_NAME: &'static str = "Nether Star";
}
pub struct r#PumpkinPie;
impl Item for r#PumpkinPie {
    const ID: i32 = 1037i32;
    const NAME: &'static str = "pumpkin_pie";
    const DISPLAY_NAME: &'static str = "Pumpkin Pie";
}
pub struct r#FireworkRocket;
impl Item for r#FireworkRocket {
    const ID: i32 = 1038i32;
    const NAME: &'static str = "firework_rocket";
    const DISPLAY_NAME: &'static str = "Firework Rocket";
}
pub struct r#FireworkStar;
impl Item for r#FireworkStar {
    const ID: i32 = 1039i32;
    const NAME: &'static str = "firework_star";
    const DISPLAY_NAME: &'static str = "Firework Star";
}
pub struct r#EnchantedBook;
impl Item for r#EnchantedBook {
    const ID: i32 = 1040i32;
    const NAME: &'static str = "enchanted_book";
    const DISPLAY_NAME: &'static str = "Enchanted Book";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#NetherBrick;
impl Item for r#NetherBrick {
    const ID: i32 = 1041i32;
    const NAME: &'static str = "nether_brick";
    const DISPLAY_NAME: &'static str = "Nether Brick";
}
pub struct r#PrismarineShard;
impl Item for r#PrismarineShard {
    const ID: i32 = 1042i32;
    const NAME: &'static str = "prismarine_shard";
    const DISPLAY_NAME: &'static str = "Prismarine Shard";
}
pub struct r#PrismarineCrystals;
impl Item for r#PrismarineCrystals {
    const ID: i32 = 1043i32;
    const NAME: &'static str = "prismarine_crystals";
    const DISPLAY_NAME: &'static str = "Prismarine Crystals";
}
pub struct r#Rabbit;
impl Item for r#Rabbit {
    const ID: i32 = 1044i32;
    const NAME: &'static str = "rabbit";
    const DISPLAY_NAME: &'static str = "Raw Rabbit";
}
pub struct r#CookedRabbit;
impl Item for r#CookedRabbit {
    const ID: i32 = 1045i32;
    const NAME: &'static str = "cooked_rabbit";
    const DISPLAY_NAME: &'static str = "Cooked Rabbit";
}
pub struct r#RabbitStew;
impl Item for r#RabbitStew {
    const ID: i32 = 1046i32;
    const NAME: &'static str = "rabbit_stew";
    const DISPLAY_NAME: &'static str = "Rabbit Stew";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#RabbitFoot;
impl Item for r#RabbitFoot {
    const ID: i32 = 1047i32;
    const NAME: &'static str = "rabbit_foot";
    const DISPLAY_NAME: &'static str = "Rabbit's Foot";
}
pub struct r#RabbitHide;
impl Item for r#RabbitHide {
    const ID: i32 = 1048i32;
    const NAME: &'static str = "rabbit_hide";
    const DISPLAY_NAME: &'static str = "Rabbit Hide";
}
pub struct r#ArmorStand;
impl Item for r#ArmorStand {
    const ID: i32 = 1049i32;
    const NAME: &'static str = "armor_stand";
    const DISPLAY_NAME: &'static str = "Armor Stand";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#IronHorseArmor;
impl Item for r#IronHorseArmor {
    const ID: i32 = 1050i32;
    const NAME: &'static str = "iron_horse_armor";
    const DISPLAY_NAME: &'static str = "Iron Horse Armor";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GoldenHorseArmor;
impl Item for r#GoldenHorseArmor {
    const ID: i32 = 1051i32;
    const NAME: &'static str = "golden_horse_armor";
    const DISPLAY_NAME: &'static str = "Golden Horse Armor";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DiamondHorseArmor;
impl Item for r#DiamondHorseArmor {
    const ID: i32 = 1052i32;
    const NAME: &'static str = "diamond_horse_armor";
    const DISPLAY_NAME: &'static str = "Diamond Horse Armor";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#LeatherHorseArmor;
impl Item for r#LeatherHorseArmor {
    const ID: i32 = 1053i32;
    const NAME: &'static str = "leather_horse_armor";
    const DISPLAY_NAME: &'static str = "Leather Horse Armor";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Lead;
impl Item for r#Lead {
    const ID: i32 = 1054i32;
    const NAME: &'static str = "lead";
    const DISPLAY_NAME: &'static str = "Lead";
}
pub struct r#NameTag;
impl Item for r#NameTag {
    const ID: i32 = 1055i32;
    const NAME: &'static str = "name_tag";
    const DISPLAY_NAME: &'static str = "Name Tag";
}
pub struct r#CommandBlockMinecart;
impl Item for r#CommandBlockMinecart {
    const ID: i32 = 1056i32;
    const NAME: &'static str = "command_block_minecart";
    const DISPLAY_NAME: &'static str = "Minecart with Command Block";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Mutton;
impl Item for r#Mutton {
    const ID: i32 = 1057i32;
    const NAME: &'static str = "mutton";
    const DISPLAY_NAME: &'static str = "Raw Mutton";
}
pub struct r#CookedMutton;
impl Item for r#CookedMutton {
    const ID: i32 = 1058i32;
    const NAME: &'static str = "cooked_mutton";
    const DISPLAY_NAME: &'static str = "Cooked Mutton";
}
pub struct r#WhiteBanner;
impl Item for r#WhiteBanner {
    const ID: i32 = 1059i32;
    const NAME: &'static str = "white_banner";
    const DISPLAY_NAME: &'static str = "White Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#OrangeBanner;
impl Item for r#OrangeBanner {
    const ID: i32 = 1060i32;
    const NAME: &'static str = "orange_banner";
    const DISPLAY_NAME: &'static str = "Orange Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#MagentaBanner;
impl Item for r#MagentaBanner {
    const ID: i32 = 1061i32;
    const NAME: &'static str = "magenta_banner";
    const DISPLAY_NAME: &'static str = "Magenta Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#LightBlueBanner;
impl Item for r#LightBlueBanner {
    const ID: i32 = 1062i32;
    const NAME: &'static str = "light_blue_banner";
    const DISPLAY_NAME: &'static str = "Light Blue Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#YellowBanner;
impl Item for r#YellowBanner {
    const ID: i32 = 1063i32;
    const NAME: &'static str = "yellow_banner";
    const DISPLAY_NAME: &'static str = "Yellow Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#LimeBanner;
impl Item for r#LimeBanner {
    const ID: i32 = 1064i32;
    const NAME: &'static str = "lime_banner";
    const DISPLAY_NAME: &'static str = "Lime Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#PinkBanner;
impl Item for r#PinkBanner {
    const ID: i32 = 1065i32;
    const NAME: &'static str = "pink_banner";
    const DISPLAY_NAME: &'static str = "Pink Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#GrayBanner;
impl Item for r#GrayBanner {
    const ID: i32 = 1066i32;
    const NAME: &'static str = "gray_banner";
    const DISPLAY_NAME: &'static str = "Gray Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#LightGrayBanner;
impl Item for r#LightGrayBanner {
    const ID: i32 = 1067i32;
    const NAME: &'static str = "light_gray_banner";
    const DISPLAY_NAME: &'static str = "Light Gray Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#CyanBanner;
impl Item for r#CyanBanner {
    const ID: i32 = 1068i32;
    const NAME: &'static str = "cyan_banner";
    const DISPLAY_NAME: &'static str = "Cyan Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#PurpleBanner;
impl Item for r#PurpleBanner {
    const ID: i32 = 1069i32;
    const NAME: &'static str = "purple_banner";
    const DISPLAY_NAME: &'static str = "Purple Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#BlueBanner;
impl Item for r#BlueBanner {
    const ID: i32 = 1070i32;
    const NAME: &'static str = "blue_banner";
    const DISPLAY_NAME: &'static str = "Blue Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#BrownBanner;
impl Item for r#BrownBanner {
    const ID: i32 = 1071i32;
    const NAME: &'static str = "brown_banner";
    const DISPLAY_NAME: &'static str = "Brown Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#GreenBanner;
impl Item for r#GreenBanner {
    const ID: i32 = 1072i32;
    const NAME: &'static str = "green_banner";
    const DISPLAY_NAME: &'static str = "Green Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#RedBanner;
impl Item for r#RedBanner {
    const ID: i32 = 1073i32;
    const NAME: &'static str = "red_banner";
    const DISPLAY_NAME: &'static str = "Red Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#BlackBanner;
impl Item for r#BlackBanner {
    const ID: i32 = 1074i32;
    const NAME: &'static str = "black_banner";
    const DISPLAY_NAME: &'static str = "Black Banner";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#EndCrystal;
impl Item for r#EndCrystal {
    const ID: i32 = 1075i32;
    const NAME: &'static str = "end_crystal";
    const DISPLAY_NAME: &'static str = "End Crystal";
}
pub struct r#ChorusFruit;
impl Item for r#ChorusFruit {
    const ID: i32 = 1076i32;
    const NAME: &'static str = "chorus_fruit";
    const DISPLAY_NAME: &'static str = "Chorus Fruit";
}
pub struct r#PoppedChorusFruit;
impl Item for r#PoppedChorusFruit {
    const ID: i32 = 1077i32;
    const NAME: &'static str = "popped_chorus_fruit";
    const DISPLAY_NAME: &'static str = "Popped Chorus Fruit";
}
pub struct r#Beetroot;
impl Item for r#Beetroot {
    const ID: i32 = 1078i32;
    const NAME: &'static str = "beetroot";
    const DISPLAY_NAME: &'static str = "Beetroot";
}
pub struct r#BeetrootSeeds;
impl Item for r#BeetrootSeeds {
    const ID: i32 = 1079i32;
    const NAME: &'static str = "beetroot_seeds";
    const DISPLAY_NAME: &'static str = "Beetroot Seeds";
}
pub struct r#BeetrootSoup;
impl Item for r#BeetrootSoup {
    const ID: i32 = 1080i32;
    const NAME: &'static str = "beetroot_soup";
    const DISPLAY_NAME: &'static str = "Beetroot Soup";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DragonBreath;
impl Item for r#DragonBreath {
    const ID: i32 = 1081i32;
    const NAME: &'static str = "dragon_breath";
    const DISPLAY_NAME: &'static str = "Dragon's Breath";
}
pub struct r#SplashPotion;
impl Item for r#SplashPotion {
    const ID: i32 = 1082i32;
    const NAME: &'static str = "splash_potion";
    const DISPLAY_NAME: &'static str = "Splash Potion";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#SpectralArrow;
impl Item for r#SpectralArrow {
    const ID: i32 = 1083i32;
    const NAME: &'static str = "spectral_arrow";
    const DISPLAY_NAME: &'static str = "Spectral Arrow";
}
pub struct r#TippedArrow;
impl Item for r#TippedArrow {
    const ID: i32 = 1084i32;
    const NAME: &'static str = "tipped_arrow";
    const DISPLAY_NAME: &'static str = "Tipped Arrow";
}
pub struct r#LingeringPotion;
impl Item for r#LingeringPotion {
    const ID: i32 = 1085i32;
    const NAME: &'static str = "lingering_potion";
    const DISPLAY_NAME: &'static str = "Lingering Potion";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Shield;
impl Item for r#Shield {
    const ID: i32 = 1086i32;
    const NAME: &'static str = "shield";
    const DISPLAY_NAME: &'static str = "Shield";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#TotemOfUndying;
impl Item for r#TotemOfUndying {
    const ID: i32 = 1087i32;
    const NAME: &'static str = "totem_of_undying";
    const DISPLAY_NAME: &'static str = "Totem of Undying";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#ShulkerShell;
impl Item for r#ShulkerShell {
    const ID: i32 = 1088i32;
    const NAME: &'static str = "shulker_shell";
    const DISPLAY_NAME: &'static str = "Shulker Shell";
}
pub struct r#IronNugget;
impl Item for r#IronNugget {
    const ID: i32 = 1089i32;
    const NAME: &'static str = "iron_nugget";
    const DISPLAY_NAME: &'static str = "Iron Nugget";
}
pub struct r#KnowledgeBook;
impl Item for r#KnowledgeBook {
    const ID: i32 = 1090i32;
    const NAME: &'static str = "knowledge_book";
    const DISPLAY_NAME: &'static str = "Knowledge Book";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DebugStick;
impl Item for r#DebugStick {
    const ID: i32 = 1091i32;
    const NAME: &'static str = "debug_stick";
    const DISPLAY_NAME: &'static str = "Debug Stick";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDisc13;
impl Item for r#MusicDisc13 {
    const ID: i32 = 1092i32;
    const NAME: &'static str = "music_disc_13";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDiscCat;
impl Item for r#MusicDiscCat {
    const ID: i32 = 1093i32;
    const NAME: &'static str = "music_disc_cat";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDiscBlocks;
impl Item for r#MusicDiscBlocks {
    const ID: i32 = 1094i32;
    const NAME: &'static str = "music_disc_blocks";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDiscChirp;
impl Item for r#MusicDiscChirp {
    const ID: i32 = 1095i32;
    const NAME: &'static str = "music_disc_chirp";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDiscFar;
impl Item for r#MusicDiscFar {
    const ID: i32 = 1096i32;
    const NAME: &'static str = "music_disc_far";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDiscMall;
impl Item for r#MusicDiscMall {
    const ID: i32 = 1097i32;
    const NAME: &'static str = "music_disc_mall";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDiscMellohi;
impl Item for r#MusicDiscMellohi {
    const ID: i32 = 1098i32;
    const NAME: &'static str = "music_disc_mellohi";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDiscStal;
impl Item for r#MusicDiscStal {
    const ID: i32 = 1099i32;
    const NAME: &'static str = "music_disc_stal";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDiscStrad;
impl Item for r#MusicDiscStrad {
    const ID: i32 = 1100i32;
    const NAME: &'static str = "music_disc_strad";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDiscWard;
impl Item for r#MusicDiscWard {
    const ID: i32 = 1101i32;
    const NAME: &'static str = "music_disc_ward";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDisc11;
impl Item for r#MusicDisc11 {
    const ID: i32 = 1102i32;
    const NAME: &'static str = "music_disc_11";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDiscWait;
impl Item for r#MusicDiscWait {
    const ID: i32 = 1103i32;
    const NAME: &'static str = "music_disc_wait";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDiscOtherside;
impl Item for r#MusicDiscOtherside {
    const ID: i32 = 1104i32;
    const NAME: &'static str = "music_disc_otherside";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDisc5;
impl Item for r#MusicDisc5 {
    const ID: i32 = 1105i32;
    const NAME: &'static str = "music_disc_5";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MusicDiscPigstep;
impl Item for r#MusicDiscPigstep {
    const ID: i32 = 1106i32;
    const NAME: &'static str = "music_disc_pigstep";
    const DISPLAY_NAME: &'static str = "Music Disc";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#DiscFragment5;
impl Item for r#DiscFragment5 {
    const ID: i32 = 1107i32;
    const NAME: &'static str = "disc_fragment_5";
    const DISPLAY_NAME: &'static str = "Disc Fragment";
}
pub struct r#Trident;
impl Item for r#Trident {
    const ID: i32 = 1108i32;
    const NAME: &'static str = "trident";
    const DISPLAY_NAME: &'static str = "Trident";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#PhantomMembrane;
impl Item for r#PhantomMembrane {
    const ID: i32 = 1109i32;
    const NAME: &'static str = "phantom_membrane";
    const DISPLAY_NAME: &'static str = "Phantom Membrane";
}
pub struct r#NautilusShell;
impl Item for r#NautilusShell {
    const ID: i32 = 1110i32;
    const NAME: &'static str = "nautilus_shell";
    const DISPLAY_NAME: &'static str = "Nautilus Shell";
}
pub struct r#HeartOfTheSea;
impl Item for r#HeartOfTheSea {
    const ID: i32 = 1111i32;
    const NAME: &'static str = "heart_of_the_sea";
    const DISPLAY_NAME: &'static str = "Heart of the Sea";
}
pub struct r#Crossbow;
impl Item for r#Crossbow {
    const ID: i32 = 1112i32;
    const NAME: &'static str = "crossbow";
    const DISPLAY_NAME: &'static str = "Crossbow";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#SuspiciousStew;
impl Item for r#SuspiciousStew {
    const ID: i32 = 1113i32;
    const NAME: &'static str = "suspicious_stew";
    const DISPLAY_NAME: &'static str = "Suspicious Stew";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Loom;
impl Item for r#Loom {
    const ID: i32 = 1114i32;
    const NAME: &'static str = "loom";
    const DISPLAY_NAME: &'static str = "Loom";
}
pub struct r#FlowerBannerPattern;
impl Item for r#FlowerBannerPattern {
    const ID: i32 = 1115i32;
    const NAME: &'static str = "flower_banner_pattern";
    const DISPLAY_NAME: &'static str = "Banner Pattern";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#CreeperBannerPattern;
impl Item for r#CreeperBannerPattern {
    const ID: i32 = 1116i32;
    const NAME: &'static str = "creeper_banner_pattern";
    const DISPLAY_NAME: &'static str = "Banner Pattern";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#SkullBannerPattern;
impl Item for r#SkullBannerPattern {
    const ID: i32 = 1117i32;
    const NAME: &'static str = "skull_banner_pattern";
    const DISPLAY_NAME: &'static str = "Banner Pattern";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#MojangBannerPattern;
impl Item for r#MojangBannerPattern {
    const ID: i32 = 1118i32;
    const NAME: &'static str = "mojang_banner_pattern";
    const DISPLAY_NAME: &'static str = "Banner Pattern";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GlobeBannerPattern;
impl Item for r#GlobeBannerPattern {
    const ID: i32 = 1119i32;
    const NAME: &'static str = "globe_banner_pattern";
    const DISPLAY_NAME: &'static str = "Banner Pattern";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#PiglinBannerPattern;
impl Item for r#PiglinBannerPattern {
    const ID: i32 = 1120i32;
    const NAME: &'static str = "piglin_banner_pattern";
    const DISPLAY_NAME: &'static str = "Banner Pattern";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#GoatHorn;
impl Item for r#GoatHorn {
    const ID: i32 = 1121i32;
    const NAME: &'static str = "goat_horn";
    const DISPLAY_NAME: &'static str = "Goat Horn";
    const STACK_SIZE: i32 = 1i32;
}
pub struct r#Composter;
impl Item for r#Composter {
    const ID: i32 = 1122i32;
    const NAME: &'static str = "composter";
    const DISPLAY_NAME: &'static str = "Composter";
}
pub struct r#Barrel;
impl Item for r#Barrel {
    const ID: i32 = 1123i32;
    const NAME: &'static str = "barrel";
    const DISPLAY_NAME: &'static str = "Barrel";
}
pub struct r#Smoker;
impl Item for r#Smoker {
    const ID: i32 = 1124i32;
    const NAME: &'static str = "smoker";
    const DISPLAY_NAME: &'static str = "Smoker";
}
pub struct r#BlastFurnace;
impl Item for r#BlastFurnace {
    const ID: i32 = 1125i32;
    const NAME: &'static str = "blast_furnace";
    const DISPLAY_NAME: &'static str = "Blast Furnace";
}
pub struct r#CartographyTable;
impl Item for r#CartographyTable {
    const ID: i32 = 1126i32;
    const NAME: &'static str = "cartography_table";
    const DISPLAY_NAME: &'static str = "Cartography Table";
}
pub struct r#FletchingTable;
impl Item for r#FletchingTable {
    const ID: i32 = 1127i32;
    const NAME: &'static str = "fletching_table";
    const DISPLAY_NAME: &'static str = "Fletching Table";
}
pub struct r#Grindstone;
impl Item for r#Grindstone {
    const ID: i32 = 1128i32;
    const NAME: &'static str = "grindstone";
    const DISPLAY_NAME: &'static str = "Grindstone";
}
pub struct r#SmithingTable;
impl Item for r#SmithingTable {
    const ID: i32 = 1129i32;
    const NAME: &'static str = "smithing_table";
    const DISPLAY_NAME: &'static str = "Smithing Table";
}
pub struct r#Stonecutter;
impl Item for r#Stonecutter {
    const ID: i32 = 1130i32;
    const NAME: &'static str = "stonecutter";
    const DISPLAY_NAME: &'static str = "Stonecutter";
}
pub struct r#Bell;
impl Item for r#Bell {
    const ID: i32 = 1131i32;
    const NAME: &'static str = "bell";
    const DISPLAY_NAME: &'static str = "Bell";
}
pub struct r#Lantern;
impl Item for r#Lantern {
    const ID: i32 = 1132i32;
    const NAME: &'static str = "lantern";
    const DISPLAY_NAME: &'static str = "Lantern";
}
pub struct r#SoulLantern;
impl Item for r#SoulLantern {
    const ID: i32 = 1133i32;
    const NAME: &'static str = "soul_lantern";
    const DISPLAY_NAME: &'static str = "Soul Lantern";
}
pub struct r#SweetBerries;
impl Item for r#SweetBerries {
    const ID: i32 = 1134i32;
    const NAME: &'static str = "sweet_berries";
    const DISPLAY_NAME: &'static str = "Sweet Berries";
}
pub struct r#GlowBerries;
impl Item for r#GlowBerries {
    const ID: i32 = 1135i32;
    const NAME: &'static str = "glow_berries";
    const DISPLAY_NAME: &'static str = "Glow Berries";
}
pub struct r#Campfire;
impl Item for r#Campfire {
    const ID: i32 = 1136i32;
    const NAME: &'static str = "campfire";
    const DISPLAY_NAME: &'static str = "Campfire";
}
pub struct r#SoulCampfire;
impl Item for r#SoulCampfire {
    const ID: i32 = 1137i32;
    const NAME: &'static str = "soul_campfire";
    const DISPLAY_NAME: &'static str = "Soul Campfire";
}
pub struct r#Shroomlight;
impl Item for r#Shroomlight {
    const ID: i32 = 1138i32;
    const NAME: &'static str = "shroomlight";
    const DISPLAY_NAME: &'static str = "Shroomlight";
}
pub struct r#Honeycomb;
impl Item for r#Honeycomb {
    const ID: i32 = 1139i32;
    const NAME: &'static str = "honeycomb";
    const DISPLAY_NAME: &'static str = "Honeycomb";
}
pub struct r#BeeNest;
impl Item for r#BeeNest {
    const ID: i32 = 1140i32;
    const NAME: &'static str = "bee_nest";
    const DISPLAY_NAME: &'static str = "Bee Nest";
}
pub struct r#Beehive;
impl Item for r#Beehive {
    const ID: i32 = 1141i32;
    const NAME: &'static str = "beehive";
    const DISPLAY_NAME: &'static str = "Beehive";
}
pub struct r#HoneyBottle;
impl Item for r#HoneyBottle {
    const ID: i32 = 1142i32;
    const NAME: &'static str = "honey_bottle";
    const DISPLAY_NAME: &'static str = "Honey Bottle";
    const STACK_SIZE: i32 = 16i32;
}
pub struct r#HoneycombBlock;
impl Item for r#HoneycombBlock {
    const ID: i32 = 1143i32;
    const NAME: &'static str = "honeycomb_block";
    const DISPLAY_NAME: &'static str = "Honeycomb Block";
}
pub struct r#Lodestone;
impl Item for r#Lodestone {
    const ID: i32 = 1144i32;
    const NAME: &'static str = "lodestone";
    const DISPLAY_NAME: &'static str = "Lodestone";
}
pub struct r#CryingObsidian;
impl Item for r#CryingObsidian {
    const ID: i32 = 1145i32;
    const NAME: &'static str = "crying_obsidian";
    const DISPLAY_NAME: &'static str = "Crying Obsidian";
}
pub struct r#Blackstone;
impl Item for r#Blackstone {
    const ID: i32 = 1146i32;
    const NAME: &'static str = "blackstone";
    const DISPLAY_NAME: &'static str = "Blackstone";
}
pub struct r#BlackstoneSlab;
impl Item for r#BlackstoneSlab {
    const ID: i32 = 1147i32;
    const NAME: &'static str = "blackstone_slab";
    const DISPLAY_NAME: &'static str = "Blackstone Slab";
}
pub struct r#BlackstoneStairs;
impl Item for r#BlackstoneStairs {
    const ID: i32 = 1148i32;
    const NAME: &'static str = "blackstone_stairs";
    const DISPLAY_NAME: &'static str = "Blackstone Stairs";
}
pub struct r#GildedBlackstone;
impl Item for r#GildedBlackstone {
    const ID: i32 = 1149i32;
    const NAME: &'static str = "gilded_blackstone";
    const DISPLAY_NAME: &'static str = "Gilded Blackstone";
}
pub struct r#PolishedBlackstone;
impl Item for r#PolishedBlackstone {
    const ID: i32 = 1150i32;
    const NAME: &'static str = "polished_blackstone";
    const DISPLAY_NAME: &'static str = "Polished Blackstone";
}
pub struct r#PolishedBlackstoneSlab;
impl Item for r#PolishedBlackstoneSlab {
    const ID: i32 = 1151i32;
    const NAME: &'static str = "polished_blackstone_slab";
    const DISPLAY_NAME: &'static str = "Polished Blackstone Slab";
}
pub struct r#PolishedBlackstoneStairs;
impl Item for r#PolishedBlackstoneStairs {
    const ID: i32 = 1152i32;
    const NAME: &'static str = "polished_blackstone_stairs";
    const DISPLAY_NAME: &'static str = "Polished Blackstone Stairs";
}
pub struct r#ChiseledPolishedBlackstone;
impl Item for r#ChiseledPolishedBlackstone {
    const ID: i32 = 1153i32;
    const NAME: &'static str = "chiseled_polished_blackstone";
    const DISPLAY_NAME: &'static str = "Chiseled Polished Blackstone";
}
pub struct r#PolishedBlackstoneBricks;
impl Item for r#PolishedBlackstoneBricks {
    const ID: i32 = 1154i32;
    const NAME: &'static str = "polished_blackstone_bricks";
    const DISPLAY_NAME: &'static str = "Polished Blackstone Bricks";
}
pub struct r#PolishedBlackstoneBrickSlab;
impl Item for r#PolishedBlackstoneBrickSlab {
    const ID: i32 = 1155i32;
    const NAME: &'static str = "polished_blackstone_brick_slab";
    const DISPLAY_NAME: &'static str = "Polished Blackstone Brick Slab";
}
pub struct r#PolishedBlackstoneBrickStairs;
impl Item for r#PolishedBlackstoneBrickStairs {
    const ID: i32 = 1156i32;
    const NAME: &'static str = "polished_blackstone_brick_stairs";
    const DISPLAY_NAME: &'static str = "Polished Blackstone Brick Stairs";
}
pub struct r#CrackedPolishedBlackstoneBricks;
impl Item for r#CrackedPolishedBlackstoneBricks {
    const ID: i32 = 1157i32;
    const NAME: &'static str = "cracked_polished_blackstone_bricks";
    const DISPLAY_NAME: &'static str = "Cracked Polished Blackstone Bricks";
}
pub struct r#RespawnAnchor;
impl Item for r#RespawnAnchor {
    const ID: i32 = 1158i32;
    const NAME: &'static str = "respawn_anchor";
    const DISPLAY_NAME: &'static str = "Respawn Anchor";
}
pub struct r#Candle;
impl Item for r#Candle {
    const ID: i32 = 1159i32;
    const NAME: &'static str = "candle";
    const DISPLAY_NAME: &'static str = "Candle";
}
pub struct r#WhiteCandle;
impl Item for r#WhiteCandle {
    const ID: i32 = 1160i32;
    const NAME: &'static str = "white_candle";
    const DISPLAY_NAME: &'static str = "White Candle";
}
pub struct r#OrangeCandle;
impl Item for r#OrangeCandle {
    const ID: i32 = 1161i32;
    const NAME: &'static str = "orange_candle";
    const DISPLAY_NAME: &'static str = "Orange Candle";
}
pub struct r#MagentaCandle;
impl Item for r#MagentaCandle {
    const ID: i32 = 1162i32;
    const NAME: &'static str = "magenta_candle";
    const DISPLAY_NAME: &'static str = "Magenta Candle";
}
pub struct r#LightBlueCandle;
impl Item for r#LightBlueCandle {
    const ID: i32 = 1163i32;
    const NAME: &'static str = "light_blue_candle";
    const DISPLAY_NAME: &'static str = "Light Blue Candle";
}
pub struct r#YellowCandle;
impl Item for r#YellowCandle {
    const ID: i32 = 1164i32;
    const NAME: &'static str = "yellow_candle";
    const DISPLAY_NAME: &'static str = "Yellow Candle";
}
pub struct r#LimeCandle;
impl Item for r#LimeCandle {
    const ID: i32 = 1165i32;
    const NAME: &'static str = "lime_candle";
    const DISPLAY_NAME: &'static str = "Lime Candle";
}
pub struct r#PinkCandle;
impl Item for r#PinkCandle {
    const ID: i32 = 1166i32;
    const NAME: &'static str = "pink_candle";
    const DISPLAY_NAME: &'static str = "Pink Candle";
}
pub struct r#GrayCandle;
impl Item for r#GrayCandle {
    const ID: i32 = 1167i32;
    const NAME: &'static str = "gray_candle";
    const DISPLAY_NAME: &'static str = "Gray Candle";
}
pub struct r#LightGrayCandle;
impl Item for r#LightGrayCandle {
    const ID: i32 = 1168i32;
    const NAME: &'static str = "light_gray_candle";
    const DISPLAY_NAME: &'static str = "Light Gray Candle";
}
pub struct r#CyanCandle;
impl Item for r#CyanCandle {
    const ID: i32 = 1169i32;
    const NAME: &'static str = "cyan_candle";
    const DISPLAY_NAME: &'static str = "Cyan Candle";
}
pub struct r#PurpleCandle;
impl Item for r#PurpleCandle {
    const ID: i32 = 1170i32;
    const NAME: &'static str = "purple_candle";
    const DISPLAY_NAME: &'static str = "Purple Candle";
}
pub struct r#BlueCandle;
impl Item for r#BlueCandle {
    const ID: i32 = 1171i32;
    const NAME: &'static str = "blue_candle";
    const DISPLAY_NAME: &'static str = "Blue Candle";
}
pub struct r#BrownCandle;
impl Item for r#BrownCandle {
    const ID: i32 = 1172i32;
    const NAME: &'static str = "brown_candle";
    const DISPLAY_NAME: &'static str = "Brown Candle";
}
pub struct r#GreenCandle;
impl Item for r#GreenCandle {
    const ID: i32 = 1173i32;
    const NAME: &'static str = "green_candle";
    const DISPLAY_NAME: &'static str = "Green Candle";
}
pub struct r#RedCandle;
impl Item for r#RedCandle {
    const ID: i32 = 1174i32;
    const NAME: &'static str = "red_candle";
    const DISPLAY_NAME: &'static str = "Red Candle";
}
pub struct r#BlackCandle;
impl Item for r#BlackCandle {
    const ID: i32 = 1175i32;
    const NAME: &'static str = "black_candle";
    const DISPLAY_NAME: &'static str = "Black Candle";
}
pub struct r#SmallAmethystBud;
impl Item for r#SmallAmethystBud {
    const ID: i32 = 1176i32;
    const NAME: &'static str = "small_amethyst_bud";
    const DISPLAY_NAME: &'static str = "Small Amethyst Bud";
}
pub struct r#MediumAmethystBud;
impl Item for r#MediumAmethystBud {
    const ID: i32 = 1177i32;
    const NAME: &'static str = "medium_amethyst_bud";
    const DISPLAY_NAME: &'static str = "Medium Amethyst Bud";
}
pub struct r#LargeAmethystBud;
impl Item for r#LargeAmethystBud {
    const ID: i32 = 1178i32;
    const NAME: &'static str = "large_amethyst_bud";
    const DISPLAY_NAME: &'static str = "Large Amethyst Bud";
}
pub struct r#AmethystCluster;
impl Item for r#AmethystCluster {
    const ID: i32 = 1179i32;
    const NAME: &'static str = "amethyst_cluster";
    const DISPLAY_NAME: &'static str = "Amethyst Cluster";
}
pub struct r#PointedDripstone;
impl Item for r#PointedDripstone {
    const ID: i32 = 1180i32;
    const NAME: &'static str = "pointed_dripstone";
    const DISPLAY_NAME: &'static str = "Pointed Dripstone";
}
pub struct r#OchreFroglight;
impl Item for r#OchreFroglight {
    const ID: i32 = 1181i32;
    const NAME: &'static str = "ochre_froglight";
    const DISPLAY_NAME: &'static str = "Ochre Froglight";
}
pub struct r#VerdantFroglight;
impl Item for r#VerdantFroglight {
    const ID: i32 = 1182i32;
    const NAME: &'static str = "verdant_froglight";
    const DISPLAY_NAME: &'static str = "Verdant Froglight";
}
pub struct r#PearlescentFroglight;
impl Item for r#PearlescentFroglight {
    const ID: i32 = 1183i32;
    const NAME: &'static str = "pearlescent_froglight";
    const DISPLAY_NAME: &'static str = "Pearlescent Froglight";
}
pub struct r#Frogspawn;
impl Item for r#Frogspawn {
    const ID: i32 = 1184i32;
    const NAME: &'static str = "frogspawn";
    const DISPLAY_NAME: &'static str = "Frogspawn";
}
pub struct r#EchoShard;
impl Item for r#EchoShard {
    const ID: i32 = 1185i32;
    const NAME: &'static str = "echo_shard";
    const DISPLAY_NAME: &'static str = "Echo Shard";
}
pub enum Items {
    Air(r#Air),
    Stone(r#Stone),
    Granite(r#Granite),
    PolishedGranite(r#PolishedGranite),
    Diorite(r#Diorite),
    PolishedDiorite(r#PolishedDiorite),
    Andesite(r#Andesite),
    PolishedAndesite(r#PolishedAndesite),
    Deepslate(r#Deepslate),
    CobbledDeepslate(r#CobbledDeepslate),
    PolishedDeepslate(r#PolishedDeepslate),
    Calcite(r#Calcite),
    Tuff(r#Tuff),
    DripstoneBlock(r#DripstoneBlock),
    GrassBlock(r#GrassBlock),
    Dirt(r#Dirt),
    CoarseDirt(r#CoarseDirt),
    Podzol(r#Podzol),
    RootedDirt(r#RootedDirt),
    Mud(r#Mud),
    CrimsonNylium(r#CrimsonNylium),
    WarpedNylium(r#WarpedNylium),
    Cobblestone(r#Cobblestone),
    OakPlanks(r#OakPlanks),
    SprucePlanks(r#SprucePlanks),
    BirchPlanks(r#BirchPlanks),
    JunglePlanks(r#JunglePlanks),
    AcaciaPlanks(r#AcaciaPlanks),
    DarkOakPlanks(r#DarkOakPlanks),
    MangrovePlanks(r#MangrovePlanks),
    BambooPlanks(r#BambooPlanks),
    CrimsonPlanks(r#CrimsonPlanks),
    WarpedPlanks(r#WarpedPlanks),
    BambooMosaic(r#BambooMosaic),
    OakSapling(r#OakSapling),
    SpruceSapling(r#SpruceSapling),
    BirchSapling(r#BirchSapling),
    JungleSapling(r#JungleSapling),
    AcaciaSapling(r#AcaciaSapling),
    DarkOakSapling(r#DarkOakSapling),
    MangrovePropagule(r#MangrovePropagule),
    Bedrock(r#Bedrock),
    Sand(r#Sand),
    RedSand(r#RedSand),
    Gravel(r#Gravel),
    CoalOre(r#CoalOre),
    DeepslateCoalOre(r#DeepslateCoalOre),
    IronOre(r#IronOre),
    DeepslateIronOre(r#DeepslateIronOre),
    CopperOre(r#CopperOre),
    DeepslateCopperOre(r#DeepslateCopperOre),
    GoldOre(r#GoldOre),
    DeepslateGoldOre(r#DeepslateGoldOre),
    RedstoneOre(r#RedstoneOre),
    DeepslateRedstoneOre(r#DeepslateRedstoneOre),
    EmeraldOre(r#EmeraldOre),
    DeepslateEmeraldOre(r#DeepslateEmeraldOre),
    LapisOre(r#LapisOre),
    DeepslateLapisOre(r#DeepslateLapisOre),
    DiamondOre(r#DiamondOre),
    DeepslateDiamondOre(r#DeepslateDiamondOre),
    NetherGoldOre(r#NetherGoldOre),
    NetherQuartzOre(r#NetherQuartzOre),
    AncientDebris(r#AncientDebris),
    CoalBlock(r#CoalBlock),
    RawIronBlock(r#RawIronBlock),
    RawCopperBlock(r#RawCopperBlock),
    RawGoldBlock(r#RawGoldBlock),
    AmethystBlock(r#AmethystBlock),
    BuddingAmethyst(r#BuddingAmethyst),
    IronBlock(r#IronBlock),
    CopperBlock(r#CopperBlock),
    GoldBlock(r#GoldBlock),
    DiamondBlock(r#DiamondBlock),
    NetheriteBlock(r#NetheriteBlock),
    ExposedCopper(r#ExposedCopper),
    WeatheredCopper(r#WeatheredCopper),
    OxidizedCopper(r#OxidizedCopper),
    CutCopper(r#CutCopper),
    ExposedCutCopper(r#ExposedCutCopper),
    WeatheredCutCopper(r#WeatheredCutCopper),
    OxidizedCutCopper(r#OxidizedCutCopper),
    CutCopperStairs(r#CutCopperStairs),
    ExposedCutCopperStairs(r#ExposedCutCopperStairs),
    WeatheredCutCopperStairs(r#WeatheredCutCopperStairs),
    OxidizedCutCopperStairs(r#OxidizedCutCopperStairs),
    CutCopperSlab(r#CutCopperSlab),
    ExposedCutCopperSlab(r#ExposedCutCopperSlab),
    WeatheredCutCopperSlab(r#WeatheredCutCopperSlab),
    OxidizedCutCopperSlab(r#OxidizedCutCopperSlab),
    WaxedCopperBlock(r#WaxedCopperBlock),
    WaxedExposedCopper(r#WaxedExposedCopper),
    WaxedWeatheredCopper(r#WaxedWeatheredCopper),
    WaxedOxidizedCopper(r#WaxedOxidizedCopper),
    WaxedCutCopper(r#WaxedCutCopper),
    WaxedExposedCutCopper(r#WaxedExposedCutCopper),
    WaxedWeatheredCutCopper(r#WaxedWeatheredCutCopper),
    WaxedOxidizedCutCopper(r#WaxedOxidizedCutCopper),
    WaxedCutCopperStairs(r#WaxedCutCopperStairs),
    WaxedExposedCutCopperStairs(r#WaxedExposedCutCopperStairs),
    WaxedWeatheredCutCopperStairs(r#WaxedWeatheredCutCopperStairs),
    WaxedOxidizedCutCopperStairs(r#WaxedOxidizedCutCopperStairs),
    WaxedCutCopperSlab(r#WaxedCutCopperSlab),
    WaxedExposedCutCopperSlab(r#WaxedExposedCutCopperSlab),
    WaxedWeatheredCutCopperSlab(r#WaxedWeatheredCutCopperSlab),
    WaxedOxidizedCutCopperSlab(r#WaxedOxidizedCutCopperSlab),
    OakLog(r#OakLog),
    SpruceLog(r#SpruceLog),
    BirchLog(r#BirchLog),
    JungleLog(r#JungleLog),
    AcaciaLog(r#AcaciaLog),
    DarkOakLog(r#DarkOakLog),
    MangroveLog(r#MangroveLog),
    MangroveRoots(r#MangroveRoots),
    MuddyMangroveRoots(r#MuddyMangroveRoots),
    CrimsonStem(r#CrimsonStem),
    WarpedStem(r#WarpedStem),
    BambooBlock(r#BambooBlock),
    StrippedOakLog(r#StrippedOakLog),
    StrippedSpruceLog(r#StrippedSpruceLog),
    StrippedBirchLog(r#StrippedBirchLog),
    StrippedJungleLog(r#StrippedJungleLog),
    StrippedAcaciaLog(r#StrippedAcaciaLog),
    StrippedDarkOakLog(r#StrippedDarkOakLog),
    StrippedMangroveLog(r#StrippedMangroveLog),
    StrippedCrimsonStem(r#StrippedCrimsonStem),
    StrippedWarpedStem(r#StrippedWarpedStem),
    StrippedOakWood(r#StrippedOakWood),
    StrippedSpruceWood(r#StrippedSpruceWood),
    StrippedBirchWood(r#StrippedBirchWood),
    StrippedJungleWood(r#StrippedJungleWood),
    StrippedAcaciaWood(r#StrippedAcaciaWood),
    StrippedDarkOakWood(r#StrippedDarkOakWood),
    StrippedMangroveWood(r#StrippedMangroveWood),
    StrippedCrimsonHyphae(r#StrippedCrimsonHyphae),
    StrippedWarpedHyphae(r#StrippedWarpedHyphae),
    StrippedBambooBlock(r#StrippedBambooBlock),
    OakWood(r#OakWood),
    SpruceWood(r#SpruceWood),
    BirchWood(r#BirchWood),
    JungleWood(r#JungleWood),
    AcaciaWood(r#AcaciaWood),
    DarkOakWood(r#DarkOakWood),
    MangroveWood(r#MangroveWood),
    CrimsonHyphae(r#CrimsonHyphae),
    WarpedHyphae(r#WarpedHyphae),
    OakLeaves(r#OakLeaves),
    SpruceLeaves(r#SpruceLeaves),
    BirchLeaves(r#BirchLeaves),
    JungleLeaves(r#JungleLeaves),
    AcaciaLeaves(r#AcaciaLeaves),
    DarkOakLeaves(r#DarkOakLeaves),
    MangroveLeaves(r#MangroveLeaves),
    AzaleaLeaves(r#AzaleaLeaves),
    FloweringAzaleaLeaves(r#FloweringAzaleaLeaves),
    Sponge(r#Sponge),
    WetSponge(r#WetSponge),
    Glass(r#Glass),
    TintedGlass(r#TintedGlass),
    LapisBlock(r#LapisBlock),
    Sandstone(r#Sandstone),
    ChiseledSandstone(r#ChiseledSandstone),
    CutSandstone(r#CutSandstone),
    Cobweb(r#Cobweb),
    Grass(r#Grass),
    Fern(r#Fern),
    Azalea(r#Azalea),
    FloweringAzalea(r#FloweringAzalea),
    DeadBush(r#DeadBush),
    Seagrass(r#Seagrass),
    SeaPickle(r#SeaPickle),
    WhiteWool(r#WhiteWool),
    OrangeWool(r#OrangeWool),
    MagentaWool(r#MagentaWool),
    LightBlueWool(r#LightBlueWool),
    YellowWool(r#YellowWool),
    LimeWool(r#LimeWool),
    PinkWool(r#PinkWool),
    GrayWool(r#GrayWool),
    LightGrayWool(r#LightGrayWool),
    CyanWool(r#CyanWool),
    PurpleWool(r#PurpleWool),
    BlueWool(r#BlueWool),
    BrownWool(r#BrownWool),
    GreenWool(r#GreenWool),
    RedWool(r#RedWool),
    BlackWool(r#BlackWool),
    Dandelion(r#Dandelion),
    Poppy(r#Poppy),
    BlueOrchid(r#BlueOrchid),
    Allium(r#Allium),
    AzureBluet(r#AzureBluet),
    RedTulip(r#RedTulip),
    OrangeTulip(r#OrangeTulip),
    WhiteTulip(r#WhiteTulip),
    PinkTulip(r#PinkTulip),
    OxeyeDaisy(r#OxeyeDaisy),
    Cornflower(r#Cornflower),
    LilyOfTheValley(r#LilyOfTheValley),
    WitherRose(r#WitherRose),
    SporeBlossom(r#SporeBlossom),
    BrownMushroom(r#BrownMushroom),
    RedMushroom(r#RedMushroom),
    CrimsonFungus(r#CrimsonFungus),
    WarpedFungus(r#WarpedFungus),
    CrimsonRoots(r#CrimsonRoots),
    WarpedRoots(r#WarpedRoots),
    NetherSprouts(r#NetherSprouts),
    WeepingVines(r#WeepingVines),
    TwistingVines(r#TwistingVines),
    SugarCane(r#SugarCane),
    Kelp(r#Kelp),
    MossCarpet(r#MossCarpet),
    MossBlock(r#MossBlock),
    HangingRoots(r#HangingRoots),
    BigDripleaf(r#BigDripleaf),
    SmallDripleaf(r#SmallDripleaf),
    Bamboo(r#Bamboo),
    OakSlab(r#OakSlab),
    SpruceSlab(r#SpruceSlab),
    BirchSlab(r#BirchSlab),
    JungleSlab(r#JungleSlab),
    AcaciaSlab(r#AcaciaSlab),
    DarkOakSlab(r#DarkOakSlab),
    MangroveSlab(r#MangroveSlab),
    BambooSlab(r#BambooSlab),
    BambooMosaicSlab(r#BambooMosaicSlab),
    CrimsonSlab(r#CrimsonSlab),
    WarpedSlab(r#WarpedSlab),
    StoneSlab(r#StoneSlab),
    SmoothStoneSlab(r#SmoothStoneSlab),
    SandstoneSlab(r#SandstoneSlab),
    CutSandstoneSlab(r#CutSandstoneSlab),
    PetrifiedOakSlab(r#PetrifiedOakSlab),
    CobblestoneSlab(r#CobblestoneSlab),
    BrickSlab(r#BrickSlab),
    StoneBrickSlab(r#StoneBrickSlab),
    MudBrickSlab(r#MudBrickSlab),
    NetherBrickSlab(r#NetherBrickSlab),
    QuartzSlab(r#QuartzSlab),
    RedSandstoneSlab(r#RedSandstoneSlab),
    CutRedSandstoneSlab(r#CutRedSandstoneSlab),
    PurpurSlab(r#PurpurSlab),
    PrismarineSlab(r#PrismarineSlab),
    PrismarineBrickSlab(r#PrismarineBrickSlab),
    DarkPrismarineSlab(r#DarkPrismarineSlab),
    SmoothQuartz(r#SmoothQuartz),
    SmoothRedSandstone(r#SmoothRedSandstone),
    SmoothSandstone(r#SmoothSandstone),
    SmoothStone(r#SmoothStone),
    Bricks(r#Bricks),
    Bookshelf(r#Bookshelf),
    ChiseledBookshelf(r#ChiseledBookshelf),
    MossyCobblestone(r#MossyCobblestone),
    Obsidian(r#Obsidian),
    Torch(r#Torch),
    EndRod(r#EndRod),
    ChorusPlant(r#ChorusPlant),
    ChorusFlower(r#ChorusFlower),
    PurpurBlock(r#PurpurBlock),
    PurpurPillar(r#PurpurPillar),
    PurpurStairs(r#PurpurStairs),
    Spawner(r#Spawner),
    Chest(r#Chest),
    CraftingTable(r#CraftingTable),
    Farmland(r#Farmland),
    Furnace(r#Furnace),
    Ladder(r#Ladder),
    CobblestoneStairs(r#CobblestoneStairs),
    Snow(r#Snow),
    Ice(r#Ice),
    SnowBlock(r#SnowBlock),
    Cactus(r#Cactus),
    Clay(r#Clay),
    Jukebox(r#Jukebox),
    OakFence(r#OakFence),
    SpruceFence(r#SpruceFence),
    BirchFence(r#BirchFence),
    JungleFence(r#JungleFence),
    AcaciaFence(r#AcaciaFence),
    DarkOakFence(r#DarkOakFence),
    MangroveFence(r#MangroveFence),
    BambooFence(r#BambooFence),
    CrimsonFence(r#CrimsonFence),
    WarpedFence(r#WarpedFence),
    Pumpkin(r#Pumpkin),
    CarvedPumpkin(r#CarvedPumpkin),
    JackOLantern(r#JackOLantern),
    Netherrack(r#Netherrack),
    SoulSand(r#SoulSand),
    SoulSoil(r#SoulSoil),
    Basalt(r#Basalt),
    PolishedBasalt(r#PolishedBasalt),
    SmoothBasalt(r#SmoothBasalt),
    SoulTorch(r#SoulTorch),
    Glowstone(r#Glowstone),
    InfestedStone(r#InfestedStone),
    InfestedCobblestone(r#InfestedCobblestone),
    InfestedStoneBricks(r#InfestedStoneBricks),
    InfestedMossyStoneBricks(r#InfestedMossyStoneBricks),
    InfestedCrackedStoneBricks(r#InfestedCrackedStoneBricks),
    InfestedChiseledStoneBricks(r#InfestedChiseledStoneBricks),
    InfestedDeepslate(r#InfestedDeepslate),
    StoneBricks(r#StoneBricks),
    MossyStoneBricks(r#MossyStoneBricks),
    CrackedStoneBricks(r#CrackedStoneBricks),
    ChiseledStoneBricks(r#ChiseledStoneBricks),
    PackedMud(r#PackedMud),
    MudBricks(r#MudBricks),
    DeepslateBricks(r#DeepslateBricks),
    CrackedDeepslateBricks(r#CrackedDeepslateBricks),
    DeepslateTiles(r#DeepslateTiles),
    CrackedDeepslateTiles(r#CrackedDeepslateTiles),
    ChiseledDeepslate(r#ChiseledDeepslate),
    ReinforcedDeepslate(r#ReinforcedDeepslate),
    BrownMushroomBlock(r#BrownMushroomBlock),
    RedMushroomBlock(r#RedMushroomBlock),
    MushroomStem(r#MushroomStem),
    IronBars(r#IronBars),
    Chain(r#Chain),
    GlassPane(r#GlassPane),
    Melon(r#Melon),
    Vine(r#Vine),
    GlowLichen(r#GlowLichen),
    BrickStairs(r#BrickStairs),
    StoneBrickStairs(r#StoneBrickStairs),
    MudBrickStairs(r#MudBrickStairs),
    Mycelium(r#Mycelium),
    LilyPad(r#LilyPad),
    NetherBricks(r#NetherBricks),
    CrackedNetherBricks(r#CrackedNetherBricks),
    ChiseledNetherBricks(r#ChiseledNetherBricks),
    NetherBrickFence(r#NetherBrickFence),
    NetherBrickStairs(r#NetherBrickStairs),
    Sculk(r#Sculk),
    SculkVein(r#SculkVein),
    SculkCatalyst(r#SculkCatalyst),
    SculkShrieker(r#SculkShrieker),
    EnchantingTable(r#EnchantingTable),
    EndPortalFrame(r#EndPortalFrame),
    EndStone(r#EndStone),
    EndStoneBricks(r#EndStoneBricks),
    DragonEgg(r#DragonEgg),
    SandstoneStairs(r#SandstoneStairs),
    EnderChest(r#EnderChest),
    EmeraldBlock(r#EmeraldBlock),
    OakStairs(r#OakStairs),
    SpruceStairs(r#SpruceStairs),
    BirchStairs(r#BirchStairs),
    JungleStairs(r#JungleStairs),
    AcaciaStairs(r#AcaciaStairs),
    DarkOakStairs(r#DarkOakStairs),
    MangroveStairs(r#MangroveStairs),
    BambooStairs(r#BambooStairs),
    BambooMosaicStairs(r#BambooMosaicStairs),
    CrimsonStairs(r#CrimsonStairs),
    WarpedStairs(r#WarpedStairs),
    CommandBlock(r#CommandBlock),
    Beacon(r#Beacon),
    CobblestoneWall(r#CobblestoneWall),
    MossyCobblestoneWall(r#MossyCobblestoneWall),
    BrickWall(r#BrickWall),
    PrismarineWall(r#PrismarineWall),
    RedSandstoneWall(r#RedSandstoneWall),
    MossyStoneBrickWall(r#MossyStoneBrickWall),
    GraniteWall(r#GraniteWall),
    StoneBrickWall(r#StoneBrickWall),
    MudBrickWall(r#MudBrickWall),
    NetherBrickWall(r#NetherBrickWall),
    AndesiteWall(r#AndesiteWall),
    RedNetherBrickWall(r#RedNetherBrickWall),
    SandstoneWall(r#SandstoneWall),
    EndStoneBrickWall(r#EndStoneBrickWall),
    DioriteWall(r#DioriteWall),
    BlackstoneWall(r#BlackstoneWall),
    PolishedBlackstoneWall(r#PolishedBlackstoneWall),
    PolishedBlackstoneBrickWall(r#PolishedBlackstoneBrickWall),
    CobbledDeepslateWall(r#CobbledDeepslateWall),
    PolishedDeepslateWall(r#PolishedDeepslateWall),
    DeepslateBrickWall(r#DeepslateBrickWall),
    DeepslateTileWall(r#DeepslateTileWall),
    Anvil(r#Anvil),
    ChippedAnvil(r#ChippedAnvil),
    DamagedAnvil(r#DamagedAnvil),
    ChiseledQuartzBlock(r#ChiseledQuartzBlock),
    QuartzBlock(r#QuartzBlock),
    QuartzBricks(r#QuartzBricks),
    QuartzPillar(r#QuartzPillar),
    QuartzStairs(r#QuartzStairs),
    WhiteTerracotta(r#WhiteTerracotta),
    OrangeTerracotta(r#OrangeTerracotta),
    MagentaTerracotta(r#MagentaTerracotta),
    LightBlueTerracotta(r#LightBlueTerracotta),
    YellowTerracotta(r#YellowTerracotta),
    LimeTerracotta(r#LimeTerracotta),
    PinkTerracotta(r#PinkTerracotta),
    GrayTerracotta(r#GrayTerracotta),
    LightGrayTerracotta(r#LightGrayTerracotta),
    CyanTerracotta(r#CyanTerracotta),
    PurpleTerracotta(r#PurpleTerracotta),
    BlueTerracotta(r#BlueTerracotta),
    BrownTerracotta(r#BrownTerracotta),
    GreenTerracotta(r#GreenTerracotta),
    RedTerracotta(r#RedTerracotta),
    BlackTerracotta(r#BlackTerracotta),
    Barrier(r#Barrier),
    Light(r#Light),
    HayBlock(r#HayBlock),
    WhiteCarpet(r#WhiteCarpet),
    OrangeCarpet(r#OrangeCarpet),
    MagentaCarpet(r#MagentaCarpet),
    LightBlueCarpet(r#LightBlueCarpet),
    YellowCarpet(r#YellowCarpet),
    LimeCarpet(r#LimeCarpet),
    PinkCarpet(r#PinkCarpet),
    GrayCarpet(r#GrayCarpet),
    LightGrayCarpet(r#LightGrayCarpet),
    CyanCarpet(r#CyanCarpet),
    PurpleCarpet(r#PurpleCarpet),
    BlueCarpet(r#BlueCarpet),
    BrownCarpet(r#BrownCarpet),
    GreenCarpet(r#GreenCarpet),
    RedCarpet(r#RedCarpet),
    BlackCarpet(r#BlackCarpet),
    Terracotta(r#Terracotta),
    PackedIce(r#PackedIce),
    DirtPath(r#DirtPath),
    Sunflower(r#Sunflower),
    Lilac(r#Lilac),
    RoseBush(r#RoseBush),
    Peony(r#Peony),
    TallGrass(r#TallGrass),
    LargeFern(r#LargeFern),
    WhiteStainedGlass(r#WhiteStainedGlass),
    OrangeStainedGlass(r#OrangeStainedGlass),
    MagentaStainedGlass(r#MagentaStainedGlass),
    LightBlueStainedGlass(r#LightBlueStainedGlass),
    YellowStainedGlass(r#YellowStainedGlass),
    LimeStainedGlass(r#LimeStainedGlass),
    PinkStainedGlass(r#PinkStainedGlass),
    GrayStainedGlass(r#GrayStainedGlass),
    LightGrayStainedGlass(r#LightGrayStainedGlass),
    CyanStainedGlass(r#CyanStainedGlass),
    PurpleStainedGlass(r#PurpleStainedGlass),
    BlueStainedGlass(r#BlueStainedGlass),
    BrownStainedGlass(r#BrownStainedGlass),
    GreenStainedGlass(r#GreenStainedGlass),
    RedStainedGlass(r#RedStainedGlass),
    BlackStainedGlass(r#BlackStainedGlass),
    WhiteStainedGlassPane(r#WhiteStainedGlassPane),
    OrangeStainedGlassPane(r#OrangeStainedGlassPane),
    MagentaStainedGlassPane(r#MagentaStainedGlassPane),
    LightBlueStainedGlassPane(r#LightBlueStainedGlassPane),
    YellowStainedGlassPane(r#YellowStainedGlassPane),
    LimeStainedGlassPane(r#LimeStainedGlassPane),
    PinkStainedGlassPane(r#PinkStainedGlassPane),
    GrayStainedGlassPane(r#GrayStainedGlassPane),
    LightGrayStainedGlassPane(r#LightGrayStainedGlassPane),
    CyanStainedGlassPane(r#CyanStainedGlassPane),
    PurpleStainedGlassPane(r#PurpleStainedGlassPane),
    BlueStainedGlassPane(r#BlueStainedGlassPane),
    BrownStainedGlassPane(r#BrownStainedGlassPane),
    GreenStainedGlassPane(r#GreenStainedGlassPane),
    RedStainedGlassPane(r#RedStainedGlassPane),
    BlackStainedGlassPane(r#BlackStainedGlassPane),
    Prismarine(r#Prismarine),
    PrismarineBricks(r#PrismarineBricks),
    DarkPrismarine(r#DarkPrismarine),
    PrismarineStairs(r#PrismarineStairs),
    PrismarineBrickStairs(r#PrismarineBrickStairs),
    DarkPrismarineStairs(r#DarkPrismarineStairs),
    SeaLantern(r#SeaLantern),
    RedSandstone(r#RedSandstone),
    ChiseledRedSandstone(r#ChiseledRedSandstone),
    CutRedSandstone(r#CutRedSandstone),
    RedSandstoneStairs(r#RedSandstoneStairs),
    RepeatingCommandBlock(r#RepeatingCommandBlock),
    ChainCommandBlock(r#ChainCommandBlock),
    MagmaBlock(r#MagmaBlock),
    NetherWartBlock(r#NetherWartBlock),
    WarpedWartBlock(r#WarpedWartBlock),
    RedNetherBricks(r#RedNetherBricks),
    BoneBlock(r#BoneBlock),
    StructureVoid(r#StructureVoid),
    ShulkerBox(r#ShulkerBox),
    WhiteShulkerBox(r#WhiteShulkerBox),
    OrangeShulkerBox(r#OrangeShulkerBox),
    MagentaShulkerBox(r#MagentaShulkerBox),
    LightBlueShulkerBox(r#LightBlueShulkerBox),
    YellowShulkerBox(r#YellowShulkerBox),
    LimeShulkerBox(r#LimeShulkerBox),
    PinkShulkerBox(r#PinkShulkerBox),
    GrayShulkerBox(r#GrayShulkerBox),
    LightGrayShulkerBox(r#LightGrayShulkerBox),
    CyanShulkerBox(r#CyanShulkerBox),
    PurpleShulkerBox(r#PurpleShulkerBox),
    BlueShulkerBox(r#BlueShulkerBox),
    BrownShulkerBox(r#BrownShulkerBox),
    GreenShulkerBox(r#GreenShulkerBox),
    RedShulkerBox(r#RedShulkerBox),
    BlackShulkerBox(r#BlackShulkerBox),
    WhiteGlazedTerracotta(r#WhiteGlazedTerracotta),
    OrangeGlazedTerracotta(r#OrangeGlazedTerracotta),
    MagentaGlazedTerracotta(r#MagentaGlazedTerracotta),
    LightBlueGlazedTerracotta(r#LightBlueGlazedTerracotta),
    YellowGlazedTerracotta(r#YellowGlazedTerracotta),
    LimeGlazedTerracotta(r#LimeGlazedTerracotta),
    PinkGlazedTerracotta(r#PinkGlazedTerracotta),
    GrayGlazedTerracotta(r#GrayGlazedTerracotta),
    LightGrayGlazedTerracotta(r#LightGrayGlazedTerracotta),
    CyanGlazedTerracotta(r#CyanGlazedTerracotta),
    PurpleGlazedTerracotta(r#PurpleGlazedTerracotta),
    BlueGlazedTerracotta(r#BlueGlazedTerracotta),
    BrownGlazedTerracotta(r#BrownGlazedTerracotta),
    GreenGlazedTerracotta(r#GreenGlazedTerracotta),
    RedGlazedTerracotta(r#RedGlazedTerracotta),
    BlackGlazedTerracotta(r#BlackGlazedTerracotta),
    WhiteConcrete(r#WhiteConcrete),
    OrangeConcrete(r#OrangeConcrete),
    MagentaConcrete(r#MagentaConcrete),
    LightBlueConcrete(r#LightBlueConcrete),
    YellowConcrete(r#YellowConcrete),
    LimeConcrete(r#LimeConcrete),
    PinkConcrete(r#PinkConcrete),
    GrayConcrete(r#GrayConcrete),
    LightGrayConcrete(r#LightGrayConcrete),
    CyanConcrete(r#CyanConcrete),
    PurpleConcrete(r#PurpleConcrete),
    BlueConcrete(r#BlueConcrete),
    BrownConcrete(r#BrownConcrete),
    GreenConcrete(r#GreenConcrete),
    RedConcrete(r#RedConcrete),
    BlackConcrete(r#BlackConcrete),
    WhiteConcretePowder(r#WhiteConcretePowder),
    OrangeConcretePowder(r#OrangeConcretePowder),
    MagentaConcretePowder(r#MagentaConcretePowder),
    LightBlueConcretePowder(r#LightBlueConcretePowder),
    YellowConcretePowder(r#YellowConcretePowder),
    LimeConcretePowder(r#LimeConcretePowder),
    PinkConcretePowder(r#PinkConcretePowder),
    GrayConcretePowder(r#GrayConcretePowder),
    LightGrayConcretePowder(r#LightGrayConcretePowder),
    CyanConcretePowder(r#CyanConcretePowder),
    PurpleConcretePowder(r#PurpleConcretePowder),
    BlueConcretePowder(r#BlueConcretePowder),
    BrownConcretePowder(r#BrownConcretePowder),
    GreenConcretePowder(r#GreenConcretePowder),
    RedConcretePowder(r#RedConcretePowder),
    BlackConcretePowder(r#BlackConcretePowder),
    TurtleEgg(r#TurtleEgg),
    DeadTubeCoralBlock(r#DeadTubeCoralBlock),
    DeadBrainCoralBlock(r#DeadBrainCoralBlock),
    DeadBubbleCoralBlock(r#DeadBubbleCoralBlock),
    DeadFireCoralBlock(r#DeadFireCoralBlock),
    DeadHornCoralBlock(r#DeadHornCoralBlock),
    TubeCoralBlock(r#TubeCoralBlock),
    BrainCoralBlock(r#BrainCoralBlock),
    BubbleCoralBlock(r#BubbleCoralBlock),
    FireCoralBlock(r#FireCoralBlock),
    HornCoralBlock(r#HornCoralBlock),
    TubeCoral(r#TubeCoral),
    BrainCoral(r#BrainCoral),
    BubbleCoral(r#BubbleCoral),
    FireCoral(r#FireCoral),
    HornCoral(r#HornCoral),
    DeadBrainCoral(r#DeadBrainCoral),
    DeadBubbleCoral(r#DeadBubbleCoral),
    DeadFireCoral(r#DeadFireCoral),
    DeadHornCoral(r#DeadHornCoral),
    DeadTubeCoral(r#DeadTubeCoral),
    TubeCoralFan(r#TubeCoralFan),
    BrainCoralFan(r#BrainCoralFan),
    BubbleCoralFan(r#BubbleCoralFan),
    FireCoralFan(r#FireCoralFan),
    HornCoralFan(r#HornCoralFan),
    DeadTubeCoralFan(r#DeadTubeCoralFan),
    DeadBrainCoralFan(r#DeadBrainCoralFan),
    DeadBubbleCoralFan(r#DeadBubbleCoralFan),
    DeadFireCoralFan(r#DeadFireCoralFan),
    DeadHornCoralFan(r#DeadHornCoralFan),
    BlueIce(r#BlueIce),
    Conduit(r#Conduit),
    PolishedGraniteStairs(r#PolishedGraniteStairs),
    SmoothRedSandstoneStairs(r#SmoothRedSandstoneStairs),
    MossyStoneBrickStairs(r#MossyStoneBrickStairs),
    PolishedDioriteStairs(r#PolishedDioriteStairs),
    MossyCobblestoneStairs(r#MossyCobblestoneStairs),
    EndStoneBrickStairs(r#EndStoneBrickStairs),
    StoneStairs(r#StoneStairs),
    SmoothSandstoneStairs(r#SmoothSandstoneStairs),
    SmoothQuartzStairs(r#SmoothQuartzStairs),
    GraniteStairs(r#GraniteStairs),
    AndesiteStairs(r#AndesiteStairs),
    RedNetherBrickStairs(r#RedNetherBrickStairs),
    PolishedAndesiteStairs(r#PolishedAndesiteStairs),
    DioriteStairs(r#DioriteStairs),
    CobbledDeepslateStairs(r#CobbledDeepslateStairs),
    PolishedDeepslateStairs(r#PolishedDeepslateStairs),
    DeepslateBrickStairs(r#DeepslateBrickStairs),
    DeepslateTileStairs(r#DeepslateTileStairs),
    PolishedGraniteSlab(r#PolishedGraniteSlab),
    SmoothRedSandstoneSlab(r#SmoothRedSandstoneSlab),
    MossyStoneBrickSlab(r#MossyStoneBrickSlab),
    PolishedDioriteSlab(r#PolishedDioriteSlab),
    MossyCobblestoneSlab(r#MossyCobblestoneSlab),
    EndStoneBrickSlab(r#EndStoneBrickSlab),
    SmoothSandstoneSlab(r#SmoothSandstoneSlab),
    SmoothQuartzSlab(r#SmoothQuartzSlab),
    GraniteSlab(r#GraniteSlab),
    AndesiteSlab(r#AndesiteSlab),
    RedNetherBrickSlab(r#RedNetherBrickSlab),
    PolishedAndesiteSlab(r#PolishedAndesiteSlab),
    DioriteSlab(r#DioriteSlab),
    CobbledDeepslateSlab(r#CobbledDeepslateSlab),
    PolishedDeepslateSlab(r#PolishedDeepslateSlab),
    DeepslateBrickSlab(r#DeepslateBrickSlab),
    DeepslateTileSlab(r#DeepslateTileSlab),
    Scaffolding(r#Scaffolding),
    Redstone(r#Redstone),
    RedstoneTorch(r#RedstoneTorch),
    RedstoneBlock(r#RedstoneBlock),
    Repeater(r#Repeater),
    Comparator(r#Comparator),
    Piston(r#Piston),
    StickyPiston(r#StickyPiston),
    SlimeBlock(r#SlimeBlock),
    HoneyBlock(r#HoneyBlock),
    Observer(r#Observer),
    Hopper(r#Hopper),
    Dispenser(r#Dispenser),
    Dropper(r#Dropper),
    Lectern(r#Lectern),
    Target(r#Target),
    Lever(r#Lever),
    LightningRod(r#LightningRod),
    DaylightDetector(r#DaylightDetector),
    SculkSensor(r#SculkSensor),
    TripwireHook(r#TripwireHook),
    TrappedChest(r#TrappedChest),
    Tnt(r#Tnt),
    RedstoneLamp(r#RedstoneLamp),
    NoteBlock(r#NoteBlock),
    StoneButton(r#StoneButton),
    PolishedBlackstoneButton(r#PolishedBlackstoneButton),
    OakButton(r#OakButton),
    SpruceButton(r#SpruceButton),
    BirchButton(r#BirchButton),
    JungleButton(r#JungleButton),
    AcaciaButton(r#AcaciaButton),
    DarkOakButton(r#DarkOakButton),
    MangroveButton(r#MangroveButton),
    BambooButton(r#BambooButton),
    CrimsonButton(r#CrimsonButton),
    WarpedButton(r#WarpedButton),
    StonePressurePlate(r#StonePressurePlate),
    PolishedBlackstonePressurePlate(r#PolishedBlackstonePressurePlate),
    LightWeightedPressurePlate(r#LightWeightedPressurePlate),
    HeavyWeightedPressurePlate(r#HeavyWeightedPressurePlate),
    OakPressurePlate(r#OakPressurePlate),
    SprucePressurePlate(r#SprucePressurePlate),
    BirchPressurePlate(r#BirchPressurePlate),
    JunglePressurePlate(r#JunglePressurePlate),
    AcaciaPressurePlate(r#AcaciaPressurePlate),
    DarkOakPressurePlate(r#DarkOakPressurePlate),
    MangrovePressurePlate(r#MangrovePressurePlate),
    BambooPressurePlate(r#BambooPressurePlate),
    CrimsonPressurePlate(r#CrimsonPressurePlate),
    WarpedPressurePlate(r#WarpedPressurePlate),
    IronDoor(r#IronDoor),
    OakDoor(r#OakDoor),
    SpruceDoor(r#SpruceDoor),
    BirchDoor(r#BirchDoor),
    JungleDoor(r#JungleDoor),
    AcaciaDoor(r#AcaciaDoor),
    DarkOakDoor(r#DarkOakDoor),
    MangroveDoor(r#MangroveDoor),
    BambooDoor(r#BambooDoor),
    CrimsonDoor(r#CrimsonDoor),
    WarpedDoor(r#WarpedDoor),
    IronTrapdoor(r#IronTrapdoor),
    OakTrapdoor(r#OakTrapdoor),
    SpruceTrapdoor(r#SpruceTrapdoor),
    BirchTrapdoor(r#BirchTrapdoor),
    JungleTrapdoor(r#JungleTrapdoor),
    AcaciaTrapdoor(r#AcaciaTrapdoor),
    DarkOakTrapdoor(r#DarkOakTrapdoor),
    MangroveTrapdoor(r#MangroveTrapdoor),
    BambooTrapdoor(r#BambooTrapdoor),
    CrimsonTrapdoor(r#CrimsonTrapdoor),
    WarpedTrapdoor(r#WarpedTrapdoor),
    OakFenceGate(r#OakFenceGate),
    SpruceFenceGate(r#SpruceFenceGate),
    BirchFenceGate(r#BirchFenceGate),
    JungleFenceGate(r#JungleFenceGate),
    AcaciaFenceGate(r#AcaciaFenceGate),
    DarkOakFenceGate(r#DarkOakFenceGate),
    MangroveFenceGate(r#MangroveFenceGate),
    BambooFenceGate(r#BambooFenceGate),
    CrimsonFenceGate(r#CrimsonFenceGate),
    WarpedFenceGate(r#WarpedFenceGate),
    PoweredRail(r#PoweredRail),
    DetectorRail(r#DetectorRail),
    Rail(r#Rail),
    ActivatorRail(r#ActivatorRail),
    Saddle(r#Saddle),
    Minecart(r#Minecart),
    ChestMinecart(r#ChestMinecart),
    FurnaceMinecart(r#FurnaceMinecart),
    TntMinecart(r#TntMinecart),
    HopperMinecart(r#HopperMinecart),
    CarrotOnAStick(r#CarrotOnAStick),
    WarpedFungusOnAStick(r#WarpedFungusOnAStick),
    Elytra(r#Elytra),
    OakBoat(r#OakBoat),
    OakChestBoat(r#OakChestBoat),
    SpruceBoat(r#SpruceBoat),
    SpruceChestBoat(r#SpruceChestBoat),
    BirchBoat(r#BirchBoat),
    BirchChestBoat(r#BirchChestBoat),
    JungleBoat(r#JungleBoat),
    JungleChestBoat(r#JungleChestBoat),
    AcaciaBoat(r#AcaciaBoat),
    AcaciaChestBoat(r#AcaciaChestBoat),
    DarkOakBoat(r#DarkOakBoat),
    DarkOakChestBoat(r#DarkOakChestBoat),
    MangroveBoat(r#MangroveBoat),
    MangroveChestBoat(r#MangroveChestBoat),
    BambooRaft(r#BambooRaft),
    BambooChestRaft(r#BambooChestRaft),
    StructureBlock(r#StructureBlock),
    Jigsaw(r#Jigsaw),
    TurtleHelmet(r#TurtleHelmet),
    Scute(r#Scute),
    FlintAndSteel(r#FlintAndSteel),
    Apple(r#Apple),
    Bow(r#Bow),
    Arrow(r#Arrow),
    Coal(r#Coal),
    Charcoal(r#Charcoal),
    Diamond(r#Diamond),
    Emerald(r#Emerald),
    LapisLazuli(r#LapisLazuli),
    Quartz(r#Quartz),
    AmethystShard(r#AmethystShard),
    RawIron(r#RawIron),
    IronIngot(r#IronIngot),
    RawCopper(r#RawCopper),
    CopperIngot(r#CopperIngot),
    RawGold(r#RawGold),
    GoldIngot(r#GoldIngot),
    NetheriteIngot(r#NetheriteIngot),
    NetheriteScrap(r#NetheriteScrap),
    WoodenSword(r#WoodenSword),
    WoodenShovel(r#WoodenShovel),
    WoodenPickaxe(r#WoodenPickaxe),
    WoodenAxe(r#WoodenAxe),
    WoodenHoe(r#WoodenHoe),
    StoneSword(r#StoneSword),
    StoneShovel(r#StoneShovel),
    StonePickaxe(r#StonePickaxe),
    StoneAxe(r#StoneAxe),
    StoneHoe(r#StoneHoe),
    GoldenSword(r#GoldenSword),
    GoldenShovel(r#GoldenShovel),
    GoldenPickaxe(r#GoldenPickaxe),
    GoldenAxe(r#GoldenAxe),
    GoldenHoe(r#GoldenHoe),
    IronSword(r#IronSword),
    IronShovel(r#IronShovel),
    IronPickaxe(r#IronPickaxe),
    IronAxe(r#IronAxe),
    IronHoe(r#IronHoe),
    DiamondSword(r#DiamondSword),
    DiamondShovel(r#DiamondShovel),
    DiamondPickaxe(r#DiamondPickaxe),
    DiamondAxe(r#DiamondAxe),
    DiamondHoe(r#DiamondHoe),
    NetheriteSword(r#NetheriteSword),
    NetheriteShovel(r#NetheriteShovel),
    NetheritePickaxe(r#NetheritePickaxe),
    NetheriteAxe(r#NetheriteAxe),
    NetheriteHoe(r#NetheriteHoe),
    Stick(r#Stick),
    Bowl(r#Bowl),
    MushroomStew(r#MushroomStew),
    String(r#String),
    Feather(r#Feather),
    Gunpowder(r#Gunpowder),
    WheatSeeds(r#WheatSeeds),
    Wheat(r#Wheat),
    Bread(r#Bread),
    LeatherHelmet(r#LeatherHelmet),
    LeatherChestplate(r#LeatherChestplate),
    LeatherLeggings(r#LeatherLeggings),
    LeatherBoots(r#LeatherBoots),
    ChainmailHelmet(r#ChainmailHelmet),
    ChainmailChestplate(r#ChainmailChestplate),
    ChainmailLeggings(r#ChainmailLeggings),
    ChainmailBoots(r#ChainmailBoots),
    IronHelmet(r#IronHelmet),
    IronChestplate(r#IronChestplate),
    IronLeggings(r#IronLeggings),
    IronBoots(r#IronBoots),
    DiamondHelmet(r#DiamondHelmet),
    DiamondChestplate(r#DiamondChestplate),
    DiamondLeggings(r#DiamondLeggings),
    DiamondBoots(r#DiamondBoots),
    GoldenHelmet(r#GoldenHelmet),
    GoldenChestplate(r#GoldenChestplate),
    GoldenLeggings(r#GoldenLeggings),
    GoldenBoots(r#GoldenBoots),
    NetheriteHelmet(r#NetheriteHelmet),
    NetheriteChestplate(r#NetheriteChestplate),
    NetheriteLeggings(r#NetheriteLeggings),
    NetheriteBoots(r#NetheriteBoots),
    Flint(r#Flint),
    Porkchop(r#Porkchop),
    CookedPorkchop(r#CookedPorkchop),
    Painting(r#Painting),
    GoldenApple(r#GoldenApple),
    EnchantedGoldenApple(r#EnchantedGoldenApple),
    OakSign(r#OakSign),
    SpruceSign(r#SpruceSign),
    BirchSign(r#BirchSign),
    JungleSign(r#JungleSign),
    AcaciaSign(r#AcaciaSign),
    DarkOakSign(r#DarkOakSign),
    MangroveSign(r#MangroveSign),
    BambooSign(r#BambooSign),
    CrimsonSign(r#CrimsonSign),
    WarpedSign(r#WarpedSign),
    OakHangingSign(r#OakHangingSign),
    SpruceHangingSign(r#SpruceHangingSign),
    BirchHangingSign(r#BirchHangingSign),
    JungleHangingSign(r#JungleHangingSign),
    AcaciaHangingSign(r#AcaciaHangingSign),
    DarkOakHangingSign(r#DarkOakHangingSign),
    MangroveHangingSign(r#MangroveHangingSign),
    BambooHangingSign(r#BambooHangingSign),
    CrimsonHangingSign(r#CrimsonHangingSign),
    WarpedHangingSign(r#WarpedHangingSign),
    Bucket(r#Bucket),
    WaterBucket(r#WaterBucket),
    LavaBucket(r#LavaBucket),
    PowderSnowBucket(r#PowderSnowBucket),
    Snowball(r#Snowball),
    Leather(r#Leather),
    MilkBucket(r#MilkBucket),
    PufferfishBucket(r#PufferfishBucket),
    SalmonBucket(r#SalmonBucket),
    CodBucket(r#CodBucket),
    TropicalFishBucket(r#TropicalFishBucket),
    AxolotlBucket(r#AxolotlBucket),
    TadpoleBucket(r#TadpoleBucket),
    Brick(r#Brick),
    ClayBall(r#ClayBall),
    DriedKelpBlock(r#DriedKelpBlock),
    Paper(r#Paper),
    Book(r#Book),
    SlimeBall(r#SlimeBall),
    Egg(r#Egg),
    Compass(r#Compass),
    RecoveryCompass(r#RecoveryCompass),
    Bundle(r#Bundle),
    FishingRod(r#FishingRod),
    Clock(r#Clock),
    Spyglass(r#Spyglass),
    GlowstoneDust(r#GlowstoneDust),
    Cod(r#Cod),
    Salmon(r#Salmon),
    TropicalFish(r#TropicalFish),
    Pufferfish(r#Pufferfish),
    CookedCod(r#CookedCod),
    CookedSalmon(r#CookedSalmon),
    InkSac(r#InkSac),
    GlowInkSac(r#GlowInkSac),
    CocoaBeans(r#CocoaBeans),
    WhiteDye(r#WhiteDye),
    OrangeDye(r#OrangeDye),
    MagentaDye(r#MagentaDye),
    LightBlueDye(r#LightBlueDye),
    YellowDye(r#YellowDye),
    LimeDye(r#LimeDye),
    PinkDye(r#PinkDye),
    GrayDye(r#GrayDye),
    LightGrayDye(r#LightGrayDye),
    CyanDye(r#CyanDye),
    PurpleDye(r#PurpleDye),
    BlueDye(r#BlueDye),
    BrownDye(r#BrownDye),
    GreenDye(r#GreenDye),
    RedDye(r#RedDye),
    BlackDye(r#BlackDye),
    BoneMeal(r#BoneMeal),
    Bone(r#Bone),
    Sugar(r#Sugar),
    Cake(r#Cake),
    WhiteBed(r#WhiteBed),
    OrangeBed(r#OrangeBed),
    MagentaBed(r#MagentaBed),
    LightBlueBed(r#LightBlueBed),
    YellowBed(r#YellowBed),
    LimeBed(r#LimeBed),
    PinkBed(r#PinkBed),
    GrayBed(r#GrayBed),
    LightGrayBed(r#LightGrayBed),
    CyanBed(r#CyanBed),
    PurpleBed(r#PurpleBed),
    BlueBed(r#BlueBed),
    BrownBed(r#BrownBed),
    GreenBed(r#GreenBed),
    RedBed(r#RedBed),
    BlackBed(r#BlackBed),
    Cookie(r#Cookie),
    FilledMap(r#FilledMap),
    Shears(r#Shears),
    MelonSlice(r#MelonSlice),
    DriedKelp(r#DriedKelp),
    PumpkinSeeds(r#PumpkinSeeds),
    MelonSeeds(r#MelonSeeds),
    Beef(r#Beef),
    CookedBeef(r#CookedBeef),
    Chicken(r#Chicken),
    CookedChicken(r#CookedChicken),
    RottenFlesh(r#RottenFlesh),
    EnderPearl(r#EnderPearl),
    BlazeRod(r#BlazeRod),
    GhastTear(r#GhastTear),
    GoldNugget(r#GoldNugget),
    NetherWart(r#NetherWart),
    Potion(r#Potion),
    GlassBottle(r#GlassBottle),
    SpiderEye(r#SpiderEye),
    FermentedSpiderEye(r#FermentedSpiderEye),
    BlazePowder(r#BlazePowder),
    MagmaCream(r#MagmaCream),
    BrewingStand(r#BrewingStand),
    Cauldron(r#Cauldron),
    EnderEye(r#EnderEye),
    GlisteringMelonSlice(r#GlisteringMelonSlice),
    AllaySpawnEgg(r#AllaySpawnEgg),
    AxolotlSpawnEgg(r#AxolotlSpawnEgg),
    BatSpawnEgg(r#BatSpawnEgg),
    BeeSpawnEgg(r#BeeSpawnEgg),
    BlazeSpawnEgg(r#BlazeSpawnEgg),
    CatSpawnEgg(r#CatSpawnEgg),
    CamelSpawnEgg(r#CamelSpawnEgg),
    CaveSpiderSpawnEgg(r#CaveSpiderSpawnEgg),
    ChickenSpawnEgg(r#ChickenSpawnEgg),
    CodSpawnEgg(r#CodSpawnEgg),
    CowSpawnEgg(r#CowSpawnEgg),
    CreeperSpawnEgg(r#CreeperSpawnEgg),
    DolphinSpawnEgg(r#DolphinSpawnEgg),
    DonkeySpawnEgg(r#DonkeySpawnEgg),
    DrownedSpawnEgg(r#DrownedSpawnEgg),
    ElderGuardianSpawnEgg(r#ElderGuardianSpawnEgg),
    EnderDragonSpawnEgg(r#EnderDragonSpawnEgg),
    EndermanSpawnEgg(r#EndermanSpawnEgg),
    EndermiteSpawnEgg(r#EndermiteSpawnEgg),
    EvokerSpawnEgg(r#EvokerSpawnEgg),
    FoxSpawnEgg(r#FoxSpawnEgg),
    FrogSpawnEgg(r#FrogSpawnEgg),
    GhastSpawnEgg(r#GhastSpawnEgg),
    GlowSquidSpawnEgg(r#GlowSquidSpawnEgg),
    GoatSpawnEgg(r#GoatSpawnEgg),
    GuardianSpawnEgg(r#GuardianSpawnEgg),
    HoglinSpawnEgg(r#HoglinSpawnEgg),
    HorseSpawnEgg(r#HorseSpawnEgg),
    HuskSpawnEgg(r#HuskSpawnEgg),
    IronGolemSpawnEgg(r#IronGolemSpawnEgg),
    LlamaSpawnEgg(r#LlamaSpawnEgg),
    MagmaCubeSpawnEgg(r#MagmaCubeSpawnEgg),
    MooshroomSpawnEgg(r#MooshroomSpawnEgg),
    MuleSpawnEgg(r#MuleSpawnEgg),
    OcelotSpawnEgg(r#OcelotSpawnEgg),
    PandaSpawnEgg(r#PandaSpawnEgg),
    ParrotSpawnEgg(r#ParrotSpawnEgg),
    PhantomSpawnEgg(r#PhantomSpawnEgg),
    PigSpawnEgg(r#PigSpawnEgg),
    PiglinSpawnEgg(r#PiglinSpawnEgg),
    PiglinBruteSpawnEgg(r#PiglinBruteSpawnEgg),
    PillagerSpawnEgg(r#PillagerSpawnEgg),
    PolarBearSpawnEgg(r#PolarBearSpawnEgg),
    PufferfishSpawnEgg(r#PufferfishSpawnEgg),
    RabbitSpawnEgg(r#RabbitSpawnEgg),
    RavagerSpawnEgg(r#RavagerSpawnEgg),
    SalmonSpawnEgg(r#SalmonSpawnEgg),
    SheepSpawnEgg(r#SheepSpawnEgg),
    ShulkerSpawnEgg(r#ShulkerSpawnEgg),
    SilverfishSpawnEgg(r#SilverfishSpawnEgg),
    SkeletonSpawnEgg(r#SkeletonSpawnEgg),
    SkeletonHorseSpawnEgg(r#SkeletonHorseSpawnEgg),
    SlimeSpawnEgg(r#SlimeSpawnEgg),
    SnowGolemSpawnEgg(r#SnowGolemSpawnEgg),
    SpiderSpawnEgg(r#SpiderSpawnEgg),
    SquidSpawnEgg(r#SquidSpawnEgg),
    StraySpawnEgg(r#StraySpawnEgg),
    StriderSpawnEgg(r#StriderSpawnEgg),
    TadpoleSpawnEgg(r#TadpoleSpawnEgg),
    TraderLlamaSpawnEgg(r#TraderLlamaSpawnEgg),
    TropicalFishSpawnEgg(r#TropicalFishSpawnEgg),
    TurtleSpawnEgg(r#TurtleSpawnEgg),
    VexSpawnEgg(r#VexSpawnEgg),
    VillagerSpawnEgg(r#VillagerSpawnEgg),
    VindicatorSpawnEgg(r#VindicatorSpawnEgg),
    WanderingTraderSpawnEgg(r#WanderingTraderSpawnEgg),
    WardenSpawnEgg(r#WardenSpawnEgg),
    WitchSpawnEgg(r#WitchSpawnEgg),
    WitherSpawnEgg(r#WitherSpawnEgg),
    WitherSkeletonSpawnEgg(r#WitherSkeletonSpawnEgg),
    WolfSpawnEgg(r#WolfSpawnEgg),
    ZoglinSpawnEgg(r#ZoglinSpawnEgg),
    ZombieSpawnEgg(r#ZombieSpawnEgg),
    ZombieHorseSpawnEgg(r#ZombieHorseSpawnEgg),
    ZombieVillagerSpawnEgg(r#ZombieVillagerSpawnEgg),
    ZombifiedPiglinSpawnEgg(r#ZombifiedPiglinSpawnEgg),
    ExperienceBottle(r#ExperienceBottle),
    FireCharge(r#FireCharge),
    WritableBook(r#WritableBook),
    WrittenBook(r#WrittenBook),
    ItemFrame(r#ItemFrame),
    GlowItemFrame(r#GlowItemFrame),
    FlowerPot(r#FlowerPot),
    Carrot(r#Carrot),
    Potato(r#Potato),
    BakedPotato(r#BakedPotato),
    PoisonousPotato(r#PoisonousPotato),
    Map(r#Map),
    GoldenCarrot(r#GoldenCarrot),
    SkeletonSkull(r#SkeletonSkull),
    WitherSkeletonSkull(r#WitherSkeletonSkull),
    PlayerHead(r#PlayerHead),
    ZombieHead(r#ZombieHead),
    CreeperHead(r#CreeperHead),
    DragonHead(r#DragonHead),
    PiglinHead(r#PiglinHead),
    NetherStar(r#NetherStar),
    PumpkinPie(r#PumpkinPie),
    FireworkRocket(r#FireworkRocket),
    FireworkStar(r#FireworkStar),
    EnchantedBook(r#EnchantedBook),
    NetherBrick(r#NetherBrick),
    PrismarineShard(r#PrismarineShard),
    PrismarineCrystals(r#PrismarineCrystals),
    Rabbit(r#Rabbit),
    CookedRabbit(r#CookedRabbit),
    RabbitStew(r#RabbitStew),
    RabbitFoot(r#RabbitFoot),
    RabbitHide(r#RabbitHide),
    ArmorStand(r#ArmorStand),
    IronHorseArmor(r#IronHorseArmor),
    GoldenHorseArmor(r#GoldenHorseArmor),
    DiamondHorseArmor(r#DiamondHorseArmor),
    LeatherHorseArmor(r#LeatherHorseArmor),
    Lead(r#Lead),
    NameTag(r#NameTag),
    CommandBlockMinecart(r#CommandBlockMinecart),
    Mutton(r#Mutton),
    CookedMutton(r#CookedMutton),
    WhiteBanner(r#WhiteBanner),
    OrangeBanner(r#OrangeBanner),
    MagentaBanner(r#MagentaBanner),
    LightBlueBanner(r#LightBlueBanner),
    YellowBanner(r#YellowBanner),
    LimeBanner(r#LimeBanner),
    PinkBanner(r#PinkBanner),
    GrayBanner(r#GrayBanner),
    LightGrayBanner(r#LightGrayBanner),
    CyanBanner(r#CyanBanner),
    PurpleBanner(r#PurpleBanner),
    BlueBanner(r#BlueBanner),
    BrownBanner(r#BrownBanner),
    GreenBanner(r#GreenBanner),
    RedBanner(r#RedBanner),
    BlackBanner(r#BlackBanner),
    EndCrystal(r#EndCrystal),
    ChorusFruit(r#ChorusFruit),
    PoppedChorusFruit(r#PoppedChorusFruit),
    Beetroot(r#Beetroot),
    BeetrootSeeds(r#BeetrootSeeds),
    BeetrootSoup(r#BeetrootSoup),
    DragonBreath(r#DragonBreath),
    SplashPotion(r#SplashPotion),
    SpectralArrow(r#SpectralArrow),
    TippedArrow(r#TippedArrow),
    LingeringPotion(r#LingeringPotion),
    Shield(r#Shield),
    TotemOfUndying(r#TotemOfUndying),
    ShulkerShell(r#ShulkerShell),
    IronNugget(r#IronNugget),
    KnowledgeBook(r#KnowledgeBook),
    DebugStick(r#DebugStick),
    MusicDisc13(r#MusicDisc13),
    MusicDiscCat(r#MusicDiscCat),
    MusicDiscBlocks(r#MusicDiscBlocks),
    MusicDiscChirp(r#MusicDiscChirp),
    MusicDiscFar(r#MusicDiscFar),
    MusicDiscMall(r#MusicDiscMall),
    MusicDiscMellohi(r#MusicDiscMellohi),
    MusicDiscStal(r#MusicDiscStal),
    MusicDiscStrad(r#MusicDiscStrad),
    MusicDiscWard(r#MusicDiscWard),
    MusicDisc11(r#MusicDisc11),
    MusicDiscWait(r#MusicDiscWait),
    MusicDiscOtherside(r#MusicDiscOtherside),
    MusicDisc5(r#MusicDisc5),
    MusicDiscPigstep(r#MusicDiscPigstep),
    DiscFragment5(r#DiscFragment5),
    Trident(r#Trident),
    PhantomMembrane(r#PhantomMembrane),
    NautilusShell(r#NautilusShell),
    HeartOfTheSea(r#HeartOfTheSea),
    Crossbow(r#Crossbow),
    SuspiciousStew(r#SuspiciousStew),
    Loom(r#Loom),
    FlowerBannerPattern(r#FlowerBannerPattern),
    CreeperBannerPattern(r#CreeperBannerPattern),
    SkullBannerPattern(r#SkullBannerPattern),
    MojangBannerPattern(r#MojangBannerPattern),
    GlobeBannerPattern(r#GlobeBannerPattern),
    PiglinBannerPattern(r#PiglinBannerPattern),
    GoatHorn(r#GoatHorn),
    Composter(r#Composter),
    Barrel(r#Barrel),
    Smoker(r#Smoker),
    BlastFurnace(r#BlastFurnace),
    CartographyTable(r#CartographyTable),
    FletchingTable(r#FletchingTable),
    Grindstone(r#Grindstone),
    SmithingTable(r#SmithingTable),
    Stonecutter(r#Stonecutter),
    Bell(r#Bell),
    Lantern(r#Lantern),
    SoulLantern(r#SoulLantern),
    SweetBerries(r#SweetBerries),
    GlowBerries(r#GlowBerries),
    Campfire(r#Campfire),
    SoulCampfire(r#SoulCampfire),
    Shroomlight(r#Shroomlight),
    Honeycomb(r#Honeycomb),
    BeeNest(r#BeeNest),
    Beehive(r#Beehive),
    HoneyBottle(r#HoneyBottle),
    HoneycombBlock(r#HoneycombBlock),
    Lodestone(r#Lodestone),
    CryingObsidian(r#CryingObsidian),
    Blackstone(r#Blackstone),
    BlackstoneSlab(r#BlackstoneSlab),
    BlackstoneStairs(r#BlackstoneStairs),
    GildedBlackstone(r#GildedBlackstone),
    PolishedBlackstone(r#PolishedBlackstone),
    PolishedBlackstoneSlab(r#PolishedBlackstoneSlab),
    PolishedBlackstoneStairs(r#PolishedBlackstoneStairs),
    ChiseledPolishedBlackstone(r#ChiseledPolishedBlackstone),
    PolishedBlackstoneBricks(r#PolishedBlackstoneBricks),
    PolishedBlackstoneBrickSlab(r#PolishedBlackstoneBrickSlab),
    PolishedBlackstoneBrickStairs(r#PolishedBlackstoneBrickStairs),
    CrackedPolishedBlackstoneBricks(r#CrackedPolishedBlackstoneBricks),
    RespawnAnchor(r#RespawnAnchor),
    Candle(r#Candle),
    WhiteCandle(r#WhiteCandle),
    OrangeCandle(r#OrangeCandle),
    MagentaCandle(r#MagentaCandle),
    LightBlueCandle(r#LightBlueCandle),
    YellowCandle(r#YellowCandle),
    LimeCandle(r#LimeCandle),
    PinkCandle(r#PinkCandle),
    GrayCandle(r#GrayCandle),
    LightGrayCandle(r#LightGrayCandle),
    CyanCandle(r#CyanCandle),
    PurpleCandle(r#PurpleCandle),
    BlueCandle(r#BlueCandle),
    BrownCandle(r#BrownCandle),
    GreenCandle(r#GreenCandle),
    RedCandle(r#RedCandle),
    BlackCandle(r#BlackCandle),
    SmallAmethystBud(r#SmallAmethystBud),
    MediumAmethystBud(r#MediumAmethystBud),
    LargeAmethystBud(r#LargeAmethystBud),
    AmethystCluster(r#AmethystCluster),
    PointedDripstone(r#PointedDripstone),
    OchreFroglight(r#OchreFroglight),
    VerdantFroglight(r#VerdantFroglight),
    PearlescentFroglight(r#PearlescentFroglight),
    Frogspawn(r#Frogspawn),
    EchoShard(r#EchoShard),
}
impl Items {
    pub fn get_id(&self) -> i32 {
        match self {
            Self::Air(_) => r#Air::ID,
            Self::Stone(_) => r#Stone::ID,
            Self::Granite(_) => r#Granite::ID,
            Self::PolishedGranite(_) => r#PolishedGranite::ID,
            Self::Diorite(_) => r#Diorite::ID,
            Self::PolishedDiorite(_) => r#PolishedDiorite::ID,
            Self::Andesite(_) => r#Andesite::ID,
            Self::PolishedAndesite(_) => r#PolishedAndesite::ID,
            Self::Deepslate(_) => r#Deepslate::ID,
            Self::CobbledDeepslate(_) => r#CobbledDeepslate::ID,
            Self::PolishedDeepslate(_) => r#PolishedDeepslate::ID,
            Self::Calcite(_) => r#Calcite::ID,
            Self::Tuff(_) => r#Tuff::ID,
            Self::DripstoneBlock(_) => r#DripstoneBlock::ID,
            Self::GrassBlock(_) => r#GrassBlock::ID,
            Self::Dirt(_) => r#Dirt::ID,
            Self::CoarseDirt(_) => r#CoarseDirt::ID,
            Self::Podzol(_) => r#Podzol::ID,
            Self::RootedDirt(_) => r#RootedDirt::ID,
            Self::Mud(_) => r#Mud::ID,
            Self::CrimsonNylium(_) => r#CrimsonNylium::ID,
            Self::WarpedNylium(_) => r#WarpedNylium::ID,
            Self::Cobblestone(_) => r#Cobblestone::ID,
            Self::OakPlanks(_) => r#OakPlanks::ID,
            Self::SprucePlanks(_) => r#SprucePlanks::ID,
            Self::BirchPlanks(_) => r#BirchPlanks::ID,
            Self::JunglePlanks(_) => r#JunglePlanks::ID,
            Self::AcaciaPlanks(_) => r#AcaciaPlanks::ID,
            Self::DarkOakPlanks(_) => r#DarkOakPlanks::ID,
            Self::MangrovePlanks(_) => r#MangrovePlanks::ID,
            Self::BambooPlanks(_) => r#BambooPlanks::ID,
            Self::CrimsonPlanks(_) => r#CrimsonPlanks::ID,
            Self::WarpedPlanks(_) => r#WarpedPlanks::ID,
            Self::BambooMosaic(_) => r#BambooMosaic::ID,
            Self::OakSapling(_) => r#OakSapling::ID,
            Self::SpruceSapling(_) => r#SpruceSapling::ID,
            Self::BirchSapling(_) => r#BirchSapling::ID,
            Self::JungleSapling(_) => r#JungleSapling::ID,
            Self::AcaciaSapling(_) => r#AcaciaSapling::ID,
            Self::DarkOakSapling(_) => r#DarkOakSapling::ID,
            Self::MangrovePropagule(_) => r#MangrovePropagule::ID,
            Self::Bedrock(_) => r#Bedrock::ID,
            Self::Sand(_) => r#Sand::ID,
            Self::RedSand(_) => r#RedSand::ID,
            Self::Gravel(_) => r#Gravel::ID,
            Self::CoalOre(_) => r#CoalOre::ID,
            Self::DeepslateCoalOre(_) => r#DeepslateCoalOre::ID,
            Self::IronOre(_) => r#IronOre::ID,
            Self::DeepslateIronOre(_) => r#DeepslateIronOre::ID,
            Self::CopperOre(_) => r#CopperOre::ID,
            Self::DeepslateCopperOre(_) => r#DeepslateCopperOre::ID,
            Self::GoldOre(_) => r#GoldOre::ID,
            Self::DeepslateGoldOre(_) => r#DeepslateGoldOre::ID,
            Self::RedstoneOre(_) => r#RedstoneOre::ID,
            Self::DeepslateRedstoneOre(_) => r#DeepslateRedstoneOre::ID,
            Self::EmeraldOre(_) => r#EmeraldOre::ID,
            Self::DeepslateEmeraldOre(_) => r#DeepslateEmeraldOre::ID,
            Self::LapisOre(_) => r#LapisOre::ID,
            Self::DeepslateLapisOre(_) => r#DeepslateLapisOre::ID,
            Self::DiamondOre(_) => r#DiamondOre::ID,
            Self::DeepslateDiamondOre(_) => r#DeepslateDiamondOre::ID,
            Self::NetherGoldOre(_) => r#NetherGoldOre::ID,
            Self::NetherQuartzOre(_) => r#NetherQuartzOre::ID,
            Self::AncientDebris(_) => r#AncientDebris::ID,
            Self::CoalBlock(_) => r#CoalBlock::ID,
            Self::RawIronBlock(_) => r#RawIronBlock::ID,
            Self::RawCopperBlock(_) => r#RawCopperBlock::ID,
            Self::RawGoldBlock(_) => r#RawGoldBlock::ID,
            Self::AmethystBlock(_) => r#AmethystBlock::ID,
            Self::BuddingAmethyst(_) => r#BuddingAmethyst::ID,
            Self::IronBlock(_) => r#IronBlock::ID,
            Self::CopperBlock(_) => r#CopperBlock::ID,
            Self::GoldBlock(_) => r#GoldBlock::ID,
            Self::DiamondBlock(_) => r#DiamondBlock::ID,
            Self::NetheriteBlock(_) => r#NetheriteBlock::ID,
            Self::ExposedCopper(_) => r#ExposedCopper::ID,
            Self::WeatheredCopper(_) => r#WeatheredCopper::ID,
            Self::OxidizedCopper(_) => r#OxidizedCopper::ID,
            Self::CutCopper(_) => r#CutCopper::ID,
            Self::ExposedCutCopper(_) => r#ExposedCutCopper::ID,
            Self::WeatheredCutCopper(_) => r#WeatheredCutCopper::ID,
            Self::OxidizedCutCopper(_) => r#OxidizedCutCopper::ID,
            Self::CutCopperStairs(_) => r#CutCopperStairs::ID,
            Self::ExposedCutCopperStairs(_) => r#ExposedCutCopperStairs::ID,
            Self::WeatheredCutCopperStairs(_) => r#WeatheredCutCopperStairs::ID,
            Self::OxidizedCutCopperStairs(_) => r#OxidizedCutCopperStairs::ID,
            Self::CutCopperSlab(_) => r#CutCopperSlab::ID,
            Self::ExposedCutCopperSlab(_) => r#ExposedCutCopperSlab::ID,
            Self::WeatheredCutCopperSlab(_) => r#WeatheredCutCopperSlab::ID,
            Self::OxidizedCutCopperSlab(_) => r#OxidizedCutCopperSlab::ID,
            Self::WaxedCopperBlock(_) => r#WaxedCopperBlock::ID,
            Self::WaxedExposedCopper(_) => r#WaxedExposedCopper::ID,
            Self::WaxedWeatheredCopper(_) => r#WaxedWeatheredCopper::ID,
            Self::WaxedOxidizedCopper(_) => r#WaxedOxidizedCopper::ID,
            Self::WaxedCutCopper(_) => r#WaxedCutCopper::ID,
            Self::WaxedExposedCutCopper(_) => r#WaxedExposedCutCopper::ID,
            Self::WaxedWeatheredCutCopper(_) => r#WaxedWeatheredCutCopper::ID,
            Self::WaxedOxidizedCutCopper(_) => r#WaxedOxidizedCutCopper::ID,
            Self::WaxedCutCopperStairs(_) => r#WaxedCutCopperStairs::ID,
            Self::WaxedExposedCutCopperStairs(_) => r#WaxedExposedCutCopperStairs::ID,
            Self::WaxedWeatheredCutCopperStairs(_) => r#WaxedWeatheredCutCopperStairs::ID,
            Self::WaxedOxidizedCutCopperStairs(_) => r#WaxedOxidizedCutCopperStairs::ID,
            Self::WaxedCutCopperSlab(_) => r#WaxedCutCopperSlab::ID,
            Self::WaxedExposedCutCopperSlab(_) => r#WaxedExposedCutCopperSlab::ID,
            Self::WaxedWeatheredCutCopperSlab(_) => r#WaxedWeatheredCutCopperSlab::ID,
            Self::WaxedOxidizedCutCopperSlab(_) => r#WaxedOxidizedCutCopperSlab::ID,
            Self::OakLog(_) => r#OakLog::ID,
            Self::SpruceLog(_) => r#SpruceLog::ID,
            Self::BirchLog(_) => r#BirchLog::ID,
            Self::JungleLog(_) => r#JungleLog::ID,
            Self::AcaciaLog(_) => r#AcaciaLog::ID,
            Self::DarkOakLog(_) => r#DarkOakLog::ID,
            Self::MangroveLog(_) => r#MangroveLog::ID,
            Self::MangroveRoots(_) => r#MangroveRoots::ID,
            Self::MuddyMangroveRoots(_) => r#MuddyMangroveRoots::ID,
            Self::CrimsonStem(_) => r#CrimsonStem::ID,
            Self::WarpedStem(_) => r#WarpedStem::ID,
            Self::BambooBlock(_) => r#BambooBlock::ID,
            Self::StrippedOakLog(_) => r#StrippedOakLog::ID,
            Self::StrippedSpruceLog(_) => r#StrippedSpruceLog::ID,
            Self::StrippedBirchLog(_) => r#StrippedBirchLog::ID,
            Self::StrippedJungleLog(_) => r#StrippedJungleLog::ID,
            Self::StrippedAcaciaLog(_) => r#StrippedAcaciaLog::ID,
            Self::StrippedDarkOakLog(_) => r#StrippedDarkOakLog::ID,
            Self::StrippedMangroveLog(_) => r#StrippedMangroveLog::ID,
            Self::StrippedCrimsonStem(_) => r#StrippedCrimsonStem::ID,
            Self::StrippedWarpedStem(_) => r#StrippedWarpedStem::ID,
            Self::StrippedOakWood(_) => r#StrippedOakWood::ID,
            Self::StrippedSpruceWood(_) => r#StrippedSpruceWood::ID,
            Self::StrippedBirchWood(_) => r#StrippedBirchWood::ID,
            Self::StrippedJungleWood(_) => r#StrippedJungleWood::ID,
            Self::StrippedAcaciaWood(_) => r#StrippedAcaciaWood::ID,
            Self::StrippedDarkOakWood(_) => r#StrippedDarkOakWood::ID,
            Self::StrippedMangroveWood(_) => r#StrippedMangroveWood::ID,
            Self::StrippedCrimsonHyphae(_) => r#StrippedCrimsonHyphae::ID,
            Self::StrippedWarpedHyphae(_) => r#StrippedWarpedHyphae::ID,
            Self::StrippedBambooBlock(_) => r#StrippedBambooBlock::ID,
            Self::OakWood(_) => r#OakWood::ID,
            Self::SpruceWood(_) => r#SpruceWood::ID,
            Self::BirchWood(_) => r#BirchWood::ID,
            Self::JungleWood(_) => r#JungleWood::ID,
            Self::AcaciaWood(_) => r#AcaciaWood::ID,
            Self::DarkOakWood(_) => r#DarkOakWood::ID,
            Self::MangroveWood(_) => r#MangroveWood::ID,
            Self::CrimsonHyphae(_) => r#CrimsonHyphae::ID,
            Self::WarpedHyphae(_) => r#WarpedHyphae::ID,
            Self::OakLeaves(_) => r#OakLeaves::ID,
            Self::SpruceLeaves(_) => r#SpruceLeaves::ID,
            Self::BirchLeaves(_) => r#BirchLeaves::ID,
            Self::JungleLeaves(_) => r#JungleLeaves::ID,
            Self::AcaciaLeaves(_) => r#AcaciaLeaves::ID,
            Self::DarkOakLeaves(_) => r#DarkOakLeaves::ID,
            Self::MangroveLeaves(_) => r#MangroveLeaves::ID,
            Self::AzaleaLeaves(_) => r#AzaleaLeaves::ID,
            Self::FloweringAzaleaLeaves(_) => r#FloweringAzaleaLeaves::ID,
            Self::Sponge(_) => r#Sponge::ID,
            Self::WetSponge(_) => r#WetSponge::ID,
            Self::Glass(_) => r#Glass::ID,
            Self::TintedGlass(_) => r#TintedGlass::ID,
            Self::LapisBlock(_) => r#LapisBlock::ID,
            Self::Sandstone(_) => r#Sandstone::ID,
            Self::ChiseledSandstone(_) => r#ChiseledSandstone::ID,
            Self::CutSandstone(_) => r#CutSandstone::ID,
            Self::Cobweb(_) => r#Cobweb::ID,
            Self::Grass(_) => r#Grass::ID,
            Self::Fern(_) => r#Fern::ID,
            Self::Azalea(_) => r#Azalea::ID,
            Self::FloweringAzalea(_) => r#FloweringAzalea::ID,
            Self::DeadBush(_) => r#DeadBush::ID,
            Self::Seagrass(_) => r#Seagrass::ID,
            Self::SeaPickle(_) => r#SeaPickle::ID,
            Self::WhiteWool(_) => r#WhiteWool::ID,
            Self::OrangeWool(_) => r#OrangeWool::ID,
            Self::MagentaWool(_) => r#MagentaWool::ID,
            Self::LightBlueWool(_) => r#LightBlueWool::ID,
            Self::YellowWool(_) => r#YellowWool::ID,
            Self::LimeWool(_) => r#LimeWool::ID,
            Self::PinkWool(_) => r#PinkWool::ID,
            Self::GrayWool(_) => r#GrayWool::ID,
            Self::LightGrayWool(_) => r#LightGrayWool::ID,
            Self::CyanWool(_) => r#CyanWool::ID,
            Self::PurpleWool(_) => r#PurpleWool::ID,
            Self::BlueWool(_) => r#BlueWool::ID,
            Self::BrownWool(_) => r#BrownWool::ID,
            Self::GreenWool(_) => r#GreenWool::ID,
            Self::RedWool(_) => r#RedWool::ID,
            Self::BlackWool(_) => r#BlackWool::ID,
            Self::Dandelion(_) => r#Dandelion::ID,
            Self::Poppy(_) => r#Poppy::ID,
            Self::BlueOrchid(_) => r#BlueOrchid::ID,
            Self::Allium(_) => r#Allium::ID,
            Self::AzureBluet(_) => r#AzureBluet::ID,
            Self::RedTulip(_) => r#RedTulip::ID,
            Self::OrangeTulip(_) => r#OrangeTulip::ID,
            Self::WhiteTulip(_) => r#WhiteTulip::ID,
            Self::PinkTulip(_) => r#PinkTulip::ID,
            Self::OxeyeDaisy(_) => r#OxeyeDaisy::ID,
            Self::Cornflower(_) => r#Cornflower::ID,
            Self::LilyOfTheValley(_) => r#LilyOfTheValley::ID,
            Self::WitherRose(_) => r#WitherRose::ID,
            Self::SporeBlossom(_) => r#SporeBlossom::ID,
            Self::BrownMushroom(_) => r#BrownMushroom::ID,
            Self::RedMushroom(_) => r#RedMushroom::ID,
            Self::CrimsonFungus(_) => r#CrimsonFungus::ID,
            Self::WarpedFungus(_) => r#WarpedFungus::ID,
            Self::CrimsonRoots(_) => r#CrimsonRoots::ID,
            Self::WarpedRoots(_) => r#WarpedRoots::ID,
            Self::NetherSprouts(_) => r#NetherSprouts::ID,
            Self::WeepingVines(_) => r#WeepingVines::ID,
            Self::TwistingVines(_) => r#TwistingVines::ID,
            Self::SugarCane(_) => r#SugarCane::ID,
            Self::Kelp(_) => r#Kelp::ID,
            Self::MossCarpet(_) => r#MossCarpet::ID,
            Self::MossBlock(_) => r#MossBlock::ID,
            Self::HangingRoots(_) => r#HangingRoots::ID,
            Self::BigDripleaf(_) => r#BigDripleaf::ID,
            Self::SmallDripleaf(_) => r#SmallDripleaf::ID,
            Self::Bamboo(_) => r#Bamboo::ID,
            Self::OakSlab(_) => r#OakSlab::ID,
            Self::SpruceSlab(_) => r#SpruceSlab::ID,
            Self::BirchSlab(_) => r#BirchSlab::ID,
            Self::JungleSlab(_) => r#JungleSlab::ID,
            Self::AcaciaSlab(_) => r#AcaciaSlab::ID,
            Self::DarkOakSlab(_) => r#DarkOakSlab::ID,
            Self::MangroveSlab(_) => r#MangroveSlab::ID,
            Self::BambooSlab(_) => r#BambooSlab::ID,
            Self::BambooMosaicSlab(_) => r#BambooMosaicSlab::ID,
            Self::CrimsonSlab(_) => r#CrimsonSlab::ID,
            Self::WarpedSlab(_) => r#WarpedSlab::ID,
            Self::StoneSlab(_) => r#StoneSlab::ID,
            Self::SmoothStoneSlab(_) => r#SmoothStoneSlab::ID,
            Self::SandstoneSlab(_) => r#SandstoneSlab::ID,
            Self::CutSandstoneSlab(_) => r#CutSandstoneSlab::ID,
            Self::PetrifiedOakSlab(_) => r#PetrifiedOakSlab::ID,
            Self::CobblestoneSlab(_) => r#CobblestoneSlab::ID,
            Self::BrickSlab(_) => r#BrickSlab::ID,
            Self::StoneBrickSlab(_) => r#StoneBrickSlab::ID,
            Self::MudBrickSlab(_) => r#MudBrickSlab::ID,
            Self::NetherBrickSlab(_) => r#NetherBrickSlab::ID,
            Self::QuartzSlab(_) => r#QuartzSlab::ID,
            Self::RedSandstoneSlab(_) => r#RedSandstoneSlab::ID,
            Self::CutRedSandstoneSlab(_) => r#CutRedSandstoneSlab::ID,
            Self::PurpurSlab(_) => r#PurpurSlab::ID,
            Self::PrismarineSlab(_) => r#PrismarineSlab::ID,
            Self::PrismarineBrickSlab(_) => r#PrismarineBrickSlab::ID,
            Self::DarkPrismarineSlab(_) => r#DarkPrismarineSlab::ID,
            Self::SmoothQuartz(_) => r#SmoothQuartz::ID,
            Self::SmoothRedSandstone(_) => r#SmoothRedSandstone::ID,
            Self::SmoothSandstone(_) => r#SmoothSandstone::ID,
            Self::SmoothStone(_) => r#SmoothStone::ID,
            Self::Bricks(_) => r#Bricks::ID,
            Self::Bookshelf(_) => r#Bookshelf::ID,
            Self::ChiseledBookshelf(_) => r#ChiseledBookshelf::ID,
            Self::MossyCobblestone(_) => r#MossyCobblestone::ID,
            Self::Obsidian(_) => r#Obsidian::ID,
            Self::Torch(_) => r#Torch::ID,
            Self::EndRod(_) => r#EndRod::ID,
            Self::ChorusPlant(_) => r#ChorusPlant::ID,
            Self::ChorusFlower(_) => r#ChorusFlower::ID,
            Self::PurpurBlock(_) => r#PurpurBlock::ID,
            Self::PurpurPillar(_) => r#PurpurPillar::ID,
            Self::PurpurStairs(_) => r#PurpurStairs::ID,
            Self::Spawner(_) => r#Spawner::ID,
            Self::Chest(_) => r#Chest::ID,
            Self::CraftingTable(_) => r#CraftingTable::ID,
            Self::Farmland(_) => r#Farmland::ID,
            Self::Furnace(_) => r#Furnace::ID,
            Self::Ladder(_) => r#Ladder::ID,
            Self::CobblestoneStairs(_) => r#CobblestoneStairs::ID,
            Self::Snow(_) => r#Snow::ID,
            Self::Ice(_) => r#Ice::ID,
            Self::SnowBlock(_) => r#SnowBlock::ID,
            Self::Cactus(_) => r#Cactus::ID,
            Self::Clay(_) => r#Clay::ID,
            Self::Jukebox(_) => r#Jukebox::ID,
            Self::OakFence(_) => r#OakFence::ID,
            Self::SpruceFence(_) => r#SpruceFence::ID,
            Self::BirchFence(_) => r#BirchFence::ID,
            Self::JungleFence(_) => r#JungleFence::ID,
            Self::AcaciaFence(_) => r#AcaciaFence::ID,
            Self::DarkOakFence(_) => r#DarkOakFence::ID,
            Self::MangroveFence(_) => r#MangroveFence::ID,
            Self::BambooFence(_) => r#BambooFence::ID,
            Self::CrimsonFence(_) => r#CrimsonFence::ID,
            Self::WarpedFence(_) => r#WarpedFence::ID,
            Self::Pumpkin(_) => r#Pumpkin::ID,
            Self::CarvedPumpkin(_) => r#CarvedPumpkin::ID,
            Self::JackOLantern(_) => r#JackOLantern::ID,
            Self::Netherrack(_) => r#Netherrack::ID,
            Self::SoulSand(_) => r#SoulSand::ID,
            Self::SoulSoil(_) => r#SoulSoil::ID,
            Self::Basalt(_) => r#Basalt::ID,
            Self::PolishedBasalt(_) => r#PolishedBasalt::ID,
            Self::SmoothBasalt(_) => r#SmoothBasalt::ID,
            Self::SoulTorch(_) => r#SoulTorch::ID,
            Self::Glowstone(_) => r#Glowstone::ID,
            Self::InfestedStone(_) => r#InfestedStone::ID,
            Self::InfestedCobblestone(_) => r#InfestedCobblestone::ID,
            Self::InfestedStoneBricks(_) => r#InfestedStoneBricks::ID,
            Self::InfestedMossyStoneBricks(_) => r#InfestedMossyStoneBricks::ID,
            Self::InfestedCrackedStoneBricks(_) => r#InfestedCrackedStoneBricks::ID,
            Self::InfestedChiseledStoneBricks(_) => r#InfestedChiseledStoneBricks::ID,
            Self::InfestedDeepslate(_) => r#InfestedDeepslate::ID,
            Self::StoneBricks(_) => r#StoneBricks::ID,
            Self::MossyStoneBricks(_) => r#MossyStoneBricks::ID,
            Self::CrackedStoneBricks(_) => r#CrackedStoneBricks::ID,
            Self::ChiseledStoneBricks(_) => r#ChiseledStoneBricks::ID,
            Self::PackedMud(_) => r#PackedMud::ID,
            Self::MudBricks(_) => r#MudBricks::ID,
            Self::DeepslateBricks(_) => r#DeepslateBricks::ID,
            Self::CrackedDeepslateBricks(_) => r#CrackedDeepslateBricks::ID,
            Self::DeepslateTiles(_) => r#DeepslateTiles::ID,
            Self::CrackedDeepslateTiles(_) => r#CrackedDeepslateTiles::ID,
            Self::ChiseledDeepslate(_) => r#ChiseledDeepslate::ID,
            Self::ReinforcedDeepslate(_) => r#ReinforcedDeepslate::ID,
            Self::BrownMushroomBlock(_) => r#BrownMushroomBlock::ID,
            Self::RedMushroomBlock(_) => r#RedMushroomBlock::ID,
            Self::MushroomStem(_) => r#MushroomStem::ID,
            Self::IronBars(_) => r#IronBars::ID,
            Self::Chain(_) => r#Chain::ID,
            Self::GlassPane(_) => r#GlassPane::ID,
            Self::Melon(_) => r#Melon::ID,
            Self::Vine(_) => r#Vine::ID,
            Self::GlowLichen(_) => r#GlowLichen::ID,
            Self::BrickStairs(_) => r#BrickStairs::ID,
            Self::StoneBrickStairs(_) => r#StoneBrickStairs::ID,
            Self::MudBrickStairs(_) => r#MudBrickStairs::ID,
            Self::Mycelium(_) => r#Mycelium::ID,
            Self::LilyPad(_) => r#LilyPad::ID,
            Self::NetherBricks(_) => r#NetherBricks::ID,
            Self::CrackedNetherBricks(_) => r#CrackedNetherBricks::ID,
            Self::ChiseledNetherBricks(_) => r#ChiseledNetherBricks::ID,
            Self::NetherBrickFence(_) => r#NetherBrickFence::ID,
            Self::NetherBrickStairs(_) => r#NetherBrickStairs::ID,
            Self::Sculk(_) => r#Sculk::ID,
            Self::SculkVein(_) => r#SculkVein::ID,
            Self::SculkCatalyst(_) => r#SculkCatalyst::ID,
            Self::SculkShrieker(_) => r#SculkShrieker::ID,
            Self::EnchantingTable(_) => r#EnchantingTable::ID,
            Self::EndPortalFrame(_) => r#EndPortalFrame::ID,
            Self::EndStone(_) => r#EndStone::ID,
            Self::EndStoneBricks(_) => r#EndStoneBricks::ID,
            Self::DragonEgg(_) => r#DragonEgg::ID,
            Self::SandstoneStairs(_) => r#SandstoneStairs::ID,
            Self::EnderChest(_) => r#EnderChest::ID,
            Self::EmeraldBlock(_) => r#EmeraldBlock::ID,
            Self::OakStairs(_) => r#OakStairs::ID,
            Self::SpruceStairs(_) => r#SpruceStairs::ID,
            Self::BirchStairs(_) => r#BirchStairs::ID,
            Self::JungleStairs(_) => r#JungleStairs::ID,
            Self::AcaciaStairs(_) => r#AcaciaStairs::ID,
            Self::DarkOakStairs(_) => r#DarkOakStairs::ID,
            Self::MangroveStairs(_) => r#MangroveStairs::ID,
            Self::BambooStairs(_) => r#BambooStairs::ID,
            Self::BambooMosaicStairs(_) => r#BambooMosaicStairs::ID,
            Self::CrimsonStairs(_) => r#CrimsonStairs::ID,
            Self::WarpedStairs(_) => r#WarpedStairs::ID,
            Self::CommandBlock(_) => r#CommandBlock::ID,
            Self::Beacon(_) => r#Beacon::ID,
            Self::CobblestoneWall(_) => r#CobblestoneWall::ID,
            Self::MossyCobblestoneWall(_) => r#MossyCobblestoneWall::ID,
            Self::BrickWall(_) => r#BrickWall::ID,
            Self::PrismarineWall(_) => r#PrismarineWall::ID,
            Self::RedSandstoneWall(_) => r#RedSandstoneWall::ID,
            Self::MossyStoneBrickWall(_) => r#MossyStoneBrickWall::ID,
            Self::GraniteWall(_) => r#GraniteWall::ID,
            Self::StoneBrickWall(_) => r#StoneBrickWall::ID,
            Self::MudBrickWall(_) => r#MudBrickWall::ID,
            Self::NetherBrickWall(_) => r#NetherBrickWall::ID,
            Self::AndesiteWall(_) => r#AndesiteWall::ID,
            Self::RedNetherBrickWall(_) => r#RedNetherBrickWall::ID,
            Self::SandstoneWall(_) => r#SandstoneWall::ID,
            Self::EndStoneBrickWall(_) => r#EndStoneBrickWall::ID,
            Self::DioriteWall(_) => r#DioriteWall::ID,
            Self::BlackstoneWall(_) => r#BlackstoneWall::ID,
            Self::PolishedBlackstoneWall(_) => r#PolishedBlackstoneWall::ID,
            Self::PolishedBlackstoneBrickWall(_) => r#PolishedBlackstoneBrickWall::ID,
            Self::CobbledDeepslateWall(_) => r#CobbledDeepslateWall::ID,
            Self::PolishedDeepslateWall(_) => r#PolishedDeepslateWall::ID,
            Self::DeepslateBrickWall(_) => r#DeepslateBrickWall::ID,
            Self::DeepslateTileWall(_) => r#DeepslateTileWall::ID,
            Self::Anvil(_) => r#Anvil::ID,
            Self::ChippedAnvil(_) => r#ChippedAnvil::ID,
            Self::DamagedAnvil(_) => r#DamagedAnvil::ID,
            Self::ChiseledQuartzBlock(_) => r#ChiseledQuartzBlock::ID,
            Self::QuartzBlock(_) => r#QuartzBlock::ID,
            Self::QuartzBricks(_) => r#QuartzBricks::ID,
            Self::QuartzPillar(_) => r#QuartzPillar::ID,
            Self::QuartzStairs(_) => r#QuartzStairs::ID,
            Self::WhiteTerracotta(_) => r#WhiteTerracotta::ID,
            Self::OrangeTerracotta(_) => r#OrangeTerracotta::ID,
            Self::MagentaTerracotta(_) => r#MagentaTerracotta::ID,
            Self::LightBlueTerracotta(_) => r#LightBlueTerracotta::ID,
            Self::YellowTerracotta(_) => r#YellowTerracotta::ID,
            Self::LimeTerracotta(_) => r#LimeTerracotta::ID,
            Self::PinkTerracotta(_) => r#PinkTerracotta::ID,
            Self::GrayTerracotta(_) => r#GrayTerracotta::ID,
            Self::LightGrayTerracotta(_) => r#LightGrayTerracotta::ID,
            Self::CyanTerracotta(_) => r#CyanTerracotta::ID,
            Self::PurpleTerracotta(_) => r#PurpleTerracotta::ID,
            Self::BlueTerracotta(_) => r#BlueTerracotta::ID,
            Self::BrownTerracotta(_) => r#BrownTerracotta::ID,
            Self::GreenTerracotta(_) => r#GreenTerracotta::ID,
            Self::RedTerracotta(_) => r#RedTerracotta::ID,
            Self::BlackTerracotta(_) => r#BlackTerracotta::ID,
            Self::Barrier(_) => r#Barrier::ID,
            Self::Light(_) => r#Light::ID,
            Self::HayBlock(_) => r#HayBlock::ID,
            Self::WhiteCarpet(_) => r#WhiteCarpet::ID,
            Self::OrangeCarpet(_) => r#OrangeCarpet::ID,
            Self::MagentaCarpet(_) => r#MagentaCarpet::ID,
            Self::LightBlueCarpet(_) => r#LightBlueCarpet::ID,
            Self::YellowCarpet(_) => r#YellowCarpet::ID,
            Self::LimeCarpet(_) => r#LimeCarpet::ID,
            Self::PinkCarpet(_) => r#PinkCarpet::ID,
            Self::GrayCarpet(_) => r#GrayCarpet::ID,
            Self::LightGrayCarpet(_) => r#LightGrayCarpet::ID,
            Self::CyanCarpet(_) => r#CyanCarpet::ID,
            Self::PurpleCarpet(_) => r#PurpleCarpet::ID,
            Self::BlueCarpet(_) => r#BlueCarpet::ID,
            Self::BrownCarpet(_) => r#BrownCarpet::ID,
            Self::GreenCarpet(_) => r#GreenCarpet::ID,
            Self::RedCarpet(_) => r#RedCarpet::ID,
            Self::BlackCarpet(_) => r#BlackCarpet::ID,
            Self::Terracotta(_) => r#Terracotta::ID,
            Self::PackedIce(_) => r#PackedIce::ID,
            Self::DirtPath(_) => r#DirtPath::ID,
            Self::Sunflower(_) => r#Sunflower::ID,
            Self::Lilac(_) => r#Lilac::ID,
            Self::RoseBush(_) => r#RoseBush::ID,
            Self::Peony(_) => r#Peony::ID,
            Self::TallGrass(_) => r#TallGrass::ID,
            Self::LargeFern(_) => r#LargeFern::ID,
            Self::WhiteStainedGlass(_) => r#WhiteStainedGlass::ID,
            Self::OrangeStainedGlass(_) => r#OrangeStainedGlass::ID,
            Self::MagentaStainedGlass(_) => r#MagentaStainedGlass::ID,
            Self::LightBlueStainedGlass(_) => r#LightBlueStainedGlass::ID,
            Self::YellowStainedGlass(_) => r#YellowStainedGlass::ID,
            Self::LimeStainedGlass(_) => r#LimeStainedGlass::ID,
            Self::PinkStainedGlass(_) => r#PinkStainedGlass::ID,
            Self::GrayStainedGlass(_) => r#GrayStainedGlass::ID,
            Self::LightGrayStainedGlass(_) => r#LightGrayStainedGlass::ID,
            Self::CyanStainedGlass(_) => r#CyanStainedGlass::ID,
            Self::PurpleStainedGlass(_) => r#PurpleStainedGlass::ID,
            Self::BlueStainedGlass(_) => r#BlueStainedGlass::ID,
            Self::BrownStainedGlass(_) => r#BrownStainedGlass::ID,
            Self::GreenStainedGlass(_) => r#GreenStainedGlass::ID,
            Self::RedStainedGlass(_) => r#RedStainedGlass::ID,
            Self::BlackStainedGlass(_) => r#BlackStainedGlass::ID,
            Self::WhiteStainedGlassPane(_) => r#WhiteStainedGlassPane::ID,
            Self::OrangeStainedGlassPane(_) => r#OrangeStainedGlassPane::ID,
            Self::MagentaStainedGlassPane(_) => r#MagentaStainedGlassPane::ID,
            Self::LightBlueStainedGlassPane(_) => r#LightBlueStainedGlassPane::ID,
            Self::YellowStainedGlassPane(_) => r#YellowStainedGlassPane::ID,
            Self::LimeStainedGlassPane(_) => r#LimeStainedGlassPane::ID,
            Self::PinkStainedGlassPane(_) => r#PinkStainedGlassPane::ID,
            Self::GrayStainedGlassPane(_) => r#GrayStainedGlassPane::ID,
            Self::LightGrayStainedGlassPane(_) => r#LightGrayStainedGlassPane::ID,
            Self::CyanStainedGlassPane(_) => r#CyanStainedGlassPane::ID,
            Self::PurpleStainedGlassPane(_) => r#PurpleStainedGlassPane::ID,
            Self::BlueStainedGlassPane(_) => r#BlueStainedGlassPane::ID,
            Self::BrownStainedGlassPane(_) => r#BrownStainedGlassPane::ID,
            Self::GreenStainedGlassPane(_) => r#GreenStainedGlassPane::ID,
            Self::RedStainedGlassPane(_) => r#RedStainedGlassPane::ID,
            Self::BlackStainedGlassPane(_) => r#BlackStainedGlassPane::ID,
            Self::Prismarine(_) => r#Prismarine::ID,
            Self::PrismarineBricks(_) => r#PrismarineBricks::ID,
            Self::DarkPrismarine(_) => r#DarkPrismarine::ID,
            Self::PrismarineStairs(_) => r#PrismarineStairs::ID,
            Self::PrismarineBrickStairs(_) => r#PrismarineBrickStairs::ID,
            Self::DarkPrismarineStairs(_) => r#DarkPrismarineStairs::ID,
            Self::SeaLantern(_) => r#SeaLantern::ID,
            Self::RedSandstone(_) => r#RedSandstone::ID,
            Self::ChiseledRedSandstone(_) => r#ChiseledRedSandstone::ID,
            Self::CutRedSandstone(_) => r#CutRedSandstone::ID,
            Self::RedSandstoneStairs(_) => r#RedSandstoneStairs::ID,
            Self::RepeatingCommandBlock(_) => r#RepeatingCommandBlock::ID,
            Self::ChainCommandBlock(_) => r#ChainCommandBlock::ID,
            Self::MagmaBlock(_) => r#MagmaBlock::ID,
            Self::NetherWartBlock(_) => r#NetherWartBlock::ID,
            Self::WarpedWartBlock(_) => r#WarpedWartBlock::ID,
            Self::RedNetherBricks(_) => r#RedNetherBricks::ID,
            Self::BoneBlock(_) => r#BoneBlock::ID,
            Self::StructureVoid(_) => r#StructureVoid::ID,
            Self::ShulkerBox(_) => r#ShulkerBox::ID,
            Self::WhiteShulkerBox(_) => r#WhiteShulkerBox::ID,
            Self::OrangeShulkerBox(_) => r#OrangeShulkerBox::ID,
            Self::MagentaShulkerBox(_) => r#MagentaShulkerBox::ID,
            Self::LightBlueShulkerBox(_) => r#LightBlueShulkerBox::ID,
            Self::YellowShulkerBox(_) => r#YellowShulkerBox::ID,
            Self::LimeShulkerBox(_) => r#LimeShulkerBox::ID,
            Self::PinkShulkerBox(_) => r#PinkShulkerBox::ID,
            Self::GrayShulkerBox(_) => r#GrayShulkerBox::ID,
            Self::LightGrayShulkerBox(_) => r#LightGrayShulkerBox::ID,
            Self::CyanShulkerBox(_) => r#CyanShulkerBox::ID,
            Self::PurpleShulkerBox(_) => r#PurpleShulkerBox::ID,
            Self::BlueShulkerBox(_) => r#BlueShulkerBox::ID,
            Self::BrownShulkerBox(_) => r#BrownShulkerBox::ID,
            Self::GreenShulkerBox(_) => r#GreenShulkerBox::ID,
            Self::RedShulkerBox(_) => r#RedShulkerBox::ID,
            Self::BlackShulkerBox(_) => r#BlackShulkerBox::ID,
            Self::WhiteGlazedTerracotta(_) => r#WhiteGlazedTerracotta::ID,
            Self::OrangeGlazedTerracotta(_) => r#OrangeGlazedTerracotta::ID,
            Self::MagentaGlazedTerracotta(_) => r#MagentaGlazedTerracotta::ID,
            Self::LightBlueGlazedTerracotta(_) => r#LightBlueGlazedTerracotta::ID,
            Self::YellowGlazedTerracotta(_) => r#YellowGlazedTerracotta::ID,
            Self::LimeGlazedTerracotta(_) => r#LimeGlazedTerracotta::ID,
            Self::PinkGlazedTerracotta(_) => r#PinkGlazedTerracotta::ID,
            Self::GrayGlazedTerracotta(_) => r#GrayGlazedTerracotta::ID,
            Self::LightGrayGlazedTerracotta(_) => r#LightGrayGlazedTerracotta::ID,
            Self::CyanGlazedTerracotta(_) => r#CyanGlazedTerracotta::ID,
            Self::PurpleGlazedTerracotta(_) => r#PurpleGlazedTerracotta::ID,
            Self::BlueGlazedTerracotta(_) => r#BlueGlazedTerracotta::ID,
            Self::BrownGlazedTerracotta(_) => r#BrownGlazedTerracotta::ID,
            Self::GreenGlazedTerracotta(_) => r#GreenGlazedTerracotta::ID,
            Self::RedGlazedTerracotta(_) => r#RedGlazedTerracotta::ID,
            Self::BlackGlazedTerracotta(_) => r#BlackGlazedTerracotta::ID,
            Self::WhiteConcrete(_) => r#WhiteConcrete::ID,
            Self::OrangeConcrete(_) => r#OrangeConcrete::ID,
            Self::MagentaConcrete(_) => r#MagentaConcrete::ID,
            Self::LightBlueConcrete(_) => r#LightBlueConcrete::ID,
            Self::YellowConcrete(_) => r#YellowConcrete::ID,
            Self::LimeConcrete(_) => r#LimeConcrete::ID,
            Self::PinkConcrete(_) => r#PinkConcrete::ID,
            Self::GrayConcrete(_) => r#GrayConcrete::ID,
            Self::LightGrayConcrete(_) => r#LightGrayConcrete::ID,
            Self::CyanConcrete(_) => r#CyanConcrete::ID,
            Self::PurpleConcrete(_) => r#PurpleConcrete::ID,
            Self::BlueConcrete(_) => r#BlueConcrete::ID,
            Self::BrownConcrete(_) => r#BrownConcrete::ID,
            Self::GreenConcrete(_) => r#GreenConcrete::ID,
            Self::RedConcrete(_) => r#RedConcrete::ID,
            Self::BlackConcrete(_) => r#BlackConcrete::ID,
            Self::WhiteConcretePowder(_) => r#WhiteConcretePowder::ID,
            Self::OrangeConcretePowder(_) => r#OrangeConcretePowder::ID,
            Self::MagentaConcretePowder(_) => r#MagentaConcretePowder::ID,
            Self::LightBlueConcretePowder(_) => r#LightBlueConcretePowder::ID,
            Self::YellowConcretePowder(_) => r#YellowConcretePowder::ID,
            Self::LimeConcretePowder(_) => r#LimeConcretePowder::ID,
            Self::PinkConcretePowder(_) => r#PinkConcretePowder::ID,
            Self::GrayConcretePowder(_) => r#GrayConcretePowder::ID,
            Self::LightGrayConcretePowder(_) => r#LightGrayConcretePowder::ID,
            Self::CyanConcretePowder(_) => r#CyanConcretePowder::ID,
            Self::PurpleConcretePowder(_) => r#PurpleConcretePowder::ID,
            Self::BlueConcretePowder(_) => r#BlueConcretePowder::ID,
            Self::BrownConcretePowder(_) => r#BrownConcretePowder::ID,
            Self::GreenConcretePowder(_) => r#GreenConcretePowder::ID,
            Self::RedConcretePowder(_) => r#RedConcretePowder::ID,
            Self::BlackConcretePowder(_) => r#BlackConcretePowder::ID,
            Self::TurtleEgg(_) => r#TurtleEgg::ID,
            Self::DeadTubeCoralBlock(_) => r#DeadTubeCoralBlock::ID,
            Self::DeadBrainCoralBlock(_) => r#DeadBrainCoralBlock::ID,
            Self::DeadBubbleCoralBlock(_) => r#DeadBubbleCoralBlock::ID,
            Self::DeadFireCoralBlock(_) => r#DeadFireCoralBlock::ID,
            Self::DeadHornCoralBlock(_) => r#DeadHornCoralBlock::ID,
            Self::TubeCoralBlock(_) => r#TubeCoralBlock::ID,
            Self::BrainCoralBlock(_) => r#BrainCoralBlock::ID,
            Self::BubbleCoralBlock(_) => r#BubbleCoralBlock::ID,
            Self::FireCoralBlock(_) => r#FireCoralBlock::ID,
            Self::HornCoralBlock(_) => r#HornCoralBlock::ID,
            Self::TubeCoral(_) => r#TubeCoral::ID,
            Self::BrainCoral(_) => r#BrainCoral::ID,
            Self::BubbleCoral(_) => r#BubbleCoral::ID,
            Self::FireCoral(_) => r#FireCoral::ID,
            Self::HornCoral(_) => r#HornCoral::ID,
            Self::DeadBrainCoral(_) => r#DeadBrainCoral::ID,
            Self::DeadBubbleCoral(_) => r#DeadBubbleCoral::ID,
            Self::DeadFireCoral(_) => r#DeadFireCoral::ID,
            Self::DeadHornCoral(_) => r#DeadHornCoral::ID,
            Self::DeadTubeCoral(_) => r#DeadTubeCoral::ID,
            Self::TubeCoralFan(_) => r#TubeCoralFan::ID,
            Self::BrainCoralFan(_) => r#BrainCoralFan::ID,
            Self::BubbleCoralFan(_) => r#BubbleCoralFan::ID,
            Self::FireCoralFan(_) => r#FireCoralFan::ID,
            Self::HornCoralFan(_) => r#HornCoralFan::ID,
            Self::DeadTubeCoralFan(_) => r#DeadTubeCoralFan::ID,
            Self::DeadBrainCoralFan(_) => r#DeadBrainCoralFan::ID,
            Self::DeadBubbleCoralFan(_) => r#DeadBubbleCoralFan::ID,
            Self::DeadFireCoralFan(_) => r#DeadFireCoralFan::ID,
            Self::DeadHornCoralFan(_) => r#DeadHornCoralFan::ID,
            Self::BlueIce(_) => r#BlueIce::ID,
            Self::Conduit(_) => r#Conduit::ID,
            Self::PolishedGraniteStairs(_) => r#PolishedGraniteStairs::ID,
            Self::SmoothRedSandstoneStairs(_) => r#SmoothRedSandstoneStairs::ID,
            Self::MossyStoneBrickStairs(_) => r#MossyStoneBrickStairs::ID,
            Self::PolishedDioriteStairs(_) => r#PolishedDioriteStairs::ID,
            Self::MossyCobblestoneStairs(_) => r#MossyCobblestoneStairs::ID,
            Self::EndStoneBrickStairs(_) => r#EndStoneBrickStairs::ID,
            Self::StoneStairs(_) => r#StoneStairs::ID,
            Self::SmoothSandstoneStairs(_) => r#SmoothSandstoneStairs::ID,
            Self::SmoothQuartzStairs(_) => r#SmoothQuartzStairs::ID,
            Self::GraniteStairs(_) => r#GraniteStairs::ID,
            Self::AndesiteStairs(_) => r#AndesiteStairs::ID,
            Self::RedNetherBrickStairs(_) => r#RedNetherBrickStairs::ID,
            Self::PolishedAndesiteStairs(_) => r#PolishedAndesiteStairs::ID,
            Self::DioriteStairs(_) => r#DioriteStairs::ID,
            Self::CobbledDeepslateStairs(_) => r#CobbledDeepslateStairs::ID,
            Self::PolishedDeepslateStairs(_) => r#PolishedDeepslateStairs::ID,
            Self::DeepslateBrickStairs(_) => r#DeepslateBrickStairs::ID,
            Self::DeepslateTileStairs(_) => r#DeepslateTileStairs::ID,
            Self::PolishedGraniteSlab(_) => r#PolishedGraniteSlab::ID,
            Self::SmoothRedSandstoneSlab(_) => r#SmoothRedSandstoneSlab::ID,
            Self::MossyStoneBrickSlab(_) => r#MossyStoneBrickSlab::ID,
            Self::PolishedDioriteSlab(_) => r#PolishedDioriteSlab::ID,
            Self::MossyCobblestoneSlab(_) => r#MossyCobblestoneSlab::ID,
            Self::EndStoneBrickSlab(_) => r#EndStoneBrickSlab::ID,
            Self::SmoothSandstoneSlab(_) => r#SmoothSandstoneSlab::ID,
            Self::SmoothQuartzSlab(_) => r#SmoothQuartzSlab::ID,
            Self::GraniteSlab(_) => r#GraniteSlab::ID,
            Self::AndesiteSlab(_) => r#AndesiteSlab::ID,
            Self::RedNetherBrickSlab(_) => r#RedNetherBrickSlab::ID,
            Self::PolishedAndesiteSlab(_) => r#PolishedAndesiteSlab::ID,
            Self::DioriteSlab(_) => r#DioriteSlab::ID,
            Self::CobbledDeepslateSlab(_) => r#CobbledDeepslateSlab::ID,
            Self::PolishedDeepslateSlab(_) => r#PolishedDeepslateSlab::ID,
            Self::DeepslateBrickSlab(_) => r#DeepslateBrickSlab::ID,
            Self::DeepslateTileSlab(_) => r#DeepslateTileSlab::ID,
            Self::Scaffolding(_) => r#Scaffolding::ID,
            Self::Redstone(_) => r#Redstone::ID,
            Self::RedstoneTorch(_) => r#RedstoneTorch::ID,
            Self::RedstoneBlock(_) => r#RedstoneBlock::ID,
            Self::Repeater(_) => r#Repeater::ID,
            Self::Comparator(_) => r#Comparator::ID,
            Self::Piston(_) => r#Piston::ID,
            Self::StickyPiston(_) => r#StickyPiston::ID,
            Self::SlimeBlock(_) => r#SlimeBlock::ID,
            Self::HoneyBlock(_) => r#HoneyBlock::ID,
            Self::Observer(_) => r#Observer::ID,
            Self::Hopper(_) => r#Hopper::ID,
            Self::Dispenser(_) => r#Dispenser::ID,
            Self::Dropper(_) => r#Dropper::ID,
            Self::Lectern(_) => r#Lectern::ID,
            Self::Target(_) => r#Target::ID,
            Self::Lever(_) => r#Lever::ID,
            Self::LightningRod(_) => r#LightningRod::ID,
            Self::DaylightDetector(_) => r#DaylightDetector::ID,
            Self::SculkSensor(_) => r#SculkSensor::ID,
            Self::TripwireHook(_) => r#TripwireHook::ID,
            Self::TrappedChest(_) => r#TrappedChest::ID,
            Self::Tnt(_) => r#Tnt::ID,
            Self::RedstoneLamp(_) => r#RedstoneLamp::ID,
            Self::NoteBlock(_) => r#NoteBlock::ID,
            Self::StoneButton(_) => r#StoneButton::ID,
            Self::PolishedBlackstoneButton(_) => r#PolishedBlackstoneButton::ID,
            Self::OakButton(_) => r#OakButton::ID,
            Self::SpruceButton(_) => r#SpruceButton::ID,
            Self::BirchButton(_) => r#BirchButton::ID,
            Self::JungleButton(_) => r#JungleButton::ID,
            Self::AcaciaButton(_) => r#AcaciaButton::ID,
            Self::DarkOakButton(_) => r#DarkOakButton::ID,
            Self::MangroveButton(_) => r#MangroveButton::ID,
            Self::BambooButton(_) => r#BambooButton::ID,
            Self::CrimsonButton(_) => r#CrimsonButton::ID,
            Self::WarpedButton(_) => r#WarpedButton::ID,
            Self::StonePressurePlate(_) => r#StonePressurePlate::ID,
            Self::PolishedBlackstonePressurePlate(_) => r#PolishedBlackstonePressurePlate::ID,
            Self::LightWeightedPressurePlate(_) => r#LightWeightedPressurePlate::ID,
            Self::HeavyWeightedPressurePlate(_) => r#HeavyWeightedPressurePlate::ID,
            Self::OakPressurePlate(_) => r#OakPressurePlate::ID,
            Self::SprucePressurePlate(_) => r#SprucePressurePlate::ID,
            Self::BirchPressurePlate(_) => r#BirchPressurePlate::ID,
            Self::JunglePressurePlate(_) => r#JunglePressurePlate::ID,
            Self::AcaciaPressurePlate(_) => r#AcaciaPressurePlate::ID,
            Self::DarkOakPressurePlate(_) => r#DarkOakPressurePlate::ID,
            Self::MangrovePressurePlate(_) => r#MangrovePressurePlate::ID,
            Self::BambooPressurePlate(_) => r#BambooPressurePlate::ID,
            Self::CrimsonPressurePlate(_) => r#CrimsonPressurePlate::ID,
            Self::WarpedPressurePlate(_) => r#WarpedPressurePlate::ID,
            Self::IronDoor(_) => r#IronDoor::ID,
            Self::OakDoor(_) => r#OakDoor::ID,
            Self::SpruceDoor(_) => r#SpruceDoor::ID,
            Self::BirchDoor(_) => r#BirchDoor::ID,
            Self::JungleDoor(_) => r#JungleDoor::ID,
            Self::AcaciaDoor(_) => r#AcaciaDoor::ID,
            Self::DarkOakDoor(_) => r#DarkOakDoor::ID,
            Self::MangroveDoor(_) => r#MangroveDoor::ID,
            Self::BambooDoor(_) => r#BambooDoor::ID,
            Self::CrimsonDoor(_) => r#CrimsonDoor::ID,
            Self::WarpedDoor(_) => r#WarpedDoor::ID,
            Self::IronTrapdoor(_) => r#IronTrapdoor::ID,
            Self::OakTrapdoor(_) => r#OakTrapdoor::ID,
            Self::SpruceTrapdoor(_) => r#SpruceTrapdoor::ID,
            Self::BirchTrapdoor(_) => r#BirchTrapdoor::ID,
            Self::JungleTrapdoor(_) => r#JungleTrapdoor::ID,
            Self::AcaciaTrapdoor(_) => r#AcaciaTrapdoor::ID,
            Self::DarkOakTrapdoor(_) => r#DarkOakTrapdoor::ID,
            Self::MangroveTrapdoor(_) => r#MangroveTrapdoor::ID,
            Self::BambooTrapdoor(_) => r#BambooTrapdoor::ID,
            Self::CrimsonTrapdoor(_) => r#CrimsonTrapdoor::ID,
            Self::WarpedTrapdoor(_) => r#WarpedTrapdoor::ID,
            Self::OakFenceGate(_) => r#OakFenceGate::ID,
            Self::SpruceFenceGate(_) => r#SpruceFenceGate::ID,
            Self::BirchFenceGate(_) => r#BirchFenceGate::ID,
            Self::JungleFenceGate(_) => r#JungleFenceGate::ID,
            Self::AcaciaFenceGate(_) => r#AcaciaFenceGate::ID,
            Self::DarkOakFenceGate(_) => r#DarkOakFenceGate::ID,
            Self::MangroveFenceGate(_) => r#MangroveFenceGate::ID,
            Self::BambooFenceGate(_) => r#BambooFenceGate::ID,
            Self::CrimsonFenceGate(_) => r#CrimsonFenceGate::ID,
            Self::WarpedFenceGate(_) => r#WarpedFenceGate::ID,
            Self::PoweredRail(_) => r#PoweredRail::ID,
            Self::DetectorRail(_) => r#DetectorRail::ID,
            Self::Rail(_) => r#Rail::ID,
            Self::ActivatorRail(_) => r#ActivatorRail::ID,
            Self::Saddle(_) => r#Saddle::ID,
            Self::Minecart(_) => r#Minecart::ID,
            Self::ChestMinecart(_) => r#ChestMinecart::ID,
            Self::FurnaceMinecart(_) => r#FurnaceMinecart::ID,
            Self::TntMinecart(_) => r#TntMinecart::ID,
            Self::HopperMinecart(_) => r#HopperMinecart::ID,
            Self::CarrotOnAStick(_) => r#CarrotOnAStick::ID,
            Self::WarpedFungusOnAStick(_) => r#WarpedFungusOnAStick::ID,
            Self::Elytra(_) => r#Elytra::ID,
            Self::OakBoat(_) => r#OakBoat::ID,
            Self::OakChestBoat(_) => r#OakChestBoat::ID,
            Self::SpruceBoat(_) => r#SpruceBoat::ID,
            Self::SpruceChestBoat(_) => r#SpruceChestBoat::ID,
            Self::BirchBoat(_) => r#BirchBoat::ID,
            Self::BirchChestBoat(_) => r#BirchChestBoat::ID,
            Self::JungleBoat(_) => r#JungleBoat::ID,
            Self::JungleChestBoat(_) => r#JungleChestBoat::ID,
            Self::AcaciaBoat(_) => r#AcaciaBoat::ID,
            Self::AcaciaChestBoat(_) => r#AcaciaChestBoat::ID,
            Self::DarkOakBoat(_) => r#DarkOakBoat::ID,
            Self::DarkOakChestBoat(_) => r#DarkOakChestBoat::ID,
            Self::MangroveBoat(_) => r#MangroveBoat::ID,
            Self::MangroveChestBoat(_) => r#MangroveChestBoat::ID,
            Self::BambooRaft(_) => r#BambooRaft::ID,
            Self::BambooChestRaft(_) => r#BambooChestRaft::ID,
            Self::StructureBlock(_) => r#StructureBlock::ID,
            Self::Jigsaw(_) => r#Jigsaw::ID,
            Self::TurtleHelmet(_) => r#TurtleHelmet::ID,
            Self::Scute(_) => r#Scute::ID,
            Self::FlintAndSteel(_) => r#FlintAndSteel::ID,
            Self::Apple(_) => r#Apple::ID,
            Self::Bow(_) => r#Bow::ID,
            Self::Arrow(_) => r#Arrow::ID,
            Self::Coal(_) => r#Coal::ID,
            Self::Charcoal(_) => r#Charcoal::ID,
            Self::Diamond(_) => r#Diamond::ID,
            Self::Emerald(_) => r#Emerald::ID,
            Self::LapisLazuli(_) => r#LapisLazuli::ID,
            Self::Quartz(_) => r#Quartz::ID,
            Self::AmethystShard(_) => r#AmethystShard::ID,
            Self::RawIron(_) => r#RawIron::ID,
            Self::IronIngot(_) => r#IronIngot::ID,
            Self::RawCopper(_) => r#RawCopper::ID,
            Self::CopperIngot(_) => r#CopperIngot::ID,
            Self::RawGold(_) => r#RawGold::ID,
            Self::GoldIngot(_) => r#GoldIngot::ID,
            Self::NetheriteIngot(_) => r#NetheriteIngot::ID,
            Self::NetheriteScrap(_) => r#NetheriteScrap::ID,
            Self::WoodenSword(_) => r#WoodenSword::ID,
            Self::WoodenShovel(_) => r#WoodenShovel::ID,
            Self::WoodenPickaxe(_) => r#WoodenPickaxe::ID,
            Self::WoodenAxe(_) => r#WoodenAxe::ID,
            Self::WoodenHoe(_) => r#WoodenHoe::ID,
            Self::StoneSword(_) => r#StoneSword::ID,
            Self::StoneShovel(_) => r#StoneShovel::ID,
            Self::StonePickaxe(_) => r#StonePickaxe::ID,
            Self::StoneAxe(_) => r#StoneAxe::ID,
            Self::StoneHoe(_) => r#StoneHoe::ID,
            Self::GoldenSword(_) => r#GoldenSword::ID,
            Self::GoldenShovel(_) => r#GoldenShovel::ID,
            Self::GoldenPickaxe(_) => r#GoldenPickaxe::ID,
            Self::GoldenAxe(_) => r#GoldenAxe::ID,
            Self::GoldenHoe(_) => r#GoldenHoe::ID,
            Self::IronSword(_) => r#IronSword::ID,
            Self::IronShovel(_) => r#IronShovel::ID,
            Self::IronPickaxe(_) => r#IronPickaxe::ID,
            Self::IronAxe(_) => r#IronAxe::ID,
            Self::IronHoe(_) => r#IronHoe::ID,
            Self::DiamondSword(_) => r#DiamondSword::ID,
            Self::DiamondShovel(_) => r#DiamondShovel::ID,
            Self::DiamondPickaxe(_) => r#DiamondPickaxe::ID,
            Self::DiamondAxe(_) => r#DiamondAxe::ID,
            Self::DiamondHoe(_) => r#DiamondHoe::ID,
            Self::NetheriteSword(_) => r#NetheriteSword::ID,
            Self::NetheriteShovel(_) => r#NetheriteShovel::ID,
            Self::NetheritePickaxe(_) => r#NetheritePickaxe::ID,
            Self::NetheriteAxe(_) => r#NetheriteAxe::ID,
            Self::NetheriteHoe(_) => r#NetheriteHoe::ID,
            Self::Stick(_) => r#Stick::ID,
            Self::Bowl(_) => r#Bowl::ID,
            Self::MushroomStew(_) => r#MushroomStew::ID,
            Self::String(_) => r#String::ID,
            Self::Feather(_) => r#Feather::ID,
            Self::Gunpowder(_) => r#Gunpowder::ID,
            Self::WheatSeeds(_) => r#WheatSeeds::ID,
            Self::Wheat(_) => r#Wheat::ID,
            Self::Bread(_) => r#Bread::ID,
            Self::LeatherHelmet(_) => r#LeatherHelmet::ID,
            Self::LeatherChestplate(_) => r#LeatherChestplate::ID,
            Self::LeatherLeggings(_) => r#LeatherLeggings::ID,
            Self::LeatherBoots(_) => r#LeatherBoots::ID,
            Self::ChainmailHelmet(_) => r#ChainmailHelmet::ID,
            Self::ChainmailChestplate(_) => r#ChainmailChestplate::ID,
            Self::ChainmailLeggings(_) => r#ChainmailLeggings::ID,
            Self::ChainmailBoots(_) => r#ChainmailBoots::ID,
            Self::IronHelmet(_) => r#IronHelmet::ID,
            Self::IronChestplate(_) => r#IronChestplate::ID,
            Self::IronLeggings(_) => r#IronLeggings::ID,
            Self::IronBoots(_) => r#IronBoots::ID,
            Self::DiamondHelmet(_) => r#DiamondHelmet::ID,
            Self::DiamondChestplate(_) => r#DiamondChestplate::ID,
            Self::DiamondLeggings(_) => r#DiamondLeggings::ID,
            Self::DiamondBoots(_) => r#DiamondBoots::ID,
            Self::GoldenHelmet(_) => r#GoldenHelmet::ID,
            Self::GoldenChestplate(_) => r#GoldenChestplate::ID,
            Self::GoldenLeggings(_) => r#GoldenLeggings::ID,
            Self::GoldenBoots(_) => r#GoldenBoots::ID,
            Self::NetheriteHelmet(_) => r#NetheriteHelmet::ID,
            Self::NetheriteChestplate(_) => r#NetheriteChestplate::ID,
            Self::NetheriteLeggings(_) => r#NetheriteLeggings::ID,
            Self::NetheriteBoots(_) => r#NetheriteBoots::ID,
            Self::Flint(_) => r#Flint::ID,
            Self::Porkchop(_) => r#Porkchop::ID,
            Self::CookedPorkchop(_) => r#CookedPorkchop::ID,
            Self::Painting(_) => r#Painting::ID,
            Self::GoldenApple(_) => r#GoldenApple::ID,
            Self::EnchantedGoldenApple(_) => r#EnchantedGoldenApple::ID,
            Self::OakSign(_) => r#OakSign::ID,
            Self::SpruceSign(_) => r#SpruceSign::ID,
            Self::BirchSign(_) => r#BirchSign::ID,
            Self::JungleSign(_) => r#JungleSign::ID,
            Self::AcaciaSign(_) => r#AcaciaSign::ID,
            Self::DarkOakSign(_) => r#DarkOakSign::ID,
            Self::MangroveSign(_) => r#MangroveSign::ID,
            Self::BambooSign(_) => r#BambooSign::ID,
            Self::CrimsonSign(_) => r#CrimsonSign::ID,
            Self::WarpedSign(_) => r#WarpedSign::ID,
            Self::OakHangingSign(_) => r#OakHangingSign::ID,
            Self::SpruceHangingSign(_) => r#SpruceHangingSign::ID,
            Self::BirchHangingSign(_) => r#BirchHangingSign::ID,
            Self::JungleHangingSign(_) => r#JungleHangingSign::ID,
            Self::AcaciaHangingSign(_) => r#AcaciaHangingSign::ID,
            Self::DarkOakHangingSign(_) => r#DarkOakHangingSign::ID,
            Self::MangroveHangingSign(_) => r#MangroveHangingSign::ID,
            Self::BambooHangingSign(_) => r#BambooHangingSign::ID,
            Self::CrimsonHangingSign(_) => r#CrimsonHangingSign::ID,
            Self::WarpedHangingSign(_) => r#WarpedHangingSign::ID,
            Self::Bucket(_) => r#Bucket::ID,
            Self::WaterBucket(_) => r#WaterBucket::ID,
            Self::LavaBucket(_) => r#LavaBucket::ID,
            Self::PowderSnowBucket(_) => r#PowderSnowBucket::ID,
            Self::Snowball(_) => r#Snowball::ID,
            Self::Leather(_) => r#Leather::ID,
            Self::MilkBucket(_) => r#MilkBucket::ID,
            Self::PufferfishBucket(_) => r#PufferfishBucket::ID,
            Self::SalmonBucket(_) => r#SalmonBucket::ID,
            Self::CodBucket(_) => r#CodBucket::ID,
            Self::TropicalFishBucket(_) => r#TropicalFishBucket::ID,
            Self::AxolotlBucket(_) => r#AxolotlBucket::ID,
            Self::TadpoleBucket(_) => r#TadpoleBucket::ID,
            Self::Brick(_) => r#Brick::ID,
            Self::ClayBall(_) => r#ClayBall::ID,
            Self::DriedKelpBlock(_) => r#DriedKelpBlock::ID,
            Self::Paper(_) => r#Paper::ID,
            Self::Book(_) => r#Book::ID,
            Self::SlimeBall(_) => r#SlimeBall::ID,
            Self::Egg(_) => r#Egg::ID,
            Self::Compass(_) => r#Compass::ID,
            Self::RecoveryCompass(_) => r#RecoveryCompass::ID,
            Self::Bundle(_) => r#Bundle::ID,
            Self::FishingRod(_) => r#FishingRod::ID,
            Self::Clock(_) => r#Clock::ID,
            Self::Spyglass(_) => r#Spyglass::ID,
            Self::GlowstoneDust(_) => r#GlowstoneDust::ID,
            Self::Cod(_) => r#Cod::ID,
            Self::Salmon(_) => r#Salmon::ID,
            Self::TropicalFish(_) => r#TropicalFish::ID,
            Self::Pufferfish(_) => r#Pufferfish::ID,
            Self::CookedCod(_) => r#CookedCod::ID,
            Self::CookedSalmon(_) => r#CookedSalmon::ID,
            Self::InkSac(_) => r#InkSac::ID,
            Self::GlowInkSac(_) => r#GlowInkSac::ID,
            Self::CocoaBeans(_) => r#CocoaBeans::ID,
            Self::WhiteDye(_) => r#WhiteDye::ID,
            Self::OrangeDye(_) => r#OrangeDye::ID,
            Self::MagentaDye(_) => r#MagentaDye::ID,
            Self::LightBlueDye(_) => r#LightBlueDye::ID,
            Self::YellowDye(_) => r#YellowDye::ID,
            Self::LimeDye(_) => r#LimeDye::ID,
            Self::PinkDye(_) => r#PinkDye::ID,
            Self::GrayDye(_) => r#GrayDye::ID,
            Self::LightGrayDye(_) => r#LightGrayDye::ID,
            Self::CyanDye(_) => r#CyanDye::ID,
            Self::PurpleDye(_) => r#PurpleDye::ID,
            Self::BlueDye(_) => r#BlueDye::ID,
            Self::BrownDye(_) => r#BrownDye::ID,
            Self::GreenDye(_) => r#GreenDye::ID,
            Self::RedDye(_) => r#RedDye::ID,
            Self::BlackDye(_) => r#BlackDye::ID,
            Self::BoneMeal(_) => r#BoneMeal::ID,
            Self::Bone(_) => r#Bone::ID,
            Self::Sugar(_) => r#Sugar::ID,
            Self::Cake(_) => r#Cake::ID,
            Self::WhiteBed(_) => r#WhiteBed::ID,
            Self::OrangeBed(_) => r#OrangeBed::ID,
            Self::MagentaBed(_) => r#MagentaBed::ID,
            Self::LightBlueBed(_) => r#LightBlueBed::ID,
            Self::YellowBed(_) => r#YellowBed::ID,
            Self::LimeBed(_) => r#LimeBed::ID,
            Self::PinkBed(_) => r#PinkBed::ID,
            Self::GrayBed(_) => r#GrayBed::ID,
            Self::LightGrayBed(_) => r#LightGrayBed::ID,
            Self::CyanBed(_) => r#CyanBed::ID,
            Self::PurpleBed(_) => r#PurpleBed::ID,
            Self::BlueBed(_) => r#BlueBed::ID,
            Self::BrownBed(_) => r#BrownBed::ID,
            Self::GreenBed(_) => r#GreenBed::ID,
            Self::RedBed(_) => r#RedBed::ID,
            Self::BlackBed(_) => r#BlackBed::ID,
            Self::Cookie(_) => r#Cookie::ID,
            Self::FilledMap(_) => r#FilledMap::ID,
            Self::Shears(_) => r#Shears::ID,
            Self::MelonSlice(_) => r#MelonSlice::ID,
            Self::DriedKelp(_) => r#DriedKelp::ID,
            Self::PumpkinSeeds(_) => r#PumpkinSeeds::ID,
            Self::MelonSeeds(_) => r#MelonSeeds::ID,
            Self::Beef(_) => r#Beef::ID,
            Self::CookedBeef(_) => r#CookedBeef::ID,
            Self::Chicken(_) => r#Chicken::ID,
            Self::CookedChicken(_) => r#CookedChicken::ID,
            Self::RottenFlesh(_) => r#RottenFlesh::ID,
            Self::EnderPearl(_) => r#EnderPearl::ID,
            Self::BlazeRod(_) => r#BlazeRod::ID,
            Self::GhastTear(_) => r#GhastTear::ID,
            Self::GoldNugget(_) => r#GoldNugget::ID,
            Self::NetherWart(_) => r#NetherWart::ID,
            Self::Potion(_) => r#Potion::ID,
            Self::GlassBottle(_) => r#GlassBottle::ID,
            Self::SpiderEye(_) => r#SpiderEye::ID,
            Self::FermentedSpiderEye(_) => r#FermentedSpiderEye::ID,
            Self::BlazePowder(_) => r#BlazePowder::ID,
            Self::MagmaCream(_) => r#MagmaCream::ID,
            Self::BrewingStand(_) => r#BrewingStand::ID,
            Self::Cauldron(_) => r#Cauldron::ID,
            Self::EnderEye(_) => r#EnderEye::ID,
            Self::GlisteringMelonSlice(_) => r#GlisteringMelonSlice::ID,
            Self::AllaySpawnEgg(_) => r#AllaySpawnEgg::ID,
            Self::AxolotlSpawnEgg(_) => r#AxolotlSpawnEgg::ID,
            Self::BatSpawnEgg(_) => r#BatSpawnEgg::ID,
            Self::BeeSpawnEgg(_) => r#BeeSpawnEgg::ID,
            Self::BlazeSpawnEgg(_) => r#BlazeSpawnEgg::ID,
            Self::CatSpawnEgg(_) => r#CatSpawnEgg::ID,
            Self::CamelSpawnEgg(_) => r#CamelSpawnEgg::ID,
            Self::CaveSpiderSpawnEgg(_) => r#CaveSpiderSpawnEgg::ID,
            Self::ChickenSpawnEgg(_) => r#ChickenSpawnEgg::ID,
            Self::CodSpawnEgg(_) => r#CodSpawnEgg::ID,
            Self::CowSpawnEgg(_) => r#CowSpawnEgg::ID,
            Self::CreeperSpawnEgg(_) => r#CreeperSpawnEgg::ID,
            Self::DolphinSpawnEgg(_) => r#DolphinSpawnEgg::ID,
            Self::DonkeySpawnEgg(_) => r#DonkeySpawnEgg::ID,
            Self::DrownedSpawnEgg(_) => r#DrownedSpawnEgg::ID,
            Self::ElderGuardianSpawnEgg(_) => r#ElderGuardianSpawnEgg::ID,
            Self::EnderDragonSpawnEgg(_) => r#EnderDragonSpawnEgg::ID,
            Self::EndermanSpawnEgg(_) => r#EndermanSpawnEgg::ID,
            Self::EndermiteSpawnEgg(_) => r#EndermiteSpawnEgg::ID,
            Self::EvokerSpawnEgg(_) => r#EvokerSpawnEgg::ID,
            Self::FoxSpawnEgg(_) => r#FoxSpawnEgg::ID,
            Self::FrogSpawnEgg(_) => r#FrogSpawnEgg::ID,
            Self::GhastSpawnEgg(_) => r#GhastSpawnEgg::ID,
            Self::GlowSquidSpawnEgg(_) => r#GlowSquidSpawnEgg::ID,
            Self::GoatSpawnEgg(_) => r#GoatSpawnEgg::ID,
            Self::GuardianSpawnEgg(_) => r#GuardianSpawnEgg::ID,
            Self::HoglinSpawnEgg(_) => r#HoglinSpawnEgg::ID,
            Self::HorseSpawnEgg(_) => r#HorseSpawnEgg::ID,
            Self::HuskSpawnEgg(_) => r#HuskSpawnEgg::ID,
            Self::IronGolemSpawnEgg(_) => r#IronGolemSpawnEgg::ID,
            Self::LlamaSpawnEgg(_) => r#LlamaSpawnEgg::ID,
            Self::MagmaCubeSpawnEgg(_) => r#MagmaCubeSpawnEgg::ID,
            Self::MooshroomSpawnEgg(_) => r#MooshroomSpawnEgg::ID,
            Self::MuleSpawnEgg(_) => r#MuleSpawnEgg::ID,
            Self::OcelotSpawnEgg(_) => r#OcelotSpawnEgg::ID,
            Self::PandaSpawnEgg(_) => r#PandaSpawnEgg::ID,
            Self::ParrotSpawnEgg(_) => r#ParrotSpawnEgg::ID,
            Self::PhantomSpawnEgg(_) => r#PhantomSpawnEgg::ID,
            Self::PigSpawnEgg(_) => r#PigSpawnEgg::ID,
            Self::PiglinSpawnEgg(_) => r#PiglinSpawnEgg::ID,
            Self::PiglinBruteSpawnEgg(_) => r#PiglinBruteSpawnEgg::ID,
            Self::PillagerSpawnEgg(_) => r#PillagerSpawnEgg::ID,
            Self::PolarBearSpawnEgg(_) => r#PolarBearSpawnEgg::ID,
            Self::PufferfishSpawnEgg(_) => r#PufferfishSpawnEgg::ID,
            Self::RabbitSpawnEgg(_) => r#RabbitSpawnEgg::ID,
            Self::RavagerSpawnEgg(_) => r#RavagerSpawnEgg::ID,
            Self::SalmonSpawnEgg(_) => r#SalmonSpawnEgg::ID,
            Self::SheepSpawnEgg(_) => r#SheepSpawnEgg::ID,
            Self::ShulkerSpawnEgg(_) => r#ShulkerSpawnEgg::ID,
            Self::SilverfishSpawnEgg(_) => r#SilverfishSpawnEgg::ID,
            Self::SkeletonSpawnEgg(_) => r#SkeletonSpawnEgg::ID,
            Self::SkeletonHorseSpawnEgg(_) => r#SkeletonHorseSpawnEgg::ID,
            Self::SlimeSpawnEgg(_) => r#SlimeSpawnEgg::ID,
            Self::SnowGolemSpawnEgg(_) => r#SnowGolemSpawnEgg::ID,
            Self::SpiderSpawnEgg(_) => r#SpiderSpawnEgg::ID,
            Self::SquidSpawnEgg(_) => r#SquidSpawnEgg::ID,
            Self::StraySpawnEgg(_) => r#StraySpawnEgg::ID,
            Self::StriderSpawnEgg(_) => r#StriderSpawnEgg::ID,
            Self::TadpoleSpawnEgg(_) => r#TadpoleSpawnEgg::ID,
            Self::TraderLlamaSpawnEgg(_) => r#TraderLlamaSpawnEgg::ID,
            Self::TropicalFishSpawnEgg(_) => r#TropicalFishSpawnEgg::ID,
            Self::TurtleSpawnEgg(_) => r#TurtleSpawnEgg::ID,
            Self::VexSpawnEgg(_) => r#VexSpawnEgg::ID,
            Self::VillagerSpawnEgg(_) => r#VillagerSpawnEgg::ID,
            Self::VindicatorSpawnEgg(_) => r#VindicatorSpawnEgg::ID,
            Self::WanderingTraderSpawnEgg(_) => r#WanderingTraderSpawnEgg::ID,
            Self::WardenSpawnEgg(_) => r#WardenSpawnEgg::ID,
            Self::WitchSpawnEgg(_) => r#WitchSpawnEgg::ID,
            Self::WitherSpawnEgg(_) => r#WitherSpawnEgg::ID,
            Self::WitherSkeletonSpawnEgg(_) => r#WitherSkeletonSpawnEgg::ID,
            Self::WolfSpawnEgg(_) => r#WolfSpawnEgg::ID,
            Self::ZoglinSpawnEgg(_) => r#ZoglinSpawnEgg::ID,
            Self::ZombieSpawnEgg(_) => r#ZombieSpawnEgg::ID,
            Self::ZombieHorseSpawnEgg(_) => r#ZombieHorseSpawnEgg::ID,
            Self::ZombieVillagerSpawnEgg(_) => r#ZombieVillagerSpawnEgg::ID,
            Self::ZombifiedPiglinSpawnEgg(_) => r#ZombifiedPiglinSpawnEgg::ID,
            Self::ExperienceBottle(_) => r#ExperienceBottle::ID,
            Self::FireCharge(_) => r#FireCharge::ID,
            Self::WritableBook(_) => r#WritableBook::ID,
            Self::WrittenBook(_) => r#WrittenBook::ID,
            Self::ItemFrame(_) => r#ItemFrame::ID,
            Self::GlowItemFrame(_) => r#GlowItemFrame::ID,
            Self::FlowerPot(_) => r#FlowerPot::ID,
            Self::Carrot(_) => r#Carrot::ID,
            Self::Potato(_) => r#Potato::ID,
            Self::BakedPotato(_) => r#BakedPotato::ID,
            Self::PoisonousPotato(_) => r#PoisonousPotato::ID,
            Self::Map(_) => r#Map::ID,
            Self::GoldenCarrot(_) => r#GoldenCarrot::ID,
            Self::SkeletonSkull(_) => r#SkeletonSkull::ID,
            Self::WitherSkeletonSkull(_) => r#WitherSkeletonSkull::ID,
            Self::PlayerHead(_) => r#PlayerHead::ID,
            Self::ZombieHead(_) => r#ZombieHead::ID,
            Self::CreeperHead(_) => r#CreeperHead::ID,
            Self::DragonHead(_) => r#DragonHead::ID,
            Self::PiglinHead(_) => r#PiglinHead::ID,
            Self::NetherStar(_) => r#NetherStar::ID,
            Self::PumpkinPie(_) => r#PumpkinPie::ID,
            Self::FireworkRocket(_) => r#FireworkRocket::ID,
            Self::FireworkStar(_) => r#FireworkStar::ID,
            Self::EnchantedBook(_) => r#EnchantedBook::ID,
            Self::NetherBrick(_) => r#NetherBrick::ID,
            Self::PrismarineShard(_) => r#PrismarineShard::ID,
            Self::PrismarineCrystals(_) => r#PrismarineCrystals::ID,
            Self::Rabbit(_) => r#Rabbit::ID,
            Self::CookedRabbit(_) => r#CookedRabbit::ID,
            Self::RabbitStew(_) => r#RabbitStew::ID,
            Self::RabbitFoot(_) => r#RabbitFoot::ID,
            Self::RabbitHide(_) => r#RabbitHide::ID,
            Self::ArmorStand(_) => r#ArmorStand::ID,
            Self::IronHorseArmor(_) => r#IronHorseArmor::ID,
            Self::GoldenHorseArmor(_) => r#GoldenHorseArmor::ID,
            Self::DiamondHorseArmor(_) => r#DiamondHorseArmor::ID,
            Self::LeatherHorseArmor(_) => r#LeatherHorseArmor::ID,
            Self::Lead(_) => r#Lead::ID,
            Self::NameTag(_) => r#NameTag::ID,
            Self::CommandBlockMinecart(_) => r#CommandBlockMinecart::ID,
            Self::Mutton(_) => r#Mutton::ID,
            Self::CookedMutton(_) => r#CookedMutton::ID,
            Self::WhiteBanner(_) => r#WhiteBanner::ID,
            Self::OrangeBanner(_) => r#OrangeBanner::ID,
            Self::MagentaBanner(_) => r#MagentaBanner::ID,
            Self::LightBlueBanner(_) => r#LightBlueBanner::ID,
            Self::YellowBanner(_) => r#YellowBanner::ID,
            Self::LimeBanner(_) => r#LimeBanner::ID,
            Self::PinkBanner(_) => r#PinkBanner::ID,
            Self::GrayBanner(_) => r#GrayBanner::ID,
            Self::LightGrayBanner(_) => r#LightGrayBanner::ID,
            Self::CyanBanner(_) => r#CyanBanner::ID,
            Self::PurpleBanner(_) => r#PurpleBanner::ID,
            Self::BlueBanner(_) => r#BlueBanner::ID,
            Self::BrownBanner(_) => r#BrownBanner::ID,
            Self::GreenBanner(_) => r#GreenBanner::ID,
            Self::RedBanner(_) => r#RedBanner::ID,
            Self::BlackBanner(_) => r#BlackBanner::ID,
            Self::EndCrystal(_) => r#EndCrystal::ID,
            Self::ChorusFruit(_) => r#ChorusFruit::ID,
            Self::PoppedChorusFruit(_) => r#PoppedChorusFruit::ID,
            Self::Beetroot(_) => r#Beetroot::ID,
            Self::BeetrootSeeds(_) => r#BeetrootSeeds::ID,
            Self::BeetrootSoup(_) => r#BeetrootSoup::ID,
            Self::DragonBreath(_) => r#DragonBreath::ID,
            Self::SplashPotion(_) => r#SplashPotion::ID,
            Self::SpectralArrow(_) => r#SpectralArrow::ID,
            Self::TippedArrow(_) => r#TippedArrow::ID,
            Self::LingeringPotion(_) => r#LingeringPotion::ID,
            Self::Shield(_) => r#Shield::ID,
            Self::TotemOfUndying(_) => r#TotemOfUndying::ID,
            Self::ShulkerShell(_) => r#ShulkerShell::ID,
            Self::IronNugget(_) => r#IronNugget::ID,
            Self::KnowledgeBook(_) => r#KnowledgeBook::ID,
            Self::DebugStick(_) => r#DebugStick::ID,
            Self::MusicDisc13(_) => r#MusicDisc13::ID,
            Self::MusicDiscCat(_) => r#MusicDiscCat::ID,
            Self::MusicDiscBlocks(_) => r#MusicDiscBlocks::ID,
            Self::MusicDiscChirp(_) => r#MusicDiscChirp::ID,
            Self::MusicDiscFar(_) => r#MusicDiscFar::ID,
            Self::MusicDiscMall(_) => r#MusicDiscMall::ID,
            Self::MusicDiscMellohi(_) => r#MusicDiscMellohi::ID,
            Self::MusicDiscStal(_) => r#MusicDiscStal::ID,
            Self::MusicDiscStrad(_) => r#MusicDiscStrad::ID,
            Self::MusicDiscWard(_) => r#MusicDiscWard::ID,
            Self::MusicDisc11(_) => r#MusicDisc11::ID,
            Self::MusicDiscWait(_) => r#MusicDiscWait::ID,
            Self::MusicDiscOtherside(_) => r#MusicDiscOtherside::ID,
            Self::MusicDisc5(_) => r#MusicDisc5::ID,
            Self::MusicDiscPigstep(_) => r#MusicDiscPigstep::ID,
            Self::DiscFragment5(_) => r#DiscFragment5::ID,
            Self::Trident(_) => r#Trident::ID,
            Self::PhantomMembrane(_) => r#PhantomMembrane::ID,
            Self::NautilusShell(_) => r#NautilusShell::ID,
            Self::HeartOfTheSea(_) => r#HeartOfTheSea::ID,
            Self::Crossbow(_) => r#Crossbow::ID,
            Self::SuspiciousStew(_) => r#SuspiciousStew::ID,
            Self::Loom(_) => r#Loom::ID,
            Self::FlowerBannerPattern(_) => r#FlowerBannerPattern::ID,
            Self::CreeperBannerPattern(_) => r#CreeperBannerPattern::ID,
            Self::SkullBannerPattern(_) => r#SkullBannerPattern::ID,
            Self::MojangBannerPattern(_) => r#MojangBannerPattern::ID,
            Self::GlobeBannerPattern(_) => r#GlobeBannerPattern::ID,
            Self::PiglinBannerPattern(_) => r#PiglinBannerPattern::ID,
            Self::GoatHorn(_) => r#GoatHorn::ID,
            Self::Composter(_) => r#Composter::ID,
            Self::Barrel(_) => r#Barrel::ID,
            Self::Smoker(_) => r#Smoker::ID,
            Self::BlastFurnace(_) => r#BlastFurnace::ID,
            Self::CartographyTable(_) => r#CartographyTable::ID,
            Self::FletchingTable(_) => r#FletchingTable::ID,
            Self::Grindstone(_) => r#Grindstone::ID,
            Self::SmithingTable(_) => r#SmithingTable::ID,
            Self::Stonecutter(_) => r#Stonecutter::ID,
            Self::Bell(_) => r#Bell::ID,
            Self::Lantern(_) => r#Lantern::ID,
            Self::SoulLantern(_) => r#SoulLantern::ID,
            Self::SweetBerries(_) => r#SweetBerries::ID,
            Self::GlowBerries(_) => r#GlowBerries::ID,
            Self::Campfire(_) => r#Campfire::ID,
            Self::SoulCampfire(_) => r#SoulCampfire::ID,
            Self::Shroomlight(_) => r#Shroomlight::ID,
            Self::Honeycomb(_) => r#Honeycomb::ID,
            Self::BeeNest(_) => r#BeeNest::ID,
            Self::Beehive(_) => r#Beehive::ID,
            Self::HoneyBottle(_) => r#HoneyBottle::ID,
            Self::HoneycombBlock(_) => r#HoneycombBlock::ID,
            Self::Lodestone(_) => r#Lodestone::ID,
            Self::CryingObsidian(_) => r#CryingObsidian::ID,
            Self::Blackstone(_) => r#Blackstone::ID,
            Self::BlackstoneSlab(_) => r#BlackstoneSlab::ID,
            Self::BlackstoneStairs(_) => r#BlackstoneStairs::ID,
            Self::GildedBlackstone(_) => r#GildedBlackstone::ID,
            Self::PolishedBlackstone(_) => r#PolishedBlackstone::ID,
            Self::PolishedBlackstoneSlab(_) => r#PolishedBlackstoneSlab::ID,
            Self::PolishedBlackstoneStairs(_) => r#PolishedBlackstoneStairs::ID,
            Self::ChiseledPolishedBlackstone(_) => r#ChiseledPolishedBlackstone::ID,
            Self::PolishedBlackstoneBricks(_) => r#PolishedBlackstoneBricks::ID,
            Self::PolishedBlackstoneBrickSlab(_) => r#PolishedBlackstoneBrickSlab::ID,
            Self::PolishedBlackstoneBrickStairs(_) => r#PolishedBlackstoneBrickStairs::ID,
            Self::CrackedPolishedBlackstoneBricks(_) => r#CrackedPolishedBlackstoneBricks::ID,
            Self::RespawnAnchor(_) => r#RespawnAnchor::ID,
            Self::Candle(_) => r#Candle::ID,
            Self::WhiteCandle(_) => r#WhiteCandle::ID,
            Self::OrangeCandle(_) => r#OrangeCandle::ID,
            Self::MagentaCandle(_) => r#MagentaCandle::ID,
            Self::LightBlueCandle(_) => r#LightBlueCandle::ID,
            Self::YellowCandle(_) => r#YellowCandle::ID,
            Self::LimeCandle(_) => r#LimeCandle::ID,
            Self::PinkCandle(_) => r#PinkCandle::ID,
            Self::GrayCandle(_) => r#GrayCandle::ID,
            Self::LightGrayCandle(_) => r#LightGrayCandle::ID,
            Self::CyanCandle(_) => r#CyanCandle::ID,
            Self::PurpleCandle(_) => r#PurpleCandle::ID,
            Self::BlueCandle(_) => r#BlueCandle::ID,
            Self::BrownCandle(_) => r#BrownCandle::ID,
            Self::GreenCandle(_) => r#GreenCandle::ID,
            Self::RedCandle(_) => r#RedCandle::ID,
            Self::BlackCandle(_) => r#BlackCandle::ID,
            Self::SmallAmethystBud(_) => r#SmallAmethystBud::ID,
            Self::MediumAmethystBud(_) => r#MediumAmethystBud::ID,
            Self::LargeAmethystBud(_) => r#LargeAmethystBud::ID,
            Self::AmethystCluster(_) => r#AmethystCluster::ID,
            Self::PointedDripstone(_) => r#PointedDripstone::ID,
            Self::OchreFroglight(_) => r#OchreFroglight::ID,
            Self::VerdantFroglight(_) => r#VerdantFroglight::ID,
            Self::PearlescentFroglight(_) => r#PearlescentFroglight::ID,
            Self::Frogspawn(_) => r#Frogspawn::ID,
            Self::EchoShard(_) => r#EchoShard::ID,
        }
    }
}
