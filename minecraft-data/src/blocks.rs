// This code was generated using data provided by PrismarineJS/minecraft-data

use crate::ConstrainedInt;
use serde::de::{MapAccess, self};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub enum Block {
	Air(Air),
	Stone(Stone),
	Granite(Granite),
	PolishedGranite(PolishedGranite),
	Diorite(Diorite),
	PolishedDiorite(PolishedDiorite),
	Andesite(Andesite),
	PolishedAndesite(PolishedAndesite),
	GrassBlock(GrassBlock),
	Dirt(Dirt),
	CoarseDirt(CoarseDirt),
	Podzol(Podzol),
	Cobblestone(Cobblestone),
	OakPlanks(OakPlanks),
	SprucePlanks(SprucePlanks),
	BirchPlanks(BirchPlanks),
	JunglePlanks(JunglePlanks),
	AcaciaPlanks(AcaciaPlanks),
	DarkOakPlanks(DarkOakPlanks),
	MangrovePlanks(MangrovePlanks),
	OakSapling(OakSapling),
	SpruceSapling(SpruceSapling),
	BirchSapling(BirchSapling),
	JungleSapling(JungleSapling),
	AcaciaSapling(AcaciaSapling),
	DarkOakSapling(DarkOakSapling),
	MangrovePropagule(MangrovePropagule),
	Bedrock(Bedrock),
	Water(Water),
	Lava(Lava),
	Sand(Sand),
	RedSand(RedSand),
	Gravel(Gravel),
	GoldOre(GoldOre),
	DeepslateGoldOre(DeepslateGoldOre),
	IronOre(IronOre),
	DeepslateIronOre(DeepslateIronOre),
	CoalOre(CoalOre),
	DeepslateCoalOre(DeepslateCoalOre),
	NetherGoldOre(NetherGoldOre),
	OakLog(OakLog),
	SpruceLog(SpruceLog),
	BirchLog(BirchLog),
	JungleLog(JungleLog),
	AcaciaLog(AcaciaLog),
	DarkOakLog(DarkOakLog),
	MangroveLog(MangroveLog),
	MangroveRoots(MangroveRoots),
	MuddyMangroveRoots(MuddyMangroveRoots),
	StrippedSpruceLog(StrippedSpruceLog),
	StrippedBirchLog(StrippedBirchLog),
	StrippedJungleLog(StrippedJungleLog),
	StrippedAcaciaLog(StrippedAcaciaLog),
	StrippedDarkOakLog(StrippedDarkOakLog),
	StrippedOakLog(StrippedOakLog),
	StrippedMangroveLog(StrippedMangroveLog),
	OakWood(OakWood),
	SpruceWood(SpruceWood),
	BirchWood(BirchWood),
	JungleWood(JungleWood),
	AcaciaWood(AcaciaWood),
	DarkOakWood(DarkOakWood),
	MangroveWood(MangroveWood),
	StrippedOakWood(StrippedOakWood),
	StrippedSpruceWood(StrippedSpruceWood),
	StrippedBirchWood(StrippedBirchWood),
	StrippedJungleWood(StrippedJungleWood),
	StrippedAcaciaWood(StrippedAcaciaWood),
	StrippedDarkOakWood(StrippedDarkOakWood),
	StrippedMangroveWood(StrippedMangroveWood),
	OakLeaves(OakLeaves),
	SpruceLeaves(SpruceLeaves),
	BirchLeaves(BirchLeaves),
	JungleLeaves(JungleLeaves),
	AcaciaLeaves(AcaciaLeaves),
	DarkOakLeaves(DarkOakLeaves),
	MangroveLeaves(MangroveLeaves),
	AzaleaLeaves(AzaleaLeaves),
	FloweringAzaleaLeaves(FloweringAzaleaLeaves),
	Sponge(Sponge),
	WetSponge(WetSponge),
	Glass(Glass),
	LapisOre(LapisOre),
	DeepslateLapisOre(DeepslateLapisOre),
	LapisBlock(LapisBlock),
	Dispenser(Dispenser),
	Sandstone(Sandstone),
	ChiseledSandstone(ChiseledSandstone),
	CutSandstone(CutSandstone),
	NoteBlock(NoteBlock),
	WhiteBed(WhiteBed),
	OrangeBed(OrangeBed),
	MagentaBed(MagentaBed),
	LightBlueBed(LightBlueBed),
	YellowBed(YellowBed),
	LimeBed(LimeBed),
	PinkBed(PinkBed),
	GrayBed(GrayBed),
	LightGrayBed(LightGrayBed),
	CyanBed(CyanBed),
	PurpleBed(PurpleBed),
	BlueBed(BlueBed),
	BrownBed(BrownBed),
	GreenBed(GreenBed),
	RedBed(RedBed),
	BlackBed(BlackBed),
	PoweredRail(PoweredRail),
	DetectorRail(DetectorRail),
	StickyPiston(StickyPiston),
	Cobweb(Cobweb),
	Grass(Grass),
	Fern(Fern),
	DeadBush(DeadBush),
	Seagrass(Seagrass),
	TallSeagrass(TallSeagrass),
	Piston(Piston),
	PistonHead(PistonHead),
	WhiteWool(WhiteWool),
	OrangeWool(OrangeWool),
	MagentaWool(MagentaWool),
	LightBlueWool(LightBlueWool),
	YellowWool(YellowWool),
	LimeWool(LimeWool),
	PinkWool(PinkWool),
	GrayWool(GrayWool),
	LightGrayWool(LightGrayWool),
	CyanWool(CyanWool),
	PurpleWool(PurpleWool),
	BlueWool(BlueWool),
	BrownWool(BrownWool),
	GreenWool(GreenWool),
	RedWool(RedWool),
	BlackWool(BlackWool),
	MovingPiston(MovingPiston),
	Dandelion(Dandelion),
	Poppy(Poppy),
	BlueOrchid(BlueOrchid),
	Allium(Allium),
	AzureBluet(AzureBluet),
	RedTulip(RedTulip),
	OrangeTulip(OrangeTulip),
	WhiteTulip(WhiteTulip),
	PinkTulip(PinkTulip),
	OxeyeDaisy(OxeyeDaisy),
	Cornflower(Cornflower),
	WitherRose(WitherRose),
	LilyOfTheValley(LilyOfTheValley),
	BrownMushroom(BrownMushroom),
	RedMushroom(RedMushroom),
	GoldBlock(GoldBlock),
	IronBlock(IronBlock),
	Bricks(Bricks),
	Tnt(Tnt),
	Bookshelf(Bookshelf),
	MossyCobblestone(MossyCobblestone),
	Obsidian(Obsidian),
	Torch(Torch),
	WallTorch(WallTorch),
	Fire(Fire),
	SoulFire(SoulFire),
	Spawner(Spawner),
	OakStairs(OakStairs),
	Chest(Chest),
	RedstoneWire(RedstoneWire),
	DiamondOre(DiamondOre),
	DeepslateDiamondOre(DeepslateDiamondOre),
	DiamondBlock(DiamondBlock),
	CraftingTable(CraftingTable),
	Wheat(Wheat),
	Farmland(Farmland),
	Furnace(Furnace),
	OakSign(OakSign),
	SpruceSign(SpruceSign),
	BirchSign(BirchSign),
	AcaciaSign(AcaciaSign),
	JungleSign(JungleSign),
	DarkOakSign(DarkOakSign),
	MangroveSign(MangroveSign),
	OakDoor(OakDoor),
	Ladder(Ladder),
	Rail(Rail),
	CobblestoneStairs(CobblestoneStairs),
	OakWallSign(OakWallSign),
	SpruceWallSign(SpruceWallSign),
	BirchWallSign(BirchWallSign),
	AcaciaWallSign(AcaciaWallSign),
	JungleWallSign(JungleWallSign),
	DarkOakWallSign(DarkOakWallSign),
	MangroveWallSign(MangroveWallSign),
	Lever(Lever),
	StonePressurePlate(StonePressurePlate),
	IronDoor(IronDoor),
	OakPressurePlate(OakPressurePlate),
	SprucePressurePlate(SprucePressurePlate),
	BirchPressurePlate(BirchPressurePlate),
	JunglePressurePlate(JunglePressurePlate),
	AcaciaPressurePlate(AcaciaPressurePlate),
	DarkOakPressurePlate(DarkOakPressurePlate),
	MangrovePressurePlate(MangrovePressurePlate),
	RedstoneOre(RedstoneOre),
	DeepslateRedstoneOre(DeepslateRedstoneOre),
	RedstoneTorch(RedstoneTorch),
	RedstoneWallTorch(RedstoneWallTorch),
	StoneButton(StoneButton),
	Snow(Snow),
	Ice(Ice),
	SnowBlock(SnowBlock),
	Cactus(Cactus),
	Clay(Clay),
	SugarCane(SugarCane),
	Jukebox(Jukebox),
	OakFence(OakFence),
	Pumpkin(Pumpkin),
	Netherrack(Netherrack),
	SoulSand(SoulSand),
	SoulSoil(SoulSoil),
	Basalt(Basalt),
	PolishedBasalt(PolishedBasalt),
	SoulTorch(SoulTorch),
	SoulWallTorch(SoulWallTorch),
	Glowstone(Glowstone),
	NetherPortal(NetherPortal),
	CarvedPumpkin(CarvedPumpkin),
	JackOLantern(JackOLantern),
	Cake(Cake),
	Repeater(Repeater),
	WhiteStainedGlass(WhiteStainedGlass),
	OrangeStainedGlass(OrangeStainedGlass),
	MagentaStainedGlass(MagentaStainedGlass),
	LightBlueStainedGlass(LightBlueStainedGlass),
	YellowStainedGlass(YellowStainedGlass),
	LimeStainedGlass(LimeStainedGlass),
	PinkStainedGlass(PinkStainedGlass),
	GrayStainedGlass(GrayStainedGlass),
	LightGrayStainedGlass(LightGrayStainedGlass),
	CyanStainedGlass(CyanStainedGlass),
	PurpleStainedGlass(PurpleStainedGlass),
	BlueStainedGlass(BlueStainedGlass),
	BrownStainedGlass(BrownStainedGlass),
	GreenStainedGlass(GreenStainedGlass),
	RedStainedGlass(RedStainedGlass),
	BlackStainedGlass(BlackStainedGlass),
	OakTrapdoor(OakTrapdoor),
	SpruceTrapdoor(SpruceTrapdoor),
	BirchTrapdoor(BirchTrapdoor),
	JungleTrapdoor(JungleTrapdoor),
	AcaciaTrapdoor(AcaciaTrapdoor),
	DarkOakTrapdoor(DarkOakTrapdoor),
	MangroveTrapdoor(MangroveTrapdoor),
	StoneBricks(StoneBricks),
	MossyStoneBricks(MossyStoneBricks),
	CrackedStoneBricks(CrackedStoneBricks),
	ChiseledStoneBricks(ChiseledStoneBricks),
	PackedMud(PackedMud),
	MudBricks(MudBricks),
	InfestedStone(InfestedStone),
	InfestedCobblestone(InfestedCobblestone),
	InfestedStoneBricks(InfestedStoneBricks),
	InfestedMossyStoneBricks(InfestedMossyStoneBricks),
	InfestedCrackedStoneBricks(InfestedCrackedStoneBricks),
	InfestedChiseledStoneBricks(InfestedChiseledStoneBricks),
	BrownMushroomBlock(BrownMushroomBlock),
	RedMushroomBlock(RedMushroomBlock),
	MushroomStem(MushroomStem),
	IronBars(IronBars),
	Chain(Chain),
	GlassPane(GlassPane),
	Melon(Melon),
	AttachedPumpkinStem(AttachedPumpkinStem),
	AttachedMelonStem(AttachedMelonStem),
	PumpkinStem(PumpkinStem),
	MelonStem(MelonStem),
	Vine(Vine),
	GlowLichen(GlowLichen),
	OakFenceGate(OakFenceGate),
	BrickStairs(BrickStairs),
	StoneBrickStairs(StoneBrickStairs),
	MudBrickStairs(MudBrickStairs),
	Mycelium(Mycelium),
	LilyPad(LilyPad),
	NetherBricks(NetherBricks),
	NetherBrickFence(NetherBrickFence),
	NetherBrickStairs(NetherBrickStairs),
	NetherWart(NetherWart),
	EnchantingTable(EnchantingTable),
	BrewingStand(BrewingStand),
	Cauldron(Cauldron),
	WaterCauldron(WaterCauldron),
	LavaCauldron(LavaCauldron),
	PowderSnowCauldron(PowderSnowCauldron),
	EndPortal(EndPortal),
	EndPortalFrame(EndPortalFrame),
	EndStone(EndStone),
	DragonEgg(DragonEgg),
	RedstoneLamp(RedstoneLamp),
	Cocoa(Cocoa),
	SandstoneStairs(SandstoneStairs),
	EmeraldOre(EmeraldOre),
	DeepslateEmeraldOre(DeepslateEmeraldOre),
	EnderChest(EnderChest),
	TripwireHook(TripwireHook),
	Tripwire(Tripwire),
	EmeraldBlock(EmeraldBlock),
	SpruceStairs(SpruceStairs),
	BirchStairs(BirchStairs),
	JungleStairs(JungleStairs),
	CommandBlock(CommandBlock),
	Beacon(Beacon),
	CobblestoneWall(CobblestoneWall),
	MossyCobblestoneWall(MossyCobblestoneWall),
	FlowerPot(FlowerPot),
	PottedOakSapling(PottedOakSapling),
	PottedSpruceSapling(PottedSpruceSapling),
	PottedBirchSapling(PottedBirchSapling),
	PottedJungleSapling(PottedJungleSapling),
	PottedAcaciaSapling(PottedAcaciaSapling),
	PottedDarkOakSapling(PottedDarkOakSapling),
	PottedMangrovePropagule(PottedMangrovePropagule),
	PottedFern(PottedFern),
	PottedDandelion(PottedDandelion),
	PottedPoppy(PottedPoppy),
	PottedBlueOrchid(PottedBlueOrchid),
	PottedAllium(PottedAllium),
	PottedAzureBluet(PottedAzureBluet),
	PottedRedTulip(PottedRedTulip),
	PottedOrangeTulip(PottedOrangeTulip),
	PottedWhiteTulip(PottedWhiteTulip),
	PottedPinkTulip(PottedPinkTulip),
	PottedOxeyeDaisy(PottedOxeyeDaisy),
	PottedCornflower(PottedCornflower),
	PottedLilyOfTheValley(PottedLilyOfTheValley),
	PottedWitherRose(PottedWitherRose),
	PottedRedMushroom(PottedRedMushroom),
	PottedBrownMushroom(PottedBrownMushroom),
	PottedDeadBush(PottedDeadBush),
	PottedCactus(PottedCactus),
	Carrots(Carrots),
	Potatoes(Potatoes),
	OakButton(OakButton),
	SpruceButton(SpruceButton),
	BirchButton(BirchButton),
	JungleButton(JungleButton),
	AcaciaButton(AcaciaButton),
	DarkOakButton(DarkOakButton),
	MangroveButton(MangroveButton),
	SkeletonSkull(SkeletonSkull),
	SkeletonWallSkull(SkeletonWallSkull),
	WitherSkeletonSkull(WitherSkeletonSkull),
	WitherSkeletonWallSkull(WitherSkeletonWallSkull),
	ZombieHead(ZombieHead),
	ZombieWallHead(ZombieWallHead),
	PlayerHead(PlayerHead),
	PlayerWallHead(PlayerWallHead),
	CreeperHead(CreeperHead),
	CreeperWallHead(CreeperWallHead),
	DragonHead(DragonHead),
	DragonWallHead(DragonWallHead),
	Anvil(Anvil),
	ChippedAnvil(ChippedAnvil),
	DamagedAnvil(DamagedAnvil),
	TrappedChest(TrappedChest),
	LightWeightedPressurePlate(LightWeightedPressurePlate),
	HeavyWeightedPressurePlate(HeavyWeightedPressurePlate),
	Comparator(Comparator),
	DaylightDetector(DaylightDetector),
	RedstoneBlock(RedstoneBlock),
	NetherQuartzOre(NetherQuartzOre),
	Hopper(Hopper),
	QuartzBlock(QuartzBlock),
	ChiseledQuartzBlock(ChiseledQuartzBlock),
	QuartzPillar(QuartzPillar),
	QuartzStairs(QuartzStairs),
	ActivatorRail(ActivatorRail),
	Dropper(Dropper),
	WhiteTerracotta(WhiteTerracotta),
	OrangeTerracotta(OrangeTerracotta),
	MagentaTerracotta(MagentaTerracotta),
	LightBlueTerracotta(LightBlueTerracotta),
	YellowTerracotta(YellowTerracotta),
	LimeTerracotta(LimeTerracotta),
	PinkTerracotta(PinkTerracotta),
	GrayTerracotta(GrayTerracotta),
	LightGrayTerracotta(LightGrayTerracotta),
	CyanTerracotta(CyanTerracotta),
	PurpleTerracotta(PurpleTerracotta),
	BlueTerracotta(BlueTerracotta),
	BrownTerracotta(BrownTerracotta),
	GreenTerracotta(GreenTerracotta),
	RedTerracotta(RedTerracotta),
	BlackTerracotta(BlackTerracotta),
	WhiteStainedGlassPane(WhiteStainedGlassPane),
	OrangeStainedGlassPane(OrangeStainedGlassPane),
	MagentaStainedGlassPane(MagentaStainedGlassPane),
	LightBlueStainedGlassPane(LightBlueStainedGlassPane),
	YellowStainedGlassPane(YellowStainedGlassPane),
	LimeStainedGlassPane(LimeStainedGlassPane),
	PinkStainedGlassPane(PinkStainedGlassPane),
	GrayStainedGlassPane(GrayStainedGlassPane),
	LightGrayStainedGlassPane(LightGrayStainedGlassPane),
	CyanStainedGlassPane(CyanStainedGlassPane),
	PurpleStainedGlassPane(PurpleStainedGlassPane),
	BlueStainedGlassPane(BlueStainedGlassPane),
	BrownStainedGlassPane(BrownStainedGlassPane),
	GreenStainedGlassPane(GreenStainedGlassPane),
	RedStainedGlassPane(RedStainedGlassPane),
	BlackStainedGlassPane(BlackStainedGlassPane),
	AcaciaStairs(AcaciaStairs),
	DarkOakStairs(DarkOakStairs),
	MangroveStairs(MangroveStairs),
	SlimeBlock(SlimeBlock),
	Barrier(Barrier),
	Light(Light),
	IronTrapdoor(IronTrapdoor),
	Prismarine(Prismarine),
	PrismarineBricks(PrismarineBricks),
	DarkPrismarine(DarkPrismarine),
	PrismarineStairs(PrismarineStairs),
	PrismarineBrickStairs(PrismarineBrickStairs),
	DarkPrismarineStairs(DarkPrismarineStairs),
	PrismarineSlab(PrismarineSlab),
	PrismarineBrickSlab(PrismarineBrickSlab),
	DarkPrismarineSlab(DarkPrismarineSlab),
	SeaLantern(SeaLantern),
	HayBlock(HayBlock),
	WhiteCarpet(WhiteCarpet),
	OrangeCarpet(OrangeCarpet),
	MagentaCarpet(MagentaCarpet),
	LightBlueCarpet(LightBlueCarpet),
	YellowCarpet(YellowCarpet),
	LimeCarpet(LimeCarpet),
	PinkCarpet(PinkCarpet),
	GrayCarpet(GrayCarpet),
	LightGrayCarpet(LightGrayCarpet),
	CyanCarpet(CyanCarpet),
	PurpleCarpet(PurpleCarpet),
	BlueCarpet(BlueCarpet),
	BrownCarpet(BrownCarpet),
	GreenCarpet(GreenCarpet),
	RedCarpet(RedCarpet),
	BlackCarpet(BlackCarpet),
	Terracotta(Terracotta),
	CoalBlock(CoalBlock),
	PackedIce(PackedIce),
	Sunflower(Sunflower),
	Lilac(Lilac),
	RoseBush(RoseBush),
	Peony(Peony),
	TallGrass(TallGrass),
	LargeFern(LargeFern),
	WhiteBanner(WhiteBanner),
	OrangeBanner(OrangeBanner),
	MagentaBanner(MagentaBanner),
	LightBlueBanner(LightBlueBanner),
	YellowBanner(YellowBanner),
	LimeBanner(LimeBanner),
	PinkBanner(PinkBanner),
	GrayBanner(GrayBanner),
	LightGrayBanner(LightGrayBanner),
	CyanBanner(CyanBanner),
	PurpleBanner(PurpleBanner),
	BlueBanner(BlueBanner),
	BrownBanner(BrownBanner),
	GreenBanner(GreenBanner),
	RedBanner(RedBanner),
	BlackBanner(BlackBanner),
	WhiteWallBanner(WhiteWallBanner),
	OrangeWallBanner(OrangeWallBanner),
	MagentaWallBanner(MagentaWallBanner),
	LightBlueWallBanner(LightBlueWallBanner),
	YellowWallBanner(YellowWallBanner),
	LimeWallBanner(LimeWallBanner),
	PinkWallBanner(PinkWallBanner),
	GrayWallBanner(GrayWallBanner),
	LightGrayWallBanner(LightGrayWallBanner),
	CyanWallBanner(CyanWallBanner),
	PurpleWallBanner(PurpleWallBanner),
	BlueWallBanner(BlueWallBanner),
	BrownWallBanner(BrownWallBanner),
	GreenWallBanner(GreenWallBanner),
	RedWallBanner(RedWallBanner),
	BlackWallBanner(BlackWallBanner),
	RedSandstone(RedSandstone),
	ChiseledRedSandstone(ChiseledRedSandstone),
	CutRedSandstone(CutRedSandstone),
	RedSandstoneStairs(RedSandstoneStairs),
	OakSlab(OakSlab),
	SpruceSlab(SpruceSlab),
	BirchSlab(BirchSlab),
	JungleSlab(JungleSlab),
	AcaciaSlab(AcaciaSlab),
	DarkOakSlab(DarkOakSlab),
	MangroveSlab(MangroveSlab),
	StoneSlab(StoneSlab),
	SmoothStoneSlab(SmoothStoneSlab),
	SandstoneSlab(SandstoneSlab),
	CutSandstoneSlab(CutSandstoneSlab),
	PetrifiedOakSlab(PetrifiedOakSlab),
	CobblestoneSlab(CobblestoneSlab),
	BrickSlab(BrickSlab),
	StoneBrickSlab(StoneBrickSlab),
	MudBrickSlab(MudBrickSlab),
	NetherBrickSlab(NetherBrickSlab),
	QuartzSlab(QuartzSlab),
	RedSandstoneSlab(RedSandstoneSlab),
	CutRedSandstoneSlab(CutRedSandstoneSlab),
	PurpurSlab(PurpurSlab),
	SmoothStone(SmoothStone),
	SmoothSandstone(SmoothSandstone),
	SmoothQuartz(SmoothQuartz),
	SmoothRedSandstone(SmoothRedSandstone),
	SpruceFenceGate(SpruceFenceGate),
	BirchFenceGate(BirchFenceGate),
	JungleFenceGate(JungleFenceGate),
	AcaciaFenceGate(AcaciaFenceGate),
	DarkOakFenceGate(DarkOakFenceGate),
	MangroveFenceGate(MangroveFenceGate),
	SpruceFence(SpruceFence),
	BirchFence(BirchFence),
	JungleFence(JungleFence),
	AcaciaFence(AcaciaFence),
	DarkOakFence(DarkOakFence),
	MangroveFence(MangroveFence),
	SpruceDoor(SpruceDoor),
	BirchDoor(BirchDoor),
	JungleDoor(JungleDoor),
	AcaciaDoor(AcaciaDoor),
	DarkOakDoor(DarkOakDoor),
	MangroveDoor(MangroveDoor),
	EndRod(EndRod),
	ChorusPlant(ChorusPlant),
	ChorusFlower(ChorusFlower),
	PurpurBlock(PurpurBlock),
	PurpurPillar(PurpurPillar),
	PurpurStairs(PurpurStairs),
	EndStoneBricks(EndStoneBricks),
	Beetroots(Beetroots),
	DirtPath(DirtPath),
	EndGateway(EndGateway),
	RepeatingCommandBlock(RepeatingCommandBlock),
	ChainCommandBlock(ChainCommandBlock),
	FrostedIce(FrostedIce),
	MagmaBlock(MagmaBlock),
	NetherWartBlock(NetherWartBlock),
	RedNetherBricks(RedNetherBricks),
	BoneBlock(BoneBlock),
	StructureVoid(StructureVoid),
	Observer(Observer),
	ShulkerBox(ShulkerBox),
	WhiteShulkerBox(WhiteShulkerBox),
	OrangeShulkerBox(OrangeShulkerBox),
	MagentaShulkerBox(MagentaShulkerBox),
	LightBlueShulkerBox(LightBlueShulkerBox),
	YellowShulkerBox(YellowShulkerBox),
	LimeShulkerBox(LimeShulkerBox),
	PinkShulkerBox(PinkShulkerBox),
	GrayShulkerBox(GrayShulkerBox),
	LightGrayShulkerBox(LightGrayShulkerBox),
	CyanShulkerBox(CyanShulkerBox),
	PurpleShulkerBox(PurpleShulkerBox),
	BlueShulkerBox(BlueShulkerBox),
	BrownShulkerBox(BrownShulkerBox),
	GreenShulkerBox(GreenShulkerBox),
	RedShulkerBox(RedShulkerBox),
	BlackShulkerBox(BlackShulkerBox),
	WhiteGlazedTerracotta(WhiteGlazedTerracotta),
	OrangeGlazedTerracotta(OrangeGlazedTerracotta),
	MagentaGlazedTerracotta(MagentaGlazedTerracotta),
	LightBlueGlazedTerracotta(LightBlueGlazedTerracotta),
	YellowGlazedTerracotta(YellowGlazedTerracotta),
	LimeGlazedTerracotta(LimeGlazedTerracotta),
	PinkGlazedTerracotta(PinkGlazedTerracotta),
	GrayGlazedTerracotta(GrayGlazedTerracotta),
	LightGrayGlazedTerracotta(LightGrayGlazedTerracotta),
	CyanGlazedTerracotta(CyanGlazedTerracotta),
	PurpleGlazedTerracotta(PurpleGlazedTerracotta),
	BlueGlazedTerracotta(BlueGlazedTerracotta),
	BrownGlazedTerracotta(BrownGlazedTerracotta),
	GreenGlazedTerracotta(GreenGlazedTerracotta),
	RedGlazedTerracotta(RedGlazedTerracotta),
	BlackGlazedTerracotta(BlackGlazedTerracotta),
	WhiteConcrete(WhiteConcrete),
	OrangeConcrete(OrangeConcrete),
	MagentaConcrete(MagentaConcrete),
	LightBlueConcrete(LightBlueConcrete),
	YellowConcrete(YellowConcrete),
	LimeConcrete(LimeConcrete),
	PinkConcrete(PinkConcrete),
	GrayConcrete(GrayConcrete),
	LightGrayConcrete(LightGrayConcrete),
	CyanConcrete(CyanConcrete),
	PurpleConcrete(PurpleConcrete),
	BlueConcrete(BlueConcrete),
	BrownConcrete(BrownConcrete),
	GreenConcrete(GreenConcrete),
	RedConcrete(RedConcrete),
	BlackConcrete(BlackConcrete),
	WhiteConcretePowder(WhiteConcretePowder),
	OrangeConcretePowder(OrangeConcretePowder),
	MagentaConcretePowder(MagentaConcretePowder),
	LightBlueConcretePowder(LightBlueConcretePowder),
	YellowConcretePowder(YellowConcretePowder),
	LimeConcretePowder(LimeConcretePowder),
	PinkConcretePowder(PinkConcretePowder),
	GrayConcretePowder(GrayConcretePowder),
	LightGrayConcretePowder(LightGrayConcretePowder),
	CyanConcretePowder(CyanConcretePowder),
	PurpleConcretePowder(PurpleConcretePowder),
	BlueConcretePowder(BlueConcretePowder),
	BrownConcretePowder(BrownConcretePowder),
	GreenConcretePowder(GreenConcretePowder),
	RedConcretePowder(RedConcretePowder),
	BlackConcretePowder(BlackConcretePowder),
	Kelp(Kelp),
	KelpPlant(KelpPlant),
	DriedKelpBlock(DriedKelpBlock),
	TurtleEgg(TurtleEgg),
	DeadTubeCoralBlock(DeadTubeCoralBlock),
	DeadBrainCoralBlock(DeadBrainCoralBlock),
	DeadBubbleCoralBlock(DeadBubbleCoralBlock),
	DeadFireCoralBlock(DeadFireCoralBlock),
	DeadHornCoralBlock(DeadHornCoralBlock),
	TubeCoralBlock(TubeCoralBlock),
	BrainCoralBlock(BrainCoralBlock),
	BubbleCoralBlock(BubbleCoralBlock),
	FireCoralBlock(FireCoralBlock),
	HornCoralBlock(HornCoralBlock),
	DeadTubeCoral(DeadTubeCoral),
	DeadBrainCoral(DeadBrainCoral),
	DeadBubbleCoral(DeadBubbleCoral),
	DeadFireCoral(DeadFireCoral),
	DeadHornCoral(DeadHornCoral),
	TubeCoral(TubeCoral),
	BrainCoral(BrainCoral),
	BubbleCoral(BubbleCoral),
	FireCoral(FireCoral),
	HornCoral(HornCoral),
	DeadTubeCoralFan(DeadTubeCoralFan),
	DeadBrainCoralFan(DeadBrainCoralFan),
	DeadBubbleCoralFan(DeadBubbleCoralFan),
	DeadFireCoralFan(DeadFireCoralFan),
	DeadHornCoralFan(DeadHornCoralFan),
	TubeCoralFan(TubeCoralFan),
	BrainCoralFan(BrainCoralFan),
	BubbleCoralFan(BubbleCoralFan),
	FireCoralFan(FireCoralFan),
	HornCoralFan(HornCoralFan),
	DeadTubeCoralWallFan(DeadTubeCoralWallFan),
	DeadBrainCoralWallFan(DeadBrainCoralWallFan),
	DeadBubbleCoralWallFan(DeadBubbleCoralWallFan),
	DeadFireCoralWallFan(DeadFireCoralWallFan),
	DeadHornCoralWallFan(DeadHornCoralWallFan),
	TubeCoralWallFan(TubeCoralWallFan),
	BrainCoralWallFan(BrainCoralWallFan),
	BubbleCoralWallFan(BubbleCoralWallFan),
	FireCoralWallFan(FireCoralWallFan),
	HornCoralWallFan(HornCoralWallFan),
	SeaPickle(SeaPickle),
	BlueIce(BlueIce),
	Conduit(Conduit),
	BambooSapling(BambooSapling),
	Bamboo(Bamboo),
	PottedBamboo(PottedBamboo),
	VoidAir(VoidAir),
	CaveAir(CaveAir),
	BubbleColumn(BubbleColumn),
	PolishedGraniteStairs(PolishedGraniteStairs),
	SmoothRedSandstoneStairs(SmoothRedSandstoneStairs),
	MossyStoneBrickStairs(MossyStoneBrickStairs),
	PolishedDioriteStairs(PolishedDioriteStairs),
	MossyCobblestoneStairs(MossyCobblestoneStairs),
	EndStoneBrickStairs(EndStoneBrickStairs),
	StoneStairs(StoneStairs),
	SmoothSandstoneStairs(SmoothSandstoneStairs),
	SmoothQuartzStairs(SmoothQuartzStairs),
	GraniteStairs(GraniteStairs),
	AndesiteStairs(AndesiteStairs),
	RedNetherBrickStairs(RedNetherBrickStairs),
	PolishedAndesiteStairs(PolishedAndesiteStairs),
	DioriteStairs(DioriteStairs),
	PolishedGraniteSlab(PolishedGraniteSlab),
	SmoothRedSandstoneSlab(SmoothRedSandstoneSlab),
	MossyStoneBrickSlab(MossyStoneBrickSlab),
	PolishedDioriteSlab(PolishedDioriteSlab),
	MossyCobblestoneSlab(MossyCobblestoneSlab),
	EndStoneBrickSlab(EndStoneBrickSlab),
	SmoothSandstoneSlab(SmoothSandstoneSlab),
	SmoothQuartzSlab(SmoothQuartzSlab),
	GraniteSlab(GraniteSlab),
	AndesiteSlab(AndesiteSlab),
	RedNetherBrickSlab(RedNetherBrickSlab),
	PolishedAndesiteSlab(PolishedAndesiteSlab),
	DioriteSlab(DioriteSlab),
	BrickWall(BrickWall),
	PrismarineWall(PrismarineWall),
	RedSandstoneWall(RedSandstoneWall),
	MossyStoneBrickWall(MossyStoneBrickWall),
	GraniteWall(GraniteWall),
	StoneBrickWall(StoneBrickWall),
	MudBrickWall(MudBrickWall),
	NetherBrickWall(NetherBrickWall),
	AndesiteWall(AndesiteWall),
	RedNetherBrickWall(RedNetherBrickWall),
	SandstoneWall(SandstoneWall),
	EndStoneBrickWall(EndStoneBrickWall),
	DioriteWall(DioriteWall),
	Scaffolding(Scaffolding),
	Loom(Loom),
	Barrel(Barrel),
	Smoker(Smoker),
	BlastFurnace(BlastFurnace),
	CartographyTable(CartographyTable),
	FletchingTable(FletchingTable),
	Grindstone(Grindstone),
	Lectern(Lectern),
	SmithingTable(SmithingTable),
	Stonecutter(Stonecutter),
	Bell(Bell),
	Lantern(Lantern),
	SoulLantern(SoulLantern),
	Campfire(Campfire),
	SoulCampfire(SoulCampfire),
	SweetBerryBush(SweetBerryBush),
	WarpedStem(WarpedStem),
	StrippedWarpedStem(StrippedWarpedStem),
	WarpedHyphae(WarpedHyphae),
	StrippedWarpedHyphae(StrippedWarpedHyphae),
	WarpedNylium(WarpedNylium),
	WarpedFungus(WarpedFungus),
	WarpedWartBlock(WarpedWartBlock),
	WarpedRoots(WarpedRoots),
	NetherSprouts(NetherSprouts),
	CrimsonStem(CrimsonStem),
	StrippedCrimsonStem(StrippedCrimsonStem),
	CrimsonHyphae(CrimsonHyphae),
	StrippedCrimsonHyphae(StrippedCrimsonHyphae),
	CrimsonNylium(CrimsonNylium),
	CrimsonFungus(CrimsonFungus),
	Shroomlight(Shroomlight),
	WeepingVines(WeepingVines),
	WeepingVinesPlant(WeepingVinesPlant),
	TwistingVines(TwistingVines),
	TwistingVinesPlant(TwistingVinesPlant),
	CrimsonRoots(CrimsonRoots),
	CrimsonPlanks(CrimsonPlanks),
	WarpedPlanks(WarpedPlanks),
	CrimsonSlab(CrimsonSlab),
	WarpedSlab(WarpedSlab),
	CrimsonPressurePlate(CrimsonPressurePlate),
	WarpedPressurePlate(WarpedPressurePlate),
	CrimsonFence(CrimsonFence),
	WarpedFence(WarpedFence),
	CrimsonTrapdoor(CrimsonTrapdoor),
	WarpedTrapdoor(WarpedTrapdoor),
	CrimsonFenceGate(CrimsonFenceGate),
	WarpedFenceGate(WarpedFenceGate),
	CrimsonStairs(CrimsonStairs),
	WarpedStairs(WarpedStairs),
	CrimsonButton(CrimsonButton),
	WarpedButton(WarpedButton),
	CrimsonDoor(CrimsonDoor),
	WarpedDoor(WarpedDoor),
	CrimsonSign(CrimsonSign),
	WarpedSign(WarpedSign),
	CrimsonWallSign(CrimsonWallSign),
	WarpedWallSign(WarpedWallSign),
	StructureBlock(StructureBlock),
	Jigsaw(Jigsaw),
	Composter(Composter),
	Target(Target),
	BeeNest(BeeNest),
	Beehive(Beehive),
	HoneyBlock(HoneyBlock),
	HoneycombBlock(HoneycombBlock),
	NetheriteBlock(NetheriteBlock),
	AncientDebris(AncientDebris),
	CryingObsidian(CryingObsidian),
	RespawnAnchor(RespawnAnchor),
	PottedCrimsonFungus(PottedCrimsonFungus),
	PottedWarpedFungus(PottedWarpedFungus),
	PottedCrimsonRoots(PottedCrimsonRoots),
	PottedWarpedRoots(PottedWarpedRoots),
	Lodestone(Lodestone),
	Blackstone(Blackstone),
	BlackstoneStairs(BlackstoneStairs),
	BlackstoneWall(BlackstoneWall),
	BlackstoneSlab(BlackstoneSlab),
	PolishedBlackstone(PolishedBlackstone),
	PolishedBlackstoneBricks(PolishedBlackstoneBricks),
	CrackedPolishedBlackstoneBricks(CrackedPolishedBlackstoneBricks),
	ChiseledPolishedBlackstone(ChiseledPolishedBlackstone),
	PolishedBlackstoneBrickSlab(PolishedBlackstoneBrickSlab),
	PolishedBlackstoneBrickStairs(PolishedBlackstoneBrickStairs),
	PolishedBlackstoneBrickWall(PolishedBlackstoneBrickWall),
	GildedBlackstone(GildedBlackstone),
	PolishedBlackstoneStairs(PolishedBlackstoneStairs),
	PolishedBlackstoneSlab(PolishedBlackstoneSlab),
	PolishedBlackstonePressurePlate(PolishedBlackstonePressurePlate),
	PolishedBlackstoneButton(PolishedBlackstoneButton),
	PolishedBlackstoneWall(PolishedBlackstoneWall),
	ChiseledNetherBricks(ChiseledNetherBricks),
	CrackedNetherBricks(CrackedNetherBricks),
	QuartzBricks(QuartzBricks),
	Candle(Candle),
	WhiteCandle(WhiteCandle),
	OrangeCandle(OrangeCandle),
	MagentaCandle(MagentaCandle),
	LightBlueCandle(LightBlueCandle),
	YellowCandle(YellowCandle),
	LimeCandle(LimeCandle),
	PinkCandle(PinkCandle),
	GrayCandle(GrayCandle),
	LightGrayCandle(LightGrayCandle),
	CyanCandle(CyanCandle),
	PurpleCandle(PurpleCandle),
	BlueCandle(BlueCandle),
	BrownCandle(BrownCandle),
	GreenCandle(GreenCandle),
	RedCandle(RedCandle),
	BlackCandle(BlackCandle),
	CandleCake(CandleCake),
	WhiteCandleCake(WhiteCandleCake),
	OrangeCandleCake(OrangeCandleCake),
	MagentaCandleCake(MagentaCandleCake),
	LightBlueCandleCake(LightBlueCandleCake),
	YellowCandleCake(YellowCandleCake),
	LimeCandleCake(LimeCandleCake),
	PinkCandleCake(PinkCandleCake),
	GrayCandleCake(GrayCandleCake),
	LightGrayCandleCake(LightGrayCandleCake),
	CyanCandleCake(CyanCandleCake),
	PurpleCandleCake(PurpleCandleCake),
	BlueCandleCake(BlueCandleCake),
	BrownCandleCake(BrownCandleCake),
	GreenCandleCake(GreenCandleCake),
	RedCandleCake(RedCandleCake),
	BlackCandleCake(BlackCandleCake),
	AmethystBlock(AmethystBlock),
	BuddingAmethyst(BuddingAmethyst),
	AmethystCluster(AmethystCluster),
	LargeAmethystBud(LargeAmethystBud),
	MediumAmethystBud(MediumAmethystBud),
	SmallAmethystBud(SmallAmethystBud),
	Tuff(Tuff),
	Calcite(Calcite),
	TintedGlass(TintedGlass),
	PowderSnow(PowderSnow),
	SculkSensor(SculkSensor),
	Sculk(Sculk),
	SculkVein(SculkVein),
	SculkCatalyst(SculkCatalyst),
	SculkShrieker(SculkShrieker),
	OxidizedCopper(OxidizedCopper),
	WeatheredCopper(WeatheredCopper),
	ExposedCopper(ExposedCopper),
	CopperBlock(CopperBlock),
	CopperOre(CopperOre),
	DeepslateCopperOre(DeepslateCopperOre),
	OxidizedCutCopper(OxidizedCutCopper),
	WeatheredCutCopper(WeatheredCutCopper),
	ExposedCutCopper(ExposedCutCopper),
	CutCopper(CutCopper),
	OxidizedCutCopperStairs(OxidizedCutCopperStairs),
	WeatheredCutCopperStairs(WeatheredCutCopperStairs),
	ExposedCutCopperStairs(ExposedCutCopperStairs),
	CutCopperStairs(CutCopperStairs),
	OxidizedCutCopperSlab(OxidizedCutCopperSlab),
	WeatheredCutCopperSlab(WeatheredCutCopperSlab),
	ExposedCutCopperSlab(ExposedCutCopperSlab),
	CutCopperSlab(CutCopperSlab),
	WaxedCopperBlock(WaxedCopperBlock),
	WaxedWeatheredCopper(WaxedWeatheredCopper),
	WaxedExposedCopper(WaxedExposedCopper),
	WaxedOxidizedCopper(WaxedOxidizedCopper),
	WaxedOxidizedCutCopper(WaxedOxidizedCutCopper),
	WaxedWeatheredCutCopper(WaxedWeatheredCutCopper),
	WaxedExposedCutCopper(WaxedExposedCutCopper),
	WaxedCutCopper(WaxedCutCopper),
	WaxedOxidizedCutCopperStairs(WaxedOxidizedCutCopperStairs),
	WaxedWeatheredCutCopperStairs(WaxedWeatheredCutCopperStairs),
	WaxedExposedCutCopperStairs(WaxedExposedCutCopperStairs),
	WaxedCutCopperStairs(WaxedCutCopperStairs),
	WaxedOxidizedCutCopperSlab(WaxedOxidizedCutCopperSlab),
	WaxedWeatheredCutCopperSlab(WaxedWeatheredCutCopperSlab),
	WaxedExposedCutCopperSlab(WaxedExposedCutCopperSlab),
	WaxedCutCopperSlab(WaxedCutCopperSlab),
	LightningRod(LightningRod),
	PointedDripstone(PointedDripstone),
	DripstoneBlock(DripstoneBlock),
	CaveVines(CaveVines),
	CaveVinesPlant(CaveVinesPlant),
	SporeBlossom(SporeBlossom),
	Azalea(Azalea),
	FloweringAzalea(FloweringAzalea),
	MossCarpet(MossCarpet),
	MossBlock(MossBlock),
	BigDripleaf(BigDripleaf),
	BigDripleafStem(BigDripleafStem),
	SmallDripleaf(SmallDripleaf),
	HangingRoots(HangingRoots),
	RootedDirt(RootedDirt),
	Mud(Mud),
	Deepslate(Deepslate),
	CobbledDeepslate(CobbledDeepslate),
	CobbledDeepslateStairs(CobbledDeepslateStairs),
	CobbledDeepslateSlab(CobbledDeepslateSlab),
	CobbledDeepslateWall(CobbledDeepslateWall),
	PolishedDeepslate(PolishedDeepslate),
	PolishedDeepslateStairs(PolishedDeepslateStairs),
	PolishedDeepslateSlab(PolishedDeepslateSlab),
	PolishedDeepslateWall(PolishedDeepslateWall),
	DeepslateTiles(DeepslateTiles),
	DeepslateTileStairs(DeepslateTileStairs),
	DeepslateTileSlab(DeepslateTileSlab),
	DeepslateTileWall(DeepslateTileWall),
	DeepslateBricks(DeepslateBricks),
	DeepslateBrickStairs(DeepslateBrickStairs),
	DeepslateBrickSlab(DeepslateBrickSlab),
	DeepslateBrickWall(DeepslateBrickWall),
	ChiseledDeepslate(ChiseledDeepslate),
	CrackedDeepslateBricks(CrackedDeepslateBricks),
	CrackedDeepslateTiles(CrackedDeepslateTiles),
	InfestedDeepslate(InfestedDeepslate),
	SmoothBasalt(SmoothBasalt),
	RawIronBlock(RawIronBlock),
	RawCopperBlock(RawCopperBlock),
	RawGoldBlock(RawGoldBlock),
	PottedAzaleaBush(PottedAzaleaBush),
	PottedFloweringAzaleaBush(PottedFloweringAzaleaBush),
	OchreFroglight(OchreFroglight),
	VerdantFroglight(VerdantFroglight),
	PearlescentFroglight(PearlescentFroglight),
	Frogspawn(Frogspawn),
	ReinforcedDeepslate(ReinforcedDeepslate),
}
pub fn deserialize_content<'de, T: MapAccess<'de>>(
	tag: &str,
	map: Option<&HashMap<String, String>>,
) -> Result<Block, String> {
	Ok(match tag {
		"minecraft:air" => Block::Air(Air {}),
		"minecraft:stone" => Block::Stone(Stone {}),
		"minecraft:granite" => Block::Granite(Granite {}),
		"minecraft:polished_granite" => Block::PolishedGranite(PolishedGranite {}),
		"minecraft:diorite" => Block::Diorite(Diorite {}),
		"minecraft:polished_diorite" => Block::PolishedDiorite(PolishedDiorite {}),
		"minecraft:andesite" => Block::Andesite(Andesite {}),
		"minecraft:polished_andesite" => Block::PolishedAndesite(PolishedAndesite {}),
		"minecraft:grass_block" => {
			let map = crate::ref_or_error(map)?;
			Block::GrassBlock(GrassBlock {
				r#snowy: map.get("snowy").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dirt" => Block::Dirt(Dirt {}),
		"minecraft:coarse_dirt" => Block::CoarseDirt(CoarseDirt {}),
		"minecraft:podzol" => {
			let map = crate::ref_or_error(map)?;
			Block::Podzol(Podzol {
				r#snowy: map.get("snowy").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cobblestone" => Block::Cobblestone(Cobblestone {}),
		"minecraft:oak_planks" => Block::OakPlanks(OakPlanks {}),
		"minecraft:spruce_planks" => Block::SprucePlanks(SprucePlanks {}),
		"minecraft:birch_planks" => Block::BirchPlanks(BirchPlanks {}),
		"minecraft:jungle_planks" => Block::JunglePlanks(JunglePlanks {}),
		"minecraft:acacia_planks" => Block::AcaciaPlanks(AcaciaPlanks {}),
		"minecraft:dark_oak_planks" => Block::DarkOakPlanks(DarkOakPlanks {}),
		"minecraft:mangrove_planks" => Block::MangrovePlanks(MangrovePlanks {}),
		"minecraft:oak_sapling" => {
			let map = crate::ref_or_error(map)?;
			Block::OakSapling(OakSapling {
				r#stage: map.get("stage").unwrap().parse().unwrap(),
			})
		},
		"minecraft:spruce_sapling" => {
			let map = crate::ref_or_error(map)?;
			Block::SpruceSapling(SpruceSapling {
				r#stage: map.get("stage").unwrap().parse().unwrap(),
			})
		},
		"minecraft:birch_sapling" => {
			let map = crate::ref_or_error(map)?;
			Block::BirchSapling(BirchSapling {
				r#stage: map.get("stage").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jungle_sapling" => {
			let map = crate::ref_or_error(map)?;
			Block::JungleSapling(JungleSapling {
				r#stage: map.get("stage").unwrap().parse().unwrap(),
			})
		},
		"minecraft:acacia_sapling" => {
			let map = crate::ref_or_error(map)?;
			Block::AcaciaSapling(AcaciaSapling {
				r#stage: map.get("stage").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_oak_sapling" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkOakSapling(DarkOakSapling {
				r#stage: map.get("stage").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_propagule" => {
			let map = crate::ref_or_error(map)?;
			Block::MangrovePropagule(MangrovePropagule {
				r#age: map.get("age").unwrap().parse().unwrap(),
				r#hanging: map.get("hanging").unwrap().parse().unwrap(),
				r#stage: map.get("stage").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:bedrock" => Block::Bedrock(Bedrock {}),
		"minecraft:water" => {
			let map = crate::ref_or_error(map)?;
			Block::Water(Water {
				r#level: map.get("level").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lava" => {
			let map = crate::ref_or_error(map)?;
			Block::Lava(Lava {
				r#level: map.get("level").unwrap().parse().unwrap(),
			})
		},
		"minecraft:sand" => Block::Sand(Sand {}),
		"minecraft:red_sand" => Block::RedSand(RedSand {}),
		"minecraft:gravel" => Block::Gravel(Gravel {}),
		"minecraft:gold_ore" => Block::GoldOre(GoldOre {}),
		"minecraft:deepslate_gold_ore" => Block::DeepslateGoldOre(DeepslateGoldOre {}),
		"minecraft:iron_ore" => Block::IronOre(IronOre {}),
		"minecraft:deepslate_iron_ore" => Block::DeepslateIronOre(DeepslateIronOre {}),
		"minecraft:coal_ore" => Block::CoalOre(CoalOre {}),
		"minecraft:deepslate_coal_ore" => Block::DeepslateCoalOre(DeepslateCoalOre {}),
		"minecraft:nether_gold_ore" => Block::NetherGoldOre(NetherGoldOre {}),
		"minecraft:oak_log" => {
			let map = crate::ref_or_error(map)?;
			Block::OakLog(OakLog {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:spruce_log" => {
			let map = crate::ref_or_error(map)?;
			Block::SpruceLog(SpruceLog {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:birch_log" => {
			let map = crate::ref_or_error(map)?;
			Block::BirchLog(BirchLog {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jungle_log" => {
			let map = crate::ref_or_error(map)?;
			Block::JungleLog(JungleLog {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:acacia_log" => {
			let map = crate::ref_or_error(map)?;
			Block::AcaciaLog(AcaciaLog {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_oak_log" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkOakLog(DarkOakLog {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_log" => {
			let map = crate::ref_or_error(map)?;
			Block::MangroveLog(MangroveLog {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_roots" => {
			let map = crate::ref_or_error(map)?;
			Block::MangroveRoots(MangroveRoots {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:muddy_mangrove_roots" => {
			let map = crate::ref_or_error(map)?;
			Block::MuddyMangroveRoots(MuddyMangroveRoots {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_spruce_log" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedSpruceLog(StrippedSpruceLog {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_birch_log" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedBirchLog(StrippedBirchLog {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_jungle_log" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedJungleLog(StrippedJungleLog {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_acacia_log" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedAcaciaLog(StrippedAcaciaLog {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_dark_oak_log" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedDarkOakLog(StrippedDarkOakLog {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_oak_log" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedOakLog(StrippedOakLog {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_mangrove_log" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedMangroveLog(StrippedMangroveLog {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:oak_wood" => {
			let map = crate::ref_or_error(map)?;
			Block::OakWood(OakWood {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:spruce_wood" => {
			let map = crate::ref_or_error(map)?;
			Block::SpruceWood(SpruceWood {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:birch_wood" => {
			let map = crate::ref_or_error(map)?;
			Block::BirchWood(BirchWood {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jungle_wood" => {
			let map = crate::ref_or_error(map)?;
			Block::JungleWood(JungleWood {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:acacia_wood" => {
			let map = crate::ref_or_error(map)?;
			Block::AcaciaWood(AcaciaWood {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_oak_wood" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkOakWood(DarkOakWood {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_wood" => {
			let map = crate::ref_or_error(map)?;
			Block::MangroveWood(MangroveWood {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_oak_wood" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedOakWood(StrippedOakWood {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_spruce_wood" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedSpruceWood(StrippedSpruceWood {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_birch_wood" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedBirchWood(StrippedBirchWood {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_jungle_wood" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedJungleWood(StrippedJungleWood {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_acacia_wood" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedAcaciaWood(StrippedAcaciaWood {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_dark_oak_wood" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedDarkOakWood(StrippedDarkOakWood {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_mangrove_wood" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedMangroveWood(StrippedMangroveWood {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:oak_leaves" => {
			let map = crate::ref_or_error(map)?;
			Block::OakLeaves(OakLeaves {
				r#distance: map.get("distance").unwrap().parse().unwrap(),
				r#persistent: map.get("persistent").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:spruce_leaves" => {
			let map = crate::ref_or_error(map)?;
			Block::SpruceLeaves(SpruceLeaves {
				r#distance: map.get("distance").unwrap().parse().unwrap(),
				r#persistent: map.get("persistent").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:birch_leaves" => {
			let map = crate::ref_or_error(map)?;
			Block::BirchLeaves(BirchLeaves {
				r#distance: map.get("distance").unwrap().parse().unwrap(),
				r#persistent: map.get("persistent").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jungle_leaves" => {
			let map = crate::ref_or_error(map)?;
			Block::JungleLeaves(JungleLeaves {
				r#distance: map.get("distance").unwrap().parse().unwrap(),
				r#persistent: map.get("persistent").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:acacia_leaves" => {
			let map = crate::ref_or_error(map)?;
			Block::AcaciaLeaves(AcaciaLeaves {
				r#distance: map.get("distance").unwrap().parse().unwrap(),
				r#persistent: map.get("persistent").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_oak_leaves" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkOakLeaves(DarkOakLeaves {
				r#distance: map.get("distance").unwrap().parse().unwrap(),
				r#persistent: map.get("persistent").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_leaves" => {
			let map = crate::ref_or_error(map)?;
			Block::MangroveLeaves(MangroveLeaves {
				r#distance: map.get("distance").unwrap().parse().unwrap(),
				r#persistent: map.get("persistent").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:azalea_leaves" => {
			let map = crate::ref_or_error(map)?;
			Block::AzaleaLeaves(AzaleaLeaves {
				r#distance: map.get("distance").unwrap().parse().unwrap(),
				r#persistent: map.get("persistent").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:flowering_azalea_leaves" => {
			let map = crate::ref_or_error(map)?;
			Block::FloweringAzaleaLeaves(FloweringAzaleaLeaves {
				r#distance: map.get("distance").unwrap().parse().unwrap(),
				r#persistent: map.get("persistent").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:sponge" => Block::Sponge(Sponge {}),
		"minecraft:wet_sponge" => Block::WetSponge(WetSponge {}),
		"minecraft:glass" => Block::Glass(Glass {}),
		"minecraft:lapis_ore" => Block::LapisOre(LapisOre {}),
		"minecraft:deepslate_lapis_ore" => Block::DeepslateLapisOre(DeepslateLapisOre {}),
		"minecraft:lapis_block" => Block::LapisBlock(LapisBlock {}),
		"minecraft:dispenser" => {
			let map = crate::ref_or_error(map)?;
			Block::Dispenser(Dispenser {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#triggered: map.get("triggered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:sandstone" => Block::Sandstone(Sandstone {}),
		"minecraft:chiseled_sandstone" => Block::ChiseledSandstone(ChiseledSandstone {}),
		"minecraft:cut_sandstone" => Block::CutSandstone(CutSandstone {}),
		"minecraft:note_block" => {
			let map = crate::ref_or_error(map)?;
			Block::NoteBlock(NoteBlock {
				r#instrument: map.get("instrument").unwrap().parse().unwrap(),
				r#note: map.get("note").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:white_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::WhiteBed(WhiteBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:orange_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::OrangeBed(OrangeBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:magenta_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::MagentaBed(MagentaBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_blue_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::LightBlueBed(LightBlueBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:yellow_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::YellowBed(YellowBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lime_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::LimeBed(LimeBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:pink_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::PinkBed(PinkBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:gray_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::GrayBed(GrayBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_gray_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::LightGrayBed(LightGrayBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cyan_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::CyanBed(CyanBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:purple_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::PurpleBed(PurpleBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:blue_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::BlueBed(BlueBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:brown_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::BrownBed(BrownBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:green_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::GreenBed(GreenBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::RedBed(RedBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:black_bed" => {
			let map = crate::ref_or_error(map)?;
			Block::BlackBed(BlackBed {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#occupied: map.get("occupied").unwrap().parse().unwrap(),
				r#part: map.get("part").unwrap().parse().unwrap(),
			})
		},
		"minecraft:powered_rail" => {
			let map = crate::ref_or_error(map)?;
			Block::PoweredRail(PoweredRail {
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:detector_rail" => {
			let map = crate::ref_or_error(map)?;
			Block::DetectorRail(DetectorRail {
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:sticky_piston" => {
			let map = crate::ref_or_error(map)?;
			Block::StickyPiston(StickyPiston {
				r#extended: map.get("extended").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cobweb" => Block::Cobweb(Cobweb {}),
		"minecraft:grass" => Block::Grass(Grass {}),
		"minecraft:fern" => Block::Fern(Fern {}),
		"minecraft:dead_bush" => Block::DeadBush(DeadBush {}),
		"minecraft:seagrass" => Block::Seagrass(Seagrass {}),
		"minecraft:tall_seagrass" => {
			let map = crate::ref_or_error(map)?;
			Block::TallSeagrass(TallSeagrass {
				r#half: map.get("half").unwrap().parse().unwrap(),
			})
		},
		"minecraft:piston" => {
			let map = crate::ref_or_error(map)?;
			Block::Piston(Piston {
				r#extended: map.get("extended").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:piston_head" => {
			let map = crate::ref_or_error(map)?;
			Block::PistonHead(PistonHead {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#short: map.get("short").unwrap().parse().unwrap(),
				r#type: map.get("type").unwrap().parse().unwrap(),
			})
		},
		"minecraft:white_wool" => Block::WhiteWool(WhiteWool {}),
		"minecraft:orange_wool" => Block::OrangeWool(OrangeWool {}),
		"minecraft:magenta_wool" => Block::MagentaWool(MagentaWool {}),
		"minecraft:light_blue_wool" => Block::LightBlueWool(LightBlueWool {}),
		"minecraft:yellow_wool" => Block::YellowWool(YellowWool {}),
		"minecraft:lime_wool" => Block::LimeWool(LimeWool {}),
		"minecraft:pink_wool" => Block::PinkWool(PinkWool {}),
		"minecraft:gray_wool" => Block::GrayWool(GrayWool {}),
		"minecraft:light_gray_wool" => Block::LightGrayWool(LightGrayWool {}),
		"minecraft:cyan_wool" => Block::CyanWool(CyanWool {}),
		"minecraft:purple_wool" => Block::PurpleWool(PurpleWool {}),
		"minecraft:blue_wool" => Block::BlueWool(BlueWool {}),
		"minecraft:brown_wool" => Block::BrownWool(BrownWool {}),
		"minecraft:green_wool" => Block::GreenWool(GreenWool {}),
		"minecraft:red_wool" => Block::RedWool(RedWool {}),
		"minecraft:black_wool" => Block::BlackWool(BlackWool {}),
		"minecraft:moving_piston" => {
			let map = crate::ref_or_error(map)?;
			Block::MovingPiston(MovingPiston {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#type: map.get("type").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dandelion" => Block::Dandelion(Dandelion {}),
		"minecraft:poppy" => Block::Poppy(Poppy {}),
		"minecraft:blue_orchid" => Block::BlueOrchid(BlueOrchid {}),
		"minecraft:allium" => Block::Allium(Allium {}),
		"minecraft:azure_bluet" => Block::AzureBluet(AzureBluet {}),
		"minecraft:red_tulip" => Block::RedTulip(RedTulip {}),
		"minecraft:orange_tulip" => Block::OrangeTulip(OrangeTulip {}),
		"minecraft:white_tulip" => Block::WhiteTulip(WhiteTulip {}),
		"minecraft:pink_tulip" => Block::PinkTulip(PinkTulip {}),
		"minecraft:oxeye_daisy" => Block::OxeyeDaisy(OxeyeDaisy {}),
		"minecraft:cornflower" => Block::Cornflower(Cornflower {}),
		"minecraft:wither_rose" => Block::WitherRose(WitherRose {}),
		"minecraft:lily_of_the_valley" => Block::LilyOfTheValley(LilyOfTheValley {}),
		"minecraft:brown_mushroom" => Block::BrownMushroom(BrownMushroom {}),
		"minecraft:red_mushroom" => Block::RedMushroom(RedMushroom {}),
		"minecraft:gold_block" => Block::GoldBlock(GoldBlock {}),
		"minecraft:iron_block" => Block::IronBlock(IronBlock {}),
		"minecraft:bricks" => Block::Bricks(Bricks {}),
		"minecraft:tnt" => {
			let map = crate::ref_or_error(map)?;
			Block::Tnt(Tnt {
				r#unstable: map.get("unstable").unwrap().parse().unwrap(),
			})
		},
		"minecraft:bookshelf" => Block::Bookshelf(Bookshelf {}),
		"minecraft:mossy_cobblestone" => Block::MossyCobblestone(MossyCobblestone {}),
		"minecraft:obsidian" => Block::Obsidian(Obsidian {}),
		"minecraft:torch" => Block::Torch(Torch {}),
		"minecraft:wall_torch" => {
			let map = crate::ref_or_error(map)?;
			Block::WallTorch(WallTorch {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:fire" => {
			let map = crate::ref_or_error(map)?;
			Block::Fire(Fire {
				r#age: map.get("age").unwrap().parse().unwrap(),
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:soul_fire" => Block::SoulFire(SoulFire {}),
		"minecraft:spawner" => Block::Spawner(Spawner {}),
		"minecraft:oak_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::OakStairs(OakStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:chest" => {
			let map = crate::ref_or_error(map)?;
			Block::Chest(Chest {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:redstone_wire" => {
			let map = crate::ref_or_error(map)?;
			Block::RedstoneWire(RedstoneWire {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#power: map.get("power").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:diamond_ore" => Block::DiamondOre(DiamondOre {}),
		"minecraft:deepslate_diamond_ore" => Block::DeepslateDiamondOre(DeepslateDiamondOre {}),
		"minecraft:diamond_block" => Block::DiamondBlock(DiamondBlock {}),
		"minecraft:crafting_table" => Block::CraftingTable(CraftingTable {}),
		"minecraft:wheat" => {
			let map = crate::ref_or_error(map)?;
			Block::Wheat(Wheat {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:farmland" => {
			let map = crate::ref_or_error(map)?;
			Block::Farmland(Farmland {
				r#moisture: map.get("moisture").unwrap().parse().unwrap(),
			})
		},
		"minecraft:furnace" => {
			let map = crate::ref_or_error(map)?;
			Block::Furnace(Furnace {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:oak_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::OakSign(OakSign {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:spruce_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::SpruceSign(SpruceSign {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:birch_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::BirchSign(BirchSign {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:acacia_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::AcaciaSign(AcaciaSign {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jungle_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::JungleSign(JungleSign {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_oak_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkOakSign(DarkOakSign {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::MangroveSign(MangroveSign {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:oak_door" => {
			let map = crate::ref_or_error(map)?;
			Block::OakDoor(OakDoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#hinge: map.get("hinge").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:ladder" => {
			let map = crate::ref_or_error(map)?;
			Block::Ladder(Ladder {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:rail" => {
			let map = crate::ref_or_error(map)?;
			Block::Rail(Rail {
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cobblestone_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::CobblestoneStairs(CobblestoneStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:oak_wall_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::OakWallSign(OakWallSign {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:spruce_wall_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::SpruceWallSign(SpruceWallSign {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:birch_wall_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::BirchWallSign(BirchWallSign {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:acacia_wall_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::AcaciaWallSign(AcaciaWallSign {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jungle_wall_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::JungleWallSign(JungleWallSign {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_oak_wall_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkOakWallSign(DarkOakWallSign {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_wall_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::MangroveWallSign(MangroveWallSign {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lever" => {
			let map = crate::ref_or_error(map)?;
			Block::Lever(Lever {
				r#face: map.get("face").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stone_pressure_plate" => {
			let map = crate::ref_or_error(map)?;
			Block::StonePressurePlate(StonePressurePlate {
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:iron_door" => {
			let map = crate::ref_or_error(map)?;
			Block::IronDoor(IronDoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#hinge: map.get("hinge").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:oak_pressure_plate" => {
			let map = crate::ref_or_error(map)?;
			Block::OakPressurePlate(OakPressurePlate {
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:spruce_pressure_plate" => {
			let map = crate::ref_or_error(map)?;
			Block::SprucePressurePlate(SprucePressurePlate {
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:birch_pressure_plate" => {
			let map = crate::ref_or_error(map)?;
			Block::BirchPressurePlate(BirchPressurePlate {
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jungle_pressure_plate" => {
			let map = crate::ref_or_error(map)?;
			Block::JunglePressurePlate(JunglePressurePlate {
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:acacia_pressure_plate" => {
			let map = crate::ref_or_error(map)?;
			Block::AcaciaPressurePlate(AcaciaPressurePlate {
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_oak_pressure_plate" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkOakPressurePlate(DarkOakPressurePlate {
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_pressure_plate" => {
			let map = crate::ref_or_error(map)?;
			Block::MangrovePressurePlate(MangrovePressurePlate {
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:redstone_ore" => {
			let map = crate::ref_or_error(map)?;
			Block::RedstoneOre(RedstoneOre {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:deepslate_redstone_ore" => {
			let map = crate::ref_or_error(map)?;
			Block::DeepslateRedstoneOre(DeepslateRedstoneOre {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:redstone_torch" => {
			let map = crate::ref_or_error(map)?;
			Block::RedstoneTorch(RedstoneTorch {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:redstone_wall_torch" => {
			let map = crate::ref_or_error(map)?;
			Block::RedstoneWallTorch(RedstoneWallTorch {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stone_button" => {
			let map = crate::ref_or_error(map)?;
			Block::StoneButton(StoneButton {
				r#face: map.get("face").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:snow" => {
			let map = crate::ref_or_error(map)?;
			Block::Snow(Snow {
				r#layers: map.get("layers").unwrap().parse().unwrap(),
			})
		},
		"minecraft:ice" => Block::Ice(Ice {}),
		"minecraft:snow_block" => Block::SnowBlock(SnowBlock {}),
		"minecraft:cactus" => {
			let map = crate::ref_or_error(map)?;
			Block::Cactus(Cactus {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:clay" => Block::Clay(Clay {}),
		"minecraft:sugar_cane" => {
			let map = crate::ref_or_error(map)?;
			Block::SugarCane(SugarCane {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jukebox" => {
			let map = crate::ref_or_error(map)?;
			Block::Jukebox(Jukebox {
				r#has_record: map.get("has_record").unwrap().parse().unwrap(),
			})
		},
		"minecraft:oak_fence" => {
			let map = crate::ref_or_error(map)?;
			Block::OakFence(OakFence {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:pumpkin" => Block::Pumpkin(Pumpkin {}),
		"minecraft:netherrack" => Block::Netherrack(Netherrack {}),
		"minecraft:soul_sand" => Block::SoulSand(SoulSand {}),
		"minecraft:soul_soil" => Block::SoulSoil(SoulSoil {}),
		"minecraft:basalt" => {
			let map = crate::ref_or_error(map)?;
			Block::Basalt(Basalt {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_basalt" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedBasalt(PolishedBasalt {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:soul_torch" => Block::SoulTorch(SoulTorch {}),
		"minecraft:soul_wall_torch" => {
			let map = crate::ref_or_error(map)?;
			Block::SoulWallTorch(SoulWallTorch {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:glowstone" => Block::Glowstone(Glowstone {}),
		"minecraft:nether_portal" => {
			let map = crate::ref_or_error(map)?;
			Block::NetherPortal(NetherPortal {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:carved_pumpkin" => {
			let map = crate::ref_or_error(map)?;
			Block::CarvedPumpkin(CarvedPumpkin {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jack_o_lantern" => {
			let map = crate::ref_or_error(map)?;
			Block::JackOLantern(JackOLantern {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cake" => {
			let map = crate::ref_or_error(map)?;
			Block::Cake(Cake {
				r#bites: map.get("bites").unwrap().parse().unwrap(),
			})
		},
		"minecraft:repeater" => {
			let map = crate::ref_or_error(map)?;
			Block::Repeater(Repeater {
				r#delay: map.get("delay").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#locked: map.get("locked").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:white_stained_glass" => Block::WhiteStainedGlass(WhiteStainedGlass {}),
		"minecraft:orange_stained_glass" => Block::OrangeStainedGlass(OrangeStainedGlass {}),
		"minecraft:magenta_stained_glass" => Block::MagentaStainedGlass(MagentaStainedGlass {}),
		"minecraft:light_blue_stained_glass" => Block::LightBlueStainedGlass(LightBlueStainedGlass {}),
		"minecraft:yellow_stained_glass" => Block::YellowStainedGlass(YellowStainedGlass {}),
		"minecraft:lime_stained_glass" => Block::LimeStainedGlass(LimeStainedGlass {}),
		"minecraft:pink_stained_glass" => Block::PinkStainedGlass(PinkStainedGlass {}),
		"minecraft:gray_stained_glass" => Block::GrayStainedGlass(GrayStainedGlass {}),
		"minecraft:light_gray_stained_glass" => Block::LightGrayStainedGlass(LightGrayStainedGlass {}),
		"minecraft:cyan_stained_glass" => Block::CyanStainedGlass(CyanStainedGlass {}),
		"minecraft:purple_stained_glass" => Block::PurpleStainedGlass(PurpleStainedGlass {}),
		"minecraft:blue_stained_glass" => Block::BlueStainedGlass(BlueStainedGlass {}),
		"minecraft:brown_stained_glass" => Block::BrownStainedGlass(BrownStainedGlass {}),
		"minecraft:green_stained_glass" => Block::GreenStainedGlass(GreenStainedGlass {}),
		"minecraft:red_stained_glass" => Block::RedStainedGlass(RedStainedGlass {}),
		"minecraft:black_stained_glass" => Block::BlackStainedGlass(BlackStainedGlass {}),
		"minecraft:oak_trapdoor" => {
			let map = crate::ref_or_error(map)?;
			Block::OakTrapdoor(OakTrapdoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:spruce_trapdoor" => {
			let map = crate::ref_or_error(map)?;
			Block::SpruceTrapdoor(SpruceTrapdoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:birch_trapdoor" => {
			let map = crate::ref_or_error(map)?;
			Block::BirchTrapdoor(BirchTrapdoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jungle_trapdoor" => {
			let map = crate::ref_or_error(map)?;
			Block::JungleTrapdoor(JungleTrapdoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:acacia_trapdoor" => {
			let map = crate::ref_or_error(map)?;
			Block::AcaciaTrapdoor(AcaciaTrapdoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_oak_trapdoor" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkOakTrapdoor(DarkOakTrapdoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_trapdoor" => {
			let map = crate::ref_or_error(map)?;
			Block::MangroveTrapdoor(MangroveTrapdoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stone_bricks" => Block::StoneBricks(StoneBricks {}),
		"minecraft:mossy_stone_bricks" => Block::MossyStoneBricks(MossyStoneBricks {}),
		"minecraft:cracked_stone_bricks" => Block::CrackedStoneBricks(CrackedStoneBricks {}),
		"minecraft:chiseled_stone_bricks" => Block::ChiseledStoneBricks(ChiseledStoneBricks {}),
		"minecraft:packed_mud" => Block::PackedMud(PackedMud {}),
		"minecraft:mud_bricks" => Block::MudBricks(MudBricks {}),
		"minecraft:infested_stone" => Block::InfestedStone(InfestedStone {}),
		"minecraft:infested_cobblestone" => Block::InfestedCobblestone(InfestedCobblestone {}),
		"minecraft:infested_stone_bricks" => Block::InfestedStoneBricks(InfestedStoneBricks {}),
		"minecraft:infested_mossy_stone_bricks" => Block::InfestedMossyStoneBricks(InfestedMossyStoneBricks {}),
		"minecraft:infested_cracked_stone_bricks" => Block::InfestedCrackedStoneBricks(InfestedCrackedStoneBricks {}),
		"minecraft:infested_chiseled_stone_bricks" => Block::InfestedChiseledStoneBricks(InfestedChiseledStoneBricks {}),
		"minecraft:brown_mushroom_block" => {
			let map = crate::ref_or_error(map)?;
			Block::BrownMushroomBlock(BrownMushroomBlock {
				r#down: map.get("down").unwrap().parse().unwrap(),
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_mushroom_block" => {
			let map = crate::ref_or_error(map)?;
			Block::RedMushroomBlock(RedMushroomBlock {
				r#down: map.get("down").unwrap().parse().unwrap(),
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mushroom_stem" => {
			let map = crate::ref_or_error(map)?;
			Block::MushroomStem(MushroomStem {
				r#down: map.get("down").unwrap().parse().unwrap(),
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:iron_bars" => {
			let map = crate::ref_or_error(map)?;
			Block::IronBars(IronBars {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:chain" => {
			let map = crate::ref_or_error(map)?;
			Block::Chain(Chain {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::GlassPane(GlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:melon" => Block::Melon(Melon {}),
		"minecraft:attached_pumpkin_stem" => {
			let map = crate::ref_or_error(map)?;
			Block::AttachedPumpkinStem(AttachedPumpkinStem {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:attached_melon_stem" => {
			let map = crate::ref_or_error(map)?;
			Block::AttachedMelonStem(AttachedMelonStem {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:pumpkin_stem" => {
			let map = crate::ref_or_error(map)?;
			Block::PumpkinStem(PumpkinStem {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:melon_stem" => {
			let map = crate::ref_or_error(map)?;
			Block::MelonStem(MelonStem {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:vine" => {
			let map = crate::ref_or_error(map)?;
			Block::Vine(Vine {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:glow_lichen" => {
			let map = crate::ref_or_error(map)?;
			Block::GlowLichen(GlowLichen {
				r#down: map.get("down").unwrap().parse().unwrap(),
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:oak_fence_gate" => {
			let map = crate::ref_or_error(map)?;
			Block::OakFenceGate(OakFenceGate {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#in_wall: map.get("in_wall").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:brick_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::BrickStairs(BrickStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stone_brick_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::StoneBrickStairs(StoneBrickStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mud_brick_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::MudBrickStairs(MudBrickStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mycelium" => {
			let map = crate::ref_or_error(map)?;
			Block::Mycelium(Mycelium {
				r#snowy: map.get("snowy").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lily_pad" => Block::LilyPad(LilyPad {}),
		"minecraft:nether_bricks" => Block::NetherBricks(NetherBricks {}),
		"minecraft:nether_brick_fence" => {
			let map = crate::ref_or_error(map)?;
			Block::NetherBrickFence(NetherBrickFence {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:nether_brick_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::NetherBrickStairs(NetherBrickStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:nether_wart" => {
			let map = crate::ref_or_error(map)?;
			Block::NetherWart(NetherWart {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:enchanting_table" => Block::EnchantingTable(EnchantingTable {}),
		"minecraft:brewing_stand" => {
			let map = crate::ref_or_error(map)?;
			Block::BrewingStand(BrewingStand {
				r#has_bottle_0: map.get("has_bottle_0").unwrap().parse().unwrap(),
				r#has_bottle_1: map.get("has_bottle_1").unwrap().parse().unwrap(),
				r#has_bottle_2: map.get("has_bottle_2").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cauldron" => Block::Cauldron(Cauldron {}),
		"minecraft:water_cauldron" => {
			let map = crate::ref_or_error(map)?;
			Block::WaterCauldron(WaterCauldron {
				r#level: map.get("level").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lava_cauldron" => Block::LavaCauldron(LavaCauldron {}),
		"minecraft:powder_snow_cauldron" => {
			let map = crate::ref_or_error(map)?;
			Block::PowderSnowCauldron(PowderSnowCauldron {
				r#level: map.get("level").unwrap().parse().unwrap(),
			})
		},
		"minecraft:end_portal" => Block::EndPortal(EndPortal {}),
		"minecraft:end_portal_frame" => {
			let map = crate::ref_or_error(map)?;
			Block::EndPortalFrame(EndPortalFrame {
				r#eye: map.get("eye").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:end_stone" => Block::EndStone(EndStone {}),
		"minecraft:dragon_egg" => Block::DragonEgg(DragonEgg {}),
		"minecraft:redstone_lamp" => {
			let map = crate::ref_or_error(map)?;
			Block::RedstoneLamp(RedstoneLamp {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cocoa" => {
			let map = crate::ref_or_error(map)?;
			Block::Cocoa(Cocoa {
				r#age: map.get("age").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:sandstone_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::SandstoneStairs(SandstoneStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:emerald_ore" => Block::EmeraldOre(EmeraldOre {}),
		"minecraft:deepslate_emerald_ore" => Block::DeepslateEmeraldOre(DeepslateEmeraldOre {}),
		"minecraft:ender_chest" => {
			let map = crate::ref_or_error(map)?;
			Block::EnderChest(EnderChest {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:tripwire_hook" => {
			let map = crate::ref_or_error(map)?;
			Block::TripwireHook(TripwireHook {
				r#attached: map.get("attached").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:tripwire" => {
			let map = crate::ref_or_error(map)?;
			Block::Tripwire(Tripwire {
				r#attached: map.get("attached").unwrap().parse().unwrap(),
				r#disarmed: map.get("disarmed").unwrap().parse().unwrap(),
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:emerald_block" => Block::EmeraldBlock(EmeraldBlock {}),
		"minecraft:spruce_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::SpruceStairs(SpruceStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:birch_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::BirchStairs(BirchStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jungle_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::JungleStairs(JungleStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:command_block" => {
			let map = crate::ref_or_error(map)?;
			Block::CommandBlock(CommandBlock {
				r#conditional: map.get("conditional").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:beacon" => Block::Beacon(Beacon {}),
		"minecraft:cobblestone_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::CobblestoneWall(CobblestoneWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mossy_cobblestone_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::MossyCobblestoneWall(MossyCobblestoneWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:flower_pot" => Block::FlowerPot(FlowerPot {}),
		"minecraft:potted_oak_sapling" => Block::PottedOakSapling(PottedOakSapling {}),
		"minecraft:potted_spruce_sapling" => Block::PottedSpruceSapling(PottedSpruceSapling {}),
		"minecraft:potted_birch_sapling" => Block::PottedBirchSapling(PottedBirchSapling {}),
		"minecraft:potted_jungle_sapling" => Block::PottedJungleSapling(PottedJungleSapling {}),
		"minecraft:potted_acacia_sapling" => Block::PottedAcaciaSapling(PottedAcaciaSapling {}),
		"minecraft:potted_dark_oak_sapling" => Block::PottedDarkOakSapling(PottedDarkOakSapling {}),
		"minecraft:potted_mangrove_propagule" => Block::PottedMangrovePropagule(PottedMangrovePropagule {}),
		"minecraft:potted_fern" => Block::PottedFern(PottedFern {}),
		"minecraft:potted_dandelion" => Block::PottedDandelion(PottedDandelion {}),
		"minecraft:potted_poppy" => Block::PottedPoppy(PottedPoppy {}),
		"minecraft:potted_blue_orchid" => Block::PottedBlueOrchid(PottedBlueOrchid {}),
		"minecraft:potted_allium" => Block::PottedAllium(PottedAllium {}),
		"minecraft:potted_azure_bluet" => Block::PottedAzureBluet(PottedAzureBluet {}),
		"minecraft:potted_red_tulip" => Block::PottedRedTulip(PottedRedTulip {}),
		"minecraft:potted_orange_tulip" => Block::PottedOrangeTulip(PottedOrangeTulip {}),
		"minecraft:potted_white_tulip" => Block::PottedWhiteTulip(PottedWhiteTulip {}),
		"minecraft:potted_pink_tulip" => Block::PottedPinkTulip(PottedPinkTulip {}),
		"minecraft:potted_oxeye_daisy" => Block::PottedOxeyeDaisy(PottedOxeyeDaisy {}),
		"minecraft:potted_cornflower" => Block::PottedCornflower(PottedCornflower {}),
		"minecraft:potted_lily_of_the_valley" => Block::PottedLilyOfTheValley(PottedLilyOfTheValley {}),
		"minecraft:potted_wither_rose" => Block::PottedWitherRose(PottedWitherRose {}),
		"minecraft:potted_red_mushroom" => Block::PottedRedMushroom(PottedRedMushroom {}),
		"minecraft:potted_brown_mushroom" => Block::PottedBrownMushroom(PottedBrownMushroom {}),
		"minecraft:potted_dead_bush" => Block::PottedDeadBush(PottedDeadBush {}),
		"minecraft:potted_cactus" => Block::PottedCactus(PottedCactus {}),
		"minecraft:carrots" => {
			let map = crate::ref_or_error(map)?;
			Block::Carrots(Carrots {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:potatoes" => {
			let map = crate::ref_or_error(map)?;
			Block::Potatoes(Potatoes {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:oak_button" => {
			let map = crate::ref_or_error(map)?;
			Block::OakButton(OakButton {
				r#face: map.get("face").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:spruce_button" => {
			let map = crate::ref_or_error(map)?;
			Block::SpruceButton(SpruceButton {
				r#face: map.get("face").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:birch_button" => {
			let map = crate::ref_or_error(map)?;
			Block::BirchButton(BirchButton {
				r#face: map.get("face").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jungle_button" => {
			let map = crate::ref_or_error(map)?;
			Block::JungleButton(JungleButton {
				r#face: map.get("face").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:acacia_button" => {
			let map = crate::ref_or_error(map)?;
			Block::AcaciaButton(AcaciaButton {
				r#face: map.get("face").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_oak_button" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkOakButton(DarkOakButton {
				r#face: map.get("face").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_button" => {
			let map = crate::ref_or_error(map)?;
			Block::MangroveButton(MangroveButton {
				r#face: map.get("face").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:skeleton_skull" => {
			let map = crate::ref_or_error(map)?;
			Block::SkeletonSkull(SkeletonSkull {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:skeleton_wall_skull" => {
			let map = crate::ref_or_error(map)?;
			Block::SkeletonWallSkull(SkeletonWallSkull {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:wither_skeleton_skull" => {
			let map = crate::ref_or_error(map)?;
			Block::WitherSkeletonSkull(WitherSkeletonSkull {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:wither_skeleton_wall_skull" => {
			let map = crate::ref_or_error(map)?;
			Block::WitherSkeletonWallSkull(WitherSkeletonWallSkull {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:zombie_head" => {
			let map = crate::ref_or_error(map)?;
			Block::ZombieHead(ZombieHead {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:zombie_wall_head" => {
			let map = crate::ref_or_error(map)?;
			Block::ZombieWallHead(ZombieWallHead {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:player_head" => {
			let map = crate::ref_or_error(map)?;
			Block::PlayerHead(PlayerHead {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:player_wall_head" => {
			let map = crate::ref_or_error(map)?;
			Block::PlayerWallHead(PlayerWallHead {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:creeper_head" => {
			let map = crate::ref_or_error(map)?;
			Block::CreeperHead(CreeperHead {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:creeper_wall_head" => {
			let map = crate::ref_or_error(map)?;
			Block::CreeperWallHead(CreeperWallHead {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dragon_head" => {
			let map = crate::ref_or_error(map)?;
			Block::DragonHead(DragonHead {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dragon_wall_head" => {
			let map = crate::ref_or_error(map)?;
			Block::DragonWallHead(DragonWallHead {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:anvil" => {
			let map = crate::ref_or_error(map)?;
			Block::Anvil(Anvil {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:chipped_anvil" => {
			let map = crate::ref_or_error(map)?;
			Block::ChippedAnvil(ChippedAnvil {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:damaged_anvil" => {
			let map = crate::ref_or_error(map)?;
			Block::DamagedAnvil(DamagedAnvil {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:trapped_chest" => {
			let map = crate::ref_or_error(map)?;
			Block::TrappedChest(TrappedChest {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_weighted_pressure_plate" => {
			let map = crate::ref_or_error(map)?;
			Block::LightWeightedPressurePlate(LightWeightedPressurePlate {
				r#power: map.get("power").unwrap().parse().unwrap(),
			})
		},
		"minecraft:heavy_weighted_pressure_plate" => {
			let map = crate::ref_or_error(map)?;
			Block::HeavyWeightedPressurePlate(HeavyWeightedPressurePlate {
				r#power: map.get("power").unwrap().parse().unwrap(),
			})
		},
		"minecraft:comparator" => {
			let map = crate::ref_or_error(map)?;
			Block::Comparator(Comparator {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#mode: map.get("mode").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:daylight_detector" => {
			let map = crate::ref_or_error(map)?;
			Block::DaylightDetector(DaylightDetector {
				r#inverted: map.get("inverted").unwrap().parse().unwrap(),
				r#power: map.get("power").unwrap().parse().unwrap(),
			})
		},
		"minecraft:redstone_block" => Block::RedstoneBlock(RedstoneBlock {}),
		"minecraft:nether_quartz_ore" => Block::NetherQuartzOre(NetherQuartzOre {}),
		"minecraft:hopper" => {
			let map = crate::ref_or_error(map)?;
			Block::Hopper(Hopper {
				r#enabled: map.get("enabled").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:quartz_block" => Block::QuartzBlock(QuartzBlock {}),
		"minecraft:chiseled_quartz_block" => Block::ChiseledQuartzBlock(ChiseledQuartzBlock {}),
		"minecraft:quartz_pillar" => {
			let map = crate::ref_or_error(map)?;
			Block::QuartzPillar(QuartzPillar {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:quartz_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::QuartzStairs(QuartzStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:activator_rail" => {
			let map = crate::ref_or_error(map)?;
			Block::ActivatorRail(ActivatorRail {
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dropper" => {
			let map = crate::ref_or_error(map)?;
			Block::Dropper(Dropper {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#triggered: map.get("triggered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:white_terracotta" => Block::WhiteTerracotta(WhiteTerracotta {}),
		"minecraft:orange_terracotta" => Block::OrangeTerracotta(OrangeTerracotta {}),
		"minecraft:magenta_terracotta" => Block::MagentaTerracotta(MagentaTerracotta {}),
		"minecraft:light_blue_terracotta" => Block::LightBlueTerracotta(LightBlueTerracotta {}),
		"minecraft:yellow_terracotta" => Block::YellowTerracotta(YellowTerracotta {}),
		"minecraft:lime_terracotta" => Block::LimeTerracotta(LimeTerracotta {}),
		"minecraft:pink_terracotta" => Block::PinkTerracotta(PinkTerracotta {}),
		"minecraft:gray_terracotta" => Block::GrayTerracotta(GrayTerracotta {}),
		"minecraft:light_gray_terracotta" => Block::LightGrayTerracotta(LightGrayTerracotta {}),
		"minecraft:cyan_terracotta" => Block::CyanTerracotta(CyanTerracotta {}),
		"minecraft:purple_terracotta" => Block::PurpleTerracotta(PurpleTerracotta {}),
		"minecraft:blue_terracotta" => Block::BlueTerracotta(BlueTerracotta {}),
		"minecraft:brown_terracotta" => Block::BrownTerracotta(BrownTerracotta {}),
		"minecraft:green_terracotta" => Block::GreenTerracotta(GreenTerracotta {}),
		"minecraft:red_terracotta" => Block::RedTerracotta(RedTerracotta {}),
		"minecraft:black_terracotta" => Block::BlackTerracotta(BlackTerracotta {}),
		"minecraft:white_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::WhiteStainedGlassPane(WhiteStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:orange_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::OrangeStainedGlassPane(OrangeStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:magenta_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::MagentaStainedGlassPane(MagentaStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_blue_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::LightBlueStainedGlassPane(LightBlueStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:yellow_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::YellowStainedGlassPane(YellowStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lime_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::LimeStainedGlassPane(LimeStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:pink_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::PinkStainedGlassPane(PinkStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:gray_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::GrayStainedGlassPane(GrayStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_gray_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::LightGrayStainedGlassPane(LightGrayStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cyan_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::CyanStainedGlassPane(CyanStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:purple_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::PurpleStainedGlassPane(PurpleStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:blue_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::BlueStainedGlassPane(BlueStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:brown_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::BrownStainedGlassPane(BrownStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:green_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::GreenStainedGlassPane(GreenStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::RedStainedGlassPane(RedStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:black_stained_glass_pane" => {
			let map = crate::ref_or_error(map)?;
			Block::BlackStainedGlassPane(BlackStainedGlassPane {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:acacia_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::AcaciaStairs(AcaciaStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_oak_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkOakStairs(DarkOakStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::MangroveStairs(MangroveStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:slime_block" => Block::SlimeBlock(SlimeBlock {}),
		"minecraft:barrier" => Block::Barrier(Barrier {}),
		"minecraft:light" => {
			let map = crate::ref_or_error(map)?;
			Block::Light(Light {
				r#level: map.get("level").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:iron_trapdoor" => {
			let map = crate::ref_or_error(map)?;
			Block::IronTrapdoor(IronTrapdoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:prismarine" => Block::Prismarine(Prismarine {}),
		"minecraft:prismarine_bricks" => Block::PrismarineBricks(PrismarineBricks {}),
		"minecraft:dark_prismarine" => Block::DarkPrismarine(DarkPrismarine {}),
		"minecraft:prismarine_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::PrismarineStairs(PrismarineStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:prismarine_brick_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::PrismarineBrickStairs(PrismarineBrickStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_prismarine_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkPrismarineStairs(DarkPrismarineStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:prismarine_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::PrismarineSlab(PrismarineSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:prismarine_brick_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::PrismarineBrickSlab(PrismarineBrickSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_prismarine_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkPrismarineSlab(DarkPrismarineSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:sea_lantern" => Block::SeaLantern(SeaLantern {}),
		"minecraft:hay_block" => {
			let map = crate::ref_or_error(map)?;
			Block::HayBlock(HayBlock {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:white_carpet" => Block::WhiteCarpet(WhiteCarpet {}),
		"minecraft:orange_carpet" => Block::OrangeCarpet(OrangeCarpet {}),
		"minecraft:magenta_carpet" => Block::MagentaCarpet(MagentaCarpet {}),
		"minecraft:light_blue_carpet" => Block::LightBlueCarpet(LightBlueCarpet {}),
		"minecraft:yellow_carpet" => Block::YellowCarpet(YellowCarpet {}),
		"minecraft:lime_carpet" => Block::LimeCarpet(LimeCarpet {}),
		"minecraft:pink_carpet" => Block::PinkCarpet(PinkCarpet {}),
		"minecraft:gray_carpet" => Block::GrayCarpet(GrayCarpet {}),
		"minecraft:light_gray_carpet" => Block::LightGrayCarpet(LightGrayCarpet {}),
		"minecraft:cyan_carpet" => Block::CyanCarpet(CyanCarpet {}),
		"minecraft:purple_carpet" => Block::PurpleCarpet(PurpleCarpet {}),
		"minecraft:blue_carpet" => Block::BlueCarpet(BlueCarpet {}),
		"minecraft:brown_carpet" => Block::BrownCarpet(BrownCarpet {}),
		"minecraft:green_carpet" => Block::GreenCarpet(GreenCarpet {}),
		"minecraft:red_carpet" => Block::RedCarpet(RedCarpet {}),
		"minecraft:black_carpet" => Block::BlackCarpet(BlackCarpet {}),
		"minecraft:terracotta" => Block::Terracotta(Terracotta {}),
		"minecraft:coal_block" => Block::CoalBlock(CoalBlock {}),
		"minecraft:packed_ice" => Block::PackedIce(PackedIce {}),
		"minecraft:sunflower" => {
			let map = crate::ref_or_error(map)?;
			Block::Sunflower(Sunflower {
				r#half: map.get("half").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lilac" => {
			let map = crate::ref_or_error(map)?;
			Block::Lilac(Lilac {
				r#half: map.get("half").unwrap().parse().unwrap(),
			})
		},
		"minecraft:rose_bush" => {
			let map = crate::ref_or_error(map)?;
			Block::RoseBush(RoseBush {
				r#half: map.get("half").unwrap().parse().unwrap(),
			})
		},
		"minecraft:peony" => {
			let map = crate::ref_or_error(map)?;
			Block::Peony(Peony {
				r#half: map.get("half").unwrap().parse().unwrap(),
			})
		},
		"minecraft:tall_grass" => {
			let map = crate::ref_or_error(map)?;
			Block::TallGrass(TallGrass {
				r#half: map.get("half").unwrap().parse().unwrap(),
			})
		},
		"minecraft:large_fern" => {
			let map = crate::ref_or_error(map)?;
			Block::LargeFern(LargeFern {
				r#half: map.get("half").unwrap().parse().unwrap(),
			})
		},
		"minecraft:white_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::WhiteBanner(WhiteBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:orange_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::OrangeBanner(OrangeBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:magenta_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::MagentaBanner(MagentaBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_blue_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::LightBlueBanner(LightBlueBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:yellow_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::YellowBanner(YellowBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lime_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::LimeBanner(LimeBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:pink_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::PinkBanner(PinkBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:gray_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::GrayBanner(GrayBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_gray_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::LightGrayBanner(LightGrayBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cyan_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::CyanBanner(CyanBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:purple_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::PurpleBanner(PurpleBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:blue_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::BlueBanner(BlueBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:brown_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::BrownBanner(BrownBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:green_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::GreenBanner(GreenBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::RedBanner(RedBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:black_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::BlackBanner(BlackBanner {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:white_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::WhiteWallBanner(WhiteWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:orange_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::OrangeWallBanner(OrangeWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:magenta_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::MagentaWallBanner(MagentaWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_blue_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::LightBlueWallBanner(LightBlueWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:yellow_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::YellowWallBanner(YellowWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lime_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::LimeWallBanner(LimeWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:pink_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::PinkWallBanner(PinkWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:gray_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::GrayWallBanner(GrayWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_gray_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::LightGrayWallBanner(LightGrayWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cyan_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::CyanWallBanner(CyanWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:purple_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::PurpleWallBanner(PurpleWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:blue_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::BlueWallBanner(BlueWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:brown_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::BrownWallBanner(BrownWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:green_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::GreenWallBanner(GreenWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::RedWallBanner(RedWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:black_wall_banner" => {
			let map = crate::ref_or_error(map)?;
			Block::BlackWallBanner(BlackWallBanner {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_sandstone" => Block::RedSandstone(RedSandstone {}),
		"minecraft:chiseled_red_sandstone" => Block::ChiseledRedSandstone(ChiseledRedSandstone {}),
		"minecraft:cut_red_sandstone" => Block::CutRedSandstone(CutRedSandstone {}),
		"minecraft:red_sandstone_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::RedSandstoneStairs(RedSandstoneStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:oak_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::OakSlab(OakSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:spruce_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::SpruceSlab(SpruceSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:birch_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::BirchSlab(BirchSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jungle_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::JungleSlab(JungleSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:acacia_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::AcaciaSlab(AcaciaSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_oak_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkOakSlab(DarkOakSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::MangroveSlab(MangroveSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stone_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::StoneSlab(StoneSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:smooth_stone_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::SmoothStoneSlab(SmoothStoneSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:sandstone_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::SandstoneSlab(SandstoneSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cut_sandstone_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::CutSandstoneSlab(CutSandstoneSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:petrified_oak_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::PetrifiedOakSlab(PetrifiedOakSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cobblestone_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::CobblestoneSlab(CobblestoneSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:brick_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::BrickSlab(BrickSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stone_brick_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::StoneBrickSlab(StoneBrickSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mud_brick_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::MudBrickSlab(MudBrickSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:nether_brick_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::NetherBrickSlab(NetherBrickSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:quartz_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::QuartzSlab(QuartzSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_sandstone_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::RedSandstoneSlab(RedSandstoneSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cut_red_sandstone_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::CutRedSandstoneSlab(CutRedSandstoneSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:purpur_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::PurpurSlab(PurpurSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:smooth_stone" => Block::SmoothStone(SmoothStone {}),
		"minecraft:smooth_sandstone" => Block::SmoothSandstone(SmoothSandstone {}),
		"minecraft:smooth_quartz" => Block::SmoothQuartz(SmoothQuartz {}),
		"minecraft:smooth_red_sandstone" => Block::SmoothRedSandstone(SmoothRedSandstone {}),
		"minecraft:spruce_fence_gate" => {
			let map = crate::ref_or_error(map)?;
			Block::SpruceFenceGate(SpruceFenceGate {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#in_wall: map.get("in_wall").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:birch_fence_gate" => {
			let map = crate::ref_or_error(map)?;
			Block::BirchFenceGate(BirchFenceGate {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#in_wall: map.get("in_wall").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jungle_fence_gate" => {
			let map = crate::ref_or_error(map)?;
			Block::JungleFenceGate(JungleFenceGate {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#in_wall: map.get("in_wall").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:acacia_fence_gate" => {
			let map = crate::ref_or_error(map)?;
			Block::AcaciaFenceGate(AcaciaFenceGate {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#in_wall: map.get("in_wall").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_oak_fence_gate" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkOakFenceGate(DarkOakFenceGate {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#in_wall: map.get("in_wall").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_fence_gate" => {
			let map = crate::ref_or_error(map)?;
			Block::MangroveFenceGate(MangroveFenceGate {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#in_wall: map.get("in_wall").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:spruce_fence" => {
			let map = crate::ref_or_error(map)?;
			Block::SpruceFence(SpruceFence {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:birch_fence" => {
			let map = crate::ref_or_error(map)?;
			Block::BirchFence(BirchFence {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jungle_fence" => {
			let map = crate::ref_or_error(map)?;
			Block::JungleFence(JungleFence {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:acacia_fence" => {
			let map = crate::ref_or_error(map)?;
			Block::AcaciaFence(AcaciaFence {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_oak_fence" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkOakFence(DarkOakFence {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_fence" => {
			let map = crate::ref_or_error(map)?;
			Block::MangroveFence(MangroveFence {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:spruce_door" => {
			let map = crate::ref_or_error(map)?;
			Block::SpruceDoor(SpruceDoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#hinge: map.get("hinge").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:birch_door" => {
			let map = crate::ref_or_error(map)?;
			Block::BirchDoor(BirchDoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#hinge: map.get("hinge").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jungle_door" => {
			let map = crate::ref_or_error(map)?;
			Block::JungleDoor(JungleDoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#hinge: map.get("hinge").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:acacia_door" => {
			let map = crate::ref_or_error(map)?;
			Block::AcaciaDoor(AcaciaDoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#hinge: map.get("hinge").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dark_oak_door" => {
			let map = crate::ref_or_error(map)?;
			Block::DarkOakDoor(DarkOakDoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#hinge: map.get("hinge").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mangrove_door" => {
			let map = crate::ref_or_error(map)?;
			Block::MangroveDoor(MangroveDoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#hinge: map.get("hinge").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:end_rod" => {
			let map = crate::ref_or_error(map)?;
			Block::EndRod(EndRod {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:chorus_plant" => {
			let map = crate::ref_or_error(map)?;
			Block::ChorusPlant(ChorusPlant {
				r#down: map.get("down").unwrap().parse().unwrap(),
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:chorus_flower" => {
			let map = crate::ref_or_error(map)?;
			Block::ChorusFlower(ChorusFlower {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:purpur_block" => Block::PurpurBlock(PurpurBlock {}),
		"minecraft:purpur_pillar" => {
			let map = crate::ref_or_error(map)?;
			Block::PurpurPillar(PurpurPillar {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:purpur_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::PurpurStairs(PurpurStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:end_stone_bricks" => Block::EndStoneBricks(EndStoneBricks {}),
		"minecraft:beetroots" => {
			let map = crate::ref_or_error(map)?;
			Block::Beetroots(Beetroots {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dirt_path" => Block::DirtPath(DirtPath {}),
		"minecraft:end_gateway" => Block::EndGateway(EndGateway {}),
		"minecraft:repeating_command_block" => {
			let map = crate::ref_or_error(map)?;
			Block::RepeatingCommandBlock(RepeatingCommandBlock {
				r#conditional: map.get("conditional").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:chain_command_block" => {
			let map = crate::ref_or_error(map)?;
			Block::ChainCommandBlock(ChainCommandBlock {
				r#conditional: map.get("conditional").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:frosted_ice" => {
			let map = crate::ref_or_error(map)?;
			Block::FrostedIce(FrostedIce {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:magma_block" => Block::MagmaBlock(MagmaBlock {}),
		"minecraft:nether_wart_block" => Block::NetherWartBlock(NetherWartBlock {}),
		"minecraft:red_nether_bricks" => Block::RedNetherBricks(RedNetherBricks {}),
		"minecraft:bone_block" => {
			let map = crate::ref_or_error(map)?;
			Block::BoneBlock(BoneBlock {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:structure_void" => Block::StructureVoid(StructureVoid {}),
		"minecraft:observer" => {
			let map = crate::ref_or_error(map)?;
			Block::Observer(Observer {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::ShulkerBox(ShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:white_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::WhiteShulkerBox(WhiteShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:orange_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::OrangeShulkerBox(OrangeShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:magenta_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::MagentaShulkerBox(MagentaShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_blue_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::LightBlueShulkerBox(LightBlueShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:yellow_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::YellowShulkerBox(YellowShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lime_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::LimeShulkerBox(LimeShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:pink_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::PinkShulkerBox(PinkShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:gray_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::GrayShulkerBox(GrayShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_gray_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::LightGrayShulkerBox(LightGrayShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cyan_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::CyanShulkerBox(CyanShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:purple_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::PurpleShulkerBox(PurpleShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:blue_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::BlueShulkerBox(BlueShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:brown_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::BrownShulkerBox(BrownShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:green_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::GreenShulkerBox(GreenShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::RedShulkerBox(RedShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:black_shulker_box" => {
			let map = crate::ref_or_error(map)?;
			Block::BlackShulkerBox(BlackShulkerBox {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:white_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::WhiteGlazedTerracotta(WhiteGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:orange_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::OrangeGlazedTerracotta(OrangeGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:magenta_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::MagentaGlazedTerracotta(MagentaGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_blue_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::LightBlueGlazedTerracotta(LightBlueGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:yellow_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::YellowGlazedTerracotta(YellowGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lime_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::LimeGlazedTerracotta(LimeGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:pink_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::PinkGlazedTerracotta(PinkGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:gray_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::GrayGlazedTerracotta(GrayGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_gray_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::LightGrayGlazedTerracotta(LightGrayGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cyan_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::CyanGlazedTerracotta(CyanGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:purple_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::PurpleGlazedTerracotta(PurpleGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:blue_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::BlueGlazedTerracotta(BlueGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:brown_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::BrownGlazedTerracotta(BrownGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:green_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::GreenGlazedTerracotta(GreenGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::RedGlazedTerracotta(RedGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:black_glazed_terracotta" => {
			let map = crate::ref_or_error(map)?;
			Block::BlackGlazedTerracotta(BlackGlazedTerracotta {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:white_concrete" => Block::WhiteConcrete(WhiteConcrete {}),
		"minecraft:orange_concrete" => Block::OrangeConcrete(OrangeConcrete {}),
		"minecraft:magenta_concrete" => Block::MagentaConcrete(MagentaConcrete {}),
		"minecraft:light_blue_concrete" => Block::LightBlueConcrete(LightBlueConcrete {}),
		"minecraft:yellow_concrete" => Block::YellowConcrete(YellowConcrete {}),
		"minecraft:lime_concrete" => Block::LimeConcrete(LimeConcrete {}),
		"minecraft:pink_concrete" => Block::PinkConcrete(PinkConcrete {}),
		"minecraft:gray_concrete" => Block::GrayConcrete(GrayConcrete {}),
		"minecraft:light_gray_concrete" => Block::LightGrayConcrete(LightGrayConcrete {}),
		"minecraft:cyan_concrete" => Block::CyanConcrete(CyanConcrete {}),
		"minecraft:purple_concrete" => Block::PurpleConcrete(PurpleConcrete {}),
		"minecraft:blue_concrete" => Block::BlueConcrete(BlueConcrete {}),
		"minecraft:brown_concrete" => Block::BrownConcrete(BrownConcrete {}),
		"minecraft:green_concrete" => Block::GreenConcrete(GreenConcrete {}),
		"minecraft:red_concrete" => Block::RedConcrete(RedConcrete {}),
		"minecraft:black_concrete" => Block::BlackConcrete(BlackConcrete {}),
		"minecraft:white_concrete_powder" => Block::WhiteConcretePowder(WhiteConcretePowder {}),
		"minecraft:orange_concrete_powder" => Block::OrangeConcretePowder(OrangeConcretePowder {}),
		"minecraft:magenta_concrete_powder" => Block::MagentaConcretePowder(MagentaConcretePowder {}),
		"minecraft:light_blue_concrete_powder" => Block::LightBlueConcretePowder(LightBlueConcretePowder {}),
		"minecraft:yellow_concrete_powder" => Block::YellowConcretePowder(YellowConcretePowder {}),
		"minecraft:lime_concrete_powder" => Block::LimeConcretePowder(LimeConcretePowder {}),
		"minecraft:pink_concrete_powder" => Block::PinkConcretePowder(PinkConcretePowder {}),
		"minecraft:gray_concrete_powder" => Block::GrayConcretePowder(GrayConcretePowder {}),
		"minecraft:light_gray_concrete_powder" => Block::LightGrayConcretePowder(LightGrayConcretePowder {}),
		"minecraft:cyan_concrete_powder" => Block::CyanConcretePowder(CyanConcretePowder {}),
		"minecraft:purple_concrete_powder" => Block::PurpleConcretePowder(PurpleConcretePowder {}),
		"minecraft:blue_concrete_powder" => Block::BlueConcretePowder(BlueConcretePowder {}),
		"minecraft:brown_concrete_powder" => Block::BrownConcretePowder(BrownConcretePowder {}),
		"minecraft:green_concrete_powder" => Block::GreenConcretePowder(GreenConcretePowder {}),
		"minecraft:red_concrete_powder" => Block::RedConcretePowder(RedConcretePowder {}),
		"minecraft:black_concrete_powder" => Block::BlackConcretePowder(BlackConcretePowder {}),
		"minecraft:kelp" => {
			let map = crate::ref_or_error(map)?;
			Block::Kelp(Kelp {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:kelp_plant" => Block::KelpPlant(KelpPlant {}),
		"minecraft:dried_kelp_block" => Block::DriedKelpBlock(DriedKelpBlock {}),
		"minecraft:turtle_egg" => {
			let map = crate::ref_or_error(map)?;
			Block::TurtleEgg(TurtleEgg {
				r#eggs: map.get("eggs").unwrap().parse().unwrap(),
				r#hatch: map.get("hatch").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_tube_coral_block" => Block::DeadTubeCoralBlock(DeadTubeCoralBlock {}),
		"minecraft:dead_brain_coral_block" => Block::DeadBrainCoralBlock(DeadBrainCoralBlock {}),
		"minecraft:dead_bubble_coral_block" => Block::DeadBubbleCoralBlock(DeadBubbleCoralBlock {}),
		"minecraft:dead_fire_coral_block" => Block::DeadFireCoralBlock(DeadFireCoralBlock {}),
		"minecraft:dead_horn_coral_block" => Block::DeadHornCoralBlock(DeadHornCoralBlock {}),
		"minecraft:tube_coral_block" => Block::TubeCoralBlock(TubeCoralBlock {}),
		"minecraft:brain_coral_block" => Block::BrainCoralBlock(BrainCoralBlock {}),
		"minecraft:bubble_coral_block" => Block::BubbleCoralBlock(BubbleCoralBlock {}),
		"minecraft:fire_coral_block" => Block::FireCoralBlock(FireCoralBlock {}),
		"minecraft:horn_coral_block" => Block::HornCoralBlock(HornCoralBlock {}),
		"minecraft:dead_tube_coral" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadTubeCoral(DeadTubeCoral {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_brain_coral" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadBrainCoral(DeadBrainCoral {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_bubble_coral" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadBubbleCoral(DeadBubbleCoral {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_fire_coral" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadFireCoral(DeadFireCoral {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_horn_coral" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadHornCoral(DeadHornCoral {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:tube_coral" => {
			let map = crate::ref_or_error(map)?;
			Block::TubeCoral(TubeCoral {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:brain_coral" => {
			let map = crate::ref_or_error(map)?;
			Block::BrainCoral(BrainCoral {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:bubble_coral" => {
			let map = crate::ref_or_error(map)?;
			Block::BubbleCoral(BubbleCoral {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:fire_coral" => {
			let map = crate::ref_or_error(map)?;
			Block::FireCoral(FireCoral {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:horn_coral" => {
			let map = crate::ref_or_error(map)?;
			Block::HornCoral(HornCoral {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_tube_coral_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadTubeCoralFan(DeadTubeCoralFan {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_brain_coral_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadBrainCoralFan(DeadBrainCoralFan {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_bubble_coral_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadBubbleCoralFan(DeadBubbleCoralFan {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_fire_coral_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadFireCoralFan(DeadFireCoralFan {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_horn_coral_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadHornCoralFan(DeadHornCoralFan {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:tube_coral_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::TubeCoralFan(TubeCoralFan {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:brain_coral_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::BrainCoralFan(BrainCoralFan {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:bubble_coral_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::BubbleCoralFan(BubbleCoralFan {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:fire_coral_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::FireCoralFan(FireCoralFan {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:horn_coral_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::HornCoralFan(HornCoralFan {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_tube_coral_wall_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadTubeCoralWallFan(DeadTubeCoralWallFan {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_brain_coral_wall_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadBrainCoralWallFan(DeadBrainCoralWallFan {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_bubble_coral_wall_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadBubbleCoralWallFan(DeadBubbleCoralWallFan {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_fire_coral_wall_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadFireCoralWallFan(DeadFireCoralWallFan {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dead_horn_coral_wall_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::DeadHornCoralWallFan(DeadHornCoralWallFan {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:tube_coral_wall_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::TubeCoralWallFan(TubeCoralWallFan {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:brain_coral_wall_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::BrainCoralWallFan(BrainCoralWallFan {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:bubble_coral_wall_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::BubbleCoralWallFan(BubbleCoralWallFan {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:fire_coral_wall_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::FireCoralWallFan(FireCoralWallFan {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:horn_coral_wall_fan" => {
			let map = crate::ref_or_error(map)?;
			Block::HornCoralWallFan(HornCoralWallFan {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:sea_pickle" => {
			let map = crate::ref_or_error(map)?;
			Block::SeaPickle(SeaPickle {
				r#pickles: map.get("pickles").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:blue_ice" => Block::BlueIce(BlueIce {}),
		"minecraft:conduit" => {
			let map = crate::ref_or_error(map)?;
			Block::Conduit(Conduit {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:bamboo_sapling" => Block::BambooSapling(BambooSapling {}),
		"minecraft:bamboo" => {
			let map = crate::ref_or_error(map)?;
			Block::Bamboo(Bamboo {
				r#age: map.get("age").unwrap().parse().unwrap(),
				r#leaves: map.get("leaves").unwrap().parse().unwrap(),
				r#stage: map.get("stage").unwrap().parse().unwrap(),
			})
		},
		"minecraft:potted_bamboo" => Block::PottedBamboo(PottedBamboo {}),
		"minecraft:void_air" => Block::VoidAir(VoidAir {}),
		"minecraft:cave_air" => Block::CaveAir(CaveAir {}),
		"minecraft:bubble_column" => {
			let map = crate::ref_or_error(map)?;
			Block::BubbleColumn(BubbleColumn {
				r#drag: map.get("drag").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_granite_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedGraniteStairs(PolishedGraniteStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:smooth_red_sandstone_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::SmoothRedSandstoneStairs(SmoothRedSandstoneStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mossy_stone_brick_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::MossyStoneBrickStairs(MossyStoneBrickStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_diorite_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedDioriteStairs(PolishedDioriteStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mossy_cobblestone_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::MossyCobblestoneStairs(MossyCobblestoneStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:end_stone_brick_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::EndStoneBrickStairs(EndStoneBrickStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stone_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::StoneStairs(StoneStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:smooth_sandstone_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::SmoothSandstoneStairs(SmoothSandstoneStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:smooth_quartz_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::SmoothQuartzStairs(SmoothQuartzStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:granite_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::GraniteStairs(GraniteStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:andesite_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::AndesiteStairs(AndesiteStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_nether_brick_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::RedNetherBrickStairs(RedNetherBrickStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_andesite_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedAndesiteStairs(PolishedAndesiteStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:diorite_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::DioriteStairs(DioriteStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_granite_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedGraniteSlab(PolishedGraniteSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:smooth_red_sandstone_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::SmoothRedSandstoneSlab(SmoothRedSandstoneSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mossy_stone_brick_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::MossyStoneBrickSlab(MossyStoneBrickSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_diorite_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedDioriteSlab(PolishedDioriteSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mossy_cobblestone_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::MossyCobblestoneSlab(MossyCobblestoneSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:end_stone_brick_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::EndStoneBrickSlab(EndStoneBrickSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:smooth_sandstone_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::SmoothSandstoneSlab(SmoothSandstoneSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:smooth_quartz_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::SmoothQuartzSlab(SmoothQuartzSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:granite_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::GraniteSlab(GraniteSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:andesite_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::AndesiteSlab(AndesiteSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_nether_brick_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::RedNetherBrickSlab(RedNetherBrickSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_andesite_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedAndesiteSlab(PolishedAndesiteSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:diorite_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::DioriteSlab(DioriteSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:brick_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::BrickWall(BrickWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:prismarine_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::PrismarineWall(PrismarineWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_sandstone_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::RedSandstoneWall(RedSandstoneWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mossy_stone_brick_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::MossyStoneBrickWall(MossyStoneBrickWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:granite_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::GraniteWall(GraniteWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stone_brick_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::StoneBrickWall(StoneBrickWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:mud_brick_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::MudBrickWall(MudBrickWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:nether_brick_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::NetherBrickWall(NetherBrickWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:andesite_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::AndesiteWall(AndesiteWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_nether_brick_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::RedNetherBrickWall(RedNetherBrickWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:sandstone_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::SandstoneWall(SandstoneWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:end_stone_brick_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::EndStoneBrickWall(EndStoneBrickWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:diorite_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::DioriteWall(DioriteWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:scaffolding" => {
			let map = crate::ref_or_error(map)?;
			Block::Scaffolding(Scaffolding {
				r#bottom: map.get("bottom").unwrap().parse().unwrap(),
				r#distance: map.get("distance").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:loom" => {
			let map = crate::ref_or_error(map)?;
			Block::Loom(Loom {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:barrel" => {
			let map = crate::ref_or_error(map)?;
			Block::Barrel(Barrel {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
			})
		},
		"minecraft:smoker" => {
			let map = crate::ref_or_error(map)?;
			Block::Smoker(Smoker {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:blast_furnace" => {
			let map = crate::ref_or_error(map)?;
			Block::BlastFurnace(BlastFurnace {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cartography_table" => Block::CartographyTable(CartographyTable {}),
		"minecraft:fletching_table" => Block::FletchingTable(FletchingTable {}),
		"minecraft:grindstone" => {
			let map = crate::ref_or_error(map)?;
			Block::Grindstone(Grindstone {
				r#face: map.get("face").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lectern" => {
			let map = crate::ref_or_error(map)?;
			Block::Lectern(Lectern {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#has_book: map.get("has_book").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:smithing_table" => Block::SmithingTable(SmithingTable {}),
		"minecraft:stonecutter" => {
			let map = crate::ref_or_error(map)?;
			Block::Stonecutter(Stonecutter {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
			})
		},
		"minecraft:bell" => {
			let map = crate::ref_or_error(map)?;
			Block::Bell(Bell {
				r#attachment: map.get("attachment").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lantern" => {
			let map = crate::ref_or_error(map)?;
			Block::Lantern(Lantern {
				r#hanging: map.get("hanging").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:soul_lantern" => {
			let map = crate::ref_or_error(map)?;
			Block::SoulLantern(SoulLantern {
				r#hanging: map.get("hanging").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:campfire" => {
			let map = crate::ref_or_error(map)?;
			Block::Campfire(Campfire {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#signal_fire: map.get("signal_fire").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:soul_campfire" => {
			let map = crate::ref_or_error(map)?;
			Block::SoulCampfire(SoulCampfire {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#signal_fire: map.get("signal_fire").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:sweet_berry_bush" => {
			let map = crate::ref_or_error(map)?;
			Block::SweetBerryBush(SweetBerryBush {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:warped_stem" => {
			let map = crate::ref_or_error(map)?;
			Block::WarpedStem(WarpedStem {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_warped_stem" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedWarpedStem(StrippedWarpedStem {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:warped_hyphae" => {
			let map = crate::ref_or_error(map)?;
			Block::WarpedHyphae(WarpedHyphae {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_warped_hyphae" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedWarpedHyphae(StrippedWarpedHyphae {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:warped_nylium" => Block::WarpedNylium(WarpedNylium {}),
		"minecraft:warped_fungus" => Block::WarpedFungus(WarpedFungus {}),
		"minecraft:warped_wart_block" => Block::WarpedWartBlock(WarpedWartBlock {}),
		"minecraft:warped_roots" => Block::WarpedRoots(WarpedRoots {}),
		"minecraft:nether_sprouts" => Block::NetherSprouts(NetherSprouts {}),
		"minecraft:crimson_stem" => {
			let map = crate::ref_or_error(map)?;
			Block::CrimsonStem(CrimsonStem {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_crimson_stem" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedCrimsonStem(StrippedCrimsonStem {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:crimson_hyphae" => {
			let map = crate::ref_or_error(map)?;
			Block::CrimsonHyphae(CrimsonHyphae {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:stripped_crimson_hyphae" => {
			let map = crate::ref_or_error(map)?;
			Block::StrippedCrimsonHyphae(StrippedCrimsonHyphae {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:crimson_nylium" => Block::CrimsonNylium(CrimsonNylium {}),
		"minecraft:crimson_fungus" => Block::CrimsonFungus(CrimsonFungus {}),
		"minecraft:shroomlight" => Block::Shroomlight(Shroomlight {}),
		"minecraft:weeping_vines" => {
			let map = crate::ref_or_error(map)?;
			Block::WeepingVines(WeepingVines {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:weeping_vines_plant" => Block::WeepingVinesPlant(WeepingVinesPlant {}),
		"minecraft:twisting_vines" => {
			let map = crate::ref_or_error(map)?;
			Block::TwistingVines(TwistingVines {
				r#age: map.get("age").unwrap().parse().unwrap(),
			})
		},
		"minecraft:twisting_vines_plant" => Block::TwistingVinesPlant(TwistingVinesPlant {}),
		"minecraft:crimson_roots" => Block::CrimsonRoots(CrimsonRoots {}),
		"minecraft:crimson_planks" => Block::CrimsonPlanks(CrimsonPlanks {}),
		"minecraft:warped_planks" => Block::WarpedPlanks(WarpedPlanks {}),
		"minecraft:crimson_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::CrimsonSlab(CrimsonSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:warped_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::WarpedSlab(WarpedSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:crimson_pressure_plate" => {
			let map = crate::ref_or_error(map)?;
			Block::CrimsonPressurePlate(CrimsonPressurePlate {
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:warped_pressure_plate" => {
			let map = crate::ref_or_error(map)?;
			Block::WarpedPressurePlate(WarpedPressurePlate {
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:crimson_fence" => {
			let map = crate::ref_or_error(map)?;
			Block::CrimsonFence(CrimsonFence {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:warped_fence" => {
			let map = crate::ref_or_error(map)?;
			Block::WarpedFence(WarpedFence {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:crimson_trapdoor" => {
			let map = crate::ref_or_error(map)?;
			Block::CrimsonTrapdoor(CrimsonTrapdoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:warped_trapdoor" => {
			let map = crate::ref_or_error(map)?;
			Block::WarpedTrapdoor(WarpedTrapdoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:crimson_fence_gate" => {
			let map = crate::ref_or_error(map)?;
			Block::CrimsonFenceGate(CrimsonFenceGate {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#in_wall: map.get("in_wall").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:warped_fence_gate" => {
			let map = crate::ref_or_error(map)?;
			Block::WarpedFenceGate(WarpedFenceGate {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#in_wall: map.get("in_wall").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:crimson_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::CrimsonStairs(CrimsonStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:warped_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::WarpedStairs(WarpedStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:crimson_button" => {
			let map = crate::ref_or_error(map)?;
			Block::CrimsonButton(CrimsonButton {
				r#face: map.get("face").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:warped_button" => {
			let map = crate::ref_or_error(map)?;
			Block::WarpedButton(WarpedButton {
				r#face: map.get("face").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:crimson_door" => {
			let map = crate::ref_or_error(map)?;
			Block::CrimsonDoor(CrimsonDoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#hinge: map.get("hinge").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:warped_door" => {
			let map = crate::ref_or_error(map)?;
			Block::WarpedDoor(WarpedDoor {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#hinge: map.get("hinge").unwrap().parse().unwrap(),
				r#open: map.get("open").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:crimson_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::CrimsonSign(CrimsonSign {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:warped_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::WarpedSign(WarpedSign {
				r#rotation: map.get("rotation").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:crimson_wall_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::CrimsonWallSign(CrimsonWallSign {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:warped_wall_sign" => {
			let map = crate::ref_or_error(map)?;
			Block::WarpedWallSign(WarpedWallSign {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:structure_block" => {
			let map = crate::ref_or_error(map)?;
			Block::StructureBlock(StructureBlock {
				r#mode: map.get("mode").unwrap().parse().unwrap(),
			})
		},
		"minecraft:jigsaw" => {
			let map = crate::ref_or_error(map)?;
			Block::Jigsaw(Jigsaw {
				r#orientation: map.get("orientation").unwrap().parse().unwrap(),
			})
		},
		"minecraft:composter" => {
			let map = crate::ref_or_error(map)?;
			Block::Composter(Composter {
				r#level: map.get("level").unwrap().parse().unwrap(),
			})
		},
		"minecraft:target" => {
			let map = crate::ref_or_error(map)?;
			Block::Target(Target {
				r#power: map.get("power").unwrap().parse().unwrap(),
			})
		},
		"minecraft:bee_nest" => {
			let map = crate::ref_or_error(map)?;
			Block::BeeNest(BeeNest {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#honey_level: map.get("honey_level").unwrap().parse().unwrap(),
			})
		},
		"minecraft:beehive" => {
			let map = crate::ref_or_error(map)?;
			Block::Beehive(Beehive {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#honey_level: map.get("honey_level").unwrap().parse().unwrap(),
			})
		},
		"minecraft:honey_block" => Block::HoneyBlock(HoneyBlock {}),
		"minecraft:honeycomb_block" => Block::HoneycombBlock(HoneycombBlock {}),
		"minecraft:netherite_block" => Block::NetheriteBlock(NetheriteBlock {}),
		"minecraft:ancient_debris" => Block::AncientDebris(AncientDebris {}),
		"minecraft:crying_obsidian" => Block::CryingObsidian(CryingObsidian {}),
		"minecraft:respawn_anchor" => {
			let map = crate::ref_or_error(map)?;
			Block::RespawnAnchor(RespawnAnchor {
				r#charges: map.get("charges").unwrap().parse().unwrap(),
			})
		},
		"minecraft:potted_crimson_fungus" => Block::PottedCrimsonFungus(PottedCrimsonFungus {}),
		"minecraft:potted_warped_fungus" => Block::PottedWarpedFungus(PottedWarpedFungus {}),
		"minecraft:potted_crimson_roots" => Block::PottedCrimsonRoots(PottedCrimsonRoots {}),
		"minecraft:potted_warped_roots" => Block::PottedWarpedRoots(PottedWarpedRoots {}),
		"minecraft:lodestone" => Block::Lodestone(Lodestone {}),
		"minecraft:blackstone" => Block::Blackstone(Blackstone {}),
		"minecraft:blackstone_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::BlackstoneStairs(BlackstoneStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:blackstone_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::BlackstoneWall(BlackstoneWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:blackstone_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::BlackstoneSlab(BlackstoneSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_blackstone" => Block::PolishedBlackstone(PolishedBlackstone {}),
		"minecraft:polished_blackstone_bricks" => Block::PolishedBlackstoneBricks(PolishedBlackstoneBricks {}),
		"minecraft:cracked_polished_blackstone_bricks" => Block::CrackedPolishedBlackstoneBricks(CrackedPolishedBlackstoneBricks {}),
		"minecraft:chiseled_polished_blackstone" => Block::ChiseledPolishedBlackstone(ChiseledPolishedBlackstone {}),
		"minecraft:polished_blackstone_brick_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedBlackstoneBrickSlab(PolishedBlackstoneBrickSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_blackstone_brick_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedBlackstoneBrickStairs(PolishedBlackstoneBrickStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_blackstone_brick_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedBlackstoneBrickWall(PolishedBlackstoneBrickWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:gilded_blackstone" => Block::GildedBlackstone(GildedBlackstone {}),
		"minecraft:polished_blackstone_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedBlackstoneStairs(PolishedBlackstoneStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_blackstone_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedBlackstoneSlab(PolishedBlackstoneSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_blackstone_pressure_plate" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedBlackstonePressurePlate(PolishedBlackstonePressurePlate {
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_blackstone_button" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedBlackstoneButton(PolishedBlackstoneButton {
				r#face: map.get("face").unwrap().parse().unwrap(),
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_blackstone_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedBlackstoneWall(PolishedBlackstoneWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:chiseled_nether_bricks" => Block::ChiseledNetherBricks(ChiseledNetherBricks {}),
		"minecraft:cracked_nether_bricks" => Block::CrackedNetherBricks(CrackedNetherBricks {}),
		"minecraft:quartz_bricks" => Block::QuartzBricks(QuartzBricks {}),
		"minecraft:candle" => {
			let map = crate::ref_or_error(map)?;
			Block::Candle(Candle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:white_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::WhiteCandle(WhiteCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:orange_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::OrangeCandle(OrangeCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:magenta_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::MagentaCandle(MagentaCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_blue_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::LightBlueCandle(LightBlueCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:yellow_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::YellowCandle(YellowCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lime_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::LimeCandle(LimeCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:pink_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::PinkCandle(PinkCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:gray_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::GrayCandle(GrayCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_gray_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::LightGrayCandle(LightGrayCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cyan_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::CyanCandle(CyanCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:purple_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::PurpleCandle(PurpleCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:blue_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::BlueCandle(BlueCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:brown_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::BrownCandle(BrownCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:green_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::GreenCandle(GreenCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::RedCandle(RedCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:black_candle" => {
			let map = crate::ref_or_error(map)?;
			Block::BlackCandle(BlackCandle {
				r#candles: map.get("candles").unwrap().parse().unwrap(),
				r#lit: map.get("lit").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::CandleCake(CandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:white_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::WhiteCandleCake(WhiteCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:orange_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::OrangeCandleCake(OrangeCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:magenta_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::MagentaCandleCake(MagentaCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_blue_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::LightBlueCandleCake(LightBlueCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:yellow_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::YellowCandleCake(YellowCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lime_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::LimeCandleCake(LimeCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:pink_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::PinkCandleCake(PinkCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:gray_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::GrayCandleCake(GrayCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:light_gray_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::LightGrayCandleCake(LightGrayCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cyan_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::CyanCandleCake(CyanCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:purple_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::PurpleCandleCake(PurpleCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:blue_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::BlueCandleCake(BlueCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:brown_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::BrownCandleCake(BrownCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:green_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::GreenCandleCake(GreenCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:red_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::RedCandleCake(RedCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:black_candle_cake" => {
			let map = crate::ref_or_error(map)?;
			Block::BlackCandleCake(BlackCandleCake {
				r#lit: map.get("lit").unwrap().parse().unwrap(),
			})
		},
		"minecraft:amethyst_block" => Block::AmethystBlock(AmethystBlock {}),
		"minecraft:budding_amethyst" => Block::BuddingAmethyst(BuddingAmethyst {}),
		"minecraft:amethyst_cluster" => {
			let map = crate::ref_or_error(map)?;
			Block::AmethystCluster(AmethystCluster {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:large_amethyst_bud" => {
			let map = crate::ref_or_error(map)?;
			Block::LargeAmethystBud(LargeAmethystBud {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:medium_amethyst_bud" => {
			let map = crate::ref_or_error(map)?;
			Block::MediumAmethystBud(MediumAmethystBud {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:small_amethyst_bud" => {
			let map = crate::ref_or_error(map)?;
			Block::SmallAmethystBud(SmallAmethystBud {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:tuff" => Block::Tuff(Tuff {}),
		"minecraft:calcite" => Block::Calcite(Calcite {}),
		"minecraft:tinted_glass" => Block::TintedGlass(TintedGlass {}),
		"minecraft:powder_snow" => Block::PowderSnow(PowderSnow {}),
		"minecraft:sculk_sensor" => {
			let map = crate::ref_or_error(map)?;
			Block::SculkSensor(SculkSensor {
				r#power: map.get("power").unwrap().parse().unwrap(),
				r#sculk_sensor_phase: map.get("sculk_sensor_phase").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:sculk" => Block::Sculk(Sculk {}),
		"minecraft:sculk_vein" => {
			let map = crate::ref_or_error(map)?;
			Block::SculkVein(SculkVein {
				r#down: map.get("down").unwrap().parse().unwrap(),
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:sculk_catalyst" => {
			let map = crate::ref_or_error(map)?;
			Block::SculkCatalyst(SculkCatalyst {
				r#bloom: map.get("bloom").unwrap().parse().unwrap(),
			})
		},
		"minecraft:sculk_shrieker" => {
			let map = crate::ref_or_error(map)?;
			Block::SculkShrieker(SculkShrieker {
				r#can_summon: map.get("can_summon").unwrap().parse().unwrap(),
				r#shrieking: map.get("shrieking").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:oxidized_copper" => Block::OxidizedCopper(OxidizedCopper {}),
		"minecraft:weathered_copper" => Block::WeatheredCopper(WeatheredCopper {}),
		"minecraft:exposed_copper" => Block::ExposedCopper(ExposedCopper {}),
		"minecraft:copper_block" => Block::CopperBlock(CopperBlock {}),
		"minecraft:copper_ore" => Block::CopperOre(CopperOre {}),
		"minecraft:deepslate_copper_ore" => Block::DeepslateCopperOre(DeepslateCopperOre {}),
		"minecraft:oxidized_cut_copper" => Block::OxidizedCutCopper(OxidizedCutCopper {}),
		"minecraft:weathered_cut_copper" => Block::WeatheredCutCopper(WeatheredCutCopper {}),
		"minecraft:exposed_cut_copper" => Block::ExposedCutCopper(ExposedCutCopper {}),
		"minecraft:cut_copper" => Block::CutCopper(CutCopper {}),
		"minecraft:oxidized_cut_copper_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::OxidizedCutCopperStairs(OxidizedCutCopperStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:weathered_cut_copper_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::WeatheredCutCopperStairs(WeatheredCutCopperStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:exposed_cut_copper_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::ExposedCutCopperStairs(ExposedCutCopperStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cut_copper_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::CutCopperStairs(CutCopperStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:oxidized_cut_copper_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::OxidizedCutCopperSlab(OxidizedCutCopperSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:weathered_cut_copper_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::WeatheredCutCopperSlab(WeatheredCutCopperSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:exposed_cut_copper_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::ExposedCutCopperSlab(ExposedCutCopperSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cut_copper_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::CutCopperSlab(CutCopperSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:waxed_copper_block" => Block::WaxedCopperBlock(WaxedCopperBlock {}),
		"minecraft:waxed_weathered_copper" => Block::WaxedWeatheredCopper(WaxedWeatheredCopper {}),
		"minecraft:waxed_exposed_copper" => Block::WaxedExposedCopper(WaxedExposedCopper {}),
		"minecraft:waxed_oxidized_copper" => Block::WaxedOxidizedCopper(WaxedOxidizedCopper {}),
		"minecraft:waxed_oxidized_cut_copper" => Block::WaxedOxidizedCutCopper(WaxedOxidizedCutCopper {}),
		"minecraft:waxed_weathered_cut_copper" => Block::WaxedWeatheredCutCopper(WaxedWeatheredCutCopper {}),
		"minecraft:waxed_exposed_cut_copper" => Block::WaxedExposedCutCopper(WaxedExposedCutCopper {}),
		"minecraft:waxed_cut_copper" => Block::WaxedCutCopper(WaxedCutCopper {}),
		"minecraft:waxed_oxidized_cut_copper_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::WaxedOxidizedCutCopperStairs(WaxedOxidizedCutCopperStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:waxed_weathered_cut_copper_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::WaxedWeatheredCutCopperStairs(WaxedWeatheredCutCopperStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:waxed_exposed_cut_copper_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::WaxedExposedCutCopperStairs(WaxedExposedCutCopperStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:waxed_cut_copper_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::WaxedCutCopperStairs(WaxedCutCopperStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:waxed_oxidized_cut_copper_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::WaxedOxidizedCutCopperSlab(WaxedOxidizedCutCopperSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:waxed_weathered_cut_copper_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::WaxedWeatheredCutCopperSlab(WaxedWeatheredCutCopperSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:waxed_exposed_cut_copper_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::WaxedExposedCutCopperSlab(WaxedExposedCutCopperSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:waxed_cut_copper_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::WaxedCutCopperSlab(WaxedCutCopperSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:lightning_rod" => {
			let map = crate::ref_or_error(map)?;
			Block::LightningRod(LightningRod {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#powered: map.get("powered").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:pointed_dripstone" => {
			let map = crate::ref_or_error(map)?;
			Block::PointedDripstone(PointedDripstone {
				r#thickness: map.get("thickness").unwrap().parse().unwrap(),
				r#vertical_direction: map.get("vertical_direction").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:dripstone_block" => Block::DripstoneBlock(DripstoneBlock {}),
		"minecraft:cave_vines" => {
			let map = crate::ref_or_error(map)?;
			Block::CaveVines(CaveVines {
				r#age: map.get("age").unwrap().parse().unwrap(),
				r#berries: map.get("berries").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cave_vines_plant" => {
			let map = crate::ref_or_error(map)?;
			Block::CaveVinesPlant(CaveVinesPlant {
				r#berries: map.get("berries").unwrap().parse().unwrap(),
			})
		},
		"minecraft:spore_blossom" => Block::SporeBlossom(SporeBlossom {}),
		"minecraft:azalea" => Block::Azalea(Azalea {}),
		"minecraft:flowering_azalea" => Block::FloweringAzalea(FloweringAzalea {}),
		"minecraft:moss_carpet" => Block::MossCarpet(MossCarpet {}),
		"minecraft:moss_block" => Block::MossBlock(MossBlock {}),
		"minecraft:big_dripleaf" => {
			let map = crate::ref_or_error(map)?;
			Block::BigDripleaf(BigDripleaf {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#tilt: map.get("tilt").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:big_dripleaf_stem" => {
			let map = crate::ref_or_error(map)?;
			Block::BigDripleafStem(BigDripleafStem {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:small_dripleaf" => {
			let map = crate::ref_or_error(map)?;
			Block::SmallDripleaf(SmallDripleaf {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:hanging_roots" => {
			let map = crate::ref_or_error(map)?;
			Block::HangingRoots(HangingRoots {
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:rooted_dirt" => Block::RootedDirt(RootedDirt {}),
		"minecraft:mud" => Block::Mud(Mud {}),
		"minecraft:deepslate" => {
			let map = crate::ref_or_error(map)?;
			Block::Deepslate(Deepslate {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cobbled_deepslate" => Block::CobbledDeepslate(CobbledDeepslate {}),
		"minecraft:cobbled_deepslate_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::CobbledDeepslateStairs(CobbledDeepslateStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cobbled_deepslate_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::CobbledDeepslateSlab(CobbledDeepslateSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:cobbled_deepslate_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::CobbledDeepslateWall(CobbledDeepslateWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_deepslate" => Block::PolishedDeepslate(PolishedDeepslate {}),
		"minecraft:polished_deepslate_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedDeepslateStairs(PolishedDeepslateStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_deepslate_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedDeepslateSlab(PolishedDeepslateSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:polished_deepslate_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::PolishedDeepslateWall(PolishedDeepslateWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:deepslate_tiles" => Block::DeepslateTiles(DeepslateTiles {}),
		"minecraft:deepslate_tile_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::DeepslateTileStairs(DeepslateTileStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:deepslate_tile_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::DeepslateTileSlab(DeepslateTileSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:deepslate_tile_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::DeepslateTileWall(DeepslateTileWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:deepslate_bricks" => Block::DeepslateBricks(DeepslateBricks {}),
		"minecraft:deepslate_brick_stairs" => {
			let map = crate::ref_or_error(map)?;
			Block::DeepslateBrickStairs(DeepslateBrickStairs {
				r#facing: map.get("facing").unwrap().parse().unwrap(),
				r#half: map.get("half").unwrap().parse().unwrap(),
				r#shape: map.get("shape").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:deepslate_brick_slab" => {
			let map = crate::ref_or_error(map)?;
			Block::DeepslateBrickSlab(DeepslateBrickSlab {
				r#type: map.get("type").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
			})
		},
		"minecraft:deepslate_brick_wall" => {
			let map = crate::ref_or_error(map)?;
			Block::DeepslateBrickWall(DeepslateBrickWall {
				r#east: map.get("east").unwrap().parse().unwrap(),
				r#north: map.get("north").unwrap().parse().unwrap(),
				r#south: map.get("south").unwrap().parse().unwrap(),
				r#up: map.get("up").unwrap().parse().unwrap(),
				r#waterlogged: map.get("waterlogged").unwrap().parse().unwrap(),
				r#west: map.get("west").unwrap().parse().unwrap(),
			})
		},
		"minecraft:chiseled_deepslate" => Block::ChiseledDeepslate(ChiseledDeepslate {}),
		"minecraft:cracked_deepslate_bricks" => Block::CrackedDeepslateBricks(CrackedDeepslateBricks {}),
		"minecraft:cracked_deepslate_tiles" => Block::CrackedDeepslateTiles(CrackedDeepslateTiles {}),
		"minecraft:infested_deepslate" => {
			let map = crate::ref_or_error(map)?;
			Block::InfestedDeepslate(InfestedDeepslate {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:smooth_basalt" => Block::SmoothBasalt(SmoothBasalt {}),
		"minecraft:raw_iron_block" => Block::RawIronBlock(RawIronBlock {}),
		"minecraft:raw_copper_block" => Block::RawCopperBlock(RawCopperBlock {}),
		"minecraft:raw_gold_block" => Block::RawGoldBlock(RawGoldBlock {}),
		"minecraft:potted_azalea_bush" => Block::PottedAzaleaBush(PottedAzaleaBush {}),
		"minecraft:potted_flowering_azalea_bush" => Block::PottedFloweringAzaleaBush(PottedFloweringAzaleaBush {}),
		"minecraft:ochre_froglight" => {
			let map = crate::ref_or_error(map)?;
			Block::OchreFroglight(OchreFroglight {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:verdant_froglight" => {
			let map = crate::ref_or_error(map)?;
			Block::VerdantFroglight(VerdantFroglight {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:pearlescent_froglight" => {
			let map = crate::ref_or_error(map)?;
			Block::PearlescentFroglight(PearlescentFroglight {
				r#axis: map.get("axis").unwrap().parse().unwrap(),
			})
		},
		"minecraft:frogspawn" => Block::Frogspawn(Frogspawn {}),
		"minecraft:reinforced_deepslate" => Block::ReinforcedDeepslate(ReinforcedDeepslate {}),
		_ => return Err(format!("unknown block: {}", tag)),
	})
}

pub mod property_enums {
	use std::{collections::HashMap, str::FromStr};
	
	#[derive(Debug)]
	pub enum Mode1 {
		save,
		load,
		corner,
		data,
	}
	impl FromStr for Mode1 {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"save" => Ok(Mode1::save),
				"load" => Ok(Mode1::load),
				"corner" => Ok(Mode1::corner),
				"data" => Ok(Mode1::data),
				_ => Err(format!("Invalid Mode1 value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum South {
		up,
		side,
		none,
	}
	impl FromStr for South {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"up" => Ok(South::up),
				"side" => Ok(South::side),
				"none" => Ok(South::none),
				_ => Err(format!("Invalid South value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Half {
		upper,
		lower,
	}
	impl FromStr for Half {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"upper" => Ok(Half::upper),
				"lower" => Ok(Half::lower),
				_ => Err(format!("Invalid Half value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Half1 {
		top,
		bottom,
	}
	impl FromStr for Half1 {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"top" => Ok(Half1::top),
				"bottom" => Ok(Half1::bottom),
				_ => Err(format!("Invalid Half1 value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Shape2 {
		north_south,
		east_west,
		ascending_east,
		ascending_west,
		ascending_north,
		ascending_south,
		south_east,
		south_west,
		north_west,
		north_east,
	}
	impl FromStr for Shape2 {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"north_south" => Ok(Shape2::north_south),
				"east_west" => Ok(Shape2::east_west),
				"ascending_east" => Ok(Shape2::ascending_east),
				"ascending_west" => Ok(Shape2::ascending_west),
				"ascending_north" => Ok(Shape2::ascending_north),
				"ascending_south" => Ok(Shape2::ascending_south),
				"south_east" => Ok(Shape2::south_east),
				"south_west" => Ok(Shape2::south_west),
				"north_west" => Ok(Shape2::north_west),
				"north_east" => Ok(Shape2::north_east),
				_ => Err(format!("Invalid Shape2 value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Face {
		floor,
		wall,
		ceiling,
	}
	impl FromStr for Face {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"floor" => Ok(Face::floor),
				"wall" => Ok(Face::wall),
				"ceiling" => Ok(Face::ceiling),
				_ => Err(format!("Invalid Face value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum South1 {
		none,
		low,
		tall,
	}
	impl FromStr for South1 {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"none" => Ok(South1::none),
				"low" => Ok(South1::low),
				"tall" => Ok(South1::tall),
				_ => Err(format!("Invalid South1 value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum West1 {
		none,
		low,
		tall,
	}
	impl FromStr for West1 {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"none" => Ok(West1::none),
				"low" => Ok(West1::low),
				"tall" => Ok(West1::tall),
				_ => Err(format!("Invalid West1 value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Tilt {
		none,
		unstable,
		partial,
		full,
	}
	impl FromStr for Tilt {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"none" => Ok(Tilt::none),
				"unstable" => Ok(Tilt::unstable),
				"partial" => Ok(Tilt::partial),
				"full" => Ok(Tilt::full),
				_ => Err(format!("Invalid Tilt value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Facing {
		north,
		east,
		south,
		west,
		up,
		down,
	}
	impl FromStr for Facing {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"north" => Ok(Facing::north),
				"east" => Ok(Facing::east),
				"south" => Ok(Facing::south),
				"west" => Ok(Facing::west),
				"up" => Ok(Facing::up),
				"down" => Ok(Facing::down),
				_ => Err(format!("Invalid Facing value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Facing1 {
		north,
		south,
		west,
		east,
	}
	impl FromStr for Facing1 {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"north" => Ok(Facing1::north),
				"south" => Ok(Facing1::south),
				"west" => Ok(Facing1::west),
				"east" => Ok(Facing1::east),
				_ => Err(format!("Invalid Facing1 value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum East1 {
		none,
		low,
		tall,
	}
	impl FromStr for East1 {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"none" => Ok(East1::none),
				"low" => Ok(East1::low),
				"tall" => Ok(East1::tall),
				_ => Err(format!("Invalid East1 value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum North1 {
		none,
		low,
		tall,
	}
	impl FromStr for North1 {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"none" => Ok(North1::none),
				"low" => Ok(North1::low),
				"tall" => Ok(North1::tall),
				_ => Err(format!("Invalid North1 value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Type1 {
		single,
		left,
		right,
	}
	impl FromStr for Type1 {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"single" => Ok(Type1::single),
				"left" => Ok(Type1::left),
				"right" => Ok(Type1::right),
				_ => Err(format!("Invalid Type1 value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Type2 {
		top,
		bottom,
		double,
	}
	impl FromStr for Type2 {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"top" => Ok(Type2::top),
				"bottom" => Ok(Type2::bottom),
				"double" => Ok(Type2::double),
				_ => Err(format!("Invalid Type2 value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Part {
		head,
		foot,
	}
	impl FromStr for Part {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"head" => Ok(Part::head),
				"foot" => Ok(Part::foot),
				_ => Err(format!("Invalid Part value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Axis {
		x,
		y,
		z,
	}
	impl FromStr for Axis {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"x" => Ok(Axis::x),
				"y" => Ok(Axis::y),
				"z" => Ok(Axis::z),
				_ => Err(format!("Invalid Axis value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Orientation {
		down_east,
		down_north,
		down_south,
		down_west,
		up_east,
		up_north,
		up_south,
		up_west,
		west_up,
		east_up,
		north_up,
		south_up,
	}
	impl FromStr for Orientation {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"down_east" => Ok(Orientation::down_east),
				"down_north" => Ok(Orientation::down_north),
				"down_south" => Ok(Orientation::down_south),
				"down_west" => Ok(Orientation::down_west),
				"up_east" => Ok(Orientation::up_east),
				"up_north" => Ok(Orientation::up_north),
				"up_south" => Ok(Orientation::up_south),
				"up_west" => Ok(Orientation::up_west),
				"west_up" => Ok(Orientation::west_up),
				"east_up" => Ok(Orientation::east_up),
				"north_up" => Ok(Orientation::north_up),
				"south_up" => Ok(Orientation::south_up),
				_ => Err(format!("Invalid Orientation value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Thickness {
		tip_merge,
		tip,
		frustum,
		middle,
		base,
	}
	impl FromStr for Thickness {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"tip_merge" => Ok(Thickness::tip_merge),
				"tip" => Ok(Thickness::tip),
				"frustum" => Ok(Thickness::frustum),
				"middle" => Ok(Thickness::middle),
				"base" => Ok(Thickness::base),
				_ => Err(format!("Invalid Thickness value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum West {
		up,
		side,
		none,
	}
	impl FromStr for West {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"up" => Ok(West::up),
				"side" => Ok(West::side),
				"none" => Ok(West::none),
				_ => Err(format!("Invalid West value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Shape1 {
		straight,
		inner_left,
		inner_right,
		outer_left,
		outer_right,
	}
	impl FromStr for Shape1 {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"straight" => Ok(Shape1::straight),
				"inner_left" => Ok(Shape1::inner_left),
				"inner_right" => Ok(Shape1::inner_right),
				"outer_left" => Ok(Shape1::outer_left),
				"outer_right" => Ok(Shape1::outer_right),
				_ => Err(format!("Invalid Shape1 value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Hinge {
		left,
		right,
	}
	impl FromStr for Hinge {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"left" => Ok(Hinge::left),
				"right" => Ok(Hinge::right),
				_ => Err(format!("Invalid Hinge value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Leaves {
		none,
		small,
		large,
	}
	impl FromStr for Leaves {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"none" => Ok(Leaves::none),
				"small" => Ok(Leaves::small),
				"large" => Ok(Leaves::large),
				_ => Err(format!("Invalid Leaves value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Instrument {
		harp,
		basedrum,
		snare,
		hat,
		bass,
		flute,
		bell,
		guitar,
		chime,
		xylophone,
		iron_xylophone,
		cow_bell,
		didgeridoo,
		bit,
		banjo,
		pling,
	}
	impl FromStr for Instrument {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"harp" => Ok(Instrument::harp),
				"basedrum" => Ok(Instrument::basedrum),
				"snare" => Ok(Instrument::snare),
				"hat" => Ok(Instrument::hat),
				"bass" => Ok(Instrument::bass),
				"flute" => Ok(Instrument::flute),
				"bell" => Ok(Instrument::bell),
				"guitar" => Ok(Instrument::guitar),
				"chime" => Ok(Instrument::chime),
				"xylophone" => Ok(Instrument::xylophone),
				"iron_xylophone" => Ok(Instrument::iron_xylophone),
				"cow_bell" => Ok(Instrument::cow_bell),
				"didgeridoo" => Ok(Instrument::didgeridoo),
				"bit" => Ok(Instrument::bit),
				"banjo" => Ok(Instrument::banjo),
				"pling" => Ok(Instrument::pling),
				_ => Err(format!("Invalid Instrument value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Type {
		normal,
		sticky,
	}
	impl FromStr for Type {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"normal" => Ok(Type::normal),
				"sticky" => Ok(Type::sticky),
				_ => Err(format!("Invalid Type value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum VerticalDirection {
		up,
		down,
	}
	impl FromStr for VerticalDirection {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"up" => Ok(VerticalDirection::up),
				"down" => Ok(VerticalDirection::down),
				_ => Err(format!("Invalid VerticalDirection value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Shape {
		north_south,
		east_west,
		ascending_east,
		ascending_west,
		ascending_north,
		ascending_south,
	}
	impl FromStr for Shape {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"north_south" => Ok(Shape::north_south),
				"east_west" => Ok(Shape::east_west),
				"ascending_east" => Ok(Shape::ascending_east),
				"ascending_west" => Ok(Shape::ascending_west),
				"ascending_north" => Ok(Shape::ascending_north),
				"ascending_south" => Ok(Shape::ascending_south),
				_ => Err(format!("Invalid Shape value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum North {
		up,
		side,
		none,
	}
	impl FromStr for North {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"up" => Ok(North::up),
				"side" => Ok(North::side),
				"none" => Ok(North::none),
				_ => Err(format!("Invalid North value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum SculkSensorPhase {
		inactive,
		active,
		cooldown,
	}
	impl FromStr for SculkSensorPhase {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"inactive" => Ok(SculkSensorPhase::inactive),
				"active" => Ok(SculkSensorPhase::active),
				"cooldown" => Ok(SculkSensorPhase::cooldown),
				_ => Err(format!("Invalid SculkSensorPhase value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Axis1 {
		x,
		z,
	}
	impl FromStr for Axis1 {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"x" => Ok(Axis1::x),
				"z" => Ok(Axis1::z),
				_ => Err(format!("Invalid Axis1 value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum East {
		up,
		side,
		none,
	}
	impl FromStr for East {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"up" => Ok(East::up),
				"side" => Ok(East::side),
				"none" => Ok(East::none),
				_ => Err(format!("Invalid East value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Mode {
		compare,
		subtract,
	}
	impl FromStr for Mode {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"compare" => Ok(Mode::compare),
				"subtract" => Ok(Mode::subtract),
				_ => Err(format!("Invalid Mode value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Attachment {
		floor,
		ceiling,
		single_wall,
		double_wall,
	}
	impl FromStr for Attachment {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"floor" => Ok(Attachment::floor),
				"ceiling" => Ok(Attachment::ceiling),
				"single_wall" => Ok(Attachment::single_wall),
				"double_wall" => Ok(Attachment::double_wall),
				_ => Err(format!("Invalid Attachment value: {}", s)),
			}
		}
	}

	#[derive(Debug)]
	pub enum Facing2 {
		down,
		north,
		south,
		west,
		east,
	}
	impl FromStr for Facing2 {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"down" => Ok(Facing2::down),
				"north" => Ok(Facing2::north),
				"south" => Ok(Facing2::south),
				"west" => Ok(Facing2::west),
				"east" => Ok(Facing2::east),
				_ => Err(format!("Invalid Facing2 value: {}", s)),
			}
		}
	}

}

pub mod blocks_props {

	use crate::BlockProperties;
	use super::*;	

	impl BlockProperties for Air {
		const DISPLAY_NAME: &'static str = "Air";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Stone {
		const DISPLAY_NAME: &'static str = "Stone";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Granite {
		const DISPLAY_NAME: &'static str = "Granite";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PolishedGranite {
		const DISPLAY_NAME: &'static str = "Polished Granite";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Diorite {
		const DISPLAY_NAME: &'static str = "Diorite";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PolishedDiorite {
		const DISPLAY_NAME: &'static str = "Polished Diorite";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Andesite {
		const DISPLAY_NAME: &'static str = "Andesite";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PolishedAndesite {
		const DISPLAY_NAME: &'static str = "Polished Andesite";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for GrassBlock {
		const DISPLAY_NAME: &'static str = "Grass Block";
		const HARDNESS: f32 = 0.6;
		const RESISTANCE: f32 = 0.6;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Dirt {
		const DISPLAY_NAME: &'static str = "Dirt";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CoarseDirt {
		const DISPLAY_NAME: &'static str = "Coarse Dirt";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Podzol {
		const DISPLAY_NAME: &'static str = "Podzol";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Cobblestone {
		const DISPLAY_NAME: &'static str = "Cobblestone";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for OakPlanks {
		const DISPLAY_NAME: &'static str = "Oak Planks";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SprucePlanks {
		const DISPLAY_NAME: &'static str = "Spruce Planks";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BirchPlanks {
		const DISPLAY_NAME: &'static str = "Birch Planks";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for JunglePlanks {
		const DISPLAY_NAME: &'static str = "Jungle Planks";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for AcaciaPlanks {
		const DISPLAY_NAME: &'static str = "Acacia Planks";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DarkOakPlanks {
		const DISPLAY_NAME: &'static str = "Dark Oak Planks";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for MangrovePlanks {
		const DISPLAY_NAME: &'static str = "Mangrove Planks";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for OakSapling {
		const DISPLAY_NAME: &'static str = "Oak Sapling";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SpruceSapling {
		const DISPLAY_NAME: &'static str = "Spruce Sapling";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BirchSapling {
		const DISPLAY_NAME: &'static str = "Birch Sapling";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for JungleSapling {
		const DISPLAY_NAME: &'static str = "Jungle Sapling";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AcaciaSapling {
		const DISPLAY_NAME: &'static str = "Acacia Sapling";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DarkOakSapling {
		const DISPLAY_NAME: &'static str = "Dark Oak Sapling";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MangrovePropagule {
		const DISPLAY_NAME: &'static str = "Mangrove Propagule";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Bedrock {
		const DISPLAY_NAME: &'static str = "Bedrock";
		const HARDNESS: f32 = -1.0;
		const RESISTANCE: f32 = 3600000.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Water {
		const DISPLAY_NAME: &'static str = "Water";
		const HARDNESS: f32 = 100.0;
		const RESISTANCE: f32 = 100.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for Lava {
		const DISPLAY_NAME: &'static str = "Lava";
		const HARDNESS: f32 = 100.0;
		const RESISTANCE: f32 = 100.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for Sand {
		const DISPLAY_NAME: &'static str = "Sand";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for RedSand {
		const DISPLAY_NAME: &'static str = "Red Sand";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Gravel {
		const DISPLAY_NAME: &'static str = "Gravel";
		const HARDNESS: f32 = 0.6;
		const RESISTANCE: f32 = 0.6;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for GoldOre {
		const DISPLAY_NAME: &'static str = "Gold Ore";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeepslateGoldOre {
		const DISPLAY_NAME: &'static str = "Deepslate Gold Ore";
		const HARDNESS: f32 = 4.5;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for IronOre {
		const DISPLAY_NAME: &'static str = "Iron Ore";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeepslateIronOre {
		const DISPLAY_NAME: &'static str = "Deepslate Iron Ore";
		const HARDNESS: f32 = 4.5;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CoalOre {
		const DISPLAY_NAME: &'static str = "Coal Ore";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeepslateCoalOre {
		const DISPLAY_NAME: &'static str = "Deepslate Coal Ore";
		const HARDNESS: f32 = 4.5;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for NetherGoldOre {
		const DISPLAY_NAME: &'static str = "Nether Gold Ore";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for OakLog {
		const DISPLAY_NAME: &'static str = "Oak Log";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SpruceLog {
		const DISPLAY_NAME: &'static str = "Spruce Log";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BirchLog {
		const DISPLAY_NAME: &'static str = "Birch Log";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for JungleLog {
		const DISPLAY_NAME: &'static str = "Jungle Log";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for AcaciaLog {
		const DISPLAY_NAME: &'static str = "Acacia Log";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DarkOakLog {
		const DISPLAY_NAME: &'static str = "Dark Oak Log";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for MangroveLog {
		const DISPLAY_NAME: &'static str = "Mangrove Log";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for MangroveRoots {
		const DISPLAY_NAME: &'static str = "Mangrove Roots";
		const HARDNESS: f32 = 0.7;
		const RESISTANCE: f32 = 0.7;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for MuddyMangroveRoots {
		const DISPLAY_NAME: &'static str = "Muddy Mangrove Roots";
		const HARDNESS: f32 = 0.7;
		const RESISTANCE: f32 = 0.7;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedSpruceLog {
		const DISPLAY_NAME: &'static str = "Stripped Spruce Log";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedBirchLog {
		const DISPLAY_NAME: &'static str = "Stripped Birch Log";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedJungleLog {
		const DISPLAY_NAME: &'static str = "Stripped Jungle Log";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedAcaciaLog {
		const DISPLAY_NAME: &'static str = "Stripped Acacia Log";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedDarkOakLog {
		const DISPLAY_NAME: &'static str = "Stripped Dark Oak Log";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedOakLog {
		const DISPLAY_NAME: &'static str = "Stripped Oak Log";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedMangroveLog {
		const DISPLAY_NAME: &'static str = "Stripped Mangrove Log";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for OakWood {
		const DISPLAY_NAME: &'static str = "Oak Wood";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SpruceWood {
		const DISPLAY_NAME: &'static str = "Spruce Wood";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BirchWood {
		const DISPLAY_NAME: &'static str = "Birch Wood";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for JungleWood {
		const DISPLAY_NAME: &'static str = "Jungle Wood";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for AcaciaWood {
		const DISPLAY_NAME: &'static str = "Acacia Wood";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DarkOakWood {
		const DISPLAY_NAME: &'static str = "Dark Oak Wood";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for MangroveWood {
		const DISPLAY_NAME: &'static str = "Mangrove Wood";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedOakWood {
		const DISPLAY_NAME: &'static str = "Stripped Oak Wood";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedSpruceWood {
		const DISPLAY_NAME: &'static str = "Stripped Spruce Wood";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedBirchWood {
		const DISPLAY_NAME: &'static str = "Stripped Birch Wood";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedJungleWood {
		const DISPLAY_NAME: &'static str = "Stripped Jungle Wood";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedAcaciaWood {
		const DISPLAY_NAME: &'static str = "Stripped Acacia Wood";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedDarkOakWood {
		const DISPLAY_NAME: &'static str = "Stripped Dark Oak Wood";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedMangroveWood {
		const DISPLAY_NAME: &'static str = "Stripped Mangrove Wood";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for OakLeaves {
		const DISPLAY_NAME: &'static str = "Oak Leaves";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for SpruceLeaves {
		const DISPLAY_NAME: &'static str = "Spruce Leaves";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for BirchLeaves {
		const DISPLAY_NAME: &'static str = "Birch Leaves";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for JungleLeaves {
		const DISPLAY_NAME: &'static str = "Jungle Leaves";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for AcaciaLeaves {
		const DISPLAY_NAME: &'static str = "Acacia Leaves";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DarkOakLeaves {
		const DISPLAY_NAME: &'static str = "Dark Oak Leaves";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for MangroveLeaves {
		const DISPLAY_NAME: &'static str = "Mangrove Leaves";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for AzaleaLeaves {
		const DISPLAY_NAME: &'static str = "Azalea Leaves";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for FloweringAzaleaLeaves {
		const DISPLAY_NAME: &'static str = "Flowering Azalea Leaves";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for Sponge {
		const DISPLAY_NAME: &'static str = "Sponge";
		const HARDNESS: f32 = 0.6;
		const RESISTANCE: f32 = 0.6;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WetSponge {
		const DISPLAY_NAME: &'static str = "Wet Sponge";
		const HARDNESS: f32 = 0.6;
		const RESISTANCE: f32 = 0.6;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Glass {
		const DISPLAY_NAME: &'static str = "Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LapisOre {
		const DISPLAY_NAME: &'static str = "Lapis Lazuli Ore";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeepslateLapisOre {
		const DISPLAY_NAME: &'static str = "Deepslate Lapis Lazuli Ore";
		const HARDNESS: f32 = 4.5;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LapisBlock {
		const DISPLAY_NAME: &'static str = "Block of Lapis Lazuli";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Dispenser {
		const DISPLAY_NAME: &'static str = "Dispenser";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 3.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Sandstone {
		const DISPLAY_NAME: &'static str = "Sandstone";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for ChiseledSandstone {
		const DISPLAY_NAME: &'static str = "Chiseled Sandstone";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CutSandstone {
		const DISPLAY_NAME: &'static str = "Cut Sandstone";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for NoteBlock {
		const DISPLAY_NAME: &'static str = "Note Block";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WhiteBed {
		const DISPLAY_NAME: &'static str = "White Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OrangeBed {
		const DISPLAY_NAME: &'static str = "Orange Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MagentaBed {
		const DISPLAY_NAME: &'static str = "Magenta Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightBlueBed {
		const DISPLAY_NAME: &'static str = "Light Blue Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for YellowBed {
		const DISPLAY_NAME: &'static str = "Yellow Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LimeBed {
		const DISPLAY_NAME: &'static str = "Lime Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PinkBed {
		const DISPLAY_NAME: &'static str = "Pink Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GrayBed {
		const DISPLAY_NAME: &'static str = "Gray Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightGrayBed {
		const DISPLAY_NAME: &'static str = "Light Gray Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CyanBed {
		const DISPLAY_NAME: &'static str = "Cyan Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PurpleBed {
		const DISPLAY_NAME: &'static str = "Purple Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlueBed {
		const DISPLAY_NAME: &'static str = "Blue Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BrownBed {
		const DISPLAY_NAME: &'static str = "Brown Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GreenBed {
		const DISPLAY_NAME: &'static str = "Green Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedBed {
		const DISPLAY_NAME: &'static str = "Red Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlackBed {
		const DISPLAY_NAME: &'static str = "Black Bed";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PoweredRail {
		const DISPLAY_NAME: &'static str = "Powered Rail";
		const HARDNESS: f32 = 0.7;
		const RESISTANCE: f32 = 0.7;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DetectorRail {
		const DISPLAY_NAME: &'static str = "Detector Rail";
		const HARDNESS: f32 = 0.7;
		const RESISTANCE: f32 = 0.7;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for StickyPiston {
		const DISPLAY_NAME: &'static str = "Sticky Piston";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 1.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Cobweb {
		const DISPLAY_NAME: &'static str = "Cobweb";
		const HARDNESS: f32 = 4.0;
		const RESISTANCE: f32 = 4.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for Grass {
		const DISPLAY_NAME: &'static str = "Grass";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Fern {
		const DISPLAY_NAME: &'static str = "Fern";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DeadBush {
		const DISPLAY_NAME: &'static str = "Dead Bush";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Seagrass {
		const DISPLAY_NAME: &'static str = "Seagrass";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for TallSeagrass {
		const DISPLAY_NAME: &'static str = "Tall Seagrass";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for Piston {
		const DISPLAY_NAME: &'static str = "Piston";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 1.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PistonHead {
		const DISPLAY_NAME: &'static str = "Piston Head";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 1.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WhiteWool {
		const DISPLAY_NAME: &'static str = "White Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for OrangeWool {
		const DISPLAY_NAME: &'static str = "Orange Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for MagentaWool {
		const DISPLAY_NAME: &'static str = "Magenta Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LightBlueWool {
		const DISPLAY_NAME: &'static str = "Light Blue Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for YellowWool {
		const DISPLAY_NAME: &'static str = "Yellow Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LimeWool {
		const DISPLAY_NAME: &'static str = "Lime Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PinkWool {
		const DISPLAY_NAME: &'static str = "Pink Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for GrayWool {
		const DISPLAY_NAME: &'static str = "Gray Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LightGrayWool {
		const DISPLAY_NAME: &'static str = "Light Gray Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CyanWool {
		const DISPLAY_NAME: &'static str = "Cyan Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PurpleWool {
		const DISPLAY_NAME: &'static str = "Purple Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BlueWool {
		const DISPLAY_NAME: &'static str = "Blue Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BrownWool {
		const DISPLAY_NAME: &'static str = "Brown Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for GreenWool {
		const DISPLAY_NAME: &'static str = "Green Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for RedWool {
		const DISPLAY_NAME: &'static str = "Red Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BlackWool {
		const DISPLAY_NAME: &'static str = "Black Wool";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for MovingPiston {
		const DISPLAY_NAME: &'static str = "Moving Piston";
		const HARDNESS: f32 = -1.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Dandelion {
		const DISPLAY_NAME: &'static str = "Dandelion";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Poppy {
		const DISPLAY_NAME: &'static str = "Poppy";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlueOrchid {
		const DISPLAY_NAME: &'static str = "Blue Orchid";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Allium {
		const DISPLAY_NAME: &'static str = "Allium";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AzureBluet {
		const DISPLAY_NAME: &'static str = "Azure Bluet";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedTulip {
		const DISPLAY_NAME: &'static str = "Red Tulip";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OrangeTulip {
		const DISPLAY_NAME: &'static str = "Orange Tulip";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WhiteTulip {
		const DISPLAY_NAME: &'static str = "White Tulip";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PinkTulip {
		const DISPLAY_NAME: &'static str = "Pink Tulip";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OxeyeDaisy {
		const DISPLAY_NAME: &'static str = "Oxeye Daisy";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Cornflower {
		const DISPLAY_NAME: &'static str = "Cornflower";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WitherRose {
		const DISPLAY_NAME: &'static str = "Wither Rose";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LilyOfTheValley {
		const DISPLAY_NAME: &'static str = "Lily of the Valley";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BrownMushroom {
		const DISPLAY_NAME: &'static str = "Brown Mushroom";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 1;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedMushroom {
		const DISPLAY_NAME: &'static str = "Red Mushroom";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GoldBlock {
		const DISPLAY_NAME: &'static str = "Block of Gold";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for IronBlock {
		const DISPLAY_NAME: &'static str = "Block of Iron";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Bricks {
		const DISPLAY_NAME: &'static str = "Bricks";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Tnt {
		const DISPLAY_NAME: &'static str = "TNT";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Bookshelf {
		const DISPLAY_NAME: &'static str = "Bookshelf";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 1.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for MossyCobblestone {
		const DISPLAY_NAME: &'static str = "Mossy Cobblestone";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Obsidian {
		const DISPLAY_NAME: &'static str = "Obsidian";
		const HARDNESS: f32 = 50.0;
		const RESISTANCE: f32 = 1200.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Torch {
		const DISPLAY_NAME: &'static str = "Torch";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 14;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WallTorch {
		const DISPLAY_NAME: &'static str = "Torch";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 14;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Fire {
		const DISPLAY_NAME: &'static str = "Fire";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SoulFire {
		const DISPLAY_NAME: &'static str = "Soul Fire";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 10;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Spawner {
		const DISPLAY_NAME: &'static str = "Spawner";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 5.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for OakStairs {
		const DISPLAY_NAME: &'static str = "Oak Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Chest {
		const DISPLAY_NAME: &'static str = "Chest";
		const HARDNESS: f32 = 2.5;
		const RESISTANCE: f32 = 2.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedstoneWire {
		const DISPLAY_NAME: &'static str = "Redstone Wire";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DiamondOre {
		const DISPLAY_NAME: &'static str = "Diamond Ore";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeepslateDiamondOre {
		const DISPLAY_NAME: &'static str = "Deepslate Diamond Ore";
		const HARDNESS: f32 = 4.5;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DiamondBlock {
		const DISPLAY_NAME: &'static str = "Block of Diamond";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CraftingTable {
		const DISPLAY_NAME: &'static str = "Crafting Table";
		const HARDNESS: f32 = 2.5;
		const RESISTANCE: f32 = 2.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Wheat {
		const DISPLAY_NAME: &'static str = "Wheat Crops";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Farmland {
		const DISPLAY_NAME: &'static str = "Farmland";
		const HARDNESS: f32 = 0.6;
		const RESISTANCE: f32 = 0.6;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Furnace {
		const DISPLAY_NAME: &'static str = "Furnace";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 3.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for OakSign {
		const DISPLAY_NAME: &'static str = "Oak Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SpruceSign {
		const DISPLAY_NAME: &'static str = "Spruce Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BirchSign {
		const DISPLAY_NAME: &'static str = "Birch Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AcaciaSign {
		const DISPLAY_NAME: &'static str = "Acacia Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for JungleSign {
		const DISPLAY_NAME: &'static str = "Jungle Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DarkOakSign {
		const DISPLAY_NAME: &'static str = "Dark Oak Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MangroveSign {
		const DISPLAY_NAME: &'static str = "Mangrove Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OakDoor {
		const DISPLAY_NAME: &'static str = "Oak Door";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Ladder {
		const DISPLAY_NAME: &'static str = "Ladder";
		const HARDNESS: f32 = 0.4;
		const RESISTANCE: f32 = 0.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Rail {
		const DISPLAY_NAME: &'static str = "Rail";
		const HARDNESS: f32 = 0.7;
		const RESISTANCE: f32 = 0.7;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CobblestoneStairs {
		const DISPLAY_NAME: &'static str = "Cobblestone Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OakWallSign {
		const DISPLAY_NAME: &'static str = "Oak Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SpruceWallSign {
		const DISPLAY_NAME: &'static str = "Spruce Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BirchWallSign {
		const DISPLAY_NAME: &'static str = "Birch Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AcaciaWallSign {
		const DISPLAY_NAME: &'static str = "Acacia Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for JungleWallSign {
		const DISPLAY_NAME: &'static str = "Jungle Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DarkOakWallSign {
		const DISPLAY_NAME: &'static str = "Dark Oak Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MangroveWallSign {
		const DISPLAY_NAME: &'static str = "Mangrove Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Lever {
		const DISPLAY_NAME: &'static str = "Lever";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for StonePressurePlate {
		const DISPLAY_NAME: &'static str = "Stone Pressure Plate";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for IronDoor {
		const DISPLAY_NAME: &'static str = "Iron Door";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 5.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OakPressurePlate {
		const DISPLAY_NAME: &'static str = "Oak Pressure Plate";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SprucePressurePlate {
		const DISPLAY_NAME: &'static str = "Spruce Pressure Plate";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BirchPressurePlate {
		const DISPLAY_NAME: &'static str = "Birch Pressure Plate";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for JunglePressurePlate {
		const DISPLAY_NAME: &'static str = "Jungle Pressure Plate";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AcaciaPressurePlate {
		const DISPLAY_NAME: &'static str = "Acacia Pressure Plate";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DarkOakPressurePlate {
		const DISPLAY_NAME: &'static str = "Dark Oak Pressure Plate";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MangrovePressurePlate {
		const DISPLAY_NAME: &'static str = "Mangrove Pressure Plate";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedstoneOre {
		const DISPLAY_NAME: &'static str = "Redstone Ore";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeepslateRedstoneOre {
		const DISPLAY_NAME: &'static str = "Deepslate Redstone Ore";
		const HARDNESS: f32 = 4.5;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for RedstoneTorch {
		const DISPLAY_NAME: &'static str = "Redstone Torch";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 7;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedstoneWallTorch {
		const DISPLAY_NAME: &'static str = "Redstone Torch";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 7;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for StoneButton {
		const DISPLAY_NAME: &'static str = "Stone Button";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Snow {
		const DISPLAY_NAME: &'static str = "Snow";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Ice {
		const DISPLAY_NAME: &'static str = "Ice";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for SnowBlock {
		const DISPLAY_NAME: &'static str = "Snow Block";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Cactus {
		const DISPLAY_NAME: &'static str = "Cactus";
		const HARDNESS: f32 = 0.4;
		const RESISTANCE: f32 = 0.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Clay {
		const DISPLAY_NAME: &'static str = "Clay";
		const HARDNESS: f32 = 0.6;
		const RESISTANCE: f32 = 0.6;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SugarCane {
		const DISPLAY_NAME: &'static str = "Sugar Cane";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Jukebox {
		const DISPLAY_NAME: &'static str = "Jukebox";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for OakFence {
		const DISPLAY_NAME: &'static str = "Oak Fence";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Pumpkin {
		const DISPLAY_NAME: &'static str = "Pumpkin";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Netherrack {
		const DISPLAY_NAME: &'static str = "Netherrack";
		const HARDNESS: f32 = 0.4;
		const RESISTANCE: f32 = 0.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SoulSand {
		const DISPLAY_NAME: &'static str = "Soul Sand";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SoulSoil {
		const DISPLAY_NAME: &'static str = "Soul Soil";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Basalt {
		const DISPLAY_NAME: &'static str = "Basalt";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PolishedBasalt {
		const DISPLAY_NAME: &'static str = "Polished Basalt";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SoulTorch {
		const DISPLAY_NAME: &'static str = "Soul Torch";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 10;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SoulWallTorch {
		const DISPLAY_NAME: &'static str = "Soul Torch";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 10;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Glowstone {
		const DISPLAY_NAME: &'static str = "Glowstone";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for NetherPortal {
		const DISPLAY_NAME: &'static str = "Nether Portal";
		const HARDNESS: f32 = -1.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 11;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CarvedPumpkin {
		const DISPLAY_NAME: &'static str = "Carved Pumpkin";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for JackOLantern {
		const DISPLAY_NAME: &'static str = "Jack o'Lantern";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Cake {
		const DISPLAY_NAME: &'static str = "Cake";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Repeater {
		const DISPLAY_NAME: &'static str = "Redstone Repeater";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WhiteStainedGlass {
		const DISPLAY_NAME: &'static str = "White Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OrangeStainedGlass {
		const DISPLAY_NAME: &'static str = "Orange Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MagentaStainedGlass {
		const DISPLAY_NAME: &'static str = "Magenta Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightBlueStainedGlass {
		const DISPLAY_NAME: &'static str = "Light Blue Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for YellowStainedGlass {
		const DISPLAY_NAME: &'static str = "Yellow Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LimeStainedGlass {
		const DISPLAY_NAME: &'static str = "Lime Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PinkStainedGlass {
		const DISPLAY_NAME: &'static str = "Pink Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GrayStainedGlass {
		const DISPLAY_NAME: &'static str = "Gray Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightGrayStainedGlass {
		const DISPLAY_NAME: &'static str = "Light Gray Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CyanStainedGlass {
		const DISPLAY_NAME: &'static str = "Cyan Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PurpleStainedGlass {
		const DISPLAY_NAME: &'static str = "Purple Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlueStainedGlass {
		const DISPLAY_NAME: &'static str = "Blue Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BrownStainedGlass {
		const DISPLAY_NAME: &'static str = "Brown Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GreenStainedGlass {
		const DISPLAY_NAME: &'static str = "Green Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedStainedGlass {
		const DISPLAY_NAME: &'static str = "Red Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlackStainedGlass {
		const DISPLAY_NAME: &'static str = "Black Stained Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OakTrapdoor {
		const DISPLAY_NAME: &'static str = "Oak Trapdoor";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SpruceTrapdoor {
		const DISPLAY_NAME: &'static str = "Spruce Trapdoor";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BirchTrapdoor {
		const DISPLAY_NAME: &'static str = "Birch Trapdoor";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for JungleTrapdoor {
		const DISPLAY_NAME: &'static str = "Jungle Trapdoor";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AcaciaTrapdoor {
		const DISPLAY_NAME: &'static str = "Acacia Trapdoor";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DarkOakTrapdoor {
		const DISPLAY_NAME: &'static str = "Dark Oak Trapdoor";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MangroveTrapdoor {
		const DISPLAY_NAME: &'static str = "Mangrove Trapdoor";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for StoneBricks {
		const DISPLAY_NAME: &'static str = "Stone Bricks";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for MossyStoneBricks {
		const DISPLAY_NAME: &'static str = "Mossy Stone Bricks";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CrackedStoneBricks {
		const DISPLAY_NAME: &'static str = "Cracked Stone Bricks";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for ChiseledStoneBricks {
		const DISPLAY_NAME: &'static str = "Chiseled Stone Bricks";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PackedMud {
		const DISPLAY_NAME: &'static str = "Packed Mud";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for MudBricks {
		const DISPLAY_NAME: &'static str = "Mud Bricks";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for InfestedStone {
		const DISPLAY_NAME: &'static str = "Infested Stone";
		const HARDNESS: f32 = 0.75;
		const RESISTANCE: f32 = 0.75;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for InfestedCobblestone {
		const DISPLAY_NAME: &'static str = "Infested Cobblestone";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 0.75;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for InfestedStoneBricks {
		const DISPLAY_NAME: &'static str = "Infested Stone Bricks";
		const HARDNESS: f32 = 0.75;
		const RESISTANCE: f32 = 0.75;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for InfestedMossyStoneBricks {
		const DISPLAY_NAME: &'static str = "Infested Mossy Stone Bricks";
		const HARDNESS: f32 = 0.75;
		const RESISTANCE: f32 = 0.75;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for InfestedCrackedStoneBricks {
		const DISPLAY_NAME: &'static str = "Infested Cracked Stone Bricks";
		const HARDNESS: f32 = 0.75;
		const RESISTANCE: f32 = 0.75;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for InfestedChiseledStoneBricks {
		const DISPLAY_NAME: &'static str = "Infested Chiseled Stone Bricks";
		const HARDNESS: f32 = 0.75;
		const RESISTANCE: f32 = 0.75;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BrownMushroomBlock {
		const DISPLAY_NAME: &'static str = "Brown Mushroom Block";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for RedMushroomBlock {
		const DISPLAY_NAME: &'static str = "Red Mushroom Block";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for MushroomStem {
		const DISPLAY_NAME: &'static str = "Mushroom Stem";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for IronBars {
		const DISPLAY_NAME: &'static str = "Iron Bars";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Chain {
		const DISPLAY_NAME: &'static str = "Chain";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GlassPane {
		const DISPLAY_NAME: &'static str = "Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Melon {
		const DISPLAY_NAME: &'static str = "Melon";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for AttachedPumpkinStem {
		const DISPLAY_NAME: &'static str = "Attached Pumpkin Stem";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AttachedMelonStem {
		const DISPLAY_NAME: &'static str = "Attached Melon Stem";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PumpkinStem {
		const DISPLAY_NAME: &'static str = "Pumpkin Stem";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MelonStem {
		const DISPLAY_NAME: &'static str = "Melon Stem";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Vine {
		const DISPLAY_NAME: &'static str = "Vines";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GlowLichen {
		const DISPLAY_NAME: &'static str = "Glow Lichen";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OakFenceGate {
		const DISPLAY_NAME: &'static str = "Oak Fence Gate";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BrickStairs {
		const DISPLAY_NAME: &'static str = "Brick Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for StoneBrickStairs {
		const DISPLAY_NAME: &'static str = "Stone Brick Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MudBrickStairs {
		const DISPLAY_NAME: &'static str = "Mud Brick Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Mycelium {
		const DISPLAY_NAME: &'static str = "Mycelium";
		const HARDNESS: f32 = 0.6;
		const RESISTANCE: f32 = 0.6;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LilyPad {
		const DISPLAY_NAME: &'static str = "Lily Pad";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for NetherBricks {
		const DISPLAY_NAME: &'static str = "Nether Bricks";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for NetherBrickFence {
		const DISPLAY_NAME: &'static str = "Nether Brick Fence";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for NetherBrickStairs {
		const DISPLAY_NAME: &'static str = "Nether Brick Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for NetherWart {
		const DISPLAY_NAME: &'static str = "Nether Wart";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for EnchantingTable {
		const DISPLAY_NAME: &'static str = "Enchanting Table";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 1200.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 7;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BrewingStand {
		const DISPLAY_NAME: &'static str = "Brewing Stand";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 1;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Cauldron {
		const DISPLAY_NAME: &'static str = "Cauldron";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WaterCauldron {
		const DISPLAY_NAME: &'static str = "Water Cauldron";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LavaCauldron {
		const DISPLAY_NAME: &'static str = "Lava Cauldron";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PowderSnowCauldron {
		const DISPLAY_NAME: &'static str = "Powder Snow Cauldron";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for EndPortal {
		const DISPLAY_NAME: &'static str = "End Portal";
		const HARDNESS: f32 = -1.0;
		const RESISTANCE: f32 = 3600000.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for EndPortalFrame {
		const DISPLAY_NAME: &'static str = "End Portal Frame";
		const HARDNESS: f32 = -1.0;
		const RESISTANCE: f32 = 3600000.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 1;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for EndStone {
		const DISPLAY_NAME: &'static str = "End Stone";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 9.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DragonEgg {
		const DISPLAY_NAME: &'static str = "Dragon Egg";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 9.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 1;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedstoneLamp {
		const DISPLAY_NAME: &'static str = "Redstone Lamp";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Cocoa {
		const DISPLAY_NAME: &'static str = "Cocoa";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SandstoneStairs {
		const DISPLAY_NAME: &'static str = "Sandstone Stairs";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for EmeraldOre {
		const DISPLAY_NAME: &'static str = "Emerald Ore";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeepslateEmeraldOre {
		const DISPLAY_NAME: &'static str = "Deepslate Emerald Ore";
		const HARDNESS: f32 = 4.5;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for EnderChest {
		const DISPLAY_NAME: &'static str = "Ender Chest";
		const HARDNESS: f32 = 22.5;
		const RESISTANCE: f32 = 600.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 7;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for TripwireHook {
		const DISPLAY_NAME: &'static str = "Tripwire Hook";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Tripwire {
		const DISPLAY_NAME: &'static str = "Tripwire";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for EmeraldBlock {
		const DISPLAY_NAME: &'static str = "Block of Emerald";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SpruceStairs {
		const DISPLAY_NAME: &'static str = "Spruce Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BirchStairs {
		const DISPLAY_NAME: &'static str = "Birch Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for JungleStairs {
		const DISPLAY_NAME: &'static str = "Jungle Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CommandBlock {
		const DISPLAY_NAME: &'static str = "Command Block";
		const HARDNESS: f32 = -1.0;
		const RESISTANCE: f32 = 3600000.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Beacon {
		const DISPLAY_NAME: &'static str = "Beacon";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for CobblestoneWall {
		const DISPLAY_NAME: &'static str = "Cobblestone Wall";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MossyCobblestoneWall {
		const DISPLAY_NAME: &'static str = "Mossy Cobblestone Wall";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for FlowerPot {
		const DISPLAY_NAME: &'static str = "Flower Pot";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedOakSapling {
		const DISPLAY_NAME: &'static str = "Potted Oak Sapling";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedSpruceSapling {
		const DISPLAY_NAME: &'static str = "Potted Spruce Sapling";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedBirchSapling {
		const DISPLAY_NAME: &'static str = "Potted Birch Sapling";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedJungleSapling {
		const DISPLAY_NAME: &'static str = "Potted Jungle Sapling";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedAcaciaSapling {
		const DISPLAY_NAME: &'static str = "Potted Acacia Sapling";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedDarkOakSapling {
		const DISPLAY_NAME: &'static str = "Potted Dark Oak Sapling";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedMangrovePropagule {
		const DISPLAY_NAME: &'static str = "Potted Mangrove Propagule";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedFern {
		const DISPLAY_NAME: &'static str = "Potted Fern";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedDandelion {
		const DISPLAY_NAME: &'static str = "Potted Dandelion";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedPoppy {
		const DISPLAY_NAME: &'static str = "Potted Poppy";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedBlueOrchid {
		const DISPLAY_NAME: &'static str = "Potted Blue Orchid";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedAllium {
		const DISPLAY_NAME: &'static str = "Potted Allium";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedAzureBluet {
		const DISPLAY_NAME: &'static str = "Potted Azure Bluet";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedRedTulip {
		const DISPLAY_NAME: &'static str = "Potted Red Tulip";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedOrangeTulip {
		const DISPLAY_NAME: &'static str = "Potted Orange Tulip";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedWhiteTulip {
		const DISPLAY_NAME: &'static str = "Potted White Tulip";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedPinkTulip {
		const DISPLAY_NAME: &'static str = "Potted Pink Tulip";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedOxeyeDaisy {
		const DISPLAY_NAME: &'static str = "Potted Oxeye Daisy";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedCornflower {
		const DISPLAY_NAME: &'static str = "Potted Cornflower";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedLilyOfTheValley {
		const DISPLAY_NAME: &'static str = "Potted Lily of the Valley";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedWitherRose {
		const DISPLAY_NAME: &'static str = "Potted Wither Rose";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedRedMushroom {
		const DISPLAY_NAME: &'static str = "Potted Red Mushroom";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedBrownMushroom {
		const DISPLAY_NAME: &'static str = "Potted Brown Mushroom";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedDeadBush {
		const DISPLAY_NAME: &'static str = "Potted Dead Bush";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedCactus {
		const DISPLAY_NAME: &'static str = "Potted Cactus";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Carrots {
		const DISPLAY_NAME: &'static str = "Carrots";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Potatoes {
		const DISPLAY_NAME: &'static str = "Potatoes";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OakButton {
		const DISPLAY_NAME: &'static str = "Oak Button";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SpruceButton {
		const DISPLAY_NAME: &'static str = "Spruce Button";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BirchButton {
		const DISPLAY_NAME: &'static str = "Birch Button";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for JungleButton {
		const DISPLAY_NAME: &'static str = "Jungle Button";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AcaciaButton {
		const DISPLAY_NAME: &'static str = "Acacia Button";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DarkOakButton {
		const DISPLAY_NAME: &'static str = "Dark Oak Button";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MangroveButton {
		const DISPLAY_NAME: &'static str = "Mangrove Button";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SkeletonSkull {
		const DISPLAY_NAME: &'static str = "Skeleton Skull";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SkeletonWallSkull {
		const DISPLAY_NAME: &'static str = "Skeleton Skull";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WitherSkeletonSkull {
		const DISPLAY_NAME: &'static str = "Wither Skeleton Skull";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WitherSkeletonWallSkull {
		const DISPLAY_NAME: &'static str = "Wither Skeleton Skull";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for ZombieHead {
		const DISPLAY_NAME: &'static str = "Zombie Head";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for ZombieWallHead {
		const DISPLAY_NAME: &'static str = "Zombie Head";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PlayerHead {
		const DISPLAY_NAME: &'static str = "Player Head";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PlayerWallHead {
		const DISPLAY_NAME: &'static str = "Player Head";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CreeperHead {
		const DISPLAY_NAME: &'static str = "Creeper Head";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CreeperWallHead {
		const DISPLAY_NAME: &'static str = "Creeper Head";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DragonHead {
		const DISPLAY_NAME: &'static str = "Dragon Head";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DragonWallHead {
		const DISPLAY_NAME: &'static str = "Dragon Head";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Anvil {
		const DISPLAY_NAME: &'static str = "Anvil";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 1200.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for ChippedAnvil {
		const DISPLAY_NAME: &'static str = "Chipped Anvil";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 1200.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DamagedAnvil {
		const DISPLAY_NAME: &'static str = "Damaged Anvil";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 1200.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for TrappedChest {
		const DISPLAY_NAME: &'static str = "Trapped Chest";
		const HARDNESS: f32 = 2.5;
		const RESISTANCE: f32 = 2.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightWeightedPressurePlate {
		const DISPLAY_NAME: &'static str = "Light Weighted Pressure Plate";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for HeavyWeightedPressurePlate {
		const DISPLAY_NAME: &'static str = "Heavy Weighted Pressure Plate";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Comparator {
		const DISPLAY_NAME: &'static str = "Redstone Comparator";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DaylightDetector {
		const DISPLAY_NAME: &'static str = "Daylight Detector";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedstoneBlock {
		const DISPLAY_NAME: &'static str = "Block of Redstone";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for NetherQuartzOre {
		const DISPLAY_NAME: &'static str = "Nether Quartz Ore";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Hopper {
		const DISPLAY_NAME: &'static str = "Hopper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 4.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for QuartzBlock {
		const DISPLAY_NAME: &'static str = "Block of Quartz";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for ChiseledQuartzBlock {
		const DISPLAY_NAME: &'static str = "Chiseled Quartz Block";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for QuartzPillar {
		const DISPLAY_NAME: &'static str = "Quartz Pillar";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for QuartzStairs {
		const DISPLAY_NAME: &'static str = "Quartz Stairs";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for ActivatorRail {
		const DISPLAY_NAME: &'static str = "Activator Rail";
		const HARDNESS: f32 = 0.7;
		const RESISTANCE: f32 = 0.7;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Dropper {
		const DISPLAY_NAME: &'static str = "Dropper";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 3.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WhiteTerracotta {
		const DISPLAY_NAME: &'static str = "White Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for OrangeTerracotta {
		const DISPLAY_NAME: &'static str = "Orange Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for MagentaTerracotta {
		const DISPLAY_NAME: &'static str = "Magenta Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LightBlueTerracotta {
		const DISPLAY_NAME: &'static str = "Light Blue Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for YellowTerracotta {
		const DISPLAY_NAME: &'static str = "Yellow Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LimeTerracotta {
		const DISPLAY_NAME: &'static str = "Lime Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PinkTerracotta {
		const DISPLAY_NAME: &'static str = "Pink Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for GrayTerracotta {
		const DISPLAY_NAME: &'static str = "Gray Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LightGrayTerracotta {
		const DISPLAY_NAME: &'static str = "Light Gray Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CyanTerracotta {
		const DISPLAY_NAME: &'static str = "Cyan Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PurpleTerracotta {
		const DISPLAY_NAME: &'static str = "Purple Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BlueTerracotta {
		const DISPLAY_NAME: &'static str = "Blue Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BrownTerracotta {
		const DISPLAY_NAME: &'static str = "Brown Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for GreenTerracotta {
		const DISPLAY_NAME: &'static str = "Green Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for RedTerracotta {
		const DISPLAY_NAME: &'static str = "Red Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BlackTerracotta {
		const DISPLAY_NAME: &'static str = "Black Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WhiteStainedGlassPane {
		const DISPLAY_NAME: &'static str = "White Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OrangeStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Orange Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MagentaStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Magenta Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightBlueStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Light Blue Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for YellowStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Yellow Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LimeStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Lime Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PinkStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Pink Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GrayStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Gray Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightGrayStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Light Gray Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CyanStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Cyan Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PurpleStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Purple Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlueStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Blue Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BrownStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Brown Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GreenStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Green Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Red Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlackStainedGlassPane {
		const DISPLAY_NAME: &'static str = "Black Stained Glass Pane";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AcaciaStairs {
		const DISPLAY_NAME: &'static str = "Acacia Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DarkOakStairs {
		const DISPLAY_NAME: &'static str = "Dark Oak Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MangroveStairs {
		const DISPLAY_NAME: &'static str = "Mangrove Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SlimeBlock {
		const DISPLAY_NAME: &'static str = "Slime Block";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for Barrier {
		const DISPLAY_NAME: &'static str = "Barrier";
		const HARDNESS: f32 = -1.0;
		const RESISTANCE: f32 = 3600000.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Light {
		const DISPLAY_NAME: &'static str = "Light";
		const HARDNESS: f32 = -1.0;
		const RESISTANCE: f32 = 3600000.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for IronTrapdoor {
		const DISPLAY_NAME: &'static str = "Iron Trapdoor";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 5.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Prismarine {
		const DISPLAY_NAME: &'static str = "Prismarine";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PrismarineBricks {
		const DISPLAY_NAME: &'static str = "Prismarine Bricks";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DarkPrismarine {
		const DISPLAY_NAME: &'static str = "Dark Prismarine";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PrismarineStairs {
		const DISPLAY_NAME: &'static str = "Prismarine Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PrismarineBrickStairs {
		const DISPLAY_NAME: &'static str = "Prismarine Brick Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DarkPrismarineStairs {
		const DISPLAY_NAME: &'static str = "Dark Prismarine Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PrismarineSlab {
		const DISPLAY_NAME: &'static str = "Prismarine Slab";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PrismarineBrickSlab {
		const DISPLAY_NAME: &'static str = "Prismarine Brick Slab";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DarkPrismarineSlab {
		const DISPLAY_NAME: &'static str = "Dark Prismarine Slab";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SeaLantern {
		const DISPLAY_NAME: &'static str = "Sea Lantern";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for HayBlock {
		const DISPLAY_NAME: &'static str = "Hay Bale";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WhiteCarpet {
		const DISPLAY_NAME: &'static str = "White Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OrangeCarpet {
		const DISPLAY_NAME: &'static str = "Orange Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MagentaCarpet {
		const DISPLAY_NAME: &'static str = "Magenta Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightBlueCarpet {
		const DISPLAY_NAME: &'static str = "Light Blue Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for YellowCarpet {
		const DISPLAY_NAME: &'static str = "Yellow Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LimeCarpet {
		const DISPLAY_NAME: &'static str = "Lime Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PinkCarpet {
		const DISPLAY_NAME: &'static str = "Pink Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GrayCarpet {
		const DISPLAY_NAME: &'static str = "Gray Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightGrayCarpet {
		const DISPLAY_NAME: &'static str = "Light Gray Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CyanCarpet {
		const DISPLAY_NAME: &'static str = "Cyan Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PurpleCarpet {
		const DISPLAY_NAME: &'static str = "Purple Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlueCarpet {
		const DISPLAY_NAME: &'static str = "Blue Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BrownCarpet {
		const DISPLAY_NAME: &'static str = "Brown Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GreenCarpet {
		const DISPLAY_NAME: &'static str = "Green Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedCarpet {
		const DISPLAY_NAME: &'static str = "Red Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlackCarpet {
		const DISPLAY_NAME: &'static str = "Black Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Terracotta {
		const DISPLAY_NAME: &'static str = "Terracotta";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CoalBlock {
		const DISPLAY_NAME: &'static str = "Block of Coal";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PackedIce {
		const DISPLAY_NAME: &'static str = "Packed Ice";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Sunflower {
		const DISPLAY_NAME: &'static str = "Sunflower";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Lilac {
		const DISPLAY_NAME: &'static str = "Lilac";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RoseBush {
		const DISPLAY_NAME: &'static str = "Rose Bush";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Peony {
		const DISPLAY_NAME: &'static str = "Peony";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for TallGrass {
		const DISPLAY_NAME: &'static str = "Tall Grass";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LargeFern {
		const DISPLAY_NAME: &'static str = "Large Fern";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WhiteBanner {
		const DISPLAY_NAME: &'static str = "White Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OrangeBanner {
		const DISPLAY_NAME: &'static str = "Orange Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MagentaBanner {
		const DISPLAY_NAME: &'static str = "Magenta Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightBlueBanner {
		const DISPLAY_NAME: &'static str = "Light Blue Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for YellowBanner {
		const DISPLAY_NAME: &'static str = "Yellow Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LimeBanner {
		const DISPLAY_NAME: &'static str = "Lime Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PinkBanner {
		const DISPLAY_NAME: &'static str = "Pink Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GrayBanner {
		const DISPLAY_NAME: &'static str = "Gray Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightGrayBanner {
		const DISPLAY_NAME: &'static str = "Light Gray Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CyanBanner {
		const DISPLAY_NAME: &'static str = "Cyan Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PurpleBanner {
		const DISPLAY_NAME: &'static str = "Purple Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlueBanner {
		const DISPLAY_NAME: &'static str = "Blue Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BrownBanner {
		const DISPLAY_NAME: &'static str = "Brown Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GreenBanner {
		const DISPLAY_NAME: &'static str = "Green Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedBanner {
		const DISPLAY_NAME: &'static str = "Red Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlackBanner {
		const DISPLAY_NAME: &'static str = "Black Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WhiteWallBanner {
		const DISPLAY_NAME: &'static str = "White Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OrangeWallBanner {
		const DISPLAY_NAME: &'static str = "Orange Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MagentaWallBanner {
		const DISPLAY_NAME: &'static str = "Magenta Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightBlueWallBanner {
		const DISPLAY_NAME: &'static str = "Light Blue Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for YellowWallBanner {
		const DISPLAY_NAME: &'static str = "Yellow Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LimeWallBanner {
		const DISPLAY_NAME: &'static str = "Lime Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PinkWallBanner {
		const DISPLAY_NAME: &'static str = "Pink Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GrayWallBanner {
		const DISPLAY_NAME: &'static str = "Gray Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightGrayWallBanner {
		const DISPLAY_NAME: &'static str = "Light Gray Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CyanWallBanner {
		const DISPLAY_NAME: &'static str = "Cyan Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PurpleWallBanner {
		const DISPLAY_NAME: &'static str = "Purple Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlueWallBanner {
		const DISPLAY_NAME: &'static str = "Blue Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BrownWallBanner {
		const DISPLAY_NAME: &'static str = "Brown Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GreenWallBanner {
		const DISPLAY_NAME: &'static str = "Green Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedWallBanner {
		const DISPLAY_NAME: &'static str = "Red Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlackWallBanner {
		const DISPLAY_NAME: &'static str = "Black Banner";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedSandstone {
		const DISPLAY_NAME: &'static str = "Red Sandstone";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for ChiseledRedSandstone {
		const DISPLAY_NAME: &'static str = "Chiseled Red Sandstone";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CutRedSandstone {
		const DISPLAY_NAME: &'static str = "Cut Red Sandstone";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for RedSandstoneStairs {
		const DISPLAY_NAME: &'static str = "Red Sandstone Stairs";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OakSlab {
		const DISPLAY_NAME: &'static str = "Oak Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SpruceSlab {
		const DISPLAY_NAME: &'static str = "Spruce Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BirchSlab {
		const DISPLAY_NAME: &'static str = "Birch Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for JungleSlab {
		const DISPLAY_NAME: &'static str = "Jungle Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AcaciaSlab {
		const DISPLAY_NAME: &'static str = "Acacia Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DarkOakSlab {
		const DISPLAY_NAME: &'static str = "Dark Oak Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MangroveSlab {
		const DISPLAY_NAME: &'static str = "Mangrove Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for StoneSlab {
		const DISPLAY_NAME: &'static str = "Stone Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SmoothStoneSlab {
		const DISPLAY_NAME: &'static str = "Smooth Stone Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SandstoneSlab {
		const DISPLAY_NAME: &'static str = "Sandstone Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CutSandstoneSlab {
		const DISPLAY_NAME: &'static str = "Cut Sandstone Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PetrifiedOakSlab {
		const DISPLAY_NAME: &'static str = "Petrified Oak Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CobblestoneSlab {
		const DISPLAY_NAME: &'static str = "Cobblestone Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BrickSlab {
		const DISPLAY_NAME: &'static str = "Brick Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for StoneBrickSlab {
		const DISPLAY_NAME: &'static str = "Stone Brick Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MudBrickSlab {
		const DISPLAY_NAME: &'static str = "Mud Brick Slab";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for NetherBrickSlab {
		const DISPLAY_NAME: &'static str = "Nether Brick Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for QuartzSlab {
		const DISPLAY_NAME: &'static str = "Quartz Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedSandstoneSlab {
		const DISPLAY_NAME: &'static str = "Red Sandstone Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CutRedSandstoneSlab {
		const DISPLAY_NAME: &'static str = "Cut Red Sandstone Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PurpurSlab {
		const DISPLAY_NAME: &'static str = "Purpur Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SmoothStone {
		const DISPLAY_NAME: &'static str = "Smooth Stone";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SmoothSandstone {
		const DISPLAY_NAME: &'static str = "Smooth Sandstone";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SmoothQuartz {
		const DISPLAY_NAME: &'static str = "Smooth Quartz Block";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SmoothRedSandstone {
		const DISPLAY_NAME: &'static str = "Smooth Red Sandstone";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SpruceFenceGate {
		const DISPLAY_NAME: &'static str = "Spruce Fence Gate";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BirchFenceGate {
		const DISPLAY_NAME: &'static str = "Birch Fence Gate";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for JungleFenceGate {
		const DISPLAY_NAME: &'static str = "Jungle Fence Gate";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AcaciaFenceGate {
		const DISPLAY_NAME: &'static str = "Acacia Fence Gate";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DarkOakFenceGate {
		const DISPLAY_NAME: &'static str = "Dark Oak Fence Gate";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MangroveFenceGate {
		const DISPLAY_NAME: &'static str = "Mangrove Fence Gate";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SpruceFence {
		const DISPLAY_NAME: &'static str = "Spruce Fence";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BirchFence {
		const DISPLAY_NAME: &'static str = "Birch Fence";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for JungleFence {
		const DISPLAY_NAME: &'static str = "Jungle Fence";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AcaciaFence {
		const DISPLAY_NAME: &'static str = "Acacia Fence";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DarkOakFence {
		const DISPLAY_NAME: &'static str = "Dark Oak Fence";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MangroveFence {
		const DISPLAY_NAME: &'static str = "Mangrove Fence";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SpruceDoor {
		const DISPLAY_NAME: &'static str = "Spruce Door";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BirchDoor {
		const DISPLAY_NAME: &'static str = "Birch Door";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for JungleDoor {
		const DISPLAY_NAME: &'static str = "Jungle Door";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AcaciaDoor {
		const DISPLAY_NAME: &'static str = "Acacia Door";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DarkOakDoor {
		const DISPLAY_NAME: &'static str = "Dark Oak Door";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MangroveDoor {
		const DISPLAY_NAME: &'static str = "Mangrove Door";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for EndRod {
		const DISPLAY_NAME: &'static str = "End Rod";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 14;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for ChorusPlant {
		const DISPLAY_NAME: &'static str = "Chorus Plant";
		const HARDNESS: f32 = 0.4;
		const RESISTANCE: f32 = 0.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for ChorusFlower {
		const DISPLAY_NAME: &'static str = "Chorus Flower";
		const HARDNESS: f32 = 0.4;
		const RESISTANCE: f32 = 0.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for PurpurBlock {
		const DISPLAY_NAME: &'static str = "Purpur Block";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PurpurPillar {
		const DISPLAY_NAME: &'static str = "Purpur Pillar";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PurpurStairs {
		const DISPLAY_NAME: &'static str = "Purpur Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for EndStoneBricks {
		const DISPLAY_NAME: &'static str = "End Stone Bricks";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 9.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Beetroots {
		const DISPLAY_NAME: &'static str = "Beetroots";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DirtPath {
		const DISPLAY_NAME: &'static str = "Dirt Path";
		const HARDNESS: f32 = 0.65;
		const RESISTANCE: f32 = 0.65;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for EndGateway {
		const DISPLAY_NAME: &'static str = "End Gateway";
		const HARDNESS: f32 = -1.0;
		const RESISTANCE: f32 = 3600000.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for RepeatingCommandBlock {
		const DISPLAY_NAME: &'static str = "Repeating Command Block";
		const HARDNESS: f32 = -1.0;
		const RESISTANCE: f32 = 3600000.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for ChainCommandBlock {
		const DISPLAY_NAME: &'static str = "Chain Command Block";
		const HARDNESS: f32 = -1.0;
		const RESISTANCE: f32 = 3600000.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for FrostedIce {
		const DISPLAY_NAME: &'static str = "Frosted Ice";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for MagmaBlock {
		const DISPLAY_NAME: &'static str = "Magma Block";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 3;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for NetherWartBlock {
		const DISPLAY_NAME: &'static str = "Nether Wart Block";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for RedNetherBricks {
		const DISPLAY_NAME: &'static str = "Red Nether Bricks";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BoneBlock {
		const DISPLAY_NAME: &'static str = "Bone Block";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StructureVoid {
		const DISPLAY_NAME: &'static str = "Structure Void";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Observer {
		const DISPLAY_NAME: &'static str = "Observer";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for ShulkerBox {
		const DISPLAY_NAME: &'static str = "Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for WhiteShulkerBox {
		const DISPLAY_NAME: &'static str = "White Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for OrangeShulkerBox {
		const DISPLAY_NAME: &'static str = "Orange Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for MagentaShulkerBox {
		const DISPLAY_NAME: &'static str = "Magenta Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for LightBlueShulkerBox {
		const DISPLAY_NAME: &'static str = "Light Blue Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for YellowShulkerBox {
		const DISPLAY_NAME: &'static str = "Yellow Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for LimeShulkerBox {
		const DISPLAY_NAME: &'static str = "Lime Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for PinkShulkerBox {
		const DISPLAY_NAME: &'static str = "Pink Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for GrayShulkerBox {
		const DISPLAY_NAME: &'static str = "Gray Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for LightGrayShulkerBox {
		const DISPLAY_NAME: &'static str = "Light Gray Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for CyanShulkerBox {
		const DISPLAY_NAME: &'static str = "Cyan Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for PurpleShulkerBox {
		const DISPLAY_NAME: &'static str = "Purple Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for BlueShulkerBox {
		const DISPLAY_NAME: &'static str = "Blue Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for BrownShulkerBox {
		const DISPLAY_NAME: &'static str = "Brown Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for GreenShulkerBox {
		const DISPLAY_NAME: &'static str = "Green Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for RedShulkerBox {
		const DISPLAY_NAME: &'static str = "Red Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for BlackShulkerBox {
		const DISPLAY_NAME: &'static str = "Black Shulker Box";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for WhiteGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "White Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for OrangeGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Orange Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for MagentaGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Magenta Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LightBlueGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Light Blue Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for YellowGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Yellow Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LimeGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Lime Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PinkGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Pink Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for GrayGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Gray Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LightGrayGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Light Gray Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CyanGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Cyan Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PurpleGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Purple Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BlueGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Blue Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BrownGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Brown Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for GreenGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Green Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for RedGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Red Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BlackGlazedTerracotta {
		const DISPLAY_NAME: &'static str = "Black Glazed Terracotta";
		const HARDNESS: f32 = 1.4;
		const RESISTANCE: f32 = 1.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WhiteConcrete {
		const DISPLAY_NAME: &'static str = "White Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for OrangeConcrete {
		const DISPLAY_NAME: &'static str = "Orange Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for MagentaConcrete {
		const DISPLAY_NAME: &'static str = "Magenta Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LightBlueConcrete {
		const DISPLAY_NAME: &'static str = "Light Blue Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for YellowConcrete {
		const DISPLAY_NAME: &'static str = "Yellow Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LimeConcrete {
		const DISPLAY_NAME: &'static str = "Lime Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PinkConcrete {
		const DISPLAY_NAME: &'static str = "Pink Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for GrayConcrete {
		const DISPLAY_NAME: &'static str = "Gray Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LightGrayConcrete {
		const DISPLAY_NAME: &'static str = "Light Gray Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CyanConcrete {
		const DISPLAY_NAME: &'static str = "Cyan Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PurpleConcrete {
		const DISPLAY_NAME: &'static str = "Purple Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BlueConcrete {
		const DISPLAY_NAME: &'static str = "Blue Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BrownConcrete {
		const DISPLAY_NAME: &'static str = "Brown Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for GreenConcrete {
		const DISPLAY_NAME: &'static str = "Green Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for RedConcrete {
		const DISPLAY_NAME: &'static str = "Red Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BlackConcrete {
		const DISPLAY_NAME: &'static str = "Black Concrete";
		const HARDNESS: f32 = 1.8;
		const RESISTANCE: f32 = 1.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WhiteConcretePowder {
		const DISPLAY_NAME: &'static str = "White Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for OrangeConcretePowder {
		const DISPLAY_NAME: &'static str = "Orange Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for MagentaConcretePowder {
		const DISPLAY_NAME: &'static str = "Magenta Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LightBlueConcretePowder {
		const DISPLAY_NAME: &'static str = "Light Blue Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for YellowConcretePowder {
		const DISPLAY_NAME: &'static str = "Yellow Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LimeConcretePowder {
		const DISPLAY_NAME: &'static str = "Lime Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PinkConcretePowder {
		const DISPLAY_NAME: &'static str = "Pink Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for GrayConcretePowder {
		const DISPLAY_NAME: &'static str = "Gray Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for LightGrayConcretePowder {
		const DISPLAY_NAME: &'static str = "Light Gray Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CyanConcretePowder {
		const DISPLAY_NAME: &'static str = "Cyan Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PurpleConcretePowder {
		const DISPLAY_NAME: &'static str = "Purple Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BlueConcretePowder {
		const DISPLAY_NAME: &'static str = "Blue Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BrownConcretePowder {
		const DISPLAY_NAME: &'static str = "Brown Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for GreenConcretePowder {
		const DISPLAY_NAME: &'static str = "Green Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for RedConcretePowder {
		const DISPLAY_NAME: &'static str = "Red Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BlackConcretePowder {
		const DISPLAY_NAME: &'static str = "Black Concrete Powder";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Kelp {
		const DISPLAY_NAME: &'static str = "Kelp";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for KelpPlant {
		const DISPLAY_NAME: &'static str = "Kelp Plant";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DriedKelpBlock {
		const DISPLAY_NAME: &'static str = "Dried Kelp Block";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 2.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for TurtleEgg {
		const DISPLAY_NAME: &'static str = "Turtle Egg";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DeadTubeCoralBlock {
		const DISPLAY_NAME: &'static str = "Dead Tube Coral Block";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeadBrainCoralBlock {
		const DISPLAY_NAME: &'static str = "Dead Brain Coral Block";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeadBubbleCoralBlock {
		const DISPLAY_NAME: &'static str = "Dead Bubble Coral Block";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeadFireCoralBlock {
		const DISPLAY_NAME: &'static str = "Dead Fire Coral Block";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeadHornCoralBlock {
		const DISPLAY_NAME: &'static str = "Dead Horn Coral Block";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for TubeCoralBlock {
		const DISPLAY_NAME: &'static str = "Tube Coral Block";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BrainCoralBlock {
		const DISPLAY_NAME: &'static str = "Brain Coral Block";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BubbleCoralBlock {
		const DISPLAY_NAME: &'static str = "Bubble Coral Block";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for FireCoralBlock {
		const DISPLAY_NAME: &'static str = "Fire Coral Block";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for HornCoralBlock {
		const DISPLAY_NAME: &'static str = "Horn Coral Block";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeadTubeCoral {
		const DISPLAY_NAME: &'static str = "Dead Tube Coral";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DeadBrainCoral {
		const DISPLAY_NAME: &'static str = "Dead Brain Coral";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DeadBubbleCoral {
		const DISPLAY_NAME: &'static str = "Dead Bubble Coral";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DeadFireCoral {
		const DISPLAY_NAME: &'static str = "Dead Fire Coral";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DeadHornCoral {
		const DISPLAY_NAME: &'static str = "Dead Horn Coral";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for TubeCoral {
		const DISPLAY_NAME: &'static str = "Tube Coral";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for BrainCoral {
		const DISPLAY_NAME: &'static str = "Brain Coral";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for BubbleCoral {
		const DISPLAY_NAME: &'static str = "Bubble Coral";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for FireCoral {
		const DISPLAY_NAME: &'static str = "Fire Coral";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for HornCoral {
		const DISPLAY_NAME: &'static str = "Horn Coral";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DeadTubeCoralFan {
		const DISPLAY_NAME: &'static str = "Dead Tube Coral Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DeadBrainCoralFan {
		const DISPLAY_NAME: &'static str = "Dead Brain Coral Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DeadBubbleCoralFan {
		const DISPLAY_NAME: &'static str = "Dead Bubble Coral Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DeadFireCoralFan {
		const DISPLAY_NAME: &'static str = "Dead Fire Coral Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DeadHornCoralFan {
		const DISPLAY_NAME: &'static str = "Dead Horn Coral Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for TubeCoralFan {
		const DISPLAY_NAME: &'static str = "Tube Coral Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for BrainCoralFan {
		const DISPLAY_NAME: &'static str = "Brain Coral Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for BubbleCoralFan {
		const DISPLAY_NAME: &'static str = "Bubble Coral Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for FireCoralFan {
		const DISPLAY_NAME: &'static str = "Fire Coral Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for HornCoralFan {
		const DISPLAY_NAME: &'static str = "Horn Coral Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DeadTubeCoralWallFan {
		const DISPLAY_NAME: &'static str = "Dead Tube Coral Wall Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DeadBrainCoralWallFan {
		const DISPLAY_NAME: &'static str = "Dead Brain Coral Wall Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DeadBubbleCoralWallFan {
		const DISPLAY_NAME: &'static str = "Dead Bubble Coral Wall Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DeadFireCoralWallFan {
		const DISPLAY_NAME: &'static str = "Dead Fire Coral Wall Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for DeadHornCoralWallFan {
		const DISPLAY_NAME: &'static str = "Dead Horn Coral Wall Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for TubeCoralWallFan {
		const DISPLAY_NAME: &'static str = "Tube Coral Wall Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for BrainCoralWallFan {
		const DISPLAY_NAME: &'static str = "Brain Coral Wall Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for BubbleCoralWallFan {
		const DISPLAY_NAME: &'static str = "Bubble Coral Wall Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for FireCoralWallFan {
		const DISPLAY_NAME: &'static str = "Fire Coral Wall Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for HornCoralWallFan {
		const DISPLAY_NAME: &'static str = "Horn Coral Wall Fan";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for SeaPickle {
		const DISPLAY_NAME: &'static str = "Sea Pickle";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 6;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for BlueIce {
		const DISPLAY_NAME: &'static str = "Blue Ice";
		const HARDNESS: f32 = 2.8;
		const RESISTANCE: f32 = 2.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Conduit {
		const DISPLAY_NAME: &'static str = "Conduit";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for BambooSapling {
		const DISPLAY_NAME: &'static str = "Bamboo Shoot";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Bamboo {
		const DISPLAY_NAME: &'static str = "Bamboo";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedBamboo {
		const DISPLAY_NAME: &'static str = "Potted Bamboo";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for VoidAir {
		const DISPLAY_NAME: &'static str = "Void Air";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CaveAir {
		const DISPLAY_NAME: &'static str = "Cave Air";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BubbleColumn {
		const DISPLAY_NAME: &'static str = "Bubble Column";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for PolishedGraniteStairs {
		const DISPLAY_NAME: &'static str = "Polished Granite Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SmoothRedSandstoneStairs {
		const DISPLAY_NAME: &'static str = "Smooth Red Sandstone Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MossyStoneBrickStairs {
		const DISPLAY_NAME: &'static str = "Mossy Stone Brick Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedDioriteStairs {
		const DISPLAY_NAME: &'static str = "Polished Diorite Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MossyCobblestoneStairs {
		const DISPLAY_NAME: &'static str = "Mossy Cobblestone Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for EndStoneBrickStairs {
		const DISPLAY_NAME: &'static str = "End Stone Brick Stairs";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 9.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for StoneStairs {
		const DISPLAY_NAME: &'static str = "Stone Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SmoothSandstoneStairs {
		const DISPLAY_NAME: &'static str = "Smooth Sandstone Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SmoothQuartzStairs {
		const DISPLAY_NAME: &'static str = "Smooth Quartz Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GraniteStairs {
		const DISPLAY_NAME: &'static str = "Granite Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AndesiteStairs {
		const DISPLAY_NAME: &'static str = "Andesite Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedNetherBrickStairs {
		const DISPLAY_NAME: &'static str = "Red Nether Brick Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedAndesiteStairs {
		const DISPLAY_NAME: &'static str = "Polished Andesite Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DioriteStairs {
		const DISPLAY_NAME: &'static str = "Diorite Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedGraniteSlab {
		const DISPLAY_NAME: &'static str = "Polished Granite Slab";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SmoothRedSandstoneSlab {
		const DISPLAY_NAME: &'static str = "Smooth Red Sandstone Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MossyStoneBrickSlab {
		const DISPLAY_NAME: &'static str = "Mossy Stone Brick Slab";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedDioriteSlab {
		const DISPLAY_NAME: &'static str = "Polished Diorite Slab";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MossyCobblestoneSlab {
		const DISPLAY_NAME: &'static str = "Mossy Cobblestone Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for EndStoneBrickSlab {
		const DISPLAY_NAME: &'static str = "End Stone Brick Slab";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 9.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SmoothSandstoneSlab {
		const DISPLAY_NAME: &'static str = "Smooth Sandstone Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SmoothQuartzSlab {
		const DISPLAY_NAME: &'static str = "Smooth Quartz Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GraniteSlab {
		const DISPLAY_NAME: &'static str = "Granite Slab";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AndesiteSlab {
		const DISPLAY_NAME: &'static str = "Andesite Slab";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedNetherBrickSlab {
		const DISPLAY_NAME: &'static str = "Red Nether Brick Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedAndesiteSlab {
		const DISPLAY_NAME: &'static str = "Polished Andesite Slab";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DioriteSlab {
		const DISPLAY_NAME: &'static str = "Diorite Slab";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BrickWall {
		const DISPLAY_NAME: &'static str = "Brick Wall";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PrismarineWall {
		const DISPLAY_NAME: &'static str = "Prismarine Wall";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedSandstoneWall {
		const DISPLAY_NAME: &'static str = "Red Sandstone Wall";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MossyStoneBrickWall {
		const DISPLAY_NAME: &'static str = "Mossy Stone Brick Wall";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GraniteWall {
		const DISPLAY_NAME: &'static str = "Granite Wall";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for StoneBrickWall {
		const DISPLAY_NAME: &'static str = "Stone Brick Wall";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MudBrickWall {
		const DISPLAY_NAME: &'static str = "Mud Brick Wall";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for NetherBrickWall {
		const DISPLAY_NAME: &'static str = "Nether Brick Wall";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AndesiteWall {
		const DISPLAY_NAME: &'static str = "Andesite Wall";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedNetherBrickWall {
		const DISPLAY_NAME: &'static str = "Red Nether Brick Wall";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SandstoneWall {
		const DISPLAY_NAME: &'static str = "Sandstone Wall";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for EndStoneBrickWall {
		const DISPLAY_NAME: &'static str = "End Stone Brick Wall";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 9.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DioriteWall {
		const DISPLAY_NAME: &'static str = "Diorite Wall";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Scaffolding {
		const DISPLAY_NAME: &'static str = "Scaffolding";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Loom {
		const DISPLAY_NAME: &'static str = "Loom";
		const HARDNESS: f32 = 2.5;
		const RESISTANCE: f32 = 2.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Barrel {
		const DISPLAY_NAME: &'static str = "Barrel";
		const HARDNESS: f32 = 2.5;
		const RESISTANCE: f32 = 2.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Smoker {
		const DISPLAY_NAME: &'static str = "Smoker";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 3.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BlastFurnace {
		const DISPLAY_NAME: &'static str = "Blast Furnace";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 3.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CartographyTable {
		const DISPLAY_NAME: &'static str = "Cartography Table";
		const HARDNESS: f32 = 2.5;
		const RESISTANCE: f32 = 2.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for FletchingTable {
		const DISPLAY_NAME: &'static str = "Fletching Table";
		const HARDNESS: f32 = 2.5;
		const RESISTANCE: f32 = 2.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Grindstone {
		const DISPLAY_NAME: &'static str = "Grindstone";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Lectern {
		const DISPLAY_NAME: &'static str = "Lectern";
		const HARDNESS: f32 = 2.5;
		const RESISTANCE: f32 = 2.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SmithingTable {
		const DISPLAY_NAME: &'static str = "Smithing Table";
		const HARDNESS: f32 = 2.5;
		const RESISTANCE: f32 = 2.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Stonecutter {
		const DISPLAY_NAME: &'static str = "Stonecutter";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 3.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Bell {
		const DISPLAY_NAME: &'static str = "Bell";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 5.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Lantern {
		const DISPLAY_NAME: &'static str = "Lantern";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 3.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SoulLantern {
		const DISPLAY_NAME: &'static str = "Soul Lantern";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 3.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 10;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Campfire {
		const DISPLAY_NAME: &'static str = "Campfire";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SoulCampfire {
		const DISPLAY_NAME: &'static str = "Soul Campfire";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 10;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SweetBerryBush {
		const DISPLAY_NAME: &'static str = "Sweet Berry Bush";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WarpedStem {
		const DISPLAY_NAME: &'static str = "Warped Stem";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedWarpedStem {
		const DISPLAY_NAME: &'static str = "Stripped Warped Stem";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WarpedHyphae {
		const DISPLAY_NAME: &'static str = "Warped Hyphae";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedWarpedHyphae {
		const DISPLAY_NAME: &'static str = "Stripped Warped Hyphae";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WarpedNylium {
		const DISPLAY_NAME: &'static str = "Warped Nylium";
		const HARDNESS: f32 = 0.4;
		const RESISTANCE: f32 = 0.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WarpedFungus {
		const DISPLAY_NAME: &'static str = "Warped Fungus";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WarpedWartBlock {
		const DISPLAY_NAME: &'static str = "Warped Wart Block";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WarpedRoots {
		const DISPLAY_NAME: &'static str = "Warped Roots";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for NetherSprouts {
		const DISPLAY_NAME: &'static str = "Nether Sprouts";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CrimsonStem {
		const DISPLAY_NAME: &'static str = "Crimson Stem";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedCrimsonStem {
		const DISPLAY_NAME: &'static str = "Stripped Crimson Stem";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CrimsonHyphae {
		const DISPLAY_NAME: &'static str = "Crimson Hyphae";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for StrippedCrimsonHyphae {
		const DISPLAY_NAME: &'static str = "Stripped Crimson Hyphae";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 2.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CrimsonNylium {
		const DISPLAY_NAME: &'static str = "Crimson Nylium";
		const HARDNESS: f32 = 0.4;
		const RESISTANCE: f32 = 0.4;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CrimsonFungus {
		const DISPLAY_NAME: &'static str = "Crimson Fungus";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Shroomlight {
		const DISPLAY_NAME: &'static str = "Shroomlight";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WeepingVines {
		const DISPLAY_NAME: &'static str = "Weeping Vines";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WeepingVinesPlant {
		const DISPLAY_NAME: &'static str = "Weeping Vines Plant";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for TwistingVines {
		const DISPLAY_NAME: &'static str = "Twisting Vines";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for TwistingVinesPlant {
		const DISPLAY_NAME: &'static str = "Twisting Vines Plant";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CrimsonRoots {
		const DISPLAY_NAME: &'static str = "Crimson Roots";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CrimsonPlanks {
		const DISPLAY_NAME: &'static str = "Crimson Planks";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WarpedPlanks {
		const DISPLAY_NAME: &'static str = "Warped Planks";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CrimsonSlab {
		const DISPLAY_NAME: &'static str = "Crimson Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WarpedSlab {
		const DISPLAY_NAME: &'static str = "Warped Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CrimsonPressurePlate {
		const DISPLAY_NAME: &'static str = "Crimson Pressure Plate";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WarpedPressurePlate {
		const DISPLAY_NAME: &'static str = "Warped Pressure Plate";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CrimsonFence {
		const DISPLAY_NAME: &'static str = "Crimson Fence";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WarpedFence {
		const DISPLAY_NAME: &'static str = "Warped Fence";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CrimsonTrapdoor {
		const DISPLAY_NAME: &'static str = "Crimson Trapdoor";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WarpedTrapdoor {
		const DISPLAY_NAME: &'static str = "Warped Trapdoor";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CrimsonFenceGate {
		const DISPLAY_NAME: &'static str = "Crimson Fence Gate";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WarpedFenceGate {
		const DISPLAY_NAME: &'static str = "Warped Fence Gate";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CrimsonStairs {
		const DISPLAY_NAME: &'static str = "Crimson Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WarpedStairs {
		const DISPLAY_NAME: &'static str = "Warped Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CrimsonButton {
		const DISPLAY_NAME: &'static str = "Crimson Button";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WarpedButton {
		const DISPLAY_NAME: &'static str = "Warped Button";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CrimsonDoor {
		const DISPLAY_NAME: &'static str = "Crimson Door";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WarpedDoor {
		const DISPLAY_NAME: &'static str = "Warped Door";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CrimsonSign {
		const DISPLAY_NAME: &'static str = "Crimson Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WarpedSign {
		const DISPLAY_NAME: &'static str = "Warped Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CrimsonWallSign {
		const DISPLAY_NAME: &'static str = "Crimson Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WarpedWallSign {
		const DISPLAY_NAME: &'static str = "Warped Sign";
		const HARDNESS: f32 = 1.0;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 16;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for StructureBlock {
		const DISPLAY_NAME: &'static str = "Structure Block";
		const HARDNESS: f32 = -1.0;
		const RESISTANCE: f32 = 3600000.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Jigsaw {
		const DISPLAY_NAME: &'static str = "Jigsaw Block";
		const HARDNESS: f32 = -1.0;
		const RESISTANCE: f32 = 3600000.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = false;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Composter {
		const DISPLAY_NAME: &'static str = "Composter";
		const HARDNESS: f32 = 0.6;
		const RESISTANCE: f32 = 0.6;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Target {
		const DISPLAY_NAME: &'static str = "Target";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BeeNest {
		const DISPLAY_NAME: &'static str = "Bee Nest";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Beehive {
		const DISPLAY_NAME: &'static str = "Beehive";
		const HARDNESS: f32 = 0.6;
		const RESISTANCE: f32 = 0.6;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for HoneyBlock {
		const DISPLAY_NAME: &'static str = "Honey Block";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for HoneycombBlock {
		const DISPLAY_NAME: &'static str = "Honeycomb Block";
		const HARDNESS: f32 = 0.6;
		const RESISTANCE: f32 = 0.6;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for NetheriteBlock {
		const DISPLAY_NAME: &'static str = "Block of Netherite";
		const HARDNESS: f32 = 50.0;
		const RESISTANCE: f32 = 1200.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for AncientDebris {
		const DISPLAY_NAME: &'static str = "Ancient Debris";
		const HARDNESS: f32 = 30.0;
		const RESISTANCE: f32 = 1200.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CryingObsidian {
		const DISPLAY_NAME: &'static str = "Crying Obsidian";
		const HARDNESS: f32 = 50.0;
		const RESISTANCE: f32 = 1200.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 10;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for RespawnAnchor {
		const DISPLAY_NAME: &'static str = "Respawn Anchor";
		const HARDNESS: f32 = 50.0;
		const RESISTANCE: f32 = 1200.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PottedCrimsonFungus {
		const DISPLAY_NAME: &'static str = "Potted Crimson Fungus";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedWarpedFungus {
		const DISPLAY_NAME: &'static str = "Potted Warped Fungus";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedCrimsonRoots {
		const DISPLAY_NAME: &'static str = "Potted Crimson Roots";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedWarpedRoots {
		const DISPLAY_NAME: &'static str = "Potted Warped Roots";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Lodestone {
		const DISPLAY_NAME: &'static str = "Lodestone";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 3.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Blackstone {
		const DISPLAY_NAME: &'static str = "Blackstone";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BlackstoneStairs {
		const DISPLAY_NAME: &'static str = "Blackstone Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlackstoneWall {
		const DISPLAY_NAME: &'static str = "Blackstone Wall";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlackstoneSlab {
		const DISPLAY_NAME: &'static str = "Blackstone Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedBlackstone {
		const DISPLAY_NAME: &'static str = "Polished Blackstone";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PolishedBlackstoneBricks {
		const DISPLAY_NAME: &'static str = "Polished Blackstone Bricks";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CrackedPolishedBlackstoneBricks {
		const DISPLAY_NAME: &'static str = "Cracked Polished Blackstone Bricks";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for ChiseledPolishedBlackstone {
		const DISPLAY_NAME: &'static str = "Chiseled Polished Blackstone";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PolishedBlackstoneBrickSlab {
		const DISPLAY_NAME: &'static str = "Polished Blackstone Brick Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedBlackstoneBrickStairs {
		const DISPLAY_NAME: &'static str = "Polished Blackstone Brick Stairs";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedBlackstoneBrickWall {
		const DISPLAY_NAME: &'static str = "Polished Blackstone Brick Wall";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GildedBlackstone {
		const DISPLAY_NAME: &'static str = "Gilded Blackstone";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PolishedBlackstoneStairs {
		const DISPLAY_NAME: &'static str = "Polished Blackstone Stairs";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedBlackstoneSlab {
		const DISPLAY_NAME: &'static str = "Polished Blackstone Slab";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedBlackstonePressurePlate {
		const DISPLAY_NAME: &'static str = "Polished Blackstone Pressure Plate";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedBlackstoneButton {
		const DISPLAY_NAME: &'static str = "Polished Blackstone Button";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedBlackstoneWall {
		const DISPLAY_NAME: &'static str = "Polished Blackstone Wall";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for ChiseledNetherBricks {
		const DISPLAY_NAME: &'static str = "Chiseled Nether Bricks";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CrackedNetherBricks {
		const DISPLAY_NAME: &'static str = "Cracked Nether Bricks";
		const HARDNESS: f32 = 2.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for QuartzBricks {
		const DISPLAY_NAME: &'static str = "Quartz Bricks";
		const HARDNESS: f32 = 0.8;
		const RESISTANCE: f32 = 0.8;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Candle {
		const DISPLAY_NAME: &'static str = "Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WhiteCandle {
		const DISPLAY_NAME: &'static str = "White Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OrangeCandle {
		const DISPLAY_NAME: &'static str = "Orange Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MagentaCandle {
		const DISPLAY_NAME: &'static str = "Magenta Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightBlueCandle {
		const DISPLAY_NAME: &'static str = "Light Blue Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for YellowCandle {
		const DISPLAY_NAME: &'static str = "Yellow Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LimeCandle {
		const DISPLAY_NAME: &'static str = "Lime Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PinkCandle {
		const DISPLAY_NAME: &'static str = "Pink Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GrayCandle {
		const DISPLAY_NAME: &'static str = "Gray Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightGrayCandle {
		const DISPLAY_NAME: &'static str = "Light Gray Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CyanCandle {
		const DISPLAY_NAME: &'static str = "Cyan Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PurpleCandle {
		const DISPLAY_NAME: &'static str = "Purple Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlueCandle {
		const DISPLAY_NAME: &'static str = "Blue Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BrownCandle {
		const DISPLAY_NAME: &'static str = "Brown Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GreenCandle {
		const DISPLAY_NAME: &'static str = "Green Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedCandle {
		const DISPLAY_NAME: &'static str = "Red Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlackCandle {
		const DISPLAY_NAME: &'static str = "Black Candle";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WhiteCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with White Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OrangeCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Orange Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MagentaCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Magenta Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightBlueCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Light Blue Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for YellowCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Yellow Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LimeCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Lime Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PinkCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Pink Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GrayCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Gray Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightGrayCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Light Gray Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CyanCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Cyan Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PurpleCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Purple Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlueCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Blue Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BrownCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Brown Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for GreenCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Green Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RedCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Red Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BlackCandleCake {
		const DISPLAY_NAME: &'static str = "Cake with Black Candle";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for AmethystBlock {
		const DISPLAY_NAME: &'static str = "Block of Amethyst";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 1.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BuddingAmethyst {
		const DISPLAY_NAME: &'static str = "Budding Amethyst";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 1.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for AmethystCluster {
		const DISPLAY_NAME: &'static str = "Amethyst Cluster";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 1.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 5;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LargeAmethystBud {
		const DISPLAY_NAME: &'static str = "Large Amethyst Bud";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 1.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 4;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MediumAmethystBud {
		const DISPLAY_NAME: &'static str = "Medium Amethyst Bud";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 1.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 2;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SmallAmethystBud {
		const DISPLAY_NAME: &'static str = "Small Amethyst Bud";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 1.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 1;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Tuff {
		const DISPLAY_NAME: &'static str = "Tuff";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Calcite {
		const DISPLAY_NAME: &'static str = "Calcite";
		const HARDNESS: f32 = 0.75;
		const RESISTANCE: f32 = 0.75;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for TintedGlass {
		const DISPLAY_NAME: &'static str = "Tinted Glass";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PowderSnow {
		const DISPLAY_NAME: &'static str = "Powder Snow";
		const HARDNESS: f32 = 0.25;
		const RESISTANCE: f32 = 0.25;
		const STACK_SIZE: u8 = 1;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for SculkSensor {
		const DISPLAY_NAME: &'static str = "Sculk Sensor";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 1.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 1;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Sculk {
		const DISPLAY_NAME: &'static str = "Sculk";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SculkVein {
		const DISPLAY_NAME: &'static str = "Sculk Vein";
		const HARDNESS: f32 = 0.2;
		const RESISTANCE: f32 = 0.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for SculkCatalyst {
		const DISPLAY_NAME: &'static str = "Sculk Catalyst";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 6;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SculkShrieker {
		const DISPLAY_NAME: &'static str = "Sculk Shrieker";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 1;
	}

	impl BlockProperties for OxidizedCopper {
		const DISPLAY_NAME: &'static str = "Oxidized Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WeatheredCopper {
		const DISPLAY_NAME: &'static str = "Weathered Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for ExposedCopper {
		const DISPLAY_NAME: &'static str = "Exposed Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CopperBlock {
		const DISPLAY_NAME: &'static str = "Block of Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CopperOre {
		const DISPLAY_NAME: &'static str = "Copper Ore";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeepslateCopperOre {
		const DISPLAY_NAME: &'static str = "Deepslate Copper Ore";
		const HARDNESS: f32 = 4.5;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for OxidizedCutCopper {
		const DISPLAY_NAME: &'static str = "Oxidized Cut Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WeatheredCutCopper {
		const DISPLAY_NAME: &'static str = "Weathered Cut Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for ExposedCutCopper {
		const DISPLAY_NAME: &'static str = "Exposed Cut Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CutCopper {
		const DISPLAY_NAME: &'static str = "Cut Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for OxidizedCutCopperStairs {
		const DISPLAY_NAME: &'static str = "Oxidized Cut Copper Stairs";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WeatheredCutCopperStairs {
		const DISPLAY_NAME: &'static str = "Weathered Cut Copper Stairs";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for ExposedCutCopperStairs {
		const DISPLAY_NAME: &'static str = "Exposed Cut Copper Stairs";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CutCopperStairs {
		const DISPLAY_NAME: &'static str = "Cut Copper Stairs";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OxidizedCutCopperSlab {
		const DISPLAY_NAME: &'static str = "Oxidized Cut Copper Slab";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WeatheredCutCopperSlab {
		const DISPLAY_NAME: &'static str = "Weathered Cut Copper Slab";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for ExposedCutCopperSlab {
		const DISPLAY_NAME: &'static str = "Exposed Cut Copper Slab";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CutCopperSlab {
		const DISPLAY_NAME: &'static str = "Cut Copper Slab";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WaxedCopperBlock {
		const DISPLAY_NAME: &'static str = "Waxed Block of Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WaxedWeatheredCopper {
		const DISPLAY_NAME: &'static str = "Waxed Weathered Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WaxedExposedCopper {
		const DISPLAY_NAME: &'static str = "Waxed Exposed Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WaxedOxidizedCopper {
		const DISPLAY_NAME: &'static str = "Waxed Oxidized Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WaxedOxidizedCutCopper {
		const DISPLAY_NAME: &'static str = "Waxed Oxidized Cut Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WaxedWeatheredCutCopper {
		const DISPLAY_NAME: &'static str = "Waxed Weathered Cut Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WaxedExposedCutCopper {
		const DISPLAY_NAME: &'static str = "Waxed Exposed Cut Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WaxedCutCopper {
		const DISPLAY_NAME: &'static str = "Waxed Cut Copper";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for WaxedOxidizedCutCopperStairs {
		const DISPLAY_NAME: &'static str = "Waxed Oxidized Cut Copper Stairs";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WaxedWeatheredCutCopperStairs {
		const DISPLAY_NAME: &'static str = "Waxed Weathered Cut Copper Stairs";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WaxedExposedCutCopperStairs {
		const DISPLAY_NAME: &'static str = "Waxed Exposed Cut Copper Stairs";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WaxedCutCopperStairs {
		const DISPLAY_NAME: &'static str = "Waxed Cut Copper Stairs";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WaxedOxidizedCutCopperSlab {
		const DISPLAY_NAME: &'static str = "Waxed Oxidized Cut Copper Slab";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WaxedWeatheredCutCopperSlab {
		const DISPLAY_NAME: &'static str = "Waxed Weathered Cut Copper Slab";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WaxedExposedCutCopperSlab {
		const DISPLAY_NAME: &'static str = "Waxed Exposed Cut Copper Slab";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for WaxedCutCopperSlab {
		const DISPLAY_NAME: &'static str = "Waxed Cut Copper Slab";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for LightningRod {
		const DISPLAY_NAME: &'static str = "Lightning Rod";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PointedDripstone {
		const DISPLAY_NAME: &'static str = "Pointed Dripstone";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 3.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DripstoneBlock {
		const DISPLAY_NAME: &'static str = "Dripstone Block";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 1.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CaveVines {
		const DISPLAY_NAME: &'static str = "Cave Vines";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CaveVinesPlant {
		const DISPLAY_NAME: &'static str = "Cave Vines Plant";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SporeBlossom {
		const DISPLAY_NAME: &'static str = "Spore Blossom";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for Azalea {
		const DISPLAY_NAME: &'static str = "Azalea";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for FloweringAzalea {
		const DISPLAY_NAME: &'static str = "Flowering Azalea";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MossCarpet {
		const DISPLAY_NAME: &'static str = "Moss Carpet";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for MossBlock {
		const DISPLAY_NAME: &'static str = "Moss Block";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for BigDripleaf {
		const DISPLAY_NAME: &'static str = "Big Dripleaf";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for BigDripleafStem {
		const DISPLAY_NAME: &'static str = "Big Dripleaf Stem";
		const HARDNESS: f32 = 0.1;
		const RESISTANCE: f32 = 0.1;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for SmallDripleaf {
		const DISPLAY_NAME: &'static str = "Small Dripleaf";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for HangingRoots {
		const DISPLAY_NAME: &'static str = "Hanging Roots";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for RootedDirt {
		const DISPLAY_NAME: &'static str = "Rooted Dirt";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Mud {
		const DISPLAY_NAME: &'static str = "Mud";
		const HARDNESS: f32 = 0.5;
		const RESISTANCE: f32 = 0.5;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Deepslate {
		const DISPLAY_NAME: &'static str = "Deepslate";
		const HARDNESS: f32 = 3.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CobbledDeepslate {
		const DISPLAY_NAME: &'static str = "Cobbled Deepslate";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CobbledDeepslateStairs {
		const DISPLAY_NAME: &'static str = "Cobbled Deepslate Stairs";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CobbledDeepslateSlab {
		const DISPLAY_NAME: &'static str = "Cobbled Deepslate Slab";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for CobbledDeepslateWall {
		const DISPLAY_NAME: &'static str = "Cobbled Deepslate Wall";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedDeepslate {
		const DISPLAY_NAME: &'static str = "Polished Deepslate";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PolishedDeepslateStairs {
		const DISPLAY_NAME: &'static str = "Polished Deepslate Stairs";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedDeepslateSlab {
		const DISPLAY_NAME: &'static str = "Polished Deepslate Slab";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PolishedDeepslateWall {
		const DISPLAY_NAME: &'static str = "Polished Deepslate Wall";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DeepslateTiles {
		const DISPLAY_NAME: &'static str = "Deepslate Tiles";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeepslateTileStairs {
		const DISPLAY_NAME: &'static str = "Deepslate Tile Stairs";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DeepslateTileSlab {
		const DISPLAY_NAME: &'static str = "Deepslate Tile Slab";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DeepslateTileWall {
		const DISPLAY_NAME: &'static str = "Deepslate Tile Wall";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DeepslateBricks {
		const DISPLAY_NAME: &'static str = "Deepslate Bricks";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for DeepslateBrickStairs {
		const DISPLAY_NAME: &'static str = "Deepslate Brick Stairs";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DeepslateBrickSlab {
		const DISPLAY_NAME: &'static str = "Deepslate Brick Slab";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for DeepslateBrickWall {
		const DISPLAY_NAME: &'static str = "Deepslate Brick Wall";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for ChiseledDeepslate {
		const DISPLAY_NAME: &'static str = "Chiseled Deepslate";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CrackedDeepslateBricks {
		const DISPLAY_NAME: &'static str = "Cracked Deepslate Bricks";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for CrackedDeepslateTiles {
		const DISPLAY_NAME: &'static str = "Cracked Deepslate Tiles";
		const HARDNESS: f32 = 3.5;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for InfestedDeepslate {
		const DISPLAY_NAME: &'static str = "Infested Deepslate";
		const HARDNESS: f32 = 1.5;
		const RESISTANCE: f32 = 0.75;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for SmoothBasalt {
		const DISPLAY_NAME: &'static str = "Smooth Basalt";
		const HARDNESS: f32 = 1.25;
		const RESISTANCE: f32 = 4.2;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for RawIronBlock {
		const DISPLAY_NAME: &'static str = "Block of Raw Iron";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for RawCopperBlock {
		const DISPLAY_NAME: &'static str = "Block of Raw Copper";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for RawGoldBlock {
		const DISPLAY_NAME: &'static str = "Block of Raw Gold";
		const HARDNESS: f32 = 5.0;
		const RESISTANCE: f32 = 6.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PottedAzaleaBush {
		const DISPLAY_NAME: &'static str = "Potted Azalea";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for PottedFloweringAzaleaBush {
		const DISPLAY_NAME: &'static str = "Potted Flowering Azalea";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for OchreFroglight {
		const DISPLAY_NAME: &'static str = "Ochre Froglight";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for VerdantFroglight {
		const DISPLAY_NAME: &'static str = "Verdant Froglight";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for PearlescentFroglight {
		const DISPLAY_NAME: &'static str = "Pearlescent Froglight";
		const HARDNESS: f32 = 0.3;
		const RESISTANCE: f32 = 0.3;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 15;
		const FILTER_LIGHT: u8 = 15;
	}

	impl BlockProperties for Frogspawn {
		const DISPLAY_NAME: &'static str = "Frogspawn";
		const HARDNESS: f32 = 0.0;
		const RESISTANCE: f32 = 0.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = true;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 0;
	}

	impl BlockProperties for ReinforcedDeepslate {
		const DISPLAY_NAME: &'static str = "Reinforced Deepslate";
		const HARDNESS: f32 = 55.0;
		const RESISTANCE: f32 = 1200.0;
		const STACK_SIZE: u8 = 64;
		const DIGGABLE: bool = true;
		const TRANSPARENT: bool = false;
		const EMIT_LIGHT: u8 = 0;
		const FILTER_LIGHT: u8 = 15;
	}
}

#[derive(Debug)]
pub struct Air;


#[derive(Debug)]
pub struct Stone;


#[derive(Debug)]
pub struct Granite;


#[derive(Debug)]
pub struct PolishedGranite;


#[derive(Debug)]
pub struct Diorite;


#[derive(Debug)]
pub struct PolishedDiorite;


#[derive(Debug)]
pub struct Andesite;


#[derive(Debug)]
pub struct PolishedAndesite;


#[derive(Debug)]
pub struct GrassBlock {
	pub r#snowy: bool,
}


#[derive(Debug)]
pub struct Dirt;


#[derive(Debug)]
pub struct CoarseDirt;


#[derive(Debug)]
pub struct Podzol {
	pub r#snowy: bool,
}


#[derive(Debug)]
pub struct Cobblestone;


#[derive(Debug)]
pub struct OakPlanks;


#[derive(Debug)]
pub struct SprucePlanks;


#[derive(Debug)]
pub struct BirchPlanks;


#[derive(Debug)]
pub struct JunglePlanks;


#[derive(Debug)]
pub struct AcaciaPlanks;


#[derive(Debug)]
pub struct DarkOakPlanks;


#[derive(Debug)]
pub struct MangrovePlanks;


#[derive(Debug)]
pub struct OakSapling {
	pub r#stage: ConstrainedInt<0, 1>,
}


#[derive(Debug)]
pub struct SpruceSapling {
	pub r#stage: ConstrainedInt<0, 1>,
}


#[derive(Debug)]
pub struct BirchSapling {
	pub r#stage: ConstrainedInt<0, 1>,
}


#[derive(Debug)]
pub struct JungleSapling {
	pub r#stage: ConstrainedInt<0, 1>,
}


#[derive(Debug)]
pub struct AcaciaSapling {
	pub r#stage: ConstrainedInt<0, 1>,
}


#[derive(Debug)]
pub struct DarkOakSapling {
	pub r#stage: ConstrainedInt<0, 1>,
}


#[derive(Debug)]
pub struct MangrovePropagule {
	pub r#age: ConstrainedInt<0, 4>,
	pub r#hanging: bool,
	pub r#stage: ConstrainedInt<0, 1>,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct Bedrock;


#[derive(Debug)]
pub struct Water {
	pub r#level: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct Lava {
	pub r#level: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct Sand;


#[derive(Debug)]
pub struct RedSand;


#[derive(Debug)]
pub struct Gravel;


#[derive(Debug)]
pub struct GoldOre;


#[derive(Debug)]
pub struct DeepslateGoldOre;


#[derive(Debug)]
pub struct IronOre;


#[derive(Debug)]
pub struct DeepslateIronOre;


#[derive(Debug)]
pub struct CoalOre;


#[derive(Debug)]
pub struct DeepslateCoalOre;


#[derive(Debug)]
pub struct NetherGoldOre;


#[derive(Debug)]
pub struct OakLog {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct SpruceLog {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct BirchLog {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct JungleLog {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct AcaciaLog {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct DarkOakLog {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct MangroveLog {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct MangroveRoots {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MuddyMangroveRoots {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedSpruceLog {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedBirchLog {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedJungleLog {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedAcaciaLog {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedDarkOakLog {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedOakLog {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedMangroveLog {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct OakWood {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct SpruceWood {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct BirchWood {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct JungleWood {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct AcaciaWood {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct DarkOakWood {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct MangroveWood {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedOakWood {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedSpruceWood {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedBirchWood {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedJungleWood {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedAcaciaWood {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedDarkOakWood {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedMangroveWood {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct OakLeaves {
	pub r#distance: ConstrainedInt<1, 7>,
	pub r#persistent: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SpruceLeaves {
	pub r#distance: ConstrainedInt<1, 7>,
	pub r#persistent: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BirchLeaves {
	pub r#distance: ConstrainedInt<1, 7>,
	pub r#persistent: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct JungleLeaves {
	pub r#distance: ConstrainedInt<1, 7>,
	pub r#persistent: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct AcaciaLeaves {
	pub r#distance: ConstrainedInt<1, 7>,
	pub r#persistent: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DarkOakLeaves {
	pub r#distance: ConstrainedInt<1, 7>,
	pub r#persistent: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MangroveLeaves {
	pub r#distance: ConstrainedInt<1, 7>,
	pub r#persistent: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct AzaleaLeaves {
	pub r#distance: ConstrainedInt<1, 7>,
	pub r#persistent: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct FloweringAzaleaLeaves {
	pub r#distance: ConstrainedInt<1, 7>,
	pub r#persistent: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct Sponge;


#[derive(Debug)]
pub struct WetSponge;


#[derive(Debug)]
pub struct Glass;


#[derive(Debug)]
pub struct LapisOre;


#[derive(Debug)]
pub struct DeepslateLapisOre;


#[derive(Debug)]
pub struct LapisBlock;


#[derive(Debug)]
pub struct Dispenser {
	pub r#facing: property_enums::Facing,
	pub r#triggered: bool,
}


#[derive(Debug)]
pub struct Sandstone;


#[derive(Debug)]
pub struct ChiseledSandstone;


#[derive(Debug)]
pub struct CutSandstone;


#[derive(Debug)]
pub struct NoteBlock {
	pub r#instrument: property_enums::Instrument,
	pub r#note: ConstrainedInt<0, 24>,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct WhiteBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct OrangeBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct MagentaBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct LightBlueBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct YellowBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct LimeBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct PinkBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct GrayBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct LightGrayBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct CyanBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct PurpleBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct BlueBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct BrownBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct GreenBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct RedBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct BlackBed {
	pub r#facing: property_enums::Facing1,
	pub r#occupied: bool,
	pub r#part: property_enums::Part,
}


#[derive(Debug)]
pub struct PoweredRail {
	pub r#powered: bool,
	pub r#shape: property_enums::Shape,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DetectorRail {
	pub r#powered: bool,
	pub r#shape: property_enums::Shape,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct StickyPiston {
	pub r#extended: bool,
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct Cobweb;


#[derive(Debug)]
pub struct Grass;


#[derive(Debug)]
pub struct Fern;


#[derive(Debug)]
pub struct DeadBush;


#[derive(Debug)]
pub struct Seagrass;


#[derive(Debug)]
pub struct TallSeagrass {
	pub r#half: property_enums::Half,
}


#[derive(Debug)]
pub struct Piston {
	pub r#extended: bool,
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct PistonHead {
	pub r#facing: property_enums::Facing,
	pub r#short: bool,
	pub r#type: property_enums::Type,
}


#[derive(Debug)]
pub struct WhiteWool;


#[derive(Debug)]
pub struct OrangeWool;


#[derive(Debug)]
pub struct MagentaWool;


#[derive(Debug)]
pub struct LightBlueWool;


#[derive(Debug)]
pub struct YellowWool;


#[derive(Debug)]
pub struct LimeWool;


#[derive(Debug)]
pub struct PinkWool;


#[derive(Debug)]
pub struct GrayWool;


#[derive(Debug)]
pub struct LightGrayWool;


#[derive(Debug)]
pub struct CyanWool;


#[derive(Debug)]
pub struct PurpleWool;


#[derive(Debug)]
pub struct BlueWool;


#[derive(Debug)]
pub struct BrownWool;


#[derive(Debug)]
pub struct GreenWool;


#[derive(Debug)]
pub struct RedWool;


#[derive(Debug)]
pub struct BlackWool;


#[derive(Debug)]
pub struct MovingPiston {
	pub r#facing: property_enums::Facing,
	pub r#type: property_enums::Type,
}


#[derive(Debug)]
pub struct Dandelion;


#[derive(Debug)]
pub struct Poppy;


#[derive(Debug)]
pub struct BlueOrchid;


#[derive(Debug)]
pub struct Allium;


#[derive(Debug)]
pub struct AzureBluet;


#[derive(Debug)]
pub struct RedTulip;


#[derive(Debug)]
pub struct OrangeTulip;


#[derive(Debug)]
pub struct WhiteTulip;


#[derive(Debug)]
pub struct PinkTulip;


#[derive(Debug)]
pub struct OxeyeDaisy;


#[derive(Debug)]
pub struct Cornflower;


#[derive(Debug)]
pub struct WitherRose;


#[derive(Debug)]
pub struct LilyOfTheValley;


#[derive(Debug)]
pub struct BrownMushroom;


#[derive(Debug)]
pub struct RedMushroom;


#[derive(Debug)]
pub struct GoldBlock;


#[derive(Debug)]
pub struct IronBlock;


#[derive(Debug)]
pub struct Bricks;


#[derive(Debug)]
pub struct Tnt {
	pub r#unstable: bool,
}


#[derive(Debug)]
pub struct Bookshelf;


#[derive(Debug)]
pub struct MossyCobblestone;


#[derive(Debug)]
pub struct Obsidian;


#[derive(Debug)]
pub struct Torch;


#[derive(Debug)]
pub struct WallTorch {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct Fire {
	pub r#age: ConstrainedInt<0, 15>,
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#up: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct SoulFire;


#[derive(Debug)]
pub struct Spawner;


#[derive(Debug)]
pub struct OakStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct Chest {
	pub r#facing: property_enums::Facing1,
	pub r#type: property_enums::Type1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct RedstoneWire {
	pub r#east: property_enums::East,
	pub r#north: property_enums::North,
	pub r#power: ConstrainedInt<0, 15>,
	pub r#south: property_enums::South,
	pub r#west: property_enums::West,
}


#[derive(Debug)]
pub struct DiamondOre;


#[derive(Debug)]
pub struct DeepslateDiamondOre;


#[derive(Debug)]
pub struct DiamondBlock;


#[derive(Debug)]
pub struct CraftingTable;


#[derive(Debug)]
pub struct Wheat {
	pub r#age: ConstrainedInt<0, 7>,
}


#[derive(Debug)]
pub struct Farmland {
	pub r#moisture: ConstrainedInt<0, 7>,
}


#[derive(Debug)]
pub struct Furnace {
	pub r#facing: property_enums::Facing1,
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct OakSign {
	pub r#rotation: ConstrainedInt<0, 15>,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SpruceSign {
	pub r#rotation: ConstrainedInt<0, 15>,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BirchSign {
	pub r#rotation: ConstrainedInt<0, 15>,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct AcaciaSign {
	pub r#rotation: ConstrainedInt<0, 15>,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct JungleSign {
	pub r#rotation: ConstrainedInt<0, 15>,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DarkOakSign {
	pub r#rotation: ConstrainedInt<0, 15>,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MangroveSign {
	pub r#rotation: ConstrainedInt<0, 15>,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct OakDoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half,
	pub r#hinge: property_enums::Hinge,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct Ladder {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct Rail {
	pub r#shape: property_enums::Shape2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CobblestoneStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct OakWallSign {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SpruceWallSign {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BirchWallSign {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct AcaciaWallSign {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct JungleWallSign {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DarkOakWallSign {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MangroveWallSign {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct Lever {
	pub r#face: property_enums::Face,
	pub r#facing: property_enums::Facing1,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct StonePressurePlate {
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct IronDoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half,
	pub r#hinge: property_enums::Hinge,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct OakPressurePlate {
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct SprucePressurePlate {
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct BirchPressurePlate {
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct JunglePressurePlate {
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct AcaciaPressurePlate {
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct DarkOakPressurePlate {
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct MangrovePressurePlate {
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct RedstoneOre {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct DeepslateRedstoneOre {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct RedstoneTorch {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct RedstoneWallTorch {
	pub r#facing: property_enums::Facing1,
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct StoneButton {
	pub r#face: property_enums::Face,
	pub r#facing: property_enums::Facing1,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct Snow {
	pub r#layers: ConstrainedInt<1, 8>,
}


#[derive(Debug)]
pub struct Ice;


#[derive(Debug)]
pub struct SnowBlock;


#[derive(Debug)]
pub struct Cactus {
	pub r#age: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct Clay;


#[derive(Debug)]
pub struct SugarCane {
	pub r#age: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct Jukebox {
	pub r#has_record: bool,
}


#[derive(Debug)]
pub struct OakFence {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct Pumpkin;


#[derive(Debug)]
pub struct Netherrack;


#[derive(Debug)]
pub struct SoulSand;


#[derive(Debug)]
pub struct SoulSoil;


#[derive(Debug)]
pub struct Basalt {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct PolishedBasalt {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct SoulTorch;


#[derive(Debug)]
pub struct SoulWallTorch {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct Glowstone;


#[derive(Debug)]
pub struct NetherPortal {
	pub r#axis: property_enums::Axis1,
}


#[derive(Debug)]
pub struct CarvedPumpkin {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct JackOLantern {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct Cake {
	pub r#bites: ConstrainedInt<0, 6>,
}


#[derive(Debug)]
pub struct Repeater {
	pub r#delay: ConstrainedInt<1, 4>,
	pub r#facing: property_enums::Facing1,
	pub r#locked: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct WhiteStainedGlass;


#[derive(Debug)]
pub struct OrangeStainedGlass;


#[derive(Debug)]
pub struct MagentaStainedGlass;


#[derive(Debug)]
pub struct LightBlueStainedGlass;


#[derive(Debug)]
pub struct YellowStainedGlass;


#[derive(Debug)]
pub struct LimeStainedGlass;


#[derive(Debug)]
pub struct PinkStainedGlass;


#[derive(Debug)]
pub struct GrayStainedGlass;


#[derive(Debug)]
pub struct LightGrayStainedGlass;


#[derive(Debug)]
pub struct CyanStainedGlass;


#[derive(Debug)]
pub struct PurpleStainedGlass;


#[derive(Debug)]
pub struct BlueStainedGlass;


#[derive(Debug)]
pub struct BrownStainedGlass;


#[derive(Debug)]
pub struct GreenStainedGlass;


#[derive(Debug)]
pub struct RedStainedGlass;


#[derive(Debug)]
pub struct BlackStainedGlass;


#[derive(Debug)]
pub struct OakTrapdoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#open: bool,
	pub r#powered: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SpruceTrapdoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#open: bool,
	pub r#powered: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BirchTrapdoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#open: bool,
	pub r#powered: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct JungleTrapdoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#open: bool,
	pub r#powered: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct AcaciaTrapdoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#open: bool,
	pub r#powered: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DarkOakTrapdoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#open: bool,
	pub r#powered: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MangroveTrapdoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#open: bool,
	pub r#powered: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct StoneBricks;


#[derive(Debug)]
pub struct MossyStoneBricks;


#[derive(Debug)]
pub struct CrackedStoneBricks;


#[derive(Debug)]
pub struct ChiseledStoneBricks;


#[derive(Debug)]
pub struct PackedMud;


#[derive(Debug)]
pub struct MudBricks;


#[derive(Debug)]
pub struct InfestedStone;


#[derive(Debug)]
pub struct InfestedCobblestone;


#[derive(Debug)]
pub struct InfestedStoneBricks;


#[derive(Debug)]
pub struct InfestedMossyStoneBricks;


#[derive(Debug)]
pub struct InfestedCrackedStoneBricks;


#[derive(Debug)]
pub struct InfestedChiseledStoneBricks;


#[derive(Debug)]
pub struct BrownMushroomBlock {
	pub r#down: bool,
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#up: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct RedMushroomBlock {
	pub r#down: bool,
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#up: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct MushroomStem {
	pub r#down: bool,
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#up: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct IronBars {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct Chain {
	pub r#axis: property_enums::Axis,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct GlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct Melon;


#[derive(Debug)]
pub struct AttachedPumpkinStem {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct AttachedMelonStem {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct PumpkinStem {
	pub r#age: ConstrainedInt<0, 7>,
}


#[derive(Debug)]
pub struct MelonStem {
	pub r#age: ConstrainedInt<0, 7>,
}


#[derive(Debug)]
pub struct Vine {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#up: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct GlowLichen {
	pub r#down: bool,
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct OakFenceGate {
	pub r#facing: property_enums::Facing1,
	pub r#in_wall: bool,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct BrickStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct StoneBrickStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MudBrickStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct Mycelium {
	pub r#snowy: bool,
}


#[derive(Debug)]
pub struct LilyPad;


#[derive(Debug)]
pub struct NetherBricks;


#[derive(Debug)]
pub struct NetherBrickFence {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct NetherBrickStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct NetherWart {
	pub r#age: ConstrainedInt<0, 3>,
}


#[derive(Debug)]
pub struct EnchantingTable;


#[derive(Debug)]
pub struct BrewingStand {
	pub r#has_bottle_0: bool,
	pub r#has_bottle_1: bool,
	pub r#has_bottle_2: bool,
}


#[derive(Debug)]
pub struct Cauldron;


#[derive(Debug)]
pub struct WaterCauldron {
	pub r#level: ConstrainedInt<1, 3>,
}


#[derive(Debug)]
pub struct LavaCauldron;


#[derive(Debug)]
pub struct PowderSnowCauldron {
	pub r#level: ConstrainedInt<1, 3>,
}


#[derive(Debug)]
pub struct EndPortal;


#[derive(Debug)]
pub struct EndPortalFrame {
	pub r#eye: bool,
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct EndStone;


#[derive(Debug)]
pub struct DragonEgg;


#[derive(Debug)]
pub struct RedstoneLamp {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct Cocoa {
	pub r#age: ConstrainedInt<0, 2>,
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct SandstoneStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct EmeraldOre;


#[derive(Debug)]
pub struct DeepslateEmeraldOre;


#[derive(Debug)]
pub struct EnderChest {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct TripwireHook {
	pub r#attached: bool,
	pub r#facing: property_enums::Facing1,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct Tripwire {
	pub r#attached: bool,
	pub r#disarmed: bool,
	pub r#east: bool,
	pub r#north: bool,
	pub r#powered: bool,
	pub r#south: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct EmeraldBlock;


#[derive(Debug)]
pub struct SpruceStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BirchStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct JungleStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CommandBlock {
	pub r#conditional: bool,
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct Beacon;


#[derive(Debug)]
pub struct CobblestoneWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct MossyCobblestoneWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct FlowerPot;


#[derive(Debug)]
pub struct PottedOakSapling;


#[derive(Debug)]
pub struct PottedSpruceSapling;


#[derive(Debug)]
pub struct PottedBirchSapling;


#[derive(Debug)]
pub struct PottedJungleSapling;


#[derive(Debug)]
pub struct PottedAcaciaSapling;


#[derive(Debug)]
pub struct PottedDarkOakSapling;


#[derive(Debug)]
pub struct PottedMangrovePropagule;


#[derive(Debug)]
pub struct PottedFern;


#[derive(Debug)]
pub struct PottedDandelion;


#[derive(Debug)]
pub struct PottedPoppy;


#[derive(Debug)]
pub struct PottedBlueOrchid;


#[derive(Debug)]
pub struct PottedAllium;


#[derive(Debug)]
pub struct PottedAzureBluet;


#[derive(Debug)]
pub struct PottedRedTulip;


#[derive(Debug)]
pub struct PottedOrangeTulip;


#[derive(Debug)]
pub struct PottedWhiteTulip;


#[derive(Debug)]
pub struct PottedPinkTulip;


#[derive(Debug)]
pub struct PottedOxeyeDaisy;


#[derive(Debug)]
pub struct PottedCornflower;


#[derive(Debug)]
pub struct PottedLilyOfTheValley;


#[derive(Debug)]
pub struct PottedWitherRose;


#[derive(Debug)]
pub struct PottedRedMushroom;


#[derive(Debug)]
pub struct PottedBrownMushroom;


#[derive(Debug)]
pub struct PottedDeadBush;


#[derive(Debug)]
pub struct PottedCactus;


#[derive(Debug)]
pub struct Carrots {
	pub r#age: ConstrainedInt<0, 7>,
}


#[derive(Debug)]
pub struct Potatoes {
	pub r#age: ConstrainedInt<0, 7>,
}


#[derive(Debug)]
pub struct OakButton {
	pub r#face: property_enums::Face,
	pub r#facing: property_enums::Facing1,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct SpruceButton {
	pub r#face: property_enums::Face,
	pub r#facing: property_enums::Facing1,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct BirchButton {
	pub r#face: property_enums::Face,
	pub r#facing: property_enums::Facing1,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct JungleButton {
	pub r#face: property_enums::Face,
	pub r#facing: property_enums::Facing1,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct AcaciaButton {
	pub r#face: property_enums::Face,
	pub r#facing: property_enums::Facing1,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct DarkOakButton {
	pub r#face: property_enums::Face,
	pub r#facing: property_enums::Facing1,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct MangroveButton {
	pub r#face: property_enums::Face,
	pub r#facing: property_enums::Facing1,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct SkeletonSkull {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct SkeletonWallSkull {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct WitherSkeletonSkull {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct WitherSkeletonWallSkull {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct ZombieHead {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct ZombieWallHead {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct PlayerHead {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct PlayerWallHead {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct CreeperHead {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct CreeperWallHead {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct DragonHead {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct DragonWallHead {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct Anvil {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct ChippedAnvil {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct DamagedAnvil {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct TrappedChest {
	pub r#facing: property_enums::Facing1,
	pub r#type: property_enums::Type1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct LightWeightedPressurePlate {
	pub r#power: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct HeavyWeightedPressurePlate {
	pub r#power: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct Comparator {
	pub r#facing: property_enums::Facing1,
	pub r#mode: property_enums::Mode,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct DaylightDetector {
	pub r#inverted: bool,
	pub r#power: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct RedstoneBlock;


#[derive(Debug)]
pub struct NetherQuartzOre;


#[derive(Debug)]
pub struct Hopper {
	pub r#enabled: bool,
	pub r#facing: property_enums::Facing2,
}


#[derive(Debug)]
pub struct QuartzBlock;


#[derive(Debug)]
pub struct ChiseledQuartzBlock;


#[derive(Debug)]
pub struct QuartzPillar {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct QuartzStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct ActivatorRail {
	pub r#powered: bool,
	pub r#shape: property_enums::Shape,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct Dropper {
	pub r#facing: property_enums::Facing,
	pub r#triggered: bool,
}


#[derive(Debug)]
pub struct WhiteTerracotta;


#[derive(Debug)]
pub struct OrangeTerracotta;


#[derive(Debug)]
pub struct MagentaTerracotta;


#[derive(Debug)]
pub struct LightBlueTerracotta;


#[derive(Debug)]
pub struct YellowTerracotta;


#[derive(Debug)]
pub struct LimeTerracotta;


#[derive(Debug)]
pub struct PinkTerracotta;


#[derive(Debug)]
pub struct GrayTerracotta;


#[derive(Debug)]
pub struct LightGrayTerracotta;


#[derive(Debug)]
pub struct CyanTerracotta;


#[derive(Debug)]
pub struct PurpleTerracotta;


#[derive(Debug)]
pub struct BlueTerracotta;


#[derive(Debug)]
pub struct BrownTerracotta;


#[derive(Debug)]
pub struct GreenTerracotta;


#[derive(Debug)]
pub struct RedTerracotta;


#[derive(Debug)]
pub struct BlackTerracotta;


#[derive(Debug)]
pub struct WhiteStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct OrangeStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct MagentaStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct LightBlueStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct YellowStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct LimeStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct PinkStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct GrayStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct LightGrayStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct CyanStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct PurpleStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct BlueStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct BrownStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct GreenStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct RedStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct BlackStainedGlassPane {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct AcaciaStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DarkOakStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MangroveStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SlimeBlock;


#[derive(Debug)]
pub struct Barrier;


#[derive(Debug)]
pub struct Light {
	pub r#level: ConstrainedInt<0, 15>,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct IronTrapdoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#open: bool,
	pub r#powered: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct Prismarine;


#[derive(Debug)]
pub struct PrismarineBricks;


#[derive(Debug)]
pub struct DarkPrismarine;


#[derive(Debug)]
pub struct PrismarineStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PrismarineBrickStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DarkPrismarineStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PrismarineSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PrismarineBrickSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DarkPrismarineSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SeaLantern;


#[derive(Debug)]
pub struct HayBlock {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct WhiteCarpet;


#[derive(Debug)]
pub struct OrangeCarpet;


#[derive(Debug)]
pub struct MagentaCarpet;


#[derive(Debug)]
pub struct LightBlueCarpet;


#[derive(Debug)]
pub struct YellowCarpet;


#[derive(Debug)]
pub struct LimeCarpet;


#[derive(Debug)]
pub struct PinkCarpet;


#[derive(Debug)]
pub struct GrayCarpet;


#[derive(Debug)]
pub struct LightGrayCarpet;


#[derive(Debug)]
pub struct CyanCarpet;


#[derive(Debug)]
pub struct PurpleCarpet;


#[derive(Debug)]
pub struct BlueCarpet;


#[derive(Debug)]
pub struct BrownCarpet;


#[derive(Debug)]
pub struct GreenCarpet;


#[derive(Debug)]
pub struct RedCarpet;


#[derive(Debug)]
pub struct BlackCarpet;


#[derive(Debug)]
pub struct Terracotta;


#[derive(Debug)]
pub struct CoalBlock;


#[derive(Debug)]
pub struct PackedIce;


#[derive(Debug)]
pub struct Sunflower {
	pub r#half: property_enums::Half,
}


#[derive(Debug)]
pub struct Lilac {
	pub r#half: property_enums::Half,
}


#[derive(Debug)]
pub struct RoseBush {
	pub r#half: property_enums::Half,
}


#[derive(Debug)]
pub struct Peony {
	pub r#half: property_enums::Half,
}


#[derive(Debug)]
pub struct TallGrass {
	pub r#half: property_enums::Half,
}


#[derive(Debug)]
pub struct LargeFern {
	pub r#half: property_enums::Half,
}


#[derive(Debug)]
pub struct WhiteBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct OrangeBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct MagentaBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct LightBlueBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct YellowBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct LimeBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct PinkBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct GrayBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct LightGrayBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct CyanBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct PurpleBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct BlueBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct BrownBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct GreenBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct RedBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct BlackBanner {
	pub r#rotation: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct WhiteWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct OrangeWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct MagentaWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct LightBlueWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct YellowWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct LimeWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct PinkWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct GrayWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct LightGrayWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct CyanWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct PurpleWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct BlueWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct BrownWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct GreenWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct RedWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct BlackWallBanner {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct RedSandstone;


#[derive(Debug)]
pub struct ChiseledRedSandstone;


#[derive(Debug)]
pub struct CutRedSandstone;


#[derive(Debug)]
pub struct RedSandstoneStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct OakSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SpruceSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BirchSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct JungleSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct AcaciaSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DarkOakSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MangroveSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct StoneSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SmoothStoneSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SandstoneSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CutSandstoneSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PetrifiedOakSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CobblestoneSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BrickSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct StoneBrickSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MudBrickSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct NetherBrickSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct QuartzSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct RedSandstoneSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CutRedSandstoneSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PurpurSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SmoothStone;


#[derive(Debug)]
pub struct SmoothSandstone;


#[derive(Debug)]
pub struct SmoothQuartz;


#[derive(Debug)]
pub struct SmoothRedSandstone;


#[derive(Debug)]
pub struct SpruceFenceGate {
	pub r#facing: property_enums::Facing1,
	pub r#in_wall: bool,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct BirchFenceGate {
	pub r#facing: property_enums::Facing1,
	pub r#in_wall: bool,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct JungleFenceGate {
	pub r#facing: property_enums::Facing1,
	pub r#in_wall: bool,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct AcaciaFenceGate {
	pub r#facing: property_enums::Facing1,
	pub r#in_wall: bool,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct DarkOakFenceGate {
	pub r#facing: property_enums::Facing1,
	pub r#in_wall: bool,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct MangroveFenceGate {
	pub r#facing: property_enums::Facing1,
	pub r#in_wall: bool,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct SpruceFence {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct BirchFence {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct JungleFence {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct AcaciaFence {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct DarkOakFence {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct MangroveFence {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct SpruceDoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half,
	pub r#hinge: property_enums::Hinge,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct BirchDoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half,
	pub r#hinge: property_enums::Hinge,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct JungleDoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half,
	pub r#hinge: property_enums::Hinge,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct AcaciaDoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half,
	pub r#hinge: property_enums::Hinge,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct DarkOakDoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half,
	pub r#hinge: property_enums::Hinge,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct MangroveDoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half,
	pub r#hinge: property_enums::Hinge,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct EndRod {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct ChorusPlant {
	pub r#down: bool,
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#up: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct ChorusFlower {
	pub r#age: ConstrainedInt<0, 5>,
}


#[derive(Debug)]
pub struct PurpurBlock;


#[derive(Debug)]
pub struct PurpurPillar {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct PurpurStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct EndStoneBricks;


#[derive(Debug)]
pub struct Beetroots {
	pub r#age: ConstrainedInt<0, 3>,
}


#[derive(Debug)]
pub struct DirtPath;


#[derive(Debug)]
pub struct EndGateway;


#[derive(Debug)]
pub struct RepeatingCommandBlock {
	pub r#conditional: bool,
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct ChainCommandBlock {
	pub r#conditional: bool,
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct FrostedIce {
	pub r#age: ConstrainedInt<0, 3>,
}


#[derive(Debug)]
pub struct MagmaBlock;


#[derive(Debug)]
pub struct NetherWartBlock;


#[derive(Debug)]
pub struct RedNetherBricks;


#[derive(Debug)]
pub struct BoneBlock {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StructureVoid;


#[derive(Debug)]
pub struct Observer {
	pub r#facing: property_enums::Facing,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct ShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct WhiteShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct OrangeShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct MagentaShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct LightBlueShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct YellowShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct LimeShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct PinkShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct GrayShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct LightGrayShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct CyanShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct PurpleShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct BlueShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct BrownShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct GreenShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct RedShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct BlackShulkerBox {
	pub r#facing: property_enums::Facing,
}


#[derive(Debug)]
pub struct WhiteGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct OrangeGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct MagentaGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct LightBlueGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct YellowGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct LimeGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct PinkGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct GrayGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct LightGrayGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct CyanGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct PurpleGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct BlueGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct BrownGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct GreenGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct RedGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct BlackGlazedTerracotta {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct WhiteConcrete;


#[derive(Debug)]
pub struct OrangeConcrete;


#[derive(Debug)]
pub struct MagentaConcrete;


#[derive(Debug)]
pub struct LightBlueConcrete;


#[derive(Debug)]
pub struct YellowConcrete;


#[derive(Debug)]
pub struct LimeConcrete;


#[derive(Debug)]
pub struct PinkConcrete;


#[derive(Debug)]
pub struct GrayConcrete;


#[derive(Debug)]
pub struct LightGrayConcrete;


#[derive(Debug)]
pub struct CyanConcrete;


#[derive(Debug)]
pub struct PurpleConcrete;


#[derive(Debug)]
pub struct BlueConcrete;


#[derive(Debug)]
pub struct BrownConcrete;


#[derive(Debug)]
pub struct GreenConcrete;


#[derive(Debug)]
pub struct RedConcrete;


#[derive(Debug)]
pub struct BlackConcrete;


#[derive(Debug)]
pub struct WhiteConcretePowder;


#[derive(Debug)]
pub struct OrangeConcretePowder;


#[derive(Debug)]
pub struct MagentaConcretePowder;


#[derive(Debug)]
pub struct LightBlueConcretePowder;


#[derive(Debug)]
pub struct YellowConcretePowder;


#[derive(Debug)]
pub struct LimeConcretePowder;


#[derive(Debug)]
pub struct PinkConcretePowder;


#[derive(Debug)]
pub struct GrayConcretePowder;


#[derive(Debug)]
pub struct LightGrayConcretePowder;


#[derive(Debug)]
pub struct CyanConcretePowder;


#[derive(Debug)]
pub struct PurpleConcretePowder;


#[derive(Debug)]
pub struct BlueConcretePowder;


#[derive(Debug)]
pub struct BrownConcretePowder;


#[derive(Debug)]
pub struct GreenConcretePowder;


#[derive(Debug)]
pub struct RedConcretePowder;


#[derive(Debug)]
pub struct BlackConcretePowder;


#[derive(Debug)]
pub struct Kelp {
	pub r#age: ConstrainedInt<0, 25>,
}


#[derive(Debug)]
pub struct KelpPlant;


#[derive(Debug)]
pub struct DriedKelpBlock;


#[derive(Debug)]
pub struct TurtleEgg {
	pub r#eggs: ConstrainedInt<1, 4>,
	pub r#hatch: ConstrainedInt<0, 2>,
}


#[derive(Debug)]
pub struct DeadTubeCoralBlock;


#[derive(Debug)]
pub struct DeadBrainCoralBlock;


#[derive(Debug)]
pub struct DeadBubbleCoralBlock;


#[derive(Debug)]
pub struct DeadFireCoralBlock;


#[derive(Debug)]
pub struct DeadHornCoralBlock;


#[derive(Debug)]
pub struct TubeCoralBlock;


#[derive(Debug)]
pub struct BrainCoralBlock;


#[derive(Debug)]
pub struct BubbleCoralBlock;


#[derive(Debug)]
pub struct FireCoralBlock;


#[derive(Debug)]
pub struct HornCoralBlock;


#[derive(Debug)]
pub struct DeadTubeCoral {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeadBrainCoral {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeadBubbleCoral {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeadFireCoral {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeadHornCoral {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct TubeCoral {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BrainCoral {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BubbleCoral {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct FireCoral {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct HornCoral {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeadTubeCoralFan {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeadBrainCoralFan {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeadBubbleCoralFan {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeadFireCoralFan {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeadHornCoralFan {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct TubeCoralFan {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BrainCoralFan {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BubbleCoralFan {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct FireCoralFan {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct HornCoralFan {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeadTubeCoralWallFan {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeadBrainCoralWallFan {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeadBubbleCoralWallFan {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeadFireCoralWallFan {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeadHornCoralWallFan {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct TubeCoralWallFan {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BrainCoralWallFan {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BubbleCoralWallFan {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct FireCoralWallFan {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct HornCoralWallFan {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SeaPickle {
	pub r#pickles: ConstrainedInt<1, 4>,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BlueIce;


#[derive(Debug)]
pub struct Conduit {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BambooSapling;


#[derive(Debug)]
pub struct Bamboo {
	pub r#age: ConstrainedInt<0, 1>,
	pub r#leaves: property_enums::Leaves,
	pub r#stage: ConstrainedInt<0, 1>,
}


#[derive(Debug)]
pub struct PottedBamboo;


#[derive(Debug)]
pub struct VoidAir;


#[derive(Debug)]
pub struct CaveAir;


#[derive(Debug)]
pub struct BubbleColumn {
	pub r#drag: bool,
}


#[derive(Debug)]
pub struct PolishedGraniteStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SmoothRedSandstoneStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MossyStoneBrickStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PolishedDioriteStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MossyCobblestoneStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct EndStoneBrickStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct StoneStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SmoothSandstoneStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SmoothQuartzStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct GraniteStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct AndesiteStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct RedNetherBrickStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PolishedAndesiteStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DioriteStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PolishedGraniteSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SmoothRedSandstoneSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MossyStoneBrickSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PolishedDioriteSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MossyCobblestoneSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct EndStoneBrickSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SmoothSandstoneSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SmoothQuartzSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct GraniteSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct AndesiteSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct RedNetherBrickSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PolishedAndesiteSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DioriteSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BrickWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct PrismarineWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct RedSandstoneWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct MossyStoneBrickWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct GraniteWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct StoneBrickWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct MudBrickWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct NetherBrickWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct AndesiteWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct RedNetherBrickWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct SandstoneWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct EndStoneBrickWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct DioriteWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct Scaffolding {
	pub r#bottom: bool,
	pub r#distance: ConstrainedInt<0, 7>,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct Loom {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct Barrel {
	pub r#facing: property_enums::Facing,
	pub r#open: bool,
}


#[derive(Debug)]
pub struct Smoker {
	pub r#facing: property_enums::Facing1,
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct BlastFurnace {
	pub r#facing: property_enums::Facing1,
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct CartographyTable;


#[derive(Debug)]
pub struct FletchingTable;


#[derive(Debug)]
pub struct Grindstone {
	pub r#face: property_enums::Face,
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct Lectern {
	pub r#facing: property_enums::Facing1,
	pub r#has_book: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct SmithingTable;


#[derive(Debug)]
pub struct Stonecutter {
	pub r#facing: property_enums::Facing1,
}


#[derive(Debug)]
pub struct Bell {
	pub r#attachment: property_enums::Attachment,
	pub r#facing: property_enums::Facing1,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct Lantern {
	pub r#hanging: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SoulLantern {
	pub r#hanging: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct Campfire {
	pub r#facing: property_enums::Facing1,
	pub r#lit: bool,
	pub r#signal_fire: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SoulCampfire {
	pub r#facing: property_enums::Facing1,
	pub r#lit: bool,
	pub r#signal_fire: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SweetBerryBush {
	pub r#age: ConstrainedInt<0, 3>,
}


#[derive(Debug)]
pub struct WarpedStem {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedWarpedStem {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct WarpedHyphae {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedWarpedHyphae {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct WarpedNylium;


#[derive(Debug)]
pub struct WarpedFungus;


#[derive(Debug)]
pub struct WarpedWartBlock;


#[derive(Debug)]
pub struct WarpedRoots;


#[derive(Debug)]
pub struct NetherSprouts;


#[derive(Debug)]
pub struct CrimsonStem {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedCrimsonStem {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct CrimsonHyphae {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct StrippedCrimsonHyphae {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct CrimsonNylium;


#[derive(Debug)]
pub struct CrimsonFungus;


#[derive(Debug)]
pub struct Shroomlight;


#[derive(Debug)]
pub struct WeepingVines {
	pub r#age: ConstrainedInt<0, 25>,
}


#[derive(Debug)]
pub struct WeepingVinesPlant;


#[derive(Debug)]
pub struct TwistingVines {
	pub r#age: ConstrainedInt<0, 25>,
}


#[derive(Debug)]
pub struct TwistingVinesPlant;


#[derive(Debug)]
pub struct CrimsonRoots;


#[derive(Debug)]
pub struct CrimsonPlanks;


#[derive(Debug)]
pub struct WarpedPlanks;


#[derive(Debug)]
pub struct CrimsonSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WarpedSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CrimsonPressurePlate {
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct WarpedPressurePlate {
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct CrimsonFence {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct WarpedFence {
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct CrimsonTrapdoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#open: bool,
	pub r#powered: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WarpedTrapdoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#open: bool,
	pub r#powered: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CrimsonFenceGate {
	pub r#facing: property_enums::Facing1,
	pub r#in_wall: bool,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct WarpedFenceGate {
	pub r#facing: property_enums::Facing1,
	pub r#in_wall: bool,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct CrimsonStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WarpedStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CrimsonButton {
	pub r#face: property_enums::Face,
	pub r#facing: property_enums::Facing1,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct WarpedButton {
	pub r#face: property_enums::Face,
	pub r#facing: property_enums::Facing1,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct CrimsonDoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half,
	pub r#hinge: property_enums::Hinge,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct WarpedDoor {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half,
	pub r#hinge: property_enums::Hinge,
	pub r#open: bool,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct CrimsonSign {
	pub r#rotation: ConstrainedInt<0, 15>,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WarpedSign {
	pub r#rotation: ConstrainedInt<0, 15>,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CrimsonWallSign {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WarpedWallSign {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct StructureBlock {
	pub r#mode: property_enums::Mode1,
}


#[derive(Debug)]
pub struct Jigsaw {
	pub r#orientation: property_enums::Orientation,
}


#[derive(Debug)]
pub struct Composter {
	pub r#level: ConstrainedInt<0, 8>,
}


#[derive(Debug)]
pub struct Target {
	pub r#power: ConstrainedInt<0, 15>,
}


#[derive(Debug)]
pub struct BeeNest {
	pub r#facing: property_enums::Facing1,
	pub r#honey_level: ConstrainedInt<0, 5>,
}


#[derive(Debug)]
pub struct Beehive {
	pub r#facing: property_enums::Facing1,
	pub r#honey_level: ConstrainedInt<0, 5>,
}


#[derive(Debug)]
pub struct HoneyBlock;


#[derive(Debug)]
pub struct HoneycombBlock;


#[derive(Debug)]
pub struct NetheriteBlock;


#[derive(Debug)]
pub struct AncientDebris;


#[derive(Debug)]
pub struct CryingObsidian;


#[derive(Debug)]
pub struct RespawnAnchor {
	pub r#charges: ConstrainedInt<0, 4>,
}


#[derive(Debug)]
pub struct PottedCrimsonFungus;


#[derive(Debug)]
pub struct PottedWarpedFungus;


#[derive(Debug)]
pub struct PottedCrimsonRoots;


#[derive(Debug)]
pub struct PottedWarpedRoots;


#[derive(Debug)]
pub struct Lodestone;


#[derive(Debug)]
pub struct Blackstone;


#[derive(Debug)]
pub struct BlackstoneStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BlackstoneWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct BlackstoneSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PolishedBlackstone;


#[derive(Debug)]
pub struct PolishedBlackstoneBricks;


#[derive(Debug)]
pub struct CrackedPolishedBlackstoneBricks;


#[derive(Debug)]
pub struct ChiseledPolishedBlackstone;


#[derive(Debug)]
pub struct PolishedBlackstoneBrickSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PolishedBlackstoneBrickStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PolishedBlackstoneBrickWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct GildedBlackstone;


#[derive(Debug)]
pub struct PolishedBlackstoneStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PolishedBlackstoneSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PolishedBlackstonePressurePlate {
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct PolishedBlackstoneButton {
	pub r#face: property_enums::Face,
	pub r#facing: property_enums::Facing1,
	pub r#powered: bool,
}


#[derive(Debug)]
pub struct PolishedBlackstoneWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct ChiseledNetherBricks;


#[derive(Debug)]
pub struct CrackedNetherBricks;


#[derive(Debug)]
pub struct QuartzBricks;


#[derive(Debug)]
pub struct Candle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WhiteCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct OrangeCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MagentaCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct LightBlueCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct YellowCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct LimeCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PinkCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct GrayCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct LightGrayCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CyanCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PurpleCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BlueCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BrownCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct GreenCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct RedCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BlackCandle {
	pub r#candles: ConstrainedInt<1, 4>,
	pub r#lit: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct WhiteCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct OrangeCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct MagentaCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct LightBlueCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct YellowCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct LimeCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct PinkCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct GrayCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct LightGrayCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct CyanCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct PurpleCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct BlueCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct BrownCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct GreenCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct RedCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct BlackCandleCake {
	pub r#lit: bool,
}


#[derive(Debug)]
pub struct AmethystBlock;


#[derive(Debug)]
pub struct BuddingAmethyst;


#[derive(Debug)]
pub struct AmethystCluster {
	pub r#facing: property_enums::Facing,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct LargeAmethystBud {
	pub r#facing: property_enums::Facing,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct MediumAmethystBud {
	pub r#facing: property_enums::Facing,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SmallAmethystBud {
	pub r#facing: property_enums::Facing,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct Tuff;


#[derive(Debug)]
pub struct Calcite;


#[derive(Debug)]
pub struct TintedGlass;


#[derive(Debug)]
pub struct PowderSnow;


#[derive(Debug)]
pub struct SculkSensor {
	pub r#power: ConstrainedInt<0, 15>,
	pub r#sculk_sensor_phase: property_enums::SculkSensorPhase,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct Sculk;


#[derive(Debug)]
pub struct SculkVein {
	pub r#down: bool,
	pub r#east: bool,
	pub r#north: bool,
	pub r#south: bool,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: bool,
}


#[derive(Debug)]
pub struct SculkCatalyst {
	pub r#bloom: bool,
}


#[derive(Debug)]
pub struct SculkShrieker {
	pub r#can_summon: bool,
	pub r#shrieking: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct OxidizedCopper;


#[derive(Debug)]
pub struct WeatheredCopper;


#[derive(Debug)]
pub struct ExposedCopper;


#[derive(Debug)]
pub struct CopperBlock;


#[derive(Debug)]
pub struct CopperOre;


#[derive(Debug)]
pub struct DeepslateCopperOre;


#[derive(Debug)]
pub struct OxidizedCutCopper;


#[derive(Debug)]
pub struct WeatheredCutCopper;


#[derive(Debug)]
pub struct ExposedCutCopper;


#[derive(Debug)]
pub struct CutCopper;


#[derive(Debug)]
pub struct OxidizedCutCopperStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WeatheredCutCopperStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct ExposedCutCopperStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CutCopperStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct OxidizedCutCopperSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WeatheredCutCopperSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct ExposedCutCopperSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CutCopperSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WaxedCopperBlock;


#[derive(Debug)]
pub struct WaxedWeatheredCopper;


#[derive(Debug)]
pub struct WaxedExposedCopper;


#[derive(Debug)]
pub struct WaxedOxidizedCopper;


#[derive(Debug)]
pub struct WaxedOxidizedCutCopper;


#[derive(Debug)]
pub struct WaxedWeatheredCutCopper;


#[derive(Debug)]
pub struct WaxedExposedCutCopper;


#[derive(Debug)]
pub struct WaxedCutCopper;


#[derive(Debug)]
pub struct WaxedOxidizedCutCopperStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WaxedWeatheredCutCopperStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WaxedExposedCutCopperStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WaxedCutCopperStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WaxedOxidizedCutCopperSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WaxedWeatheredCutCopperSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WaxedExposedCutCopperSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct WaxedCutCopperSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct LightningRod {
	pub r#facing: property_enums::Facing,
	pub r#powered: bool,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PointedDripstone {
	pub r#thickness: property_enums::Thickness,
	pub r#vertical_direction: property_enums::VerticalDirection,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DripstoneBlock;


#[derive(Debug)]
pub struct CaveVines {
	pub r#age: ConstrainedInt<0, 25>,
	pub r#berries: bool,
}


#[derive(Debug)]
pub struct CaveVinesPlant {
	pub r#berries: bool,
}


#[derive(Debug)]
pub struct SporeBlossom;


#[derive(Debug)]
pub struct Azalea;


#[derive(Debug)]
pub struct FloweringAzalea;


#[derive(Debug)]
pub struct MossCarpet;


#[derive(Debug)]
pub struct MossBlock;


#[derive(Debug)]
pub struct BigDripleaf {
	pub r#facing: property_enums::Facing1,
	pub r#tilt: property_enums::Tilt,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct BigDripleafStem {
	pub r#facing: property_enums::Facing1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct SmallDripleaf {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct HangingRoots {
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct RootedDirt;


#[derive(Debug)]
pub struct Mud;


#[derive(Debug)]
pub struct Deepslate {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct CobbledDeepslate;


#[derive(Debug)]
pub struct CobbledDeepslateStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CobbledDeepslateSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct CobbledDeepslateWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct PolishedDeepslate;


#[derive(Debug)]
pub struct PolishedDeepslateStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PolishedDeepslateSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct PolishedDeepslateWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct DeepslateTiles;


#[derive(Debug)]
pub struct DeepslateTileStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeepslateTileSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeepslateTileWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct DeepslateBricks;


#[derive(Debug)]
pub struct DeepslateBrickStairs {
	pub r#facing: property_enums::Facing1,
	pub r#half: property_enums::Half1,
	pub r#shape: property_enums::Shape1,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeepslateBrickSlab {
	pub r#type: property_enums::Type2,
	pub r#waterlogged: bool,
}


#[derive(Debug)]
pub struct DeepslateBrickWall {
	pub r#east: property_enums::East1,
	pub r#north: property_enums::North1,
	pub r#south: property_enums::South1,
	pub r#up: bool,
	pub r#waterlogged: bool,
	pub r#west: property_enums::West1,
}


#[derive(Debug)]
pub struct ChiseledDeepslate;


#[derive(Debug)]
pub struct CrackedDeepslateBricks;


#[derive(Debug)]
pub struct CrackedDeepslateTiles;


#[derive(Debug)]
pub struct InfestedDeepslate {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct SmoothBasalt;


#[derive(Debug)]
pub struct RawIronBlock;


#[derive(Debug)]
pub struct RawCopperBlock;


#[derive(Debug)]
pub struct RawGoldBlock;


#[derive(Debug)]
pub struct PottedAzaleaBush;


#[derive(Debug)]
pub struct PottedFloweringAzaleaBush;


#[derive(Debug)]
pub struct OchreFroglight {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct VerdantFroglight {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct PearlescentFroglight {
	pub r#axis: property_enums::Axis,
}


#[derive(Debug)]
pub struct Frogspawn;


#[derive(Debug)]
pub struct ReinforcedDeepslate;

