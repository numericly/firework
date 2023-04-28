use crate::{
    entities::EntityDataFlags,
    gui::{GUIEvent, GUIInit, GuiScreen, WindowType},
    PlayerHandler,
};
use crate::{
    entities::{EntityMetadata, END_INDEX},
    {ClientData, ConnectionError, Rotation, Server, ServerHandler, ServerProxy, Vec3},
};
use firework_authentication::Profile;
use firework_data::tags::{REGISTRY, TAGS};
use firework_protocol::{
    client_bound::{
        BossBar, ChangeDifficulty, ClientBoundKeepAlive, ClientBoundPacket, CloseContainer,
        CommandSuggestionsResponse, Commands, CustomSound, EntityAnimation, EntityEvent,
        HurtAnimation, IdMapHolder, LoginPlay, OpenScreen, ParticlePacket, PlayDisconnect,
        PlayerAbilities, PlayerInfo, PluginMessage, RemoveEntities, RemoveInfoPlayer, Respawn,
        SerializePacket, SetCenterChunk, SetContainerContent, SetContainerSlot, SetDefaultSpawn,
        SetEntityMetadata, SetEntityVelocity, SetHealth, SetHeldItem, SetRecipes, SetSubtitleText,
        SetTags, SetTitleAnimationTimes, SetTitleText, SoundEffect, SoundSource, SpawnPlayer,
        SynchronizePlayerPosition, SystemChatMessage, TeleportEntity, UnloadChunk,
        UpdateAttributes, UpdateEntityHeadRotation, UpdateEntityPosition,
        UpdateEntityPositionAndRotation, UpdateEntityRotation, VanillaSound,
    },
    core::{DeserializeField, Position, SerializeField, UnsizedVec, VarInt},
    data_types::{
        AddPlayer, Attribute, BossBarAction, EntityAnimationType, EntityEventStatus, Hand,
        Particle, PlayerAbilityFlags, PlayerActionStatus, PlayerCommandAction, PlayerInfoAction,
        PlayerPositionFlags, Slot, UpdateGameMode, UpdateLatency, UpdateListed,
    },
    read_specific_packet,
    server_bound::{
        ChatCommand, ChatMessage, PlayerCommand, ServerBoundKeepAlive, ServerBoundPacket,
    },
    ConnectionState, Protocol, ProtocolError,
};
use rand::Rng;
use serde_json::json;
use std::collections::HashMap;
use std::{
    fmt::Debug,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{broadcast, Mutex, RwLock};

const SERVER_RENDER_DISTANCE: i32 = 12;

#[derive(Debug, Clone)]
pub enum ClientCommand<TransferData>
where
    TransferData: Clone + Send + Sync + 'static,
{
    Transfer {
        data: TransferData,
    },
    SyncPosition {
        position: Vec3,
        rotation: Option<Rotation>,
    },
    Disconnect {
        reason: String,
    },
    MoveEntity {
        entity_id: i32,
        position: Option<Vec3>,
        previous_position: Vec3,
        rotation: Option<Rotation>,
        previous_rotation: Rotation,
        on_ground: bool,
    },
    ChatMessage {
        message: String,
    },
    UpdateEntityMetadata {
        entity_id: i32,
        metadata: Vec<EntityMetadata>,
    },
    RemovePlayer {
        entity_id: i32,
        uuid: u128,
    },
    AddPlayer {
        client_data: Arc<ClientData>,
        gamemode: u8,
        position: Vec3,
        rotation: Rotation,
    },
    KillPlayer {
        entity_id: i32,
        uuid: u128,
    },
    SetHealth {
        health: f32,
    },
    SetVelocity {
        velocity: Vec3,
    },
    DisplayParticles {
        particles: Vec<Particle>,
    },
    InitGui {
        title: String,
        window_type: WindowType,
        items: Vec<Slot>,
    },
    UpdateSlot {
        window_id: i8,
        slot: u16,
        item: Slot,
    },
    CloseGui,
    SendTitle {
        title: String,
        subtitle: String,
        fade_in: i32,
        fade_out: i32,
        stay: i32,
    },
    SendSystemChatMessage {
        message: String, // TODO: chat
        action_bar: bool,
    },
    SendSound {
        sound: IdMapHolder<CustomSound, VanillaSound>,
        source: SoundSource,
        position: Vec3,
        volume: f32,
        pitch: f32,
    },
    BossBarAction {
        uuid: u128,
        action: BossBarAction,
    },
    SetFlying {
        flying: bool,
    },
    UpdateAttributes {
        attributes: Vec<Attribute>,
    },
    SendHurtAnimation {
        entity_id: i32,
        yaw: f32,
    },
    SendEntityAnimation {
        entity_id: i32,
        animation: EntityAnimationType,
    },
    SendEntityEvent {
        entity_id: i32,
        status: EntityEventStatus,
    },
}

#[derive(Debug, Clone, Default)]
#[repr(u8)]
pub enum GameMode {
    #[default]
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

#[derive(Debug, Clone)]
pub struct PreviousPosition {
    pub position: Vec3,
    pub time: Instant,
}

#[derive(Debug, Default)]
pub struct Player {
    pub uuid: u128,
    pub profile: Profile,
    pub gamemode: GameMode,
    // Previous gamemode can be -1 if there is no previous gamemode
    pub previous_gamemode: i8,
    pub reduced_debug_info: bool,
    pub selected_slot: u8,

    pub position: Vec3,
    pub previous_position: Option<PreviousPosition>,
    pub velocity: Vec3,

    pub on_ground: bool,
    pub rotation: Rotation,
    pub sneaking: bool,
    pub sprinting: bool,
    pub first_sprinting_hit: bool,
    pub fall_distance: f64,

    pub elytra_flying: bool,

    pub flying: bool,
    pub flying_allowed: bool,
    pub inventory: Inventory,

    pub health: f32,
    pub max_health: f32,
    pub invulnerable_time: u32,  // ticks the player is invulnerable for
    pub last_damage_amount: f32, // see https://minecraft.fandom.com/wiki/Damage#Immunity
    pub attack_strength_ticker: u32, // ticks since last swing or block break or slot swap or whatever
}

impl Player {
    pub fn entity_flags(&self) -> EntityDataFlags {
        EntityDataFlags::new()
            .with_is_elytra_flying(self.elytra_flying)
            .with_is_crouching(self.sneaking)
            .with_is_sprinting(self.sprinting)
    }
}

#[derive(Debug)]
pub struct Inventory {
    pub held_item: Slot,
    slots: [Slot; 46],
}

impl Default for Inventory {
    fn default() -> Self {
        const EMPTY_SLOT: Slot = None;
        Inventory {
            held_item: EMPTY_SLOT,
            slots: [EMPTY_SLOT; 46],
        }
    }
}

trait SlotValue {
    fn value(&self) -> usize;
}

#[derive(Clone, Debug)]
#[repr(usize)]
pub enum InventorySlot {
    Boots,
    Leggings,
    Chestplate,
    Helmet,
    Hotbar { slot: usize },
    MainInventory { slot: usize },
    CraftingGrid { slot: usize },
    CraftingResult,
    Offhand,
}

impl InventorySlot {
    const CRAFTING_OFFSET: usize = 0;
    const ARMOR_OFFSET: usize = 5;
    const HOTBAR_OFFSET: usize = 36;
    const OFFHAND_OFFSET: usize = 45;
    const MAIN_INVENTORY_OFFSET: usize = 9;
    pub fn value(&self) -> usize {
        match self {
            InventorySlot::Hotbar { slot } => Self::HOTBAR_OFFSET + slot,
            InventorySlot::MainInventory { slot } => Self::HOTBAR_OFFSET + 9 + slot,
            InventorySlot::CraftingGrid { slot } => Self::CRAFTING_OFFSET + 1 + slot,
            InventorySlot::CraftingResult => Self::CRAFTING_OFFSET,
            InventorySlot::Offhand => Self::OFFHAND_OFFSET,
            InventorySlot::Boots => Self::ARMOR_OFFSET + 3,
            InventorySlot::Leggings => Self::ARMOR_OFFSET + 2,
            InventorySlot::Chestplate => Self::ARMOR_OFFSET + 1,
            InventorySlot::Helmet => Self::ARMOR_OFFSET,
        }
    }
    pub fn from_value(value: usize) -> Option<Self> {
        match value {
            v if v >= Self::HOTBAR_OFFSET && v < Self::HOTBAR_OFFSET + 9 => {
                Some(InventorySlot::Hotbar {
                    slot: v - Self::HOTBAR_OFFSET,
                })
            }
            v if v >= Self::MAIN_INVENTORY_OFFSET && v < Self::MAIN_INVENTORY_OFFSET + 27 => {
                Some(InventorySlot::MainInventory {
                    slot: v - Self::MAIN_INVENTORY_OFFSET,
                })
            }
            v if v >= Self::CRAFTING_OFFSET + 1 && v < Self::CRAFTING_OFFSET + 5 => {
                Some(InventorySlot::CraftingGrid {
                    slot: v - Self::CRAFTING_OFFSET - 1,
                })
            }
            v if v == Self::CRAFTING_OFFSET => Some(InventorySlot::CraftingResult),
            v if v == Self::OFFHAND_OFFSET => Some(InventorySlot::Offhand),
            v if v >= Self::ARMOR_OFFSET && v < Self::ARMOR_OFFSET + 4 => {
                match v - Self::ARMOR_OFFSET {
                    0 => Some(InventorySlot::Helmet),
                    1 => Some(InventorySlot::Chestplate),
                    2 => Some(InventorySlot::Leggings),
                    3 => Some(InventorySlot::Boots),
                    _ => unreachable!(),
                }
            }
            _ => None,
        }
    }
}

impl Inventory {
    const CRAFTING_OFFSET: usize = 0;
    const ARMOR_OFFSET: usize = 5;
    const HOTBAR_OFFSET: usize = 36;
    const OFFHAND_OFFSET: usize = 45;
    const MAIN_INVENTORY_OFFSET: usize = 9;
    pub fn get_slot(&self, slot: &InventorySlot) -> &Slot {
        match slot {
            InventorySlot::Helmet => &self.slots[Self::ARMOR_OFFSET],
            InventorySlot::Chestplate => &self.slots[Self::ARMOR_OFFSET + 1],
            InventorySlot::Leggings => &self.slots[Self::ARMOR_OFFSET + 2],
            InventorySlot::Boots => &self.slots[Self::ARMOR_OFFSET + 3],
            InventorySlot::CraftingResult => &self.slots[Self::CRAFTING_OFFSET],
            InventorySlot::Offhand => &self.slots[Self::OFFHAND_OFFSET],
            InventorySlot::CraftingGrid { slot } => {
                assert!(*slot < 4);
                &self.slots[Self::CRAFTING_OFFSET + slot + 1]
            }
            InventorySlot::Hotbar { slot } => {
                assert!(*slot < 9);
                &self.slots[Self::HOTBAR_OFFSET + slot]
            }
            InventorySlot::MainInventory { slot } => {
                assert!(*slot < 27);
                &self.slots[Self::MAIN_INVENTORY_OFFSET + slot]
            }
        }
    }
    pub fn get_hotbar_slot_from_container(&self, slot: usize) -> &Slot {
        const HOTBAR_OFFSET: usize = 36;
        assert!(slot < 9);
        &self.slots[slot + HOTBAR_OFFSET]
    }
    pub fn get_main_slot_from_container(&self, slot: usize) -> &Slot {
        const MAIN_OFFSET: usize = 0;
        assert!(slot < 36);
        &self.slots[slot + MAIN_OFFSET]
    }
    pub fn set_slot(&mut self, slot: InventorySlot, item: Slot) {
        match slot {
            InventorySlot::Helmet => self.slots[Self::ARMOR_OFFSET] = item,
            InventorySlot::Chestplate => self.slots[Self::ARMOR_OFFSET + 1] = item,
            InventorySlot::Leggings => self.slots[Self::ARMOR_OFFSET + 2] = item,
            InventorySlot::Boots => self.slots[Self::ARMOR_OFFSET + 3] = item,
            InventorySlot::CraftingResult => self.slots[Self::CRAFTING_OFFSET] = item,
            InventorySlot::Offhand => self.slots[Self::OFFHAND_OFFSET] = item,
            InventorySlot::CraftingGrid { slot } => {
                assert!(slot < 4);
                self.slots[Self::CRAFTING_OFFSET + slot + 1] = item;
            }
            InventorySlot::Hotbar { slot } => {
                assert!(slot < 9);
                self.slots[slot + Self::HOTBAR_OFFSET] = item
            }
            InventorySlot::MainInventory { slot } => {
                assert!(slot < 27);
                self.slots[slot + Self::MAIN_INVENTORY_OFFSET] = item
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ActivePing {
    response_time: Option<Duration>,
    start: Instant,
    id: u64,
}

pub struct ClientGUI<Handler, Proxy>
where
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
    Proxy: ServerProxy + Send + Sync + 'static,
{
    pub gui: Arc<Handler::ServerGUI>,
    pub events: broadcast::Receiver<GUIEvent>,
}

// #[derive(Debug)]
pub struct Client<Handler, Proxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    pub client_data: Arc<ClientData>,
    pub player: RwLock<Player>,
    pub gui: RwLock<Option<ClientGUI<Handler, Proxy>>>,
    unsynced_entities: RwLock<HashMap<i32, Vec3>>,
    to_client: broadcast::Sender<ClientCommand<Proxy::TransferData>>,
    to_client_visual: broadcast::Sender<ClientCommand<Proxy::TransferData>>,
    connection: Arc<Protocol>,
    active_pings: Mutex<Vec<ActivePing>>,
    pub ping: Mutex<Duration>,
    pub server: Arc<Server<Handler, Proxy>>,
    pub proxy: Arc<Proxy>,
    pub handler: Handler::PlayerHandler,
}

impl<Handler: ServerHandler<Proxy>, Proxy> Client<Handler, Proxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: Send + Sync + 'static,
{
    pub(super) fn new(
        server: Arc<Server<Handler, Proxy>>,
        proxy: Arc<Proxy>,
        connection: Arc<Protocol>,
        client_data: Arc<ClientData>,
        player: Player,
        to_client: broadcast::Sender<ClientCommand<Proxy::TransferData>>,
        to_client_visual: broadcast::Sender<ClientCommand<Proxy::TransferData>>,
    ) -> Client<Handler, Proxy> {
        Client {
            ping: Mutex::new(Duration::from_secs(0)),
            connection,
            client_data,
            to_client,
            unsynced_entities: RwLock::new(HashMap::new()),
            to_client_visual,
            gui: RwLock::new(None),
            player: RwLock::new(player),
            active_pings: Mutex::new(Vec::new()),
            server: server.clone(),
            proxy: proxy.clone(),
            handler: Handler::PlayerHandler::new(server.clone(), proxy.clone()),
        }
    }
    pub(super) async fn connection_state(&self) -> ConnectionState {
        *self.connection.connection_state.read().await
    }
    pub(super) async fn read_packet(&self) -> Result<ServerBoundPacket, ConnectionError> {
        Ok(self.connection.read_and_deserialize().await?)
    }
    pub(super) async fn transfer_world(&self) -> Result<(), ConnectionError> {
        // This packet is send to the client to tell it to show the loading world screen

        self.send_packet(Respawn {
            dimension_type: "minecraft:the_end".to_string(),
            dimension_name: "minecraft:the_end".to_string(),
            hashed_seed: 0,
            gamemode: 0,
            previous_gamemode: -1,
            is_debug: false,
            is_flat: false,
            copy_metadata: true,
            death_location: None,
        })
        .await?;

        self.send_packet({
            let player = self.player.read().await;
            Respawn {
                dimension_type: "minecraft:overworld".to_string(),
                dimension_name: "minecraft:overworld".to_string(),
                hashed_seed: 0,
                gamemode: player.gamemode.clone() as u8,
                previous_gamemode: player.previous_gamemode,
                is_debug: false,
                is_flat: self.server.get_world().flat,
                copy_metadata: true,
                death_location: None,
            }
        })
        .await?;

        for (x, z) in self.client_data.loaded_chunks.lock().await.drain() {
            self.unload_chunk(x, z).await?;
        }
        Ok(())
    }
    pub(super) async fn change_to_play(&self) -> Result<(), ConnectionError> {
        self.send_packet({
            let player = self.player.read().await;
            LoginPlay {
                entity_id: self.client_data.entity_id,
                is_hardcore: false,
                game_mode: player.gamemode.clone() as u8,
                previous_game_mode: player.previous_gamemode,
                dimensions: vec![
                    "minecraft:overworld".to_string(),
                    "minecraft:the_nether".to_string(),
                    "minecraft:the_end".to_string(),
                ],
                registry_codec: UnsizedVec(REGISTRY.clone()),
                dimension_type: "minecraft:overworld".to_string(),
                dimension_name: "minecraft:overworld".to_string(),
                hashed_seed: 0,
                max_players: VarInt(10),
                view_distance: VarInt(SERVER_RENDER_DISTANCE),
                simulation_distance: VarInt(5),
                reduced_debug_info: player.reduced_debug_info,
                enable_respawn_screen: true,
                is_debug: false,
                is_flat: false,
                death_location: None,
            }
        })
        .await?;

        *self.connection.connection_state.write().await = ConnectionState::Play;

        self.send_packet(SetTags { tags: &TAGS }).await?;

        let client_settings =
            read_specific_packet!(self.connection.as_ref(), ClientInformation).await?;

        *self.client_data.settings.write().await = Some(client_settings);

        let client_brand_packet =
            read_specific_packet!(self.connection.as_ref(), PluginMessageServerBound).await?;

        let client_brand = String::deserialize(&mut client_brand_packet.data.0.as_slice())
            .map_err(|deserialize_error| ProtocolError::from(deserialize_error))?;

        *self.client_data.brand.write().await = Some(client_brand);
        Ok(())
    }
    pub(super) async fn load_world(&self) -> Result<(), ConnectionError> {
        self.send_packet({
            let mut buf = Vec::new();
            self.server.brand.to_string().serialize(&mut buf);
            PluginMessage {
                channel: "minecraft:brand".to_string(),
                data: UnsizedVec(buf),
            }
        })
        .await?;

        self.send_packet(ChangeDifficulty {
            difficulty: *self.server.difficulty.read().await,
            locked: *self.server.difficulty_locked.read().await,
        })
        .await?;

        self.send_packet({
            let player = self.player.read().await;
            PlayerAbilities {
                flags: PlayerAbilityFlags::new()
                    .with_flying(player.flying)
                    .with_allow_flying(player.flying_allowed),
                flying_speed: 0.05,
                walking_speed: 0.1,
            }
        })
        .await?;

        self.send_packet(UpdateAttributes {
            entity_id: VarInt::from(self.client_data.entity_id),
            attributes: vec![Attribute::MaxHealth {
                value: self.player.read().await.max_health as f64,
            }],
        })
        .await?;

        self.send_packet(SetHealth {
            health: self.player.read().await.health as f32,
            food: VarInt(20),
            food_saturation: 5.0,
        })
        .await?;

        self.send_packet(SetHeldItem {
            slot: self.player.read().await.selected_slot,
        })
        .await?;

        self.send_packet(SetRecipes {
            recipes: Vec::new(),
        })
        .await?;

        // OP permission level packet here

        let mut buf = Vec::new();
        let root = self
            .server
            .handler
            .get_commands(&self.server, &self.proxy)
            .await?;
        root.serialize(&mut buf).await;

        self.send_packet(Commands {
            data: UnsizedVec(buf),
        })
        .await?;

        // Unlock recipes packet

        self.send_packet({
            let player = self.player.read().await;
            SynchronizePlayerPosition {
                x: player.position.x,
                y: player.position.y,
                z: player.position.z,
                yaw: player.rotation.yaw,
                pitch: player.rotation.pitch,
                flags: PlayerPositionFlags::new(),
                teleport_id: VarInt(0),
            }
        })
        .await?;

        self.add_all_players().await?;

        // Initialize world border packet

        self.send_packet(SetDefaultSpawn {
            position: Position { x: 0, y: 47, z: 0 },
            yaw: 90.0,
        })
        .await?;

        self.send_packet(SetContainerContent {
            window_id: 0,
            state_id: VarInt(0),
            items: self.player.read().await.inventory.slots.to_vec(),
            held_item: None,
        })
        .await?;

        // Advancements packet

        self.send_packet({
            let player = self.player.read().await;
            SetCenterChunk {
                x: VarInt((player.position.x as i32).rem_euclid(16)),
                z: VarInt((player.position.z as i32).rem_euclid(16)),
            }
        })
        .await?;

        for x in -2..=2 {
            for z in -2..=2 {
                let packet = {
                    let chunk_data = self.server.get_world().get_chunk(x, z).await?;
                    if let Some(chunk_lock) = chunk_data {
                        let chunk = chunk_lock.read().await;
                        Some(chunk.into_packet())
                    } else {
                        None
                    }
                };
                if let Some(packet) = packet {
                    self.send_packet(packet).await?;
                }
                self.client_data.loaded_chunks.lock().await.insert((x, z));
            }
        }

        self.send_packet({
            let player = self.player.read().await;
            SynchronizePlayerPosition {
                x: player.position.x,
                y: player.position.y,
                z: player.position.z,
                yaw: player.rotation.yaw,
                pitch: player.rotation.pitch,
                flags: PlayerPositionFlags::new(),
                teleport_id: VarInt(0),
            }
        })
        .await?;

        if let Some(information) = self.client_data.settings.read().await.as_ref() {
            self.update_entity_metadata(
                self.client_data.entity_id,
                vec![EntityMetadata::PlayerDisplayedSkinParts(
                    information.displayed_skin_parts.clone(),
                )],
            )
            .await?;
        }
        let (chunk_x, chunk_z) = {
            let position = &self.player.read().await.position;
            (position.x as i32 >> 4, position.z as i32 >> 4)
        };

        self.move_chunk(chunk_x, chunk_z).await?;

        Ok(())
    }
    pub(super) async fn handle_command(
        &self,
        command: ClientCommand<Proxy::TransferData>,
    ) -> Result<Option<Proxy::TransferData>, ConnectionError> {
        match command {
            ClientCommand::UpdateSlot {
                window_id,
                slot,
                item,
            } => {
                self.send_packet(SetContainerSlot {
                    window_id,
                    slot: slot as i16,
                    item,
                    state_id: VarInt(0),
                })
                .await?;
            }
            ClientCommand::CloseGui => {
                self.send_packet(CloseContainer { window_id: 1 }).await?;
                self.gui.write().await.take();
            }
            ClientCommand::UpdateAttributes { attributes } => {
                self.send_packet(UpdateAttributes {
                    entity_id: VarInt::from(self.client_data.entity_id),
                    attributes,
                })
                .await?;
            }
            ClientCommand::SetFlying { flying } => {
                self.player.write().await.flying = flying;
                self.send_packet(PlayerAbilities {
                    flags: PlayerAbilityFlags::new()
                        .with_flying(flying)
                        .with_allow_flying(self.player.read().await.flying_allowed),
                    flying_speed: 0.05,
                    walking_speed: 0.1,
                })
                .await?;
            }
            ClientCommand::BossBarAction { uuid, action } => {
                self.send_packet(BossBar { uuid, action }).await?;
            }
            ClientCommand::SyncPosition { position, rotation } => {
                let (rotation, flags) = if let Some(rotation) = rotation {
                    let mut flags = PlayerPositionFlags::new();
                    flags.set_pitch(false);
                    flags.set_yaw(false);
                    (rotation, flags)
                } else {
                    let mut flags = PlayerPositionFlags::new();
                    flags.set_pitch(true);
                    flags.set_yaw(true);
                    (Rotation::new(0., 0.), flags)
                };
                self.send_packet(SynchronizePlayerPosition {
                    x: position.x,
                    y: position.y,
                    z: position.z,
                    yaw: rotation.yaw,
                    pitch: rotation.pitch,
                    flags: flags,
                    teleport_id: VarInt(0),
                })
                .await?;

                let previous_position = self.player.read().await.position.clone();

                let previous_chunk_x = previous_position.x as i32 >> 4;
                let previous_chunk_z = previous_position.z as i32 >> 4;

                let chunk_x = position.x as i32 >> 4;
                let chunk_z = position.z as i32 >> 4;

                self.player.write().await.position = position.clone();

                if previous_chunk_x != chunk_x || previous_chunk_z != chunk_z {
                    self.move_chunk(chunk_x, chunk_z).await?;
                }

                self.server
                    .broadcast_entity_move(
                        &self,
                        Some(position),
                        previous_position,
                        None,
                        rotation,
                        false,
                    )
                    .await;
            }
            ClientCommand::MoveEntity {
                entity_id,
                position,
                previous_position,
                rotation,
                previous_rotation,
                on_ground,
            } => {
                if let Some(position) = &position {
                    let unsynced_entities = self.unsynced_entities.read().await;
                    if let Some(_old_pos) = unsynced_entities.get(&entity_id).clone() {
                        // dbg!(old_pos, position, entity_id);
                        let (yaw, pitch) = previous_rotation.serialize();
                        self.send_packet(TeleportEntity {
                            entity_id: VarInt(entity_id),
                            x: position.x,
                            y: position.y,
                            z: position.z,
                            yaw,
                            pitch,
                            on_ground,
                        })
                        .await?;
                        self.send_packet(UpdateEntityHeadRotation {
                            entity_id: VarInt(entity_id),
                            yaw,
                        })
                        .await?;
                        drop(unsynced_entities);
                        self.unsynced_entities.write().await.remove(&entity_id);
                        return Ok(None);
                    }
                }
                match (position, rotation) {
                    (Some(pos), Some(rot)) => {
                        let (delta_x, delta_y, delta_z) = (
                            ((pos.x * 32. - previous_position.x * 32.) * 128.),
                            ((pos.y * 32. - previous_position.y * 32.) * 128.),
                            ((pos.z * 32. - previous_position.z * 32.) * 128.),
                        );

                        if (delta_x < i16::MIN as f64 || delta_x > i16::MAX as f64)
                            || (delta_y < i16::MIN as f64 || delta_y > i16::MAX as f64)
                            || (delta_z < i16::MIN as f64 || delta_z > i16::MAX as f64)
                        {
                            let (yaw, pitch) = previous_rotation.serialize();
                            self.send_packet(TeleportEntity {
                                entity_id: VarInt(entity_id),
                                x: pos.x,
                                y: pos.y,
                                z: pos.z,
                                yaw,
                                pitch,
                                on_ground,
                            })
                            .await?;
                            self.send_packet(UpdateEntityHeadRotation {
                                entity_id: VarInt(entity_id),
                                yaw,
                            })
                            .await?;
                        } else {
                            let (yaw, pitch) = rot.serialize();
                            let (delta_x, delta_y, delta_z) =
                                (delta_x as i16, delta_y as i16, delta_z as i16);

                            self.send_packet(UpdateEntityPositionAndRotation {
                                entity_id: VarInt(entity_id),
                                delta_x,
                                delta_y,
                                delta_z,
                                yaw,
                                pitch,
                                on_ground,
                            })
                            .await?;
                            self.send_packet(UpdateEntityHeadRotation {
                                entity_id: VarInt(entity_id),
                                yaw,
                            })
                            .await?;
                        }
                    }
                    (Some(pos), None) => {
                        let (delta_x, delta_y, delta_z) = (
                            ((pos.x * 32. - previous_position.x * 32.) * 128.),
                            ((pos.y * 32. - previous_position.y * 32.) * 128.),
                            ((pos.z * 32. - previous_position.z * 32.) * 128.),
                        );

                        if (delta_x < i16::MIN as f64 || delta_x > i16::MAX as f64)
                            || (delta_y < i16::MIN as f64 || delta_y > i16::MAX as f64)
                            || (delta_z < i16::MIN as f64 || delta_z > i16::MAX as f64)
                        {
                            let (yaw, pitch) = previous_rotation.serialize();
                            self.send_packet(TeleportEntity {
                                entity_id: VarInt(entity_id),
                                x: pos.x,
                                y: pos.y,
                                z: pos.z,
                                yaw,
                                pitch,
                                on_ground,
                            })
                            .await?;
                            self.send_packet(UpdateEntityHeadRotation {
                                entity_id: VarInt(entity_id),
                                yaw,
                            })
                            .await?;
                        } else {
                            let (delta_x, delta_y, delta_z) =
                                (delta_x as i16, delta_y as i16, delta_z as i16);
                            self.send_packet(UpdateEntityPosition {
                                entity_id: VarInt(entity_id),
                                delta_x,
                                delta_y,
                                delta_z,
                                on_ground,
                            })
                            .await?;
                        }
                    }
                    (None, Some(rot)) => {
                        let (yaw, pitch) = rot.serialize();

                        self.send_packet(UpdateEntityRotation {
                            entity_id: VarInt(entity_id),
                            yaw,
                            pitch,
                            on_ground,
                        })
                        .await?;
                        self.send_packet(UpdateEntityHeadRotation {
                            entity_id: VarInt(entity_id),
                            yaw,
                        })
                        .await?;
                    }
                    _ => (),
                };
            }
            ClientCommand::ChatMessage { message } => {
                self.send_packet(SystemChatMessage {
                    message: message.to_string(),
                    action_bar: false,
                })
                .await?;
            }
            ClientCommand::UpdateEntityMetadata {
                entity_id,
                metadata,
            } => {
                self.update_entity_metadata(entity_id, metadata).await?;
            }
            ClientCommand::RemovePlayer { entity_id, uuid } => {
                self.send_packet(RemoveInfoPlayer {
                    players: vec![uuid],
                })
                .await?;
                self.send_packet(RemoveEntities {
                    entity_ids: vec![VarInt(entity_id)],
                })
                .await?;
            }
            ClientCommand::AddPlayer {
                client_data,
                gamemode,
                position,
                rotation,
            } => {
                let player_info = PlayerInfo {
                    action: PlayerInfoAction::AddSinglePlayer(
                        client_data.uuid,
                        AddPlayer {
                            name: client_data.profile.name.clone(),
                            properties: client_data.profile.properties.clone(),
                        },
                        UpdateGameMode {
                            gamemode: VarInt::from(gamemode as i32),
                        },
                        UpdateListed { listed: true },
                        UpdateLatency {
                            latency: VarInt::from(0),
                        },
                    ),
                };

                let (yaw, pitch) = rotation.serialize();
                let spawn_player = SpawnPlayer {
                    entity_id: VarInt(client_data.entity_id),
                    uuid: client_data.uuid,
                    x: position.x,
                    y: position.y,
                    z: position.z,
                    yaw,
                    pitch,
                };

                self.send_packet(player_info).await?;
                self.send_packet(spawn_player).await?;

                if let Some(information) = client_data.settings.read().await.as_ref() {
                    self.update_entity_metadata(
                        client_data.entity_id,
                        vec![EntityMetadata::PlayerDisplayedSkinParts(
                            information.displayed_skin_parts.clone(),
                        )],
                    )
                    .await?;
                }
            }
            ClientCommand::Disconnect { reason } => {
                self.send_packet(PlayDisconnect {
                    reason: reason.clone(),
                })
                .await?;
                return Err(ConnectionError::ClientDisconnected { reason });
            }
            ClientCommand::Transfer { data } => {
                let players: Vec<(u128, i32)> = self
                    .server
                    .player_list
                    .iter()
                    .filter(|client| client.client_data.uuid != self.client_data.uuid)
                    .map(|client| (client.client_data.uuid, client.client_data.entity_id))
                    .collect();

                self.send_packet(RemoveEntities {
                    entity_ids: players
                        .iter()
                        .map(|(_, entity_id)| VarInt(*entity_id))
                        .collect(),
                })
                .await?;
                self.send_packet(RemoveInfoPlayer {
                    players: players.iter().map(|(uuid, _)| *uuid).collect(),
                })
                .await?;

                self.send_packet(SetContainerContent {
                    window_id: 0,
                    state_id: VarInt(0),
                    items: Inventory::default().slots.to_vec(),
                    held_item: None,
                })
                .await?;

                if let Some(_) = self.gui.write().await.take() {
                    const GUI_ID: u8 = 1;
                    self.send_packet(CloseContainer { window_id: 1 }).await?;
                }

                return Ok(Some(data));
            }
            ClientCommand::SetHealth { health } => {
                self.player.write().await.health = health.clone();
                self.send_packet(SetHealth {
                    health,
                    food: VarInt(20),
                    food_saturation: 5.,
                })
                .await?;
            }
            ClientCommand::SetVelocity { velocity } => {
                self.send_packet(SetEntityVelocity {
                    entity_id: VarInt(self.client_data.entity_id),
                    velocity_x: (velocity.x * 8000.) as i16,
                    velocity_y: (velocity.y * 8000.) as i16,
                    velocity_z: (velocity.z * 8000.) as i16,
                })
                .await?;
            }
            ClientCommand::DisplayParticles { particles } => {
                for particle in particles {
                    self.send_packet(ParticlePacket { particle }).await?;
                }
            }
            ClientCommand::InitGui {
                items,
                title,
                window_type,
            } => {
                const GUI_ID: u8 = 1;
                self.send_packet(OpenScreen {
                    window_id: VarInt::from(GUI_ID as i32),
                    window_type: VarInt::from(window_type as i32),
                    title,
                })
                .await?;

                self.send_packet(SetContainerContent {
                    window_id: GUI_ID,
                    state_id: VarInt(0),
                    items,
                    held_item: None,
                })
                .await?;
            }
            ClientCommand::SendTitle {
                title,
                subtitle,
                fade_in,
                fade_out,
                stay,
            } => {
                self.send_packet(SetTitleText {
                    title: title.clone(),
                })
                .await?;
                self.send_packet(SetSubtitleText {
                    subtitle: subtitle.clone(),
                })
                .await?;
                self.send_packet(SetTitleAnimationTimes {
                    fade_in,
                    fade_out,
                    stay,
                })
                .await?;
            }
            ClientCommand::SendSystemChatMessage {
                message,
                action_bar,
            } => {
                self.send_packet(SystemChatMessage {
                    message,
                    action_bar,
                })
                .await?;
            }
            ClientCommand::SendSound {
                sound,
                source,
                position,
                volume,
                pitch,
            } => {
                fn process_number(input: f64) -> i32 {
                    (input * 8.) as i32
                }

                self.send_packet(SoundEffect {
                    sound,
                    sound_source: source,
                    x: process_number(position.x),
                    y: process_number(position.y),
                    z: process_number(position.z),
                    volume,
                    pitch,
                    seed: rand::random(),
                })
                .await?;
            }
            ClientCommand::SendHurtAnimation { entity_id, yaw } => {
                self.send_packet(HurtAnimation {
                    entity_id: VarInt(entity_id),
                    yaw,
                })
                .await?;
            }
            ClientCommand::SendEntityAnimation {
                entity_id,
                animation,
            } => {
                self.send_packet(EntityAnimation {
                    entity_id: VarInt(entity_id),
                    animation,
                })
                .await?;
            }
            ClientCommand::SendEntityEvent { entity_id, status } => {
                self.send_packet(EntityEvent {
                    entity_id,
                    event_id: status,
                })
                .await?;
            }
            ClientCommand::KillPlayer {
                entity_id,
                uuid: _u128,
            } => {
                self.send_packet(EntityEvent {
                    entity_id,
                    event_id: EntityEventStatus::SpawnDeathSmokeParticles,
                })
                .await?;
                self.send_packet(EntityEvent {
                    entity_id,
                    event_id: EntityEventStatus::PlayLivingEntityDeathSoundAndAnimation,
                })
                .await?;

                // self.send_packet(RemoveEntities {
                //     entity_ids: vec![VarInt(entity_id)],
                // })
                // .await?;
            }
        }
        Ok(None)
    }
    pub(super) async fn handle_packet(
        &self,
        packet: ServerBoundPacket,
    ) -> Result<(), ConnectionError> {
        self.handler.on_server_bound_packet(self).await?;
        match packet {
            ServerBoundPacket::PlayerAction(action) => match action.status {
                PlayerActionStatus::DropItem => {
                    self.handler.on_drop_item(self, false).await?;
                }
                PlayerActionStatus::DropItemStack => {
                    self.handler.on_drop_item(self, true).await?;
                }
                PlayerActionStatus::SwapItemInHand => {
                    self.handler.on_swap_item(self).await?;
                }
                a => println!("PlayerAction: {:?}", a),
            },
            ServerBoundPacket::CommandSuggestionsRequest(packet) => {
                let root = self
                    .server
                    .handler
                    .get_commands(&self.server, &self.proxy)
                    .await?;

                let mut args = Vec::new();

                let result = root.parse(&mut args, &packet.command[1..], 1).await;

                if let Some(suggestions) = result.suggestions {
                    self.send_packet(CommandSuggestionsResponse {
                        transaction_id: packet.transaction_id,
                        start: VarInt::from(suggestions.start as i32),
                        length: VarInt::from(suggestions.length as i32),
                        suggestions: suggestions.matches,
                    })
                    .await?;
                }
            }
            ServerBoundPacket::ServerBoundKeepAlive(ServerBoundKeepAlive { id }) => {
                let mut active_pings = self.active_pings.lock().await;

                let ping = active_pings.iter_mut().find(|ping| ping.id == id);

                // Ping is no longer active
                let Some(ping) = ping else {
                    return Ok(());
                };

                ping.response_time.replace(ping.start.elapsed());
            }
            ServerBoundPacket::ChatMessage(ChatMessage { message }) => {
                self.server.handle_chat(&self.proxy, self, message).await?
            }
            ServerBoundPacket::ChatCommand(ChatCommand { command }) => {
                let root = self
                    .server
                    .handler
                    .get_commands(&self.server, &self.proxy)
                    .await?;

                let mut args = Vec::new();

                let res = root.parse(&mut args, &command, 0).await;

                match res.executable {
                    Err(err) => {
                        self.show_chat_message(format!(
                            r##"{{"text":"/{}","color":"#AAB0BC"}}"##,
                            command
                        ));
                        self.show_chat_message(
                        json!(
                            {
                                "text": format!("help: {}, type /help for a list of commands", err.to_string()),
                                "color": "#E96A70"
                            }
                        )
                        .to_string(),
                    )
                    }
                    Ok(exec) => exec(args, &self, &self.server, &self.proxy).await,
                }
                // // dbg!(result);

                // self.server
                //     .handle_chat_command(&self.proxy, self, command)
                //     .await?
            }
            ServerBoundPacket::PlayerCommand(PlayerCommand {
                entity_id,
                action,
                // This is only used for jumping on a horse which is weird and we don't have horses
                action_parameter: _action_parameter,
            }) => {
                if i32::from(entity_id) != self.client_data.entity_id {
                    return Ok(());
                }

                match action {
                    PlayerCommandAction::StartSprinting => {
                        self.server
                            .handle_sprinting(&self.proxy, self, true)
                            .await?
                    }
                    PlayerCommandAction::StopSprinting => {
                        self.server
                            .handle_sprinting(&self.proxy, self, false)
                            .await?
                    }
                    PlayerCommandAction::StartSneaking => {
                        self.server.handle_sneaking(&self.proxy, self, true).await?
                    }
                    PlayerCommandAction::StopSneaking => {
                        self.server
                            .handle_sneaking(&self.proxy, self, false)
                            .await?
                    }
                    PlayerCommandAction::StartFlyingWithElytra => {
                        self.server
                            .handle_elytra_flying(&self.proxy, self, true)
                            .await?
                    }
                    _ => {}
                }
            }
            ServerBoundPacket::SetPlayerPositionAndRotation(pos_rot) => {
                self.server
                    .handle_position_update(
                        &self.proxy,
                        self,
                        pos_rot.on_ground,
                        Some(Vec3::new(pos_rot.x, pos_rot.y, pos_rot.z)),
                        Some(Rotation::new(pos_rot.yaw, pos_rot.pitch)),
                    )
                    .await?;
            }
            ServerBoundPacket::SetPlayerPosition(pos) => {
                self.server
                    .handle_position_update(
                        &self.proxy,
                        self,
                        pos.on_ground,
                        Some(Vec3::new(pos.x, pos.y, pos.z)),
                        None,
                    )
                    .await?;
            }
            ServerBoundPacket::SetPlayerRotation(rot) => {
                self.server
                    .handle_position_update(
                        &self.proxy,
                        self,
                        rot.on_ground,
                        None,
                        Some(Rotation::new(rot.yaw, rot.pitch)),
                    )
                    .await?;
            }
            ServerBoundPacket::ClickContainer(click) => {
                if let Some(gui) = self.gui.read().await.as_ref() {
                    gui.gui.handle_click(click, self).await?;
                    // gui.handle_click(click, self).await?;
                } else {
                    self.handler.on_click_container(self, click).await?;
                }
            }
            ServerBoundPacket::SetHeldItemServerBound(set_held_item) => {
                let mut player = self.player.write().await;
                player.selected_slot = set_held_item.slot as u8;
                player.attack_strength_ticker = 0;
            }
            ServerBoundPacket::Interact(interact) => {
                self.server
                    .handle_interact(&self.proxy, self, interact)
                    .await?;
            }
            ServerBoundPacket::SwingArm(swing_arm) => {
                self.server
                    .handle_swing(&self.proxy, self, swing_arm.arm)
                    .await?;
            }
            ServerBoundPacket::UseItem(use_item) => {
                let used_item_slot = {
                    match use_item.arm {
                        Hand::MainHand => {
                            let player = self.player.read().await;
                            InventorySlot::Hotbar {
                                slot: player.selected_slot as usize,
                            }
                        }
                        Hand::OffHand => InventorySlot::Offhand,
                    }
                };

                let used_item = {
                    let player_read = self.player.read().await;
                    match player_read.inventory.get_slot(&used_item_slot) {
                        Some(item) => Some(item.clone()),
                        None => None,
                    }
                };

                self.handler
                    .on_use_item(self, used_item, used_item_slot)
                    .await?;
            }
            ServerBoundPacket::CloseContainerServerBound(container) => {
                if container.window_id == 1 {
                    self.gui.write().await.take();
                }
            }
            _ => (),
        };
        Ok(())
    }
    pub(super) async fn move_chunk(
        &self,
        chunk_x: i32,
        chunk_z: i32,
    ) -> Result<(), ConnectionError> {
        let mut player_loaded_chunks = self.client_data.loaded_chunks.lock().await;

        for (loaded_chunk_x, loaded_chunk_z) in player_loaded_chunks.clone().iter() {
            if loaded_chunk_x + SERVER_RENDER_DISTANCE < chunk_x
                || loaded_chunk_x - SERVER_RENDER_DISTANCE > chunk_x
                || loaded_chunk_z + SERVER_RENDER_DISTANCE < chunk_z
                || loaded_chunk_z - SERVER_RENDER_DISTANCE > chunk_z
            {
                self.unload_chunk(*loaded_chunk_x, *loaded_chunk_z).await?;
                player_loaded_chunks.remove(&(*loaded_chunk_x, *loaded_chunk_z));
            }
        }

        self.send_packet(SetCenterChunk {
            x: VarInt(chunk_x),
            z: VarInt(chunk_z),
        })
        .await?;

        for x in -SERVER_RENDER_DISTANCE..=SERVER_RENDER_DISTANCE {
            for z in -SERVER_RENDER_DISTANCE..=SERVER_RENDER_DISTANCE {
                if player_loaded_chunks.contains(&(x + chunk_x, z + chunk_z)) {
                    continue;
                }
                let packet = {
                    let chunk_data = self
                        .server
                        .get_world()
                        .get_chunk(x + chunk_x, z + chunk_z)
                        .await?;
                    if let Some(chunk_lock) = chunk_data {
                        let chunk = chunk_lock.read().await;
                        Some(chunk.into_packet())
                    } else {
                        None
                    }
                };
                if let Some(packet) = packet {
                    // dbg!(x + chunk_x, z + chunk_z);
                    self.send_packet(packet).await?;
                }
                player_loaded_chunks.insert((x + chunk_x, z + chunk_z));
            }
        }

        Ok(())
    }
    pub(super) async fn unload_chunk(&self, x: i32, z: i32) -> Result<(), ConnectionError> {
        self.send_packet(UnloadChunk { x, z }).await?;
        Ok(())
    }
    pub(super) async fn ping(&self) -> Result<bool, ConnectionError> {
        let mut active_pings = self.active_pings.lock().await;

        let received_pings = active_pings
            .iter()
            .filter(|ping| ping.response_time.is_some())
            .collect::<Vec<_>>();

        if !received_pings.is_empty() {
            let mut total_time = Duration::from_secs(0);
            let total_pings = received_pings.len() as u32;

            for ping in received_pings {
                let Some(response_time) = ping.response_time else {
                continue;
            };
                total_time += response_time;
            }

            let average_time = total_time / total_pings;

            *self.ping.lock().await = average_time;
        }

        active_pings
            .retain(|ping| ping.response_time.is_none() && ping.start.elapsed().as_secs() < 15);

        let id = rand::thread_rng().gen();
        self.send_packet(ClientBoundKeepAlive { id }).await?;

        active_pings.push(ActivePing {
            id,
            start: Instant::now(),
            response_time: None,
        });

        Ok(true)
    }
    async fn update_entity_metadata(
        &self,
        entity_id: i32,
        metadata: Vec<EntityMetadata>,
    ) -> Result<(), ConnectionError> {
        self.send_packet(SetEntityMetadata {
            entity_id: VarInt(entity_id),
            metadata: UnsizedVec({
                let mut entity_metadata_inner = Vec::new();
                for meta in metadata {
                    meta.serialize(&mut entity_metadata_inner);
                }
                entity_metadata_inner.push(END_INDEX);
                entity_metadata_inner
            }),
        })
        .await?;
        Ok(())
    }
    async fn add_all_players(&self) -> Result<(), ConnectionError> {
        self.send_packet({
            let mut add_players = Vec::new();

            for client in self.server.player_list.iter() {
                let player = client.player.read().await;
                add_players.push((
                    client.client_data.uuid,
                    AddPlayer {
                        name: player.profile.name.clone(),
                        properties: player.profile.properties.clone(),
                    },
                    UpdateGameMode {
                        gamemode: VarInt::from(player.gamemode.clone() as i32),
                    },
                    UpdateListed { listed: true },
                    UpdateLatency {
                        latency: VarInt::from(0),
                    },
                ));
            }

            PlayerInfo {
                action: PlayerInfoAction::AddAllPlayers(add_players),
            }
        })
        .await?;

        let mut unsynced_players = self.unsynced_entities.write().await;
        for client in self.server.player_list.iter() {
            if client.client_data.uuid == self.client_data.uuid {
                continue;
            }

            self.send_packet({
                let player = client.player.read().await;
                let (yaw, pitch) = player.rotation.serialize();

                unsynced_players.insert(client.client_data.entity_id, player.position.clone());

                SpawnPlayer {
                    entity_id: VarInt(client.client_data.entity_id),
                    uuid: client.client_data.uuid,
                    x: player.position.x,
                    y: player.position.y,
                    z: player.position.z,
                    yaw,
                    pitch,
                }
            })
            .await?;

            if let Some(information) = client.client_data.settings.read().await.as_ref() {
                self.update_entity_metadata(
                    client.client_data.entity_id,
                    vec![
                        EntityMetadata::EntityFlags(client.player.read().await.entity_flags()),
                        EntityMetadata::PlayerDisplayedSkinParts(
                            information.displayed_skin_parts.clone(),
                        ),
                    ],
                )
                .await?;
            }
        }

        Ok(())
    }
    pub async fn send_packet<T: SerializePacket + ClientBoundPacket + Debug>(
        &self,
        packet: T,
    ) -> Result<(), ConnectionError> {
        self.handler.on_client_bound_packet(self).await?;
        self.connection.write_packet(packet).await?;
        Ok(())
    }
    #[allow(unused_must_use)]
    pub async fn display_gui(&self, gui: Arc<Handler::ServerGUI>) {
        let init = gui.init(self).await;

        *self.gui.write().await = Some(ClientGUI {
            gui,
            events: init.receiver,
        });

        self.to_client.send(ClientCommand::InitGui {
            items: init.items,
            title: init.title,
            window_type: init.window_type,
        });
    }
    #[allow(unused_must_use)]
    pub fn close_gui(&self) {
        self.to_client.send(ClientCommand::CloseGui);
    }
    #[allow(unused_must_use)]
    pub async fn update_inventory_slot(&self, slot: InventorySlot, item: Slot) {
        self.player
            .write()
            .await
            .inventory
            .set_slot(slot.clone(), item.clone());

        self.to_client.send(ClientCommand::UpdateSlot {
            slot: slot.value() as u16,
            item,
            window_id: 0,
        });
    }
    #[allow(unused_must_use)]
    pub fn show_chat_message(&self, message: String) {
        self.to_client.send(ClientCommand::ChatMessage { message });
    }
    #[allow(unused_must_use)]
    pub fn send_system_chat_message(&self, message: String, action_bar: bool) {
        self.to_client.send(ClientCommand::SendSystemChatMessage {
            message,
            action_bar,
        });
    }
    #[allow(unused_must_use)]
    pub fn transfer(&self, transfer_data: Proxy::TransferData) {
        self.to_client.send(ClientCommand::Transfer {
            data: transfer_data,
        });
    }
    #[allow(unused_must_use)]
    pub fn sync_position(&self, position: Vec3, rotation: Option<Rotation>) {
        self.to_client
            .send(ClientCommand::SyncPosition { position, rotation });
    }
    #[allow(unused_must_use)]
    pub fn disconnect(&self, reason: String) {
        self.to_client.send(ClientCommand::Disconnect { reason });
    }
    #[allow(unused_must_use)]
    pub fn set_health(&self, health: f32) {
        assert!(
            health > 0.0,
            "Health must be greater than 0, don't actually kill the client"
        );
        self.to_client.send(ClientCommand::SetHealth { health });
    }
    #[allow(unused_must_use)]
    pub fn send_particles(&self, particles: Vec<Particle>) {
        self.to_client_visual
            .send(ClientCommand::DisplayParticles { particles });
    }
    #[allow(unused_must_use)]
    pub fn set_velocity(&self, velocity: Vec3) {
        self.to_client.send(ClientCommand::SetVelocity { velocity });
    }
    #[allow(unused_must_use)]
    pub fn send_title(
        &self,
        title: String,
        subtitle: String,
        fade_in: i32,
        stay: i32,
        fade_out: i32,
    ) {
        self.to_client.send(ClientCommand::SendTitle {
            title,
            subtitle,
            fade_in,
            stay,
            fade_out,
        });
    }
    #[allow(unused_must_use)]
    pub fn send_sound(
        &self,
        sound: IdMapHolder<CustomSound, VanillaSound>,
        source: SoundSource,
        position: Vec3,
        volume: f32,
        pitch: f32,
    ) {
        self.to_client_visual.send(ClientCommand::SendSound {
            sound,
            source,
            position,
            volume,
            pitch,
        });
    }
    #[allow(unused_must_use)]
    pub fn send_boss_bar_action(&self, uuid: u128, action: BossBarAction) {
        match &action {
            BossBarAction::UpdateHealth { .. } => {
                self.to_client_visual
                    .send(ClientCommand::BossBarAction { uuid, action });
            }
            _ => {
                self.to_client
                    .send(ClientCommand::BossBarAction { uuid, action });
            }
        }
    }
    #[allow(unused_must_use)]
    pub fn set_flying(&self, flying: bool) {
        self.to_client.send(ClientCommand::SetFlying { flying });
    }
    #[allow(unused_must_use)]
    pub async fn set_max_health(&self, max_health: f32) {
        self.player.write().await.max_health = max_health;
        self.to_client.send(ClientCommand::UpdateAttributes {
            attributes: vec![Attribute::MaxHealth {
                value: self.player.read().await.max_health as f64,
            }],
        });
    }
    #[allow(unused_must_use)]
    pub fn send_hurt_animation(&self, entity_id: i32, yaw: f32) {
        self.to_client
            .send(ClientCommand::SendHurtAnimation { entity_id, yaw });
    }
    #[allow(unused_must_use)]
    pub fn send_entity_animation(&self, entity_id: i32, animation: EntityAnimationType) {
        self.to_client.send(ClientCommand::SendEntityAnimation {
            entity_id,
            animation,
        });
    }
    #[allow(unused_must_use)]
    pub fn send_entity_event(&self, entity_id: i32, status: EntityEventStatus) {
        self.to_client
            .send(ClientCommand::SendEntityEvent { entity_id, status });
    }
    #[allow(unused_must_use)]
    pub fn kill_player(&self, entity_id: i32, uuid: u128) {
        self.to_client
            .send(ClientCommand::KillPlayer { entity_id, uuid });
    }
    #[allow(unused_must_use)]
    pub fn __move_entity(
        &self,
        entity_id: i32,
        position: Option<Vec3>,
        previous_position: Vec3,
        rotation: Option<Rotation>,
        previous_rotation: Rotation,
        on_ground: bool,
    ) {
        self.to_client.send(ClientCommand::MoveEntity {
            entity_id,
            position,
            previous_position,
            rotation,
            previous_rotation,
            on_ground,
        });
    }
    #[allow(unused_must_use)]
    pub fn __update_entity_metadata(&self, entity_id: i32, metadata: Vec<EntityMetadata>) {
        self.to_client.send(ClientCommand::UpdateEntityMetadata {
            entity_id,
            metadata,
        });
    }
    #[allow(unused_must_use)]
    pub fn __add_player(
        &self,
        client_data: Arc<ClientData>,
        gamemode: u8,
        position: Vec3,
        rotation: Rotation,
    ) {
        self.to_client.send(ClientCommand::AddPlayer {
            client_data,
            gamemode,
            position,
            rotation,
        });
    }
    #[allow(unused_must_use)]
    pub fn __remove_player(&self, entity_id: i32, uuid: u128) {
        self.to_client
            .send(ClientCommand::RemovePlayer { entity_id, uuid });
    }
    pub async fn get_velocity(&self) -> Vec3 {
        let player = self.player.read().await;
        let Some(previous_position) = &player.previous_position else {
            return Vec3::new(0., 0., 0.);
        };

        let delta = player.position.clone() - previous_position.position.clone();
        let multiplier = previous_position.time.elapsed().as_secs_f64()
            / Duration::from_millis(50).as_secs_f64();

        println!("{:?} {}", delta.clone() * Vec3::scalar(20.), multiplier);
        delta * Vec3::new(multiplier, multiplier, multiplier)
    }
}

mod internal {}
