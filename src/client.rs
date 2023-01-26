use crate::server::{
    read_packet_or_err, ConnectionError, Rotation, Server, ServerHandler, ServerManager, Vec3,
};
use authentication::Profile;
use futures::{future::BoxFuture, Future};
use minecraft_data::tags::{REGISTRY, TAGS};
use protocol::{
    client_bound::{
        ChangeDifficulty, ClientBoundKeepAlive, Commands, InitializeWorldBorder, LoginWorld,
        PlayerAbilities, PlayerInfo, SetCenterChunk, SetContainerContent, SetHeldItem, SetRecipes,
        SetTags, SpawnPlayer, SynchronizePlayerPosition, SystemChatMessage, TeleportEntity,
        UpdateEntityHeadRotation, UpdateEntityPosition, UpdateEntityPositionAndRotation,
        UpdateEntityRotation,
    },
    data_types::{
        Arm, CommandNode, FloatProps, NodeType, Parser, PlayerAbilityFlags, PlayerCommandAction,
        PlayerInfoAction, PlayerInfoAddPlayer, PlayerPositionFlags, Slot,
    },
    server_bound::{ChatCommand, ChatMessage, ServerBoundPacket},
    ConnectionState, Protocol,
};
use protocol_core::{UnsizedVec, VarInt};
use rand::Rng;
use serde_json::json;
use std::{sync::Arc, time::Duration};
use tokio::{
    select,
    sync::{broadcast, Mutex, RwLock},
    time::{sleep, timeout, Timeout},
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub enum ClientEvent {
    Move {
        old_pos: Vec3,
        pos: Vec3,
        on_ground: bool,
    },
    MoveAndRotate {
        old_pos: Vec3,
        pos: Vec3,
        rotation: Rotation,
        on_ground: bool,
    },
    MoveChunk {
        x: i32,
        z: i32,
    },
    Rotation {
        rotation: Rotation,
    },
    Sprinting {
        sprinting: bool,
    },
    Sneaking {
        sneaking: bool,
    },
    SwingArm {
        hand: Arm,
    },
    ChatMessage {
        message: String,
    },
}

#[derive(Debug, Clone)]
pub enum ClientCommand {
    SpawnPlayer {
        uuid: u128,
    },
    MoveEntity {
        entity_id: i32,
        delta_x: i16,
        delta_y: i16,
        delta_z: i16,
        on_ground: bool,
        rotation: Option<Rotation>,
    },
    RotateEntity {
        entity_id: i32,
        rotation: Rotation,
        on_ground: bool,
    },
    TeleportEntity {
        entity_id: i32,
        position: Vec3,
        rotation: Rotation,
        on_ground: bool,
    },
    SendSystemMessage {
        message: String,
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
    pub from_client: Mutex<broadcast::Receiver<ClientEvent>>,
    connection: Arc<Protocol>,
    ping_acknowledged: Mutex<bool>,
}

impl Client {
    pub fn new(
        connection: Arc<Protocol>,
        player: Player,
        entity_id: i32,
        uuid: u128,
        to_client: broadcast::Sender<ClientCommand>,
        from_client: broadcast::Receiver<ClientEvent>,
    ) -> Client {
        Client {
            player: RwLock::new(player),
            connection,
            uuid,
            entity_id,
            to_client,
            from_client: Mutex::new(from_client),
            ping_acknowledged: Mutex::new(true),
        }
    }
    #[doc(hidden)]
    pub async fn read_packet(&self) -> Result<ServerBoundPacket, ConnectionError> {
        Ok(self.connection.read_and_serialize().await?)
    }
    pub async fn load_world<T: ServerHandler>(
        &self,
        server: &Server<T>,
    ) -> Result<(), ConnectionError> {
        {
            let player = self.player.read().await;
            let world_login = LoginWorld {
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

        let player_abilities = PlayerAbilities {
            flags: PlayerAbilityFlags::new()
                .with_flying(true)
                .with_allow_flying(true),
            flying_speed: 0.05,
            walking_speed: 0.1,
        };

        self.connection.write_packet(player_abilities).await?;

        read_packet_or_err!(ClientInformation, self.connection, ConnectionState::Play);

        {
            let player = self.player.read().await;
            let set_selected_slot = SetHeldItem {
                slot: player.selected_slot,
            };
            self.connection.write_packet(set_selected_slot).await?;
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
            player_list.push(PlayerInfoAddPlayer {
                uuid: player.uuid,
                name: player.profile.name.clone(),
                properties: player.profile.properties.clone(),
                gamemode: VarInt(player.gamemode as i32),
                ping: VarInt(0),
                display_name: None,
                has_signature: false,
            });
        }

        let player_info = PlayerInfo {
            action: PlayerInfoAction::AddPlayer(player_list),
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
        let start = std::time::Instant::now();
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
        println!("Chunk sending took {:?}", start.elapsed());

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

                SpawnPlayer {
                    entity_id: VarInt(client.entity_id),
                    uuid: client.uuid,
                    x: player.position.x,
                    y: player.position.y,
                    z: player.position.z,
                    yaw: 0,
                    pitch: 0,
                }
            };

            self.connection.write_packet(spawn_player).await?;
        }

        Ok(())
    }

    pub async fn handle_command<T: ServerHandler>(
        &self,
        command: ClientCommand,
        server: &Server<T>,
    ) -> Result<(), ConnectionError> {
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
        match packet {
            ServerBoundPacket::ServerBoundKeepAlive(_) => {
                let mut ping_acknowledged = self.ping_acknowledged.lock().await;

                if *ping_acknowledged == true {
                    println!("Client is being weird");
                }

                *ping_acknowledged = true;
            }
            ServerBoundPacket::ChatMessage(ChatMessage { message }) => {
                server.handle_chat(self, message).await?
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
            _ => (),
        };
        Ok(())
    }
    pub async fn ping(&self) -> Result<(), ConnectionError> {
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
    pub async fn display_message(&self, message: &str) -> Result<(), ConnectionError> {
        let chat_message = SystemChatMessage {
            message: message.to_string(),
            action_bar: false,
        };

        self.connection.write_packet(chat_message).await?;

        Ok(())
    }
    pub async fn move_entity(
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
}
//     pub async fn register_packet_listener(
//         &self,
//         server: Arc<ServerManager>,
//         tx: broadcast::Sender<ClientEvent>,
//     ) -> Result<(), ConnectionError> {
//         let ping = ClientBoundKeepAlive {
//             id: rand::thread_rng().gen(),
//         };

//         self.connection.write_packet(ping).await?;

//         loop {
//             let packet = self.connection.read_and_serialize().await?;
//             self.handle_packet(packet, server.clone(), tx.clone())
//                 .await?;
//         }
//     }
//     pub async fn register_command_listener(
//         &self,
//         server: Arc<ServerManager>,
//         mut rx: broadcast::Receiver<ClientCommand>,
//         tx: broadcast::Sender<ClientEvent>,
//         token: CancellationToken,
//     ) -> Result<(), ConnectionError> {
//         loop {
//             select! {
//                 command = rx.recv() => {
//                     let command = command?;
//                     self.handle_command(command, server.clone(), tx.clone()).await?;
//                 }
//                 _ = token.cancelled() => {
//                     for _ in 0..rx.len() {
//                         let command = rx.recv().await?;
//                         self.handle_command(command, server.clone(), tx.clone()).await?;
//                     }
//                     return Ok(());
//                 }
//             }
//         }
//     }
//     async fn handle_packet(
//         &self,
//         packet: ServerBoundPacket,
//         _server: Arc<ServerManager>,
//         tx: broadcast::Sender<ClientEvent>,
//     ) -> Result<(), ConnectionError> {
//         match packet {
//             ServerBoundPacket::SetPlayerRotation(rot) => {
//                 let mut player = self.player.write().await;
//                 player.rotation.yaw = rot.yaw;
//                 player.rotation.pitch = rot.pitch;

//                 tx.send(ClientEvent::Rotation {
//                     rotation: player.rotation.clone(),
//                 })?;
//             }
//             ServerBoundPacket::SetPlayerPosition(pos) => {
//                 if {
//                     let mut player = self.player.write().await;
//                     let moved_chunks = (player.position.x / 16.0).floor() as i32
//                         != (pos.x / 16.0).floor() as i32
//                         || (player.position.z / 16.0).floor() as i32
//                             != (pos.z / 16.0).floor() as i32;
//                     let old_pos = player.position.clone();
//                     player.position.x = pos.x;
//                     player.position.y = pos.y;
//                     player.position.z = pos.z;
//                     player.on_ground = pos.on_ground;

//                     tx.send(ClientEvent::Move {
//                         old_pos,
//                         pos: player.position.clone(),
//                         on_ground: pos.on_ground,
//                     })?;

//                     moved_chunks
//                 } {
//                     let set_center_chunk = {
//                         let player = self.player.write().await;
//                         let chunk_x = (player.position.x / 16.0).floor() as i32;
//                         let chunk_z = (player.position.z / 16.0).floor() as i32;

//                         tx.send(ClientEvent::MoveChunk {
//                             x: chunk_x,
//                             z: chunk_z,
//                         })?;

//                         SetCenterChunk {
//                             x: VarInt(chunk_x),
//                             z: VarInt(chunk_z),
//                         }
//                     };
//                     self.connection.write_packet(set_center_chunk).await?;
//                 }
//             }
//             ServerBoundPacket::SetPlayerPositionAndRotation(pos_rot) => {
//                 if {
//                     let mut player = self.player.write().await;
//                     let moved_chunks = (player.position.x / 16.0).floor() as i32
//                         != (pos_rot.x / 16.0).floor() as i32
//                         || (player.position.z / 16.0).floor() as i32
//                             != (pos_rot.z / 16.0).floor() as i32;
//                     let old_pos = player.position.clone();
//                     player.position.x = pos_rot.x;
//                     player.position.y = pos_rot.y;
//                     player.position.z = pos_rot.z;
//                     player.on_ground = pos_rot.on_ground;
//                     player.rotation.yaw = pos_rot.yaw;
//                     player.rotation.pitch = pos_rot.pitch;

//                     tx.send(ClientEvent::MoveAndRotate {
//                         old_pos,
//                         pos: player.position.clone(),
//                         rotation: player.rotation.clone(),
//                         on_ground: pos_rot.on_ground,
//                     })?;

//                     moved_chunks
//                 } {
//                     let set_center_chunk = {
//                         let player = self.player.write().await;
//                         let chunk_x = (player.position.x / 16.0).floor() as i32;
//                         let chunk_z = (player.position.z / 16.0).floor() as i32;

//                         tx.send(ClientEvent::MoveChunk {
//                             x: chunk_x,
//                             z: chunk_z,
//                         })?;

//                         SetCenterChunk {
//                             x: VarInt(chunk_x),
//                             z: VarInt(chunk_z),
//                         }
//                     };
//                     self.connection.write_packet(set_center_chunk).await?;
//                 }
//             }
//             ServerBoundPacket::ServerBoundKeepAlive(_) => {
//                 let connection = self.connection.clone();
//                 #[allow(unused_must_use)]
//                 tokio::task::spawn(async move {
//                     sleep(Duration::from_secs(15)).await;
//                     let keep_alive = ClientBoundKeepAlive {
//                         id: rand::thread_rng().gen(),
//                     };
//                     connection.write_packet(keep_alive).await;
//                 });
//             }
//             ServerBoundPacket::PlayerCommand(command) => match command.action {
//                 PlayerCommandAction::StartSneaking => {
//                     let mut player = self.player.write().await;
//                     player.sneaking = true;
//                     tx.send(ClientEvent::Sneaking { sneaking: true })?;
//                 }
//                 PlayerCommandAction::StopSneaking => {
//                     let mut player = self.player.write().await;
//                     player.sneaking = false;
//                     tx.send(ClientEvent::Sneaking { sneaking: false })?;
//                 }
//                 PlayerCommandAction::LeaveBed => todo!(),
//                 PlayerCommandAction::StartSprinting => {
//                     let mut player = self.player.write().await;
//                     player.sprinting = true;
//                     tx.send(ClientEvent::Sprinting { sprinting: true })?;
//                 }
//                 PlayerCommandAction::StopSprinting => {
//                     let mut player = self.player.write().await;
//                     player.sprinting = false;
//                     tx.send(ClientEvent::Sprinting { sprinting: false })?;
//                 }
//                 PlayerCommandAction::StartJumpWithHorse => todo!(),
//                 PlayerCommandAction::StopJumpWithHorse => todo!(),
//                 PlayerCommandAction::OpenHorseInventory => todo!(),
//                 PlayerCommandAction::StartFlyingWithElytra => {
//                     println!("Start flying with elytra");
//                 }
//             },
//             ServerBoundPacket::PlayerAbilitiesServerBound(abilities) => {
//                 let mut player = self.player.write().await;
//                 player.flying = abilities.flags.flying();
//             }
//             ServerBoundPacket::SwingArm(arm) => {
//                 tx.send(ClientEvent::SwingArm { hand: arm.arm })?;
//             }
//             ServerBoundPacket::ChatMessage(message) => {
//                 let ChatMessage { message } = message;
//                 let event = ClientEvent::ChatMessage {
//                     message: message.clone(),
//                 };
//                 tx.send(event)?;

//                 let chat_message = {
//                     let player = self.player.read().await;
//                     SystemChatMessage {
//                         action_bar: false,
//                         message: json!({
//                             "text": format!("<{}> {}", player.profile.name, message),
//                             "color": "#800000",
//                             "clickEvent": {
//                                 "action": "open_url",
//                                 "value": "https://www.google.com"
//                             },
//                             "hoverEvent": {
//                                 "action": "show_text",
//                                 "value": {
//                                     "text": "https://www.google.com",
//                                     "color": "#3abff8",
//                                     "underlined": true,
//                                 },
//                             }
//                         })
//                         .to_string(),
//                     }
//                 };
//                 self.connection.write_packet(chat_message).await?;
//             }
//             _ => {
//                 println!("Received packet: {:?}", packet);
//             }
//         }
//         Ok(())
//     }
//     async fn handle_command(
//         &self,
//         command: ClientCommand,
//         server: Arc<ServerManager>,
//         _tx: broadcast::Sender<ClientEvent>,
//     ) -> Result<(), ConnectionError> {
//         match command {
//             ClientCommand::SpawnPlayer { uuid } => {
//                 let client = server.player_list.get(&uuid).unwrap();

//                 let (player_info, spawn_player) = {
//                     let player = client.player.read().await;
//                     (
//                         PlayerInfo {
//                             action: PlayerInfoAction::AddPlayer(vec![PlayerInfoAddPlayer {
//                                 uuid: player.uuid,
//                                 name: player.profile.name.clone(),
//                                 properties: player.profile.properties.clone(),
//                                 gamemode: VarInt(player.gamemode as i32),
//                                 ping: VarInt(0),
//                                 display_name: None,
//                                 has_signature: false,
//                             }]),
//                         },
//                         SpawnPlayer {
//                             entity_id: VarInt(client.entity_id),
//                             uuid,
//                             x: player.position.x,
//                             y: player.position.y,
//                             z: player.position.z,
//                             yaw: 0,
//                             pitch: 0,
//                         },
//                     )
//                 };

//                 println!("Spawning player: {:?}", spawn_player);

//                 self.connection.write_packet(player_info).await?;
//                 self.connection.write_packet(spawn_player).await?;
//             }
//             ClientCommand::MoveEntity {
//                 entity_id,
//                 delta_x,
//                 delta_y,
//                 delta_z,
//                 on_ground,
//                 rotation,
//             } => match rotation {
//                 Some(_rotation) => {
//                     let entity_move_rotate = UpdateEntityPositionAndRotation {
//                         entity_id: VarInt(entity_id),
//                         delta_x,
//                         delta_y,
//                         delta_z,
//                         yaw: 0,
//                         pitch: 0,
//                         on_ground,
//                     };
//                     self.connection.write_packet(entity_move_rotate).await?;
//                 }
//                 None => {
//                     let entity_move_rotate = UpdateEntityPosition {
//                         entity_id: VarInt(entity_id),
//                         delta_x,
//                         delta_y,
//                         delta_z,
//                         on_ground,
//                     };
//                     self.connection.write_packet(entity_move_rotate).await?;
//                 }
//             },
//             ClientCommand::RotateEntity {
//                 entity_id,
//                 rotation: _rotation,
//                 on_ground,
//             } => {
//                 let entity_rotate = UpdateEntityRotation {
//                     entity_id: VarInt(entity_id),
//                     yaw: 0,
//                     pitch: 0,
//                     on_ground,
//                 };
//                 self.connection.write_packet(entity_rotate).await?;
//             }
//             ClientCommand::TeleportEntity {
//                 entity_id,
//                 position,
//                 rotation: _rotation,
//                 on_ground,
//             } => {
//                 let entity_teleport = TeleportEntity {
//                     entity_id: VarInt(entity_id),
//                     x: position.x,
//                     y: position.y,
//                     z: position.z,
//                     yaw: 0,
//                     pitch: 0,
//                     on_ground,
//                 };
//                 self.connection.write_packet(entity_teleport).await?;
//             }
//             ClientCommand::SendSystemMessage { message } => {
//                 let chat_message = SystemChatMessage {
//                     message: message,
//                     action_bar: false,
//                 };

//                 println!("Sending system message: {:?}", chat_message);

//                 self.connection.write_packet(chat_message).await?;
//             } // command => {
//               //     println!("Received command: {:?}", command);
//               // }
//         };
//         Ok(())
//     }
// }
