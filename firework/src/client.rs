use crate::{entities::EntityDataFlags, PlayerHandler};
use crate::{
    entities::{EntityMetadata, END_INDEX},
    gui::{GameQueueMenuGui, Gui, Gui::*, GuiPackets},
    {ClientData, ConnectionError, Rotation, Server, ServerHandler, ServerProxy, Vec3},
};
use firework_authentication::Profile;
use firework_data::{
    items::{Compass, GrayDye, Item, LimeDye, RedDye},
    tags::{REGISTRY, TAGS},
};
use firework_protocol::{
    client_bound::{
        ChangeDifficulty, ClientBoundKeepAlive, ClientBoundPacketID, CloseContainer, Commands,
        CustomSound, IdMapHolder, LoginPlay, ParticlePacket, PlayDisconnect, PlayerAbilities,
        PlayerInfo, PluginMessage, RemoveEntities, RemoveInfoPlayer, ResourcePack, Respawn,
        SerializePacket, SetCenterChunk, SetContainerContent, SetContainerSlot, SetDefaultSpawn,
        SetEntityMetadata, SetEntityVelocity, SetHealth, SetHeldItem, SetRecipes, SetTags,
        SoundEffect, SoundSource, SpawnPlayer, SynchronizePlayerPosition, SystemChatMessage,
        TeleportEntity, UnloadChunk, UpdateAttributes, UpdateEntityHeadRotation,
        UpdateEntityPosition, UpdateEntityPositionAndRotation, UpdateEntityRotation,
    },
    data_types::{
        commands::{ArgumentType, CommandNode, SuggestionsType},
        AddPlayer, Attribute, ItemNbt, ItemNbtDisplay, Particle, PlayerAbilityFlags,
        PlayerCommandAction, PlayerInfoAction, PlayerPositionFlags, Slot, UpdateGameMode,
        UpdateLatency, UpdateListed,
    },
    read_specific_packet,
    server_bound::{ChatMessage, PlayerCommand, ServerBoundPacket},
    ConnectionState, Protocol, ProtocolError,
};
use firework_protocol_core::{DeserializeField, Position, SerializeField, UnsizedVec, VarInt};
use rand::Rng;
use std::{
    fmt::Debug,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{broadcast, Mutex, RwLock};

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
        rotation: Rotation,
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
    SetHealth {
        health: f32,
    },
    SetVelocity {
        velocity: Vec3,
    },
    DisplayParticles {
        particles: Vec<Particle>,
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

    pub elytra_flying: bool,

    pub flying: bool,
    pub flying_allowed: bool,
    pub inventory: Inventory,
    pub open_gui: Option<Gui>,
    pub health: f32,
    pub max_health: f32,
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
    slots: [Option<Slot>; 46],
}

impl Default for Inventory {
    fn default() -> Self {
        const EMPTY_SLOT: Option<Slot> = None;
        Inventory {
            slots: [EMPTY_SLOT; 46],
        }
    }
}

trait SlotValue {
    fn value(&self) -> usize;
}

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

impl Inventory {
    const CRAFTING_OFFSET: usize = 0;
    const ARMOR_OFFSET: usize = 5;
    const HOTBAR_OFFSET: usize = 36;
    const OFFHAND_OFFSET: usize = 45;
    pub fn get_slot(&self, slot: InventorySlot) -> &Option<Slot> {
        match slot {
            InventorySlot::Helmet => &self.slots[Self::ARMOR_OFFSET],
            InventorySlot::Chestplate => &self.slots[Self::ARMOR_OFFSET + 1],
            InventorySlot::Leggings => &self.slots[Self::ARMOR_OFFSET + 2],
            InventorySlot::Boots => &self.slots[Self::ARMOR_OFFSET + 3],
            InventorySlot::CraftingResult => &self.slots[Self::CRAFTING_OFFSET],
            InventorySlot::Offhand => &self.slots[Self::OFFHAND_OFFSET],
            InventorySlot::CraftingGrid { slot } => {
                assert!(slot < 4);
                &self.slots[Self::CRAFTING_OFFSET + slot + 1]
            }
            InventorySlot::Hotbar { slot } => {
                assert!(slot < 9);
                &self.slots[Self::HOTBAR_OFFSET + slot]
            }
            InventorySlot::MainInventory { slot } => {
                assert!(slot < 36);
                &self.slots[Self::HOTBAR_OFFSET + slot]
            }
        }
    }
    pub fn set_slot(&mut self, slot: InventorySlot, item: Option<Slot>) {
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
                assert!(slot < 36);
                self.slots[slot] = item
            }
        }
    }
}

// #[derive(Debug)]
pub struct Client<Handler, Proxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    pub client_data: Arc<ClientData>,
    pub player: RwLock<Player>,
    to_client: broadcast::Sender<ClientCommand<Proxy::TransferData>>,
    connection: Arc<Protocol>,
    ping_acknowledged: Mutex<bool>,
    server: Arc<Server<Handler, Proxy>>,
    proxy: Arc<Proxy>,
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
    ) -> Client<Handler, Proxy> {
        Client {
            connection,
            client_data,
            to_client,
            player: RwLock::new(player),
            ping_acknowledged: Mutex::new(true),
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
                is_flat: self.server.world.flat,
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
                view_distance: VarInt(7),
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

        self.send_packet(SetHealth {
            health: self.player.read().await.health as f32,
            food: VarInt(20),
            food_saturation: 5.0,
        })
        .await?;

        self.send_packet(UpdateAttributes {
            entity_id: VarInt::from(self.client_data.entity_id),
            attributes: vec![Attribute::MaxHealth {
                value: self.player.read().await.max_health as f64,
            }],
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
        {
            let root = CommandNode::root()
                .sub_command(
                    CommandNode::literal("play").sub_command(CommandNode::argument(
                        "game",
                        ArgumentType::Float {
                            min: Some(10.),
                            max: None,
                        },
                        Some(SuggestionsType::AskServer),
                    )),
                )
                .sub_command(CommandNode::literal("echo"));

            root.serialize(&mut buf);
        }
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
                dismount_vehicle: false,
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

        for x in -7..=7 {
            for z in -7..=7 {
                let packet = {
                    let chunk_data = self.server.world.get_chunk(x, z).await?;
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
                dismount_vehicle: false,
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

        Ok(())
    }
    pub(super) async fn handle_command(
        &self,
        command: ClientCommand<Proxy::TransferData>,
    ) -> Result<Option<Proxy::TransferData>, ConnectionError> {
        match command {
            ClientCommand::SyncPosition { position, rotation } => {
                self.send_packet(SynchronizePlayerPosition {
                    x: position.x,
                    y: position.y,
                    z: position.z,
                    yaw: rotation.yaw,
                    pitch: rotation.pitch,
                    flags: PlayerPositionFlags::new(),
                    teleport_id: VarInt(0),
                    dismount_vehicle: false,
                })
                .await?;

                let previous_position = self.player.read().await.position.clone();

                let previous_chunk_x = previous_position.x as i32 >> 4;
                let previous_chunk_z = previous_position.z as i32 >> 4;

                let chunk_x = position.x as i32 >> 4;
                let chunk_z = position.z as i32 >> 4;

                self.player.write().await.position = position;

                if previous_chunk_x != chunk_x || previous_chunk_z != chunk_z {
                    self.move_chunk(chunk_x, chunk_z).await?;
                }
            }
            ClientCommand::MoveEntity {
                entity_id,
                position,
                previous_position,
                rotation,
                previous_rotation,
                on_ground,
            } => {
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

                if let Some(gui) = &self.player.read().await.open_gui {
                    let close_container = CloseContainer { window_id: 42 };
                    drop(gui);
                    self.send_packet(close_container).await?;
                };

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
        }
        Ok(None)
    }
    pub(super) async fn handle_packet(
        &self,
        packet: ServerBoundPacket,
    ) -> Result<(), ConnectionError> {
        self.handler.on_server_bound_packet(self).await?;
        match packet {
            ServerBoundPacket::CommandSuggestionsRequest(packet) => {
                println!("CommandSuggestionsRequest: {:?}", packet);
            }
            ServerBoundPacket::ServerBoundKeepAlive(_) => {
                let mut ping_acknowledged = self.ping_acknowledged.lock().await;

                if *ping_acknowledged == true {
                    println!("Client is being weird");
                }

                *ping_acknowledged = true;
            }
            ServerBoundPacket::ChatMessage(ChatMessage { message }) => {
                let gui = GameQueueMenuGui {};

                self.send_packet(gui.open()).await?;

                self.send_packet(gui.draw()).await?;
                self.player.write().await.open_gui = Some(GameQueueMenuGui(gui));

                self.server.handle_chat(&self.proxy, self, message).await?
            }
            ServerBoundPacket::PlayerCommand(PlayerCommand {
                entity_id,
                action,
                // This is only used for jumping on a horse which is weird and we don' have horses
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
                let mut player = self.player.write().await;

                if let Some(gui) = player.open_gui.as_mut() {
                    gui.handle_click(click.slot, &self, &self.server).await?;
                }
            }
            ServerBoundPacket::SetHeldItemServerBound(set_held_item) => {
                let mut player = self.player.write().await;
                player.selected_slot = set_held_item.slot as u8;
            }
            ServerBoundPacket::UseItem(use_item) => {
                let used_item_slot;
                let used_item;

                {
                    let player_read = self.player.read().await;
                    used_item_slot = player_read.selected_slot.clone();
                    used_item = match player_read.inventory.get_slot(InventorySlot::Hotbar {
                        slot: used_item_slot.clone() as usize,
                    }) {
                        Some(item) => Some(item.clone()),
                        None => None,
                    }
                }
                if let Some(used_item) = used_item {
                    // logic for using items (this is hardcoded for now lol also it only works for the lobby server)
                    // sorry future will probably doing other servers and being like why the heck doesn't this work

                    match used_item.item_id.0.try_into().unwrap() {
                        Compass::ID => {
                            // open game queue menu
                            let gui = GameQueueMenuGui {};

                            self.send_packet(gui.open()).await?;

                            self.send_packet(gui.draw()).await?;

                            self.player.write().await.open_gui = Some(GameQueueMenuGui(gui));
                        }
                        RedDye::ID => {
                            // while the resource pack is loading, use a placeholder inert item
                            let inert_slot = Slot {
                                item_id: VarInt(GrayDye::ID as i32),
                                item_count: 1,
                                nbt: ItemNbt {
                                    display: Some(ItemNbtDisplay {
                                        name: Some(
                                            r#"{"italic":false,"extra":[
                                            {"color":"white","text":"Resource Pack: "},
                                            {"color":"aqua","text":"Loading"},
                                            {"color":"gray","text":" (Right Click)"}
                                            ],"text":""}"#
                                                .to_string(),
                                        ),
                                        lore: None,
                                    }),
                                    ..Default::default()
                                },
                            };
                            self.connection
                                .write_packet(SetContainerSlot {
                                    window_id: 0,
                                    slot: used_item_slot as i16 + 36,
                                    item: Some(inert_slot),
                                    state_id: VarInt(0),
                                })
                                .await?;

                            println!("Sending resource pack");

                            // i just hosted it on my dropbox, you can host it on your own server or something if you want
                            let packet = ResourcePack::new(
                                "https://cdn.discordapp.com/attachments/921939326517002241/1072220230836826202/MusicPack.zip"
                                    .to_string(),
                                None,
                            )
                            .await;
                            if let Err(e) = packet {
                                println!("Error sending resource pack: {:?}", e);
                            } else {
                                self.send_packet(packet.unwrap()).await?;
                            }
                            let green_dye_slot = Slot {
                                item_id: VarInt(LimeDye::ID as i32),
                                item_count: 1,
                                nbt: ItemNbt {
                                    display: Some(ItemNbtDisplay {
                                        name: Some(
                                            r#"{"italic":false,"extra":[
                                            {"color":"white","text":"Resource Pack: "},
                                            {"color":"green","text":"Enabled"},
                                            {"color":"gray","text":" (Right click)"}
                                            ],"text":""}"#
                                                .to_string(),
                                        ),
                                        lore: None,
                                    }),
                                    ..Default::default()
                                },
                            };
                            // set the item in the hotbar
                            self.connection
                                .write_packet(SetContainerSlot {
                                    window_id: 0,
                                    state_id: VarInt(0),
                                    slot: (used_item_slot as i16) + 36,
                                    item: Some(green_dye_slot.clone()), // clone bad but this code will be changed anyways
                                })
                                .await?;

                            self.player.write().await.inventory.set_slot(
                                InventorySlot::Hotbar {
                                    slot: used_item_slot as usize,
                                },
                                Some(green_dye_slot),
                            );
                        }
                        LimeDye::ID => {
                            // remove the resource pack from the client

                            // maybe this works by sending a resource pack packet with an empty url and forced to true
                            println!("Removing resource pack");
                            let packet = ResourcePack {
                                url: "".to_string(),
                                hash: "".to_string(),
                                forced: false,
                                prompt: None,
                            };
                            self.send_packet(packet).await?;
                            let red_dye_slot = Slot {
                                item_id: VarInt(RedDye::ID as i32),
                                item_count: 1,
                                nbt: ItemNbt {
                                    display: Some(ItemNbtDisplay {
                                        name: Some(
                                            r#"{"italic":false,"extra":[
                                            {"color":"white","text":"Resource Pack: "},
                                            {"color":"red","text":"Disabled"},
                                            {"color":"gray","text":" (Right click)"}
                                            ],"text":""}"#
                                                .to_string(),
                                        ),
                                        lore: None,
                                    }),
                                    ..Default::default()
                                },
                            };
                            self.connection
                                .write_packet(SetContainerSlot {
                                    window_id: 0,
                                    state_id: VarInt(0),
                                    slot: (used_item_slot as i16) + 36,
                                    item: Some(red_dye_slot.clone()), // clone bad but this code will be changed anyways
                                })
                                .await?;
                            // set the item in the hotbar
                            self.player.write().await.inventory.set_slot(
                                InventorySlot::Hotbar {
                                    slot: used_item_slot as usize,
                                },
                                Some(red_dye_slot),
                            )
                        }
                        _ => {}
                    }
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

        for chunk in player_loaded_chunks.clone().iter() {
            if chunk.0 + 7 < chunk_x
                || chunk.0 - 7 > chunk_x
                || chunk.1 + 7 < chunk_z
                || chunk.1 - 7 > chunk_z
            {
                self.unload_chunk(chunk.0, chunk.1).await?;
                player_loaded_chunks.remove(chunk);
            }
        }

        self.send_packet(SetCenterChunk {
            x: VarInt(chunk_x),
            z: VarInt(chunk_z),
        })
        .await?;

        for x in -7..=7 {
            for z in -7..=7 {
                if player_loaded_chunks.contains(&(x + chunk_x, z + chunk_z)) {
                    continue;
                }
                let packet = {
                    let chunk_data = self
                        .server
                        .world
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
                    self.send_packet(packet).await?;
                }
                player_loaded_chunks.insert((x + chunk_x, z + chunk_z));
            }
        }

        Ok(())
    }
    pub(super) async fn set_container_content(
        &self,
        content: SetContainerContent,
    ) -> Result<(), ConnectionError> {
        self.send_packet(content).await?;
        // Lmao this is test code
        self.send_packet(SoundEffect {
            sound: IdMapHolder::Direct(CustomSound {
                resource_location: "minecraft:music.glide_map_1".to_string(),
                range: None,
            }),
            sound_source: SoundSource::Player,
            x: 0,
            y: 374,
            z: 0,
            volume: 1.,
            pitch: 1.,
            seed: 0,
        })
        .await?;

        Ok(())
    }
    pub(super) async fn unload_chunk(&self, x: i32, z: i32) -> Result<(), ConnectionError> {
        self.send_packet(UnloadChunk { x, z }).await?;
        Ok(())
    }
    pub(super) async fn ping(&self) -> Result<(), ConnectionError> {
        {
            let mut ping_acknowledged = self.ping_acknowledged.lock().await;
            if !*ping_acknowledged {
                return Err(ConnectionError::ClientTimedOut);
            } else {
                *ping_acknowledged = false;
            }
        }

        let id = rand::thread_rng().gen();
        self.send_packet(ClientBoundKeepAlive { id }).await?;

        Ok(())
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

        for client in self.server.player_list.iter() {
            if client.client_data.uuid == self.client_data.uuid {
                continue;
            }

            self.send_packet({
                let player = client.player.read().await;
                let (yaw, pitch) = player.rotation.serialize();

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
    async fn send_packet<T: SerializePacket + ClientBoundPacketID + Debug>(
        &self,
        packet: T,
    ) -> Result<(), ConnectionError> {
        self.handler.on_client_bound_packet(self).await?;
        self.connection.write_packet(packet).await?;
        Ok(())
    }
    pub fn show_chat_message(&self, message: String) {
        self.to_client.send(ClientCommand::ChatMessage { message });
    }
    pub fn transfer(&self, transfer_data: Proxy::TransferData) {
        self.to_client.send(ClientCommand::Transfer {
            data: transfer_data,
        });
    }
    pub fn sync_position(&self, position: Vec3, rotation: Rotation) {
        self.to_client
            .send(ClientCommand::SyncPosition { position, rotation });
    }
    pub fn disconnect(&self, reason: String) {
        self.to_client.send(ClientCommand::Disconnect { reason });
    }
    pub fn set_health(&self, health: f32) {
        self.to_client.send(ClientCommand::SetHealth { health });
    }
    fn send_particles(&self, particles: Vec<Particle>) {
        self.to_client
            .send(ClientCommand::DisplayParticles { particles });
    }
    pub fn set_velocity(&self, velocity: Vec3) {
        self.to_client.send(ClientCommand::SetVelocity { velocity });
    }
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
    pub fn __update_entity_metadata(&self, entity_id: i32, metadata: Vec<EntityMetadata>) {
        self.to_client.send(ClientCommand::UpdateEntityMetadata {
            entity_id,
            metadata,
        });
    }
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

        delta * Vec3::new(multiplier, multiplier, multiplier)
    }
}

//  hi xavier

mod internal {}
