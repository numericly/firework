use crate::{
    entities::{DisplayedSkinPartsFlags, EntityMetadata, END_INDEX},
    gui::{GameQueueMenuGui, Gui, Gui::*, GuiPackets, TestGui},
    server::{self, read_packet_or_err, ConnectionError, Rotation, Server, ServerHandler, Vec3},
};
use authentication::{Profile, ProfileProperty};
use minecraft_data::{
    items::{Compass, Elytra, GrayDye, Item, LightGrayDye, LimeDye, RedDye},
    tags::{REGISTRY, TAGS},
};
use nbt::Blob;
use protocol::{
    client_bound::{
        ChangeDifficulty, ClientBoundKeepAlive, Commands, CustomSound, IdMapHolder,
        InitializeWorldBorder, LoginPlay, PlayDisconnect, PlayerAbilities, PlayerInfo,
        RemoveEntities, RemoveInfoPlayer, ResourcePack, SetCenterChunk, SetContainerContent,
        SetContainerSlot, SetDefaultSpawn, SetEntityMetadata, SetEntityVelocity, SetHeldItem,
        SetRecipes, SetTags, SoundEffect, SoundSource, SpawnPlayer, SynchronizePlayerPosition,
        SystemChatMessage, TeleportEntity, UpdateEntityHeadRotation, UpdateEntityPosition,
        UpdateEntityPositionAndRotation, UpdateEntityRotation,
    },
    data_types::{
        AddPlayer, CommandNode, FloatProps, ItemNbt, ItemNbtDisplay, NodeType, Parser,
        PlayerAbilityFlags, PlayerCommandAction, PlayerInfoAction, PlayerPositionFlags, Slot,
        UpdateGameMode, UpdateLatency, UpdateListed,
    },
    server_bound::{ChatMessage, PlayerCommand, ServerBoundPacket},
    ConnectionState, Protocol,
};
use protocol_core::{Position, UnsizedVec, VarInt};
use rand::Rng;
use std::{ops::Add, sync::Arc};
use tokio::sync::{broadcast, Mutex, RwLock};

#[derive(Debug, Clone)]
pub enum ClientCommand {
    Disconnect {
        reason: String,
    },
    Ping,
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
        uuid: u128,
        name: String,
        properties: Vec<ProfileProperty>,
        gamemode: u8,
        entity_id: i32,
        position: Vec3,
        rotation: Rotation,
    },
}

const SPAWN_POSITION: Vec3 = Vec3::new(0.5, 47.0, 0.5);

#[derive(Debug)]
pub struct Player {
    pub uuid: u128,
    pub profile: Profile,
    pub gamemode: u8,
    // Previous gamemode can be -1 if there is no previous gamemode
    pub previous_gamemode: i8,
    pub reduced_debug_info: bool,
    pub selected_slot: u8,
    pub position: Vec3,
    pub on_ground: bool,
    pub rotation: Rotation,
    pub sneaking: bool,
    pub sprinting: bool,
    pub flying: bool,
    pub inventory: Inventory,
}

impl Player {
    pub fn new(uuid: u128, profile: Profile) -> Player {
        Player {
            uuid,
            profile,
            gamemode: 0,
            previous_gamemode: 0,
            reduced_debug_info: false,
            selected_slot: 0,
            position: SPAWN_POSITION,
            on_ground: false,
            rotation: Rotation::new(0.0, 0.0),
            sneaking: false,
            sprinting: false,
            flying: false,
            inventory: Inventory::new(),
        }
    }
}

#[derive(Debug)]
pub struct Inventory {
    slots: [Option<Slot>; 46],
}

impl Inventory {
    const ARMOR_OFFSET: usize = 5;
    const HOTBAR_OFFSET: usize = 36;
    pub const fn new() -> Inventory {
        const EMPTY_SLOT: Option<Slot> = None;
        Inventory {
            slots: [EMPTY_SLOT; 46],
        }
    }
    pub fn get_hotbar_slot(&self, slot: usize) -> Option<&Slot> {
        self.slots
            .get(slot + Self::HOTBAR_OFFSET)
            .and_then(|slot| slot.as_ref())
    }
    pub fn set_hotbar_slot(&mut self, slot: usize, item: Slot) {
        self.slots[slot + Self::HOTBAR_OFFSET] = Some(item);
    }
    pub fn get_armor_slot(&self, slot: usize) -> Option<&Slot> {
        self.slots
            .get(slot + Self::ARMOR_OFFSET)
            .and_then(|slot| slot.as_ref())
    }
    pub fn set_armor_slot(&mut self, slot: usize, item: Slot) {
        self.slots[slot + Self::ARMOR_OFFSET] = Some(item);
    }
}

#[derive(Debug)]
pub struct Client {
    pub player: RwLock<Player>,
    pub uuid: u128,
    pub entity_id: i32,
    pub to_client: broadcast::Sender<ClientCommand>,
    connection: Arc<Protocol>,
    ping_acknowledged: Mutex<bool>,
    open_gui: Mutex<Option<Gui>>,
}

impl Client {
    pub fn new(
        connection: Arc<Protocol>,
        player: Player,
        entity_id: i32,
        uuid: u128,
        to_client: broadcast::Sender<ClientCommand>,
    ) -> Client {
        Client {
            player: RwLock::new(player),
            connection,
            uuid,
            entity_id,
            to_client,
            ping_acknowledged: Mutex::new(true),
            open_gui: Mutex::new(None),
        }
    }
    pub async fn read_packet(&self) -> Result<ServerBoundPacket, ConnectionError> {
        Ok(self.connection.read_and_deserialize().await?)
    }
    pub async fn load_world<T: ServerHandler>(
        &self,
        server: &Server<T>,
    ) -> Result<(), ConnectionError> {
        {
            let player = self.player.read().await;
            let world_login = LoginPlay {
                entity_id: self.entity_id,
                is_hardcore: false,
                game_mode: player.gamemode,
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
            };
            self.connection.write_packet(world_login).await?;
        }

        {
            *self.connection.connection_state.write().await = ConnectionState::Play;
        }

        {
            let change_difficulty = ChangeDifficulty {
                difficulty: *server.world.difficulty.read().unwrap(),
                locked: *server.world.difficulty_locked.read().unwrap(),
            };
            self.connection.write_packet(change_difficulty).await?;
        }

        let set_default_spawn = SetDefaultSpawn {
            position: Position { x: 0, y: 47, z: 0 },
            yaw: 90.0,
        };

        self.connection.write_packet(set_default_spawn).await?;

        let player_abilities = PlayerAbilities {
            flags: PlayerAbilityFlags::new()
                .with_flying(true)
                .with_allow_flying(true),
            flying_speed: 0.05,
            walking_speed: 0.1,
        };

        self.connection.write_packet(player_abilities).await?;

        let client_information =
            read_packet_or_err!(ClientInformation, self.connection, ConnectionState::Play);

        {
            let player = self.player.read().await;
            let set_selected_slot = SetHeldItem {
                slot: player.selected_slot,
            };
            self.connection.write_packet(set_selected_slot).await?;
        }

        // TODO dont do this

        {
            self.player.write().await.inventory.set_armor_slot(
                1,
                Slot {
                    item_id: VarInt(Elytra::ID.try_into().unwrap()),
                    item_count: 1,
                    nbt: ItemNbt {
                        ..Default::default()
                    },
                },
            );
        }

        {
            self.player.write().await.inventory.set_hotbar_slot(7, Slot {
                item_id: VarInt(RedDye::ID.try_into().unwrap()),
                item_count: 1,
                nbt: ItemNbt {
                    display: Some(ItemNbtDisplay {
                        name: Some(r#"{"italic":false,"extra":[
                        {"color":"white","text":"Resource Pack: "},
                        {"color":"red","text":"Disabled"},
                        {"color":"gray","text":" (Right click)"}
                            ],"text":""}"#.to_string()),
                        lore: Some(vec![r#"{"text":"Right click to enable. The resource pack adds custom music to the minigames, and it's like 10mb probably.","italic":"false"}"#.to_string()]),
                    }),
                },
            })
        }

        {
            self.player.write().await.inventory.set_hotbar_slot(
                0,
                Slot {
                    item_id: VarInt(Compass::ID.try_into().unwrap()),
                    item_count: 1,
                    nbt: ItemNbt {
                        display: Some(ItemNbtDisplay {
                            name: Some(
                                r#"{"italic":false,"extra":[
                                {"color":"green","text":"Join Minigame"},
                                {"color":"gray","text":" (Right click)"}
                                ],"text":""}"#
                                    .to_string(),
                            ),
                            lore: Some(vec![
                                r#"{"text":"Click to open the minigames menu","italic":"false"}"#
                                    .to_string(),
                            ]),
                        }),
                    },
                },
            )
        }

        {
            let player = self.player.read().await;

            let container_content = SetContainerContent {
                window_id: 0,
                state_id: VarInt(0),
                items: player.inventory.slots.to_vec(),
                held_item: None,
            };

            self.connection.write_packet(container_content).await?;
        }

        let update_recipes = SetRecipes {
            recipes: Vec::new(),
        };

        self.connection.write_packet(update_recipes).await?;

        let update_tags = SetTags { tags: &TAGS };

        self.connection.write_packet(update_tags).await?;

        let node = CommandNode::new(
            NodeType::Root,
            None,
            false,
            vec![CommandNode::new(
                NodeType::Literal {
                    name: "test_command".to_string(),
                },
                None,
                false,
                vec![
                    CommandNode::new(
                        NodeType::Argument {
                            name: "true_or_false".to_string(),
                            parser: Parser::Bool,
                            suggestions_type: None,
                        },
                        None,
                        true,
                        vec![],
                    ),
                    CommandNode::new(
                        NodeType::Argument {
                            name: "0_to_1".to_string(),
                            parser: Parser::Float(FloatProps {
                                min: Some(0.0),
                                max: Some(1.0),
                            }),
                            suggestions_type: None,
                        },
                        None,
                        true,
                        vec![],
                    ),
                ],
            )],
        );

        let commands = Commands { root: node };

        self.connection.write_packet(commands).await?;

        {
            let player = self.player.read().await;
            let position_sync = SynchronizePlayerPosition {
                x: player.position.x,
                y: player.position.y,
                z: player.position.z,
                yaw: player.rotation.yaw,
                pitch: player.rotation.pitch,
                flags: PlayerPositionFlags::new(),
                teleport_id: VarInt(0),
                dismount_vehicle: false,
            };

            self.connection.write_packet(position_sync).await?;
        }

        let mut player_list = Vec::new();

        for client in server.player_list.iter() {
            let player = client.player.read().await;
            player_list.push((
                client.uuid,
                AddPlayer {
                    name: player.profile.name.clone(),
                    properties: player.profile.properties.clone(),
                },
                UpdateGameMode {
                    gamemode: VarInt::from(player.gamemode as i32),
                },
                UpdateListed { listed: true },
                UpdateLatency {
                    latency: VarInt::from(0),
                },
            ));
        }

        let player_info = PlayerInfo {
            action: PlayerInfoAction::AddAllPlayers(player_list),
        };

        self.connection.write_packet(player_info).await?;

        {
            let player = self.player.read().await;
            let set_center_chunk = SetCenterChunk {
                x: VarInt((player.position.x as i32).rem_euclid(16)),
                z: VarInt((player.position.z as i32).rem_euclid(16)),
            };
            self.connection.write_packet(set_center_chunk).await?;
        }
        for x in -7..=7 {
            for z in -7..=7 {
                let packet;
                {
                    let chunk_lock = server.world.get_chunk(x, z).await.unwrap().unwrap();
                    let chunk = chunk_lock.read().unwrap();
                    packet = chunk.into_packet();
                }
                self.connection.write_packet(packet).await?;
            }
        }

        let initialize_world_border = InitializeWorldBorder {
            x: 0.0,
            z: 0.0,
            old_diameter: 0.0,
            new_diameter: 1000000.0,
            speed: VarInt(0),
            portal_teleport_boundary: VarInt(29999984),
            warning_blocks: VarInt(5),
            warning_time: VarInt(15),
        };

        self.connection
            .write_packet(initialize_world_border)
            .await?;

        {
            let player = self.player.read().await;
            let position_sync = SynchronizePlayerPosition {
                x: player.position.x,
                y: player.position.y,
                z: player.position.z,
                yaw: 0.0,
                pitch: 0.0,
                flags: PlayerPositionFlags::new(),
                teleport_id: VarInt(0),
                dismount_vehicle: false,
            };

            self.connection.write_packet(position_sync).await?;
        }

        for client in server.player_list.iter() {
            if client.uuid == self.uuid {
                continue;
            }
            let spawn_player = {
                let player = client.player.read().await;
                let (yaw, pitch) = player.rotation.serialize();

                SpawnPlayer {
                    entity_id: VarInt(client.entity_id),
                    uuid: client.uuid,
                    x: player.position.x,
                    y: player.position.y,
                    z: player.position.z,
                    yaw,
                    pitch,
                }
            };

            self.connection.write_packet(spawn_player).await?;

            self.update_entity_metadata(
                client.entity_id,
                vec![EntityMetadata::PlayerDisplayedSkinParts(
                    DisplayedSkinPartsFlags::new()
                        .with_cape(true)
                        .with_hat(true)
                        .with_left_pants(true)
                        .with_right_pants(true)
                        .with_left_sleeve(true)
                        .with_right_sleeve(true)
                        .with_jacket(true),
                )],
            )
            .await?;
        }

        self.update_entity_metadata(
            self.entity_id,
            vec![EntityMetadata::PlayerDisplayedSkinParts(
                DisplayedSkinPartsFlags::new()
                    .with_cape(true)
                    .with_hat(true)
                    .with_left_pants(true)
                    .with_right_pants(true)
                    .with_left_sleeve(true)
                    .with_right_sleeve(true)
                    .with_jacket(true),
            )],
        )
        .await?;

        Ok(())
    }
    pub async fn handle_command<T: ServerHandler>(
        &self,
        command: ClientCommand,
        server: &Server<T>,
    ) -> Result<(), ConnectionError> {
        match command {
            ClientCommand::MoveEntity {
                entity_id,
                position,
                previous_position,
                rotation,
                previous_rotation,
                on_ground,
            } => {
                self.move_entity(
                    entity_id,
                    position,
                    previous_position,
                    rotation,
                    previous_rotation,
                    on_ground,
                )
                .await?;
            }
            ClientCommand::ChatMessage { message } => {
                self.display_message(&message).await?;
            }
            ClientCommand::Ping => {
                self.ping().await?;
            }
            ClientCommand::UpdateEntityMetadata {
                entity_id,
                metadata,
            } => {
                self.update_entity_metadata(entity_id, metadata).await?;
            }
            ClientCommand::RemovePlayer { entity_id, uuid } => {
                self.remove_player(uuid, entity_id).await?;
            }
            ClientCommand::AddPlayer {
                uuid,
                name,
                properties,
                gamemode,
                entity_id,
                position,
                rotation,
            } => {
                self.add_player(
                    uuid, name, properties, gamemode, entity_id, position, rotation,
                )
                .await?;
            }
            ClientCommand::Disconnect { reason } => {
                self.disconnect(reason.clone()).await?;
                return Err(ConnectionError::ClientDisconnected { reason });
            }
        }
        Ok(())
    }
    pub async fn handle_packet<T>(
        &self,
        packet: ServerBoundPacket,
        server: &Server<T>,
    ) -> Result<(), ConnectionError>
    where
        T: ServerHandler + Send + Sync + 'static,
    {
        // println!("Received packet: {:?}", packet);
        match packet {
            ServerBoundPacket::ServerBoundKeepAlive(_) => {
                let mut ping_acknowledged = self.ping_acknowledged.lock().await;

                if *ping_acknowledged == true {
                    println!("Client is being weird");
                }

                *ping_acknowledged = true;
            }
            ServerBoundPacket::ChatMessage(ChatMessage { message }) => {
                let gui = GameQueueMenuGui {};

                self.connection.write_packet(gui.open()).await?;

                self.connection.write_packet(gui.draw()).await?;
                *self.open_gui.lock().await = Some(GameQueueMenuGui(gui));

                server.handle_chat(self, message).await?
            }
            ServerBoundPacket::PlayerCommand(PlayerCommand {
                entity_id,
                action,
                action_parameter,
            }) => {
                if i32::from(entity_id) != self.entity_id {
                    return Ok(());
                }

                match action {
                    PlayerCommandAction::StartSprinting => {
                        server.handle_sprinting(self, true).await?
                    }
                    PlayerCommandAction::StopSprinting => {
                        server.handle_sprinting(self, false).await?
                    }
                    PlayerCommandAction::StartSneaking => {
                        server.handle_sneaking(self, true).await?
                    }
                    PlayerCommandAction::StopSneaking => {
                        server.handle_sneaking(self, false).await?
                    }
                    _ => {}
                }
            }
            ServerBoundPacket::SetPlayerPositionAndRotation(pos_rot) => {
                server
                    .handle_position_update(
                        self,
                        pos_rot.on_ground,
                        Some(Vec3::new(pos_rot.x, pos_rot.y, pos_rot.z)),
                        Some(Rotation::new(pos_rot.yaw, pos_rot.pitch)),
                    )
                    .await?;
            }
            ServerBoundPacket::SetPlayerPosition(pos) => {
                server
                    .handle_position_update(
                        self,
                        pos.on_ground,
                        Some(Vec3::new(pos.x, pos.y, pos.z)),
                        None,
                    )
                    .await?;
            }
            ServerBoundPacket::SetPlayerRotation(rot) => {
                server
                    .handle_position_update(
                        self,
                        rot.on_ground,
                        None,
                        Some(Rotation::new(rot.yaw, rot.pitch)),
                    )
                    .await?;
            }
            ServerBoundPacket::ClickContainer(click) => {
                let mut gui_lock = self.open_gui.lock().await;
                let gui = gui_lock.as_mut();

                if let Some(gui) = gui {
                    gui.handle_click(click.slot, &self, server).await?;
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
                    used_item = match player_read
                        .inventory
                        .get_hotbar_slot(used_item_slot.clone() as usize)
                    {
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

                            self.connection.write_packet(gui.open()).await?;

                            self.connection.write_packet(gui.draw()).await?;
                            *self.open_gui.lock().await = Some(GameQueueMenuGui(gui));
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
                                self.connection.write_packet(packet.unwrap()).await?;
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

                            self.player
                                .write()
                                .await
                                .inventory
                                .set_hotbar_slot(used_item_slot as usize, green_dye_slot);
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
                            self.connection.write_packet(packet).await?;
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
                            self.player
                                .write()
                                .await
                                .inventory
                                .set_hotbar_slot(used_item_slot as usize, red_dye_slot)
                        }
                        _ => {}
                    }
                }
            }
            _ => (),
        };
        Ok(())
    }
    async fn ping(&self) -> Result<(), ConnectionError> {
        {
            let mut ping_acknowledged = self.ping_acknowledged.lock().await;
            if !*ping_acknowledged {
                return Err(ConnectionError::ClientTimedOut);
            } else {
                *ping_acknowledged = false;
            }
        }
        let ping = ClientBoundKeepAlive {
            id: rand::thread_rng().gen(),
        };

        self.connection.write_packet(ping).await?;

        Ok(())
    }
    async fn display_message(&self, message: &str) -> Result<(), ConnectionError> {
        let chat_message = SystemChatMessage {
            message: message.to_string(),
            action_bar: false,
        };

        self.connection.write_packet(chat_message).await?;

        Ok(())
    }
    async fn move_entity(
        &self,
        entity_id: i32,
        position: Option<Vec3>,
        previous_position: Vec3,
        rotation: Option<Rotation>,
        previous_rotation: Rotation,
        on_ground: bool,
    ) -> Result<(), ConnectionError> {
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
                    let entity_teleport = TeleportEntity {
                        entity_id: VarInt(entity_id),
                        x: pos.x,
                        y: pos.y,
                        z: pos.z,
                        yaw,
                        pitch,
                        on_ground,
                    };
                    self.connection.write_packet(entity_teleport).await?;
                    let head_rotation = UpdateEntityHeadRotation {
                        entity_id: VarInt(entity_id),
                        yaw,
                    };
                    self.connection.write_packet(head_rotation).await?;
                } else {
                    let (yaw, pitch) = rot.serialize();
                    let (delta_x, delta_y, delta_z) =
                        (delta_x as i16, delta_y as i16, delta_z as i16);

                    let entity_move_rotate = UpdateEntityPositionAndRotation {
                        entity_id: VarInt(entity_id),
                        delta_x,
                        delta_y,
                        delta_z,
                        yaw,
                        pitch,
                        on_ground,
                    };
                    self.connection.write_packet(entity_move_rotate).await?;
                    let head_rotation = UpdateEntityHeadRotation {
                        entity_id: VarInt(entity_id),
                        yaw,
                    };
                    self.connection.write_packet(head_rotation).await?;
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
                    let entity_teleport = TeleportEntity {
                        entity_id: VarInt(entity_id),
                        x: pos.x,
                        y: pos.y,
                        z: pos.z,
                        yaw,
                        pitch,
                        on_ground,
                    };
                    self.connection.write_packet(entity_teleport).await?;
                    let head_rotation = UpdateEntityHeadRotation {
                        entity_id: VarInt(entity_id),
                        yaw,
                    };
                    self.connection.write_packet(head_rotation).await?;
                } else {
                    let (delta_x, delta_y, delta_z) =
                        (delta_x as i16, delta_y as i16, delta_z as i16);
                    let entity_move = UpdateEntityPosition {
                        entity_id: VarInt(entity_id),
                        delta_x,
                        delta_y,
                        delta_z,
                        on_ground,
                    };
                    self.connection.write_packet(entity_move).await?;
                }
            }
            (None, Some(rot)) => {
                let (yaw, pitch) = rot.serialize();

                let entity_move_rotate = UpdateEntityRotation {
                    entity_id: VarInt(entity_id),
                    yaw,
                    pitch,
                    on_ground,
                };
                self.connection.write_packet(entity_move_rotate).await?;
                let head_rotation = UpdateEntityHeadRotation {
                    entity_id: VarInt(entity_id),
                    yaw,
                };
                self.connection.write_packet(head_rotation).await?;
            }
            _ => {
                return Ok(());
            }
        };
        Ok(())
    }
    async fn update_entity_metadata(
        &self,
        entity_id: i32,
        metadata: Vec<EntityMetadata>,
    ) -> Result<(), ConnectionError> {
        let mut entity_metadata_inner = Vec::new();
        for meta in metadata {
            meta.serialize(&mut entity_metadata_inner);
        }
        entity_metadata_inner.push(END_INDEX);
        let entity_metadata = SetEntityMetadata {
            entity_id: VarInt(entity_id),
            metadata: UnsizedVec(entity_metadata_inner),
        };
        self.connection.write_packet(entity_metadata).await?;
        Ok(())
    }
    async fn remove_player(&self, uuid: u128, entity_id: i32) -> Result<(), ConnectionError> {
        let player_info = RemoveInfoPlayer {
            players: vec![uuid],
        };
        self.connection.write_packet(player_info).await?;
        let remove_entities = RemoveEntities {
            entity_ids: vec![VarInt(entity_id)],
        };
        self.connection.write_packet(remove_entities).await?;
        Ok(())
    }
    async fn add_player(
        &self,
        uuid: u128,
        name: String,
        properties: Vec<ProfileProperty>,
        gamemode: u8,
        entity_id: i32,
        position: Vec3,
        rotation: Rotation,
    ) -> Result<(), ConnectionError> {
        let player_info = PlayerInfo {
            action: PlayerInfoAction::AddSinglePlayer(
                uuid,
                AddPlayer { name, properties },
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
            entity_id: VarInt(entity_id),
            uuid,
            x: position.x,
            y: position.y,
            z: position.z,
            yaw,
            pitch,
        };

        self.connection.write_packet(player_info).await?;
        self.connection.write_packet(spawn_player).await?;

        self.update_entity_metadata(
            entity_id,
            vec![EntityMetadata::PlayerDisplayedSkinParts(
                DisplayedSkinPartsFlags::new()
                    .with_cape(true)
                    .with_hat(true)
                    .with_left_pants(true)
                    .with_right_pants(true)
                    .with_left_sleeve(true)
                    .with_right_sleeve(true)
                    .with_jacket(true),
            )],
        )
        .await?;
        Ok(())
    }
    pub(crate) async fn disconnect(&self, reason: String) -> Result<(), ConnectionError> {
        let disconnect = PlayDisconnect { reason };
        self.connection.write_packet(disconnect).await?;
        Ok(())
    }
    pub async fn set_container_content(
        &self,
        content: SetContainerContent,
    ) -> Result<(), ConnectionError> {
        self.connection.write_packet(content).await?;
        // Lmao this is test code
        self.connection
            .write_packet(SoundEffect {
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
}
