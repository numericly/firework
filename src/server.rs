use async_trait::async_trait;
use authentication::{authenticate, AuthenticationError};
use dashmap::DashMap;
use nbt::Blob;
use protocol::client_bound::{
    EncryptionRequest, LoginDisconnect, LoginSuccess, Pong, ServerStatus, SetCompression,
    UnloadChunk,
};
use protocol::data_types::Slot;
use protocol::server_bound::Ping;
use protocol::{ConnectionState, Protocol, ProtocolError};
use protocol_core::VarInt;
use rsa::{PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use std::borrow::BorrowMut;
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::vec;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::{broadcast, Mutex};
use tokio_util::sync::CancellationToken;
use world::World;

macro_rules! read_packet_or_err {
    ($packet:ident, $stream:expr, $connection_state:expr) => {
        match $stream.read_and_serialize().await {
            Ok(protocol::server_bound::ServerBoundPacket::$packet(param)) => param,
            Ok(packet) => {
                let error = crate::server::ConnectionError::UnexpectedPacket {
                    expected: stringify!($packet),
                    got: format!("{:?}", packet),
                    state: stringify!($connection_state),
                };
                match $connection_state {
                    ConnectionState::Login => {
                        let disconnect = protocol::client_bound::LoginDisconnect {
                            reason: format!(r#"{{"text": "{}"}}"#, error),
                        };
                        $stream.write_packet(disconnect).await.unwrap();
                        return Err(error);
                    }
                    ConnectionState::Play => {
                        let disconnect = protocol::client_bound::PlayDisconnect {
                            reason: format!(r#"{{"text": "{}"}}"#, error),
                        };
                        $stream.write_packet(disconnect).await.unwrap();
                        return Err(error);
                    }
                    _ => return Err(error),
                }
            }
            Err(err) => {
                return Err(crate::server::ConnectionError::ProtocolError(err));
            }
        }
    };
}

pub(crate) use read_packet_or_err;

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
    #[error("Recv error: {0}")]
    RecvError(#[from] broadcast::error::RecvError),
    #[error("client command send error: {0}")]
    ClientCommandSendError(#[from] broadcast::error::SendError<ClientEvent>),
    #[error("Send error: {0}")]
    ServerCommandSendError(#[from] broadcast::error::SendError<ServerCommand>),
    #[error("Client is already connected")]
    ClientAlreadyConnected,
}

#[derive(Clone, Debug)]
pub enum Entity {
    Player(u128),
}

pub struct ServerManager {
    pub world: Arc<World>,
    pub entities: Arc<DashMap<i32, Entity>>,
    pub player_list: Arc<DashMap<u128, Client>>,
    encryption: Arc<Encryption>,
    lowest_free_id: Mutex<i32>,

    token: CancellationToken,
}

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
}

#[derive(Debug, Clone)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Rotation {
    pub fn new(yaw: f32, pitch: f32) -> Rotation {
        Rotation { yaw, pitch }
    }
}

#[async_trait]
pub trait Server {
    type Error: Error + Send + Sync + 'static;
    async fn created(manager: &ServerManager) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone)]
pub enum ServerCommand {
    SpawnPlayer { uuid: u128 },
    RemovePlayer { entity_id: i32, uuid: u128 },
}

const SERVER_MOTD: DefaultMOTD = DefaultMOTD {
    favicon: "data:image/png;base64,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",
    previews_chat: false,
    enforces_secure_chat: true,
    description: r#"                  \u00A7dRust Network \u00A75\u00A7l[1.19.2]\u00A7r\n            \u00A76\u00A7lNOW IN PRE\u00A7f\u00A7l-\u00A76\u00A7lPRE\u00A7f\u00A7l-\u00A76\u00A7lPRE\u00A7f\u00A7l-\u00A76\u00A7lALPHA!"#,
    max_players: 100,
    version_name: "1.19.2",
    version_protocol: 760,
};
const PORT: u16 = 25566;

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
            pub_key,
            priv_key,
            encoded_pub: pub_encoded_bytes,
        }
    }
}

impl ServerManager {
    pub fn run() {
        let token = CancellationToken::new();

        let (sender, receiver) = broadcast::channel(10);

        let encryption = Arc::new(Encryption::new());

        let server = Arc::new(ServerManager {
            world: Arc::new(World::new("./world/region/")),
            entities: Arc::new(DashMap::new()),
            player_list: Arc::new(DashMap::new()),
            lowest_free_id: Mutex::new(0),
            token,
            encryption,
        });

        let cloned_server = server.clone();
        let cloned_sender = sender.clone();

        tokio::task::spawn(async move {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT))
                .await
                .expect("Failed to bind ser to port");
            println!("Server started listening on port: {}", PORT);
            loop {
                let (stream, _socket_address) = listener.accept().await.unwrap();

                let connection = Protocol::new(stream);
                let server = cloned_server.clone();
                let sender = cloned_sender.clone();
                tokio::task::spawn(async move {
                    let res = server.handle_connection(connection, sender).await;
                    if let Err(e) = res {
                        println!("{}", e);
                    }
                });
            }
        });

        let cloned_server = server.clone();
        tokio::task::spawn(async move {
            cloned_server.tick_loop(receiver).await;
        });
    }
    async fn handle_connection(
        self: Arc<Self>,
        mut connection: Protocol,
        sender: broadcast::Sender<ServerCommand>,
    ) -> Result<(), ConnectionError> {
        let handshake = read_packet_or_err!(Handshake, connection, ConnectionState::HandShaking);

        match handshake.next_state {
            // Handle server ping
            ConnectionState::Status => {
                {
                    *connection.connection_state.write().await = ConnectionState::Status;
                }
                read_packet_or_err!(StatusRequest, connection, ConnectionState::Status);

                let motd = SERVER_MOTD.motd(&self);

                let server_status = ServerStatus { motd };

                connection.write_packet(server_status).await?;

                let Ping { payload, .. } =
                    read_packet_or_err!(Ping, connection, ConnectionState::Status);

                let pong = Pong { payload };
                connection.write_packet(pong).await?;

                Ok(())
            }
            // Handle login sequence
            ConnectionState::Login => {
                async fn handle_login(
                    connection: &mut Protocol,
                    encryption: Arc<Encryption>,
                ) -> Result<Player, ConnectionError> {
                    let login_start =
                        read_packet_or_err!(LoginStart, connection, ConnectionState::Login);

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

                    let uuid = u128::from_str_radix(&profile.id, 16).unwrap();

                    let login_success = LoginSuccess {
                        uuid: uuid.clone(),
                        username: profile.name.clone(),
                        properties: profile.properties.clone(),
                    };

                    connection.write_packet(login_success).await.unwrap();

                    let player = Player::new(uuid, profile);
                    Ok(player)
                }

                {
                    *connection.connection_state.write().await = ConnectionState::Login;
                }

                let mut player = handle_login(&mut connection, self.encryption.clone()).await?;

                player.inventory.set_hotbar_slot(
                    4,
                    Slot {
                        item_id: VarInt(833),
                        item_count: 1,
                        nbt: Blob::new(),
                    },
                );

                player.inventory.set_armor_slot(
                    1,
                    Slot {
                        item_id: VarInt(697),
                        item_count: 1,
                        nbt: Blob::new(),
                    },
                );

                let (to_client_sender, to_client_receiver) =
                    broadcast::channel::<ClientCommand>(10);
                let (from_client_sender, from_client_receiver) =
                    broadcast::channel::<ClientEvent>(10);

                let client = {
                    let mut lowest_id = self.lowest_free_id.lock().await;
                    let uuid = player.uuid.clone();

                    if self.player_list.get(&uuid).is_some() {
                        return Err(ConnectionError::ClientAlreadyConnected);
                    }

                    let client = Client::new(
                        connection,
                        player,
                        *lowest_id,
                        uuid.clone(),
                        to_client_sender,
                        from_client_receiver,
                    );

                    self.player_list.insert(client.uuid.clone(), client);

                    let client = self
                        .player_list
                        .get(&uuid)
                        .ok_or(ConnectionError::ClientAlreadyConnected)?;

                    self.entities.insert(*lowest_id, Entity::Player(uuid));

                    for _ in *lowest_id..i32::MAX {
                        if self.entities.contains_key(&lowest_id) {
                            *lowest_id += 1;
                        } else {
                            break;
                        }
                    }

                    client
                };

                sender.send(ServerCommand::SpawnPlayer { uuid: client.uuid })?;

                client.load_world(self.as_ref()).await?;

                println!("Loaded world");

                let token = CancellationToken::new();

                let cloned_server = self.clone();
                let cloned_uuid = client.uuid.clone();
                let cloned_from_client_sender = from_client_sender.clone();
                let cloned_token = token.clone();

                let command_listener = tokio::task::spawn(async move {
                    let client = cloned_server.player_list.get(&cloned_uuid).unwrap();

                    client
                        .register_command_listener(
                            cloned_server.clone(),
                            to_client_receiver,
                            cloned_from_client_sender,
                            cloned_token,
                        )
                        .await
                        .unwrap();
                });

                // client
                //     .to_client
                //     .send(ClientCommand::SendSystemMessage {
                //         message: r#"{"text": "Welcome to the server!"}"#.to_string(),
                //     })
                //     .unwrap();

                if let Err(err) = client
                    .register_packet_listener(self.clone(), from_client_sender.clone())
                    .await
                {
                    let uuid = client.uuid.clone();
                    let entity_id = client.entity_id.clone();
                    drop(client);
                    token.cancel();
                    command_listener.await.unwrap();
                    sender.send(ServerCommand::RemovePlayer { uuid, entity_id })?;
                    self.player_list.remove(&uuid);
                    return Err(err);
                }

                Ok(())
            }
            // If the client tried to change state to the current state or play state which is not allowed
            state => Err(ConnectionError::UnexpectedNextState(state)),
        }
    }
    async fn tick_loop(&self, mut rx: broadcast::Receiver<ServerCommand>) {
        const TICK_DURATION: Duration = Duration::from_millis(50);
        let mut last_tick = Instant::now();
        let mut ticks = -1;

        loop {
            ticks += 1;
            // Tick logic
            {
                let now = Instant::now();
                let delta = now - last_tick;
                if delta < TICK_DURATION {
                    tokio::time::sleep(TICK_DURATION - delta).await;
                }
                last_tick = Instant::now();
            }

            for _ in 0..rx.len() {
                let command = rx.recv().await.unwrap();

                match command {
                    ServerCommand::SpawnPlayer { uuid } => {
                        let client = self.player_list.get(&uuid);

                        let Some(client) = client else {
                            continue;
                        };

                        for other_client in self.player_list.iter() {
                            if other_client.uuid == client.uuid {
                                continue;
                            }

                            other_client
                                .to_client
                                .send(ClientCommand::SpawnPlayer {
                                    uuid: client.uuid.clone(),
                                })
                                .unwrap();
                        }
                    }
                    ServerCommand::RemovePlayer {
                        uuid,
                        entity_id: _entity_id,
                    } => {
                        for client in self.player_list.iter() {
                            if client.uuid == uuid {
                                continue;
                            }
                        }
                    }
                }
            }

            for client in self.player_list.iter() {
                let mut from_client = client.from_client.lock().await;

                for _ in 0..from_client.len() {
                    let event = from_client.recv().await.unwrap();

                    match event {
                        ClientEvent::Move {
                            old_pos,
                            pos,
                            on_ground,
                        } => {
                            for other_client in self.player_list.iter() {
                                if other_client.uuid == client.uuid {
                                    continue;
                                }

                                let pos = pos.clone();

                                let (delta_x, delta_y, delta_z) = (
                                    ((pos.x * 32. - old_pos.x * 32.) * 128.),
                                    ((pos.y * 32. - old_pos.y * 32.) * 128.),
                                    ((pos.z * 32. - old_pos.z * 32.) * 128.),
                                );

                                if (delta_x < i16::MIN as f64 || delta_x > i16::MAX as f64)
                                    || (delta_y < i16::MIN as f64 || delta_y > i16::MAX as f64)
                                    || (delta_z < i16::MIN as f64 || delta_z > i16::MAX as f64)
                                {
                                    let player = client.player.read().await;
                                    other_client
                                        .to_client
                                        .send(ClientCommand::TeleportEntity {
                                            entity_id: client.entity_id,
                                            position: pos,
                                            rotation: player.rotation.clone(),
                                            on_ground,
                                        })
                                        .unwrap();
                                } else {
                                    other_client
                                        .to_client
                                        .send(ClientCommand::MoveEntity {
                                            entity_id: client.entity_id,
                                            delta_x: delta_x as i16,
                                            delta_y: delta_y as i16,
                                            delta_z: delta_z as i16,
                                            rotation: None,
                                            on_ground,
                                        })
                                        .unwrap();
                                }
                            }
                        }
                        ClientEvent::MoveAndRotate {
                            old_pos,
                            pos,
                            rotation,
                            on_ground,
                        } => {
                            for other_client in self.player_list.iter() {
                                if other_client.uuid == client.uuid {
                                    continue;
                                }

                                let pos = pos.clone();

                                let (delta_x, delta_y, delta_z) = (
                                    ((pos.x * 32. - old_pos.x * 32.) * 128.),
                                    ((pos.y * 32. - old_pos.y * 32.) * 128.),
                                    ((pos.z * 32. - old_pos.z * 32.) * 128.),
                                );

                                if (delta_x < i16::MIN as f64 || delta_x > i16::MAX as f64)
                                    || (delta_y < i16::MIN as f64 || delta_y > i16::MAX as f64)
                                    || (delta_z < i16::MIN as f64 || delta_z > i16::MAX as f64)
                                {
                                    other_client
                                        .to_client
                                        .send(ClientCommand::TeleportEntity {
                                            entity_id: client.entity_id,
                                            position: pos,
                                            rotation: rotation.clone(),
                                            on_ground,
                                        })
                                        .unwrap();
                                } else {
                                    other_client
                                        .to_client
                                        .send(ClientCommand::MoveEntity {
                                            entity_id: client.entity_id,
                                            delta_x: delta_x as i16,
                                            delta_y: delta_y as i16,
                                            delta_z: delta_z as i16,
                                            rotation: Some(rotation.clone()),
                                            on_ground,
                                        })
                                        .unwrap();
                                }
                            }
                        }
                        ClientEvent::Rotation { rotation } => {
                            for other_client in self.player_list.iter() {
                                if other_client.uuid == client.uuid {
                                    continue;
                                }

                                other_client
                                    .to_client
                                    .send(ClientCommand::RotateEntity {
                                        entity_id: client.entity_id,
                                        rotation: rotation.clone(),
                                        on_ground: false,
                                    })
                                    .unwrap();
                            }
                        }
                        event => {
                            println!("Received event: {:?}", event);
                        }
                    }
                }
            }
            // println!("Took {:?}", start.elapsed());
        }
    }
}

impl Drop for ServerManager {
    fn drop(&mut self) {
        self.token.cancel();
    }
}

pub trait MOTD {
    fn motd(&self, server: &ServerManager) -> String;
}

#[derive(Debug)]
pub struct DefaultMOTD {
    favicon: &'static str,
    previews_chat: bool,
    enforces_secure_chat: bool,
    description: &'static str,
    max_players: u32,
    version_name: &'static str,
    version_protocol: u32,
}

impl MOTD for DefaultMOTD {
    fn motd(&self, server: &ServerManager) -> String {
        let online_players = server.player_list.len() as u32;
        format!(
            "{{\"favicon\": \"{}\", \"previewsChat\":{},\"enforcesSecureChat\":{},\"description\":{{\"text\":\"{}\"}},\"players\":{{\"max\":{},\"online\":{}}},\"version\":{{\"name\":\"{}\",\"protocol\":{}}}}}",
            self.favicon,
            self.previews_chat,
            self.enforces_secure_chat,
            self.description,
            self.max_players,
            online_players,
            self.version_name,
            self.version_protocol
        )
    }
}

use crate::client::{Client, ClientCommand, ClientEvent, Player};
