use std::sync::Arc;
use std::thread;
use std::time::Duration;

use authentication::{authenticate, AuthenticationError};
use dashmap::mapref::entry::Entry::Vacant;
use dashmap::DashMap;
use minecraft_data::tags::{REGISTRY, TAGS};
use protocol::client_bound::{
    ChangeDifficulty, ClientBoundKeepAlive, EncryptionRequest, InitializeWorldBorder,
    LoginDisconnect, LoginSuccess, LoginWorld, PlayDisconnect, PlayerAbilities, Pong, ServerStatus,
    SetCenterChunk, SetCompression, SetHeldItem, SetRecipes, SetTags, SynchronizePlayerPosition,
};
use protocol::data_types::{PlayerAbilityFlags, PlayerPositionFlags, TestBytes, VarInt};
use protocol::protocol::{ConnectionState, Protocol, ProtocolError};
use protocol::server_bound::ServerBoundPacket;
use rand::Rng;
use rsa::{PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use thiserror::Error;
use tokio::sync::{broadcast, mpsc};
use tokio::time::sleep;
use tokio::{net::TcpListener, select};
use tokio_util::sync::CancellationToken;
use world::world::World;

macro_rules! read_packet_or_err {
    ($packet:ident, $stream:expr, $connection_state:expr) => {
        match $stream.read_and_serialize().await {
            Ok(ServerBoundPacket::$packet(param)) => param,
            Ok(packet) => {
                let error = ConnectionError::UnexpectedPacket {
                    expected: stringify!($packet),
                    got: format!("{:?}", packet),
                    state: stringify!($connection_state),
                };
                match $connection_state {
                    ConnectionState::Login => {
                        let disconnect = LoginDisconnect {
                            reason: format!(r#"{{"text": "{}"}}"#, error),
                        };
                        $stream.write_packet(disconnect).await.unwrap();
                        return Err(error);
                    }
                    ConnectionState::Play => {
                        let disconnect = PlayDisconnect {
                            reason: format!(r#"{{"text": "{}"}}"#, error),
                        };
                        $stream.write_packet(disconnect).await.unwrap();
                        return Err(error);
                    }
                    _ => return Err(error),
                }
            }
            Err(err) => {
                return Err(ConnectionError::ProtocolError(err));
            }
        }
    };
}

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Protocol error: {0}")]
    ProtocolError(#[from] ProtocolError),
    #[error("Unexpected next connection state: {0:?}")]
    UnexpectedNextState(ConnectionState),
    #[error("Invalid shared secret length, expected 16, got {0}")]
    InvalidSharedSecretLength(usize),
    #[error("Unexpected packet: expected {expected}, got {got} in state {state}")]
    UnexpectedPacket {
        got: String,
        expected: &'static str,
        state: &'static str,
    },
    #[error("Authentication error: {0}")]
    AuthenticationError(#[from] AuthenticationError),
    #[error("Player unregistered")]
    PlayerUnregisteredError,
}

pub struct ServerManager {
    token: CancellationToken,
}

pub struct VanillaServerHandler {
    world: World,
    player_list: DashMap<i32, Player>,
}
#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
}

#[derive(Debug)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Rotation {
    pub fn new(yaw: f32, pitch: f32) -> Rotation {
        Rotation { yaw, pitch }
    }
}

#[derive(Debug)]
pub struct Player {
    pub uuid: u128,
    pub player_name: String,
    pub gamemode: u8,
    pub previous_gamemode: i8,
    pub reduced_debug_info: bool,
    pub selected_slot: u8,
    pub position: Vec3,
    pub rotation: Rotation,
    pub rx: Option<mpsc::Sender<ClientCommand>>,
    pub tx: Option<broadcast::Receiver<ClientEvent>>,
}

impl Player {
    fn new(uuid: u128, player_name: String) -> Player {
        Player {
            uuid,
            player_name,
            gamemode: 0,
            previous_gamemode: 0,
            reduced_debug_info: false,
            selected_slot: 0,
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Rotation::new(0.0, 0.0),
            rx: None,
            tx: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClientEvent {
    ClientDisconnect,
    Move,
    MoveChunk,
}

#[derive(Debug)]
pub enum ClientCommand {}

impl VanillaServerHandler {
    fn new() -> Self {
        let mut test = VanillaServerHandler {
            world: World::new("./world/region/"),
            player_list: DashMap::new(),
        };
        tokio::task::spawn(async move {
            test.world.increase_block_light(20, 120, 19, 15).await;
            test.world.increase_block_light(20, 256, 19, 15).await;
            test.world.increase_block_light(20, 73, 19, 15).await;
            test.world.increase_block_light(20, 73, 19, 15).await;
        });
        VanillaServerHandler {
            world: World::new("./world/region/"),
            player_list: DashMap::new(),
        }
    }
    async fn on_connect(
        &self,
        mut connection: Protocol,
        encryption: Arc<Encryption>,
    ) -> Result<(), ConnectionError> {
        let handshake = read_packet_or_err!(Handshake, connection, ConnectionState::HandShaking);

        connection.connection_state = handshake.next_state;

        if handshake.next_state == ConnectionState::Status {
            self.handle_ping(connection).await?;
            return Ok(());
        }

        // The client might have set the connection state to Play or Status which is not allowed
        if connection.connection_state != ConnectionState::Login {
            return Err(ConnectionError::UnexpectedNextState(
                connection.connection_state,
            ));
        }

        self.handle_login_start(connection, encryption).await?;

        Ok(())
    }
    async fn handle_ping(&self, connection: Protocol) -> Result<(), ConnectionError> {
        let _ = read_packet_or_err!(StatusRequest, connection, ConnectionState::Status);

        let motd = DefaultMOTD {
            favicon: "data:image/png;base64,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".to_string(),
            previews_chat: false,
            enforces_secure_chat: true,
            description: r#"                  \u00A7dRust Network \u00A75\u00A7l[1.19.2]\u00A7r\n            \u00A76\u00A7lNOW IN PRE\u00A7f\u00A7l-\u00A76\u00A7lPRE\u00A7f\u00A7l-\u00A76\u00A7lPRE\u00A7f\u00A7l-\u00A76\u00A7lALPHA!"#.to_string(),
            max_players: 100,
            online_players: 0,
            version_name: "1.19.2".to_string(),
            version_protocol: 760,
        }
        .to_string();
        let server_status = ServerStatus { response: motd };

        connection.write_packet(server_status).await?;

        let ping_request = read_packet_or_err!(Ping, connection, ConnectionState::Status);

        let pong = Pong {
            payload: ping_request.payload,
        };
        connection.write_packet(pong).await?;
        return Ok(());
    }
    async fn handle_login_start(
        &self,
        mut connection: Protocol,
        encryption: Arc<Encryption>,
    ) -> Result<(), ConnectionError> {
        let login_start = read_packet_or_err!(LoginStart, connection, ConnectionState::Login);

        let client_username = login_start.name;

        let encryption_request = EncryptionRequest {
            server_id: "".to_string(), // deprecated after 1.7
            public_key: encryption.encoded_pub.clone(),
            verify_token: Vec::new(),
        };
        connection.write_packet(encryption_request).await.unwrap();

        let encryption_response =
            read_packet_or_err!(EncryptionResponse, connection, ConnectionState::Login);

        let shared_secret = encryption
            .priv_key
            .decrypt(
                rsa::PaddingScheme::PKCS1v15Encrypt,
                encryption_response.shared_secret.as_slice(),
            )
            .unwrap();

        if shared_secret.len() != 16usize {
            return Err(ConnectionError::InvalidSharedSecretLength(
                shared_secret.len(),
            ));
        }

        let profile = match authenticate(
            shared_secret.as_slice(),
            encryption.encoded_pub.as_slice(),
            client_username,
        )
        .await
        {
            Ok(profile) => profile,
            Err(err) => {
                let disconnect = LoginDisconnect {
                    reason: format!(r#"{{"text": "{}"}}"#, err),
                };
                connection.write_packet(disconnect).await.unwrap();
                return Err(ConnectionError::AuthenticationError(err));
            }
        };

        connection
            .enable_encryption(shared_secret.as_slice(), shared_secret.as_slice())
            .await;

        let set_compression = SetCompression {
            threshold: VarInt(0),
        };

        connection.write_packet(set_compression).await.unwrap();

        connection.enable_compression();
        println!("Compression enabled");

        let uuid = u128::from_str_radix(&profile.id, 16).unwrap();

        let login_success = LoginSuccess {
            uuid: uuid.clone(),
            username: profile.name.clone(),
            properties: profile.properties,
        };

        connection.write_packet(login_success).await.unwrap();
        let entity_id = self.add_client(uuid, profile.name)?;

        self.handle_world_loading(connection, entity_id).await?;

        self.remove_client(entity_id);
        Ok(())
    }
    async fn handle_world_loading(
        &self,
        mut connection: Protocol,
        entity_id: i32,
    ) -> Result<(), ConnectionError> {
        {
            let player = self
                .player_list
                .get(&entity_id)
                .ok_or_else(|| ConnectionError::PlayerUnregisteredError)?;
            let world_login = LoginWorld {
                entity_id: entity_id.clone(),
                is_hardcore: false,
                game_mode: player.gamemode,
                previous_game_mode: player.previous_gamemode,
                dimensions: vec![
                    "minecraft:overworld".to_string(),
                    "minecraft:the_nether".to_string(),
                    "minecraft:the_end".to_string(),
                ],
                registry_codec: TestBytes(REGISTRY.clone()),
                dimension_type: "minecraft:overworld".to_string(),
                dimension_name: "minecraft:overworld".to_string(),
                hashed_seed: 0,
                max_players: VarInt(10),
                view_distance: VarInt(5),
                simulation_distance: VarInt(5),
                reduced_debug_info: player.reduced_debug_info,
                enable_respawn_screen: true,
                is_debug: false,
                is_flat: false,
                death_location: None,
            };
            connection.write_packet(world_login).await.unwrap();
        }

        connection.connection_state = ConnectionState::Play;

        let change_difficulty = ChangeDifficulty {
            difficulty: *self.world.difficulty.read().unwrap(),
            locked: *self.world.difficulty_locked.read().unwrap(),
        };

        connection.write_packet(change_difficulty).await.unwrap();

        let player_abilities = PlayerAbilities {
            flags: PlayerAbilityFlags::new()
                .with_flying(true)
                .with_allow_flying(true),
            flying_speed: 0.05,
            walking_speed: 0.1,
        };
        connection.write_packet(player_abilities).await.unwrap();

        let _ = read_packet_or_err!(ClientInformation, connection, ConnectionState::Play);

        {
            let player = self
                .player_list
                .get(&entity_id)
                .expect("Failed to insert client");
            let set_selected_slot = SetHeldItem {
                slot: player.selected_slot,
            };
            connection.write_packet(set_selected_slot).await.unwrap();
        }

        let update_recipes = SetRecipes {
            recipes: Vec::new(),
        };

        connection.write_packet(update_recipes).await.unwrap();

        let update_tags = SetTags { tags: &TAGS };

        connection.write_packet(update_tags).await.unwrap();

        {
            let player = self
                .player_list
                .get(&entity_id)
                .expect("Failed to insert client");
            let set_center_chunk = SetCenterChunk {
                x: VarInt((player.position.x as i32).rem_euclid(16)),
                z: VarInt((player.position.z as i32).rem_euclid(16)),
            };
            connection.write_packet(set_center_chunk).await.unwrap();
        }
        let start = std::time::Instant::now();
        for x in -3..=3 {
            for z in -3..=3 {
                let packet;
                {
                    let start = std::time::Instant::now();
                    let chunk_lock = self.world.get_chunk(x, z).await.unwrap().unwrap();
                    let chunk = chunk_lock.read().unwrap();
                    packet = chunk.into_packet();
                    println!("Time to get chunk {:?}", start.elapsed());
                }
                connection.write_packet(packet).await.unwrap();
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

        connection
            .write_packet(initialize_world_border)
            .await
            .unwrap();

        {
            let mut player = self
                .player_list
                .get_mut(&entity_id)
                .expect("Failed to insert client");

            player.position.x = 0.0;
            player.position.y = 100.0;
            player.position.z = 0.0;
        }

        {
            let player = self
                .player_list
                .get(&entity_id)
                .ok_or_else(|| ConnectionError::PlayerUnregisteredError)?;
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

            connection.write_packet(position_sync).await.unwrap();
        }

        self.register_client(connection, entity_id).await?;
        Ok(())
    }
    async fn register_client(
        &self,
        connection: Protocol,
        entity_id: i32,
    ) -> Result<(), ConnectionError> {
        let ping = ClientBoundKeepAlive {
            id: rand::thread_rng().gen(),
        };

        connection.write_packet(ping).await.unwrap();

        {
            let chunk_locked = self
                .world
                .get_chunk_from_pos(-3, 32)
                .await
                .unwrap()
                .unwrap();
            let chunk_lock = chunk_locked.read().unwrap();

            let block = chunk_lock.get_block(-3, 76, 32);
            println!("Block: {:?}, {}", block, block.unwrap().get_emit_light());
        }

        let (tx, rx) = broadcast::channel::<ClientEvent>(10);

        let connection = Arc::new(connection);
        let token = CancellationToken::new();
        loop {
            select! {
                packet = connection.read_and_serialize() => async {
                    // Add type annotation for rust analyzer
                    let packet: Result<ServerBoundPacket, ProtocolError> = packet;
                    let packet = match packet {
                        Ok(packet) => packet,
                        Err(err) => {
                            let disconnect = PlayDisconnect {
                                reason: format!(r#"{{"text": "Error: {}"}}"#, err),
                            };
                            connection.write_packet(disconnect).await.unwrap();
                            token.cancel();
                            return Err(ConnectionError::ProtocolError(err));
                        }
                    };
                    self.handle_packet(packet, connection.clone(), entity_id, token.clone()).await
                }.await?
            };
        }
    }
    fn get_lowest_entity_id(&self) -> Option<i32> {
        for i in 0..i32::MAX {
            if !self.player_list.contains_key(&i) {
                return Some(i);
            }
        }
        None
    }
    fn add_client(&self, uuid: u128, username: String) -> Result<i32, ConnectionError> {
        let entity_id = self
            .get_lowest_entity_id()
            .expect("Could not get an id for player");

        let player_arc = Player::new(uuid, username);
        let entry = self.player_list.entry(entity_id.clone());
        let Vacant(e) = entry else {
            panic!("Entity already exists");
        };
        e.insert(player_arc);
        Ok(entity_id)
    }
    fn remove_client(&self, entity_id: i32) {
        self.player_list.remove(&entity_id);
    }
    async fn handle_packet(
        &self,
        packet: ServerBoundPacket,
        connection: Arc<Protocol>,
        entity_id: i32,
        token: CancellationToken,
    ) -> Result<(), ConnectionError> {
        match packet {
            ServerBoundPacket::SetPlayerRotation(rot) => {
                let mut player = self
                    .player_list
                    .get_mut(&entity_id)
                    .ok_or_else(|| ConnectionError::PlayerUnregisteredError)?;

                player.rotation.yaw = rot.yaw;
                player.rotation.pitch = rot.pitch;
            }
            ServerBoundPacket::SetPlayerPosition(pos) => {
                let moved_chunks;
                {
                    let mut player = self
                        .player_list
                        .get_mut(&entity_id)
                        .ok_or_else(|| ConnectionError::PlayerUnregisteredError)?;

                    moved_chunks = (player.position.x / 16.0).floor() as i32
                        != (pos.x / 16.0).floor() as i32
                        || (player.position.z / 16.0).floor() as i32
                            != (pos.z / 16.0).floor() as i32;
                    player.position.x = pos.x;
                    player.position.y = pos.y;
                    player.position.z = pos.z;
                }
                if moved_chunks {
                    let player = self
                        .player_list
                        .get(&entity_id)
                        .ok_or_else(|| ConnectionError::PlayerUnregisteredError)?;
                    let set_center_chunk = SetCenterChunk {
                        x: VarInt((player.position.x / 16.0).floor() as i32),
                        z: VarInt((player.position.z / 16.0).floor() as i32),
                    };
                    println!("Sending set center chunk {:?}", set_center_chunk);
                    connection.write_packet(set_center_chunk).await.unwrap();
                }
            }
            ServerBoundPacket::SetPlayerPositionAndRotation(pos_rot) => {
                let moved_chunks;
                {
                    let mut player = self
                        .player_list
                        .get_mut(&entity_id)
                        .ok_or_else(|| ConnectionError::PlayerUnregisteredError)?;

                    moved_chunks = (player.position.x / 16.0).floor() as i32
                        != (pos_rot.x / 16.0).floor() as i32
                        || (player.position.z / 16.0).floor() as i32
                            != (pos_rot.z / 16.0).floor() as i32;
                    player.position.x = pos_rot.x;
                    player.position.y = pos_rot.y;
                    player.position.z = pos_rot.z;
                    player.rotation.yaw = pos_rot.yaw;
                    player.rotation.pitch = pos_rot.pitch;
                }
                if moved_chunks {
                    let player = self
                        .player_list
                        .get(&entity_id)
                        .ok_or_else(|| ConnectionError::PlayerUnregisteredError)?;
                    let set_center_chunk = SetCenterChunk {
                        x: VarInt((player.position.x / 16.0).floor() as i32),
                        z: VarInt((player.position.z / 16.0).floor() as i32),
                    };

                    println!("Sending set center chunk {:?}", set_center_chunk);
                    connection.write_packet(set_center_chunk).await.unwrap();
                }
            }
            ServerBoundPacket::ServerBoundKeepAlive(_) => {
                let connection = Arc::clone(&connection);
                let cloned_token = token.clone();
                #[allow(unused_must_use)]
                tokio::task::spawn(async move {
                    select! {
                        _ = cloned_token.cancelled() => (),
                        _ = sleep(Duration::from_secs(15)) => {
                            let keep_alive = ClientBoundKeepAlive {
                                id: rand::thread_rng().gen(),
                            };
                            connection.write_packet(keep_alive).await;
                        }
                    }
                });
            }
            _ => {
                println!("Received packet: {:?}", packet);
            }
        };
        Ok(())
    }
}
pub struct Encryption {
    pub pub_key: RsaPublicKey,
    pub priv_key: RsaPrivateKey,
    pub encoded_pub: Vec<u8>,
}
impl Encryption {
    pub fn new() -> Encryption {
        const BITS: usize = 1024;

        let mut rng = rand::thread_rng();

        let priv_key = RsaPrivateKey::new(&mut rng, BITS).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        let pub_encoded_bytes =
            rsa_der::public_key_to_der(&pub_key.n().to_bytes_be(), &pub_key.e().to_bytes_be());

        Encryption {
            pub_key: pub_key,
            priv_key: priv_key,
            encoded_pub: pub_encoded_bytes,
        }
    }
}

impl ServerManager {
    pub fn new() -> Arc<ServerManager> {
        let token = CancellationToken::new();

        let server = Arc::new(ServerManager { token });
        let server_handler = Arc::new(VanillaServerHandler::new());
        let encryption = Arc::new(Encryption::new());

        let cloned_server = server.clone();
        tokio::task::spawn(async move {
            let token = cloned_server.token.clone();
            async fn bind_server(
                token: CancellationToken,
                server: Arc<VanillaServerHandler>,
                encryption: Arc<Encryption>,
            ) {
                let listener = TcpListener::bind("127.0.0.1:25566").await.unwrap();
                loop {
                    let (stream, _socket_address) = listener.accept().await.unwrap();

                    let connection = Protocol::new(stream);
                    let cloned_token = token.clone();
                    let cloned_handler = server.clone();
                    let cloned_encryption = encryption.clone();
                    tokio::task::spawn(async move {
                        select! {
                            _ = cloned_token.cancelled() => (),
                            _ = cloned_handler.on_connect(connection, cloned_encryption) => ()
                        }
                    });
                }
            }
            select! {
                _ = token.cancelled() => (),
                _ = bind_server(token.clone(), server_handler, encryption) => {
                    panic!("The TCP listener shut down???")
                }
            }
        });

        server
    }
}

impl Drop for ServerManager {
    fn drop(&mut self) {
        self.token.cancel();
    }
}

pub trait MOTD {
    fn motd(server: Arc<ServerManager>) -> String;
}

#[derive(Debug)]
pub struct DefaultMOTD {
    favicon: String,
    previews_chat: bool,
    enforces_secure_chat: bool,
    description: String,
    max_players: u32,
    online_players: u32,
    version_name: String,
    version_protocol: u32,
}

impl ToString for DefaultMOTD {
    fn to_string(&self) -> String {
        format!(
            "{{\"favicon\": \"{}\", \"previewsChat\":{},\"enforcesSecureChat\":{},\"description\":{{\"text\":\"{}\"}},\"players\":{{\"max\":{},\"online\":{}}},\"version\":{{\"name\":\"{}\",\"protocol\":{}}}}}",
            self.favicon,
            self.previews_chat,
            self.enforces_secure_chat,
            self.description,
            self.max_players,
            self.online_players,
            self.version_name,
            self.version_protocol
        )
    }
}
