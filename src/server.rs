use async_trait::async_trait;
use authentication::{authenticate, AuthenticationError, Profile};
use dashmap::{DashMap, DashSet};
use protocol::client_bound::{
    EncryptionRequest, LoginDisconnect, LoginSuccess, Pong, ServerStatus, SetCompression,
};
use protocol::server_bound::{Ping, ServerBoundPacket};
use protocol::{ConnectionState, Protocol, ProtocolError};
use protocol_core::VarInt;
use rsa::{PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio::{select, task};
use tokio_util::sync::CancellationToken;
use world::World;

macro_rules! read_packet_or_err {
    ($packet:ident, $stream:expr, $connection_state:expr) => {
        match $stream.read_and_deserialize().await {
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

use crate::client::{Client, ClientCommand, Player};
use crate::entities::{EntityDataFlags, EntityMetadata, Pose};

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("protocol error {0}")]
    ProtocolError(#[from] ProtocolError),
    #[error("unexpected next connection state {0:?}")]
    UnexpectedNextState(ConnectionState),
    #[error("invalid shared secret length, expected 16, got {0}")]
    InvalidSharedSecretLength(usize),
    #[error("unexpected packet, expected {expected}, got {got} in state {state}")]
    UnexpectedPacket {
        got: String,
        expected: &'static str,
        state: &'static str,
    },
    #[error("authentication error {0}")]
    AuthenticationError(#[from] AuthenticationError),
    #[error("recv error {0}")]
    RecvError(#[from] broadcast::error::RecvError),
    #[error("client is already connected")]
    ClientAlreadyConnected,
    #[error("the client was not in the player list")]
    ClientNotInPlayerList,
    #[error("client rejected by proxy for {reason}")]
    ClientRejected {
        reason: String,
        disconnect_message: String,
    },
    #[error("client timed out")]
    ClientTimedOut,
    #[error("client disconnected")]
    ClientDisconnected { reason: String },
    #[error("client cancelled")]
    ClientCancelled,
}

#[derive(Clone, Debug)]
pub enum Entities {}

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
    pub fn serialize(&self) -> (i8, i8) {
        let pitch_ratio = 64. / 90.;
        let yaw_ratio = 255. / 360.;

        let pitch = (self.pitch * pitch_ratio) as i8;
        let yaw_mod = self.yaw.rem_euclid(360.);
        let yaw_fixed = if yaw_mod > 180. {
            yaw_mod - 360.
        } else {
            yaw_mod
        };
        let yaw = (yaw_fixed * yaw_ratio) as i8;
        (yaw, pitch)
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
            pub_key,
            priv_key,
            encoded_pub: pub_encoded_bytes,
        }
    }
}

pub struct LimboPlayer {
    pub profile: Profile,
    pub uuid: u128,
    pub connection: Arc<Protocol>,
    pub entity_id: i32,
}

pub struct ServerManager<T: Sized, const PLAYER_RESERVED_ENTITY_IDS: i32 = 1_000_000> {
    encryption: Arc<Encryption>,
    proxy: UnsafeCell<Option<T>>,
    player_entity_ids: DashSet<i32>,
    lowest_free_entity_id: Mutex<i32>,
}

unsafe impl<T: Sized, const PLAYER_RESERVED_ENTITY_IDS: i32> Send
    for ServerManager<T, PLAYER_RESERVED_ENTITY_IDS>
where
    T: ?Sized + Send,
{
}
unsafe impl<T: Sized> Sync for ServerManager<T> where T: ?Sized + Send {}

#[async_trait]
pub trait ServerProxy {
    type TransferData;
    fn new(server_manager: Arc<ServerManager<Self>>) -> Self
    where
        Self: Sized;
    async fn handle_connection(&self, limbo_player: LimboPlayer) -> Result<(), ConnectionError>;
    fn motd(&self) -> Result<String, ConnectionError>;
}

impl<T: ServerProxy + std::marker::Send + std::marker::Sync + 'static> ServerManager<T> {
    pub async fn run(port: u16) {
        let encryption = Arc::new(Encryption::new());

        let server = Arc::new(ServerManager {
            encryption,
            proxy: UnsafeCell::new(None),
            lowest_free_entity_id: Mutex::new(0),
            player_entity_ids: DashSet::new(),
        });

        let proxy = T::new(server.clone());

        unsafe {
            server.proxy.get().write(Some(proxy));
        }

        let cloned_server = server.clone();

        tokio::task::spawn(async move {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
                .await
                .expect("Failed to bind server to port");
            println!("Server started listening on port: {}", port);
            loop {
                let (stream, _socket_address) = listener.accept().await.unwrap();

                let connection = Protocol::new(stream);
                let server = cloned_server.clone();
                tokio::task::spawn(async move {
                    let res = server.handle_connection(connection).await;
                    if let Err(e) = res {
                        println!("{}", e);
                    }
                });
            }
        });
    }
    async fn handle_connection(
        self: Arc<Self>,
        mut connection: Protocol,
    ) -> Result<(), ConnectionError> {
        let handshake = read_packet_or_err!(Handshake, connection, ConnectionState::HandShaking);

        match handshake.next_state {
            // Handle server ping
            ConnectionState::Status => {
                {
                    *connection.connection_state.write().await = ConnectionState::Status;
                }
                read_packet_or_err!(StatusRequest, connection, ConnectionState::Status);

                // Fix me
                let motd = self.proxy().motd()?;

                let server_status = ServerStatus {
                    motd: motd.to_string(),
                };

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
                ) -> Result<(u128, Profile), ConnectionError> {
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

                    Ok((uuid, profile))
                }

                {
                    *connection.connection_state.write().await = ConnectionState::Login;
                }

                let (uuid, profile) =
                    handle_login(&mut connection, self.encryption.clone()).await?;

                let connection = Arc::new(connection);

                let entity_id = self.get_entity_id().await;

                let limbo_player = LimboPlayer {
                    connection: connection.clone(),
                    entity_id: entity_id.clone(),
                    uuid,
                    profile,
                };

                let disconnect_status = self.proxy().handle_connection(limbo_player).await;

                self.remove_entity_id(entity_id).await;

                disconnect_status
            }
            // If the client tried to change state to the current state or play state which is not allowed
            state => Err(ConnectionError::UnexpectedNextState(state)),
        }
    }
    async fn get_entity_id(&self) -> i32 {
        let mut lowest_free_id = self.lowest_free_entity_id.lock().await;
        let id = *lowest_free_id;
        *lowest_free_id += 1;
        if self.player_entity_ids.contains(&lowest_free_id) {
            loop {
                *lowest_free_id += 1;
                if !self.player_entity_ids.contains(&lowest_free_id) {
                    break;
                }
            }
        }
        self.player_entity_ids.insert(id);
        id
    }
    async fn remove_entity_id(&self, id: i32) {
        let mut lowest_free_id = self.lowest_free_entity_id.lock().await;
        if id < *lowest_free_id {
            *lowest_free_id = id;
        }
        self.player_entity_ids.remove(&id);
    }
    pub fn proxy(&self) -> &T {
        unsafe { self.proxy.get().as_ref().unwrap().as_ref().unwrap() }
    }
}

pub struct Server<T, U> {
    pub world: Arc<World>,
    pub entities: Arc<DashMap<i32, Entities>>,
    pub player_list: Arc<DashMap<u128, Client>>,
    lowest_free_id: Mutex<i32>,

    pub handler: T,
    pub proxy: Arc<ServerManager<U>>,
}

#[async_trait]
pub trait ServerHandler<T> {
    fn new(server_manager: Arc<ServerManager<T>>) -> Self
    where
        Self: Sized;
    fn load_player(&self, profile: Profile, uuid: u128) -> Result<Player, ConnectionError>;
    async fn client_command(
        &self,
        client: &Client,
        command: ClientCommand,
    ) -> Result<Option<ClientCommand>, ConnectionError> {
        Ok(Some(command))
    }
    async fn client_packet(
        &self,
        client: &Client,
        packet: ServerBoundPacket,
    ) -> Result<Option<ServerBoundPacket>, ConnectionError> {
        Ok(Some(packet))
    }
    async fn on_chat(
        &self,
        client: &Client,
        chat: String,
    ) -> Result<Option<String>, ConnectionError> {
        let name = &client.player.read().await.profile.name;
        Ok(Some(format!(r#"{{ "text": "<{}> {}"}}"#, name, chat)))
    }
    async fn on_player_move(
        &self,
        client: &Client,
        pos: Vec3,
    ) -> Result<Option<Vec3>, ConnectionError> {
        Ok(Some(pos))
    }
    async fn on_player_look(
        &self,
        client: &Client,
        rotation: Rotation,
    ) -> Result<Option<Rotation>, ConnectionError> {
        Ok(Some(rotation))
    }
    async fn on_player_sneak(
        &self,
        client: &Client,
        sneaking: bool,
    ) -> Result<Option<bool>, ConnectionError> {
        Ok(Some(sneaking))
    }
    async fn on_player_sprint(
        &self,
        client: &Client,
        sprinting: bool,
    ) -> Result<Option<bool>, ConnectionError> {
        Ok(Some(sprinting))
    }
}

impl<Handler: ServerHandler<Proxy>, Proxy> Server<Handler, Proxy>
where
    Proxy: Send + Sync + 'static,
    Handler: Send + Sync + 'static,
{
    pub fn new(world: World, proxy: Arc<ServerManager<Proxy>>) -> Arc<Self> {
        let handler = Handler::new(proxy.clone());
        let server = Arc::new(Self {
            world: Arc::new(world),
            player_list: Arc::new(DashMap::new()),
            entities: Arc::new(DashMap::new()),
            lowest_free_id: Mutex::new(0),
            handler,
            proxy,
        });
        server
    }
    pub async fn handle_connection(
        self: Arc<Self>,
        limbo_player: LimboPlayer,
    ) -> Result<Option<()>, ConnectionError> {
        let player = self
            .handler
            .load_player(limbo_player.profile, limbo_player.uuid)?;

        let (to_client_sender, to_client_receiver) = broadcast::channel::<ClientCommand>(10);

        // generate client
        let uuid = player.uuid.clone();
        let client = {
            let client = Client::new(
                limbo_player.connection,
                player,
                limbo_player.entity_id,
                uuid.clone(),
                to_client_sender,
            );

            #[allow(unused_must_use)]
            if let Some(other_client) = self.player_list.get(&uuid) {
                other_client.to_client.send(ClientCommand::Disconnect {
                    reason: r#"{"text": "You logged in from another location"}"#.to_string(),
                });
                client
                    .handle_command(
                        ClientCommand::Disconnect {
                            reason: r#"{"text": "You are already logged in"}"#.to_string(),
                        },
                        &self,
                    )
                    .await;
                return Err(ConnectionError::ClientAlreadyConnected);
            }

            self.player_list.insert(client.uuid.clone(), client);

            let client = self
                .player_list
                .get(&uuid)
                .ok_or(ConnectionError::ClientNotInPlayerList)?;

            client
        };

        self.broadcast_player_join(&client).await;
        self.broadcast_chat(format!(
            r#"{{"text": "{} joined the game","color":"yellow"}}"#,
            client.player.read().await.profile.name
        ))
        .await;

        let status = self
            .clone()
            .handle_player(&client, uuid, to_client_receiver)
            .await;

        self.broadcast_chat(format!(
            r#"{{"text": "{} left the game","color":"yellow"}}"#,
            client.player.read().await.profile.name
        ))
        .await;
        self.broadcast_player_leave(&client).await;

        drop(client);

        self.player_list.remove(&uuid);
        status
    }
    pub async fn handle_player(
        self: Arc<Self>,
        client: &Client,
        uuid: u128,
        mut to_client_receiver: broadcast::Receiver<ClientCommand>,
    ) -> Result<Option<()>, ConnectionError> {
        client.load_world(&self).await?;

        let token = CancellationToken::new();

        let command_listener_server = self.clone();
        let command_listener_token = token.clone();

        let command_listener_handle: JoinHandle<Result<Option<()>, ConnectionError>> =
            task::spawn(async move {
                let client = command_listener_server
                    .player_list
                    .get(&uuid)
                    .ok_or(ConnectionError::ClientAlreadyConnected)?;

                loop {
                    select! {
                        command = to_client_receiver.recv() => {
                            let command = command?;

                            if let Some(command) = command_listener_server
                                .handler
                                .client_command(client.value(), command)
                                .await?
                            {
                                client
                                    .handle_command(command, &command_listener_server)
                                    .await?;
                            }
                        }
                        _ = command_listener_token.cancelled() => {
                            return Err(ConnectionError::ClientCancelled)
                        }
                    };
                }
            });

        let event_listener_server = self.clone();
        let event_listener_token = token.clone();

        let event_listener_handle: JoinHandle<Result<Option<()>, ConnectionError>> =
            task::spawn(async move {
                let client = event_listener_server
                    .player_list
                    .get(&uuid)
                    .ok_or(ConnectionError::ClientAlreadyConnected)?;

                loop {
                    select! {
                        event = client.read_packet() => {
                            let event = event?;
                            if let Some(event) = event_listener_server
                                .handler
                                .client_packet(client.value(), event)
                                .await?
                            {
                                client
                                    .handle_packet(event, &event_listener_server)
                                    .await?;
                            }
                        }
                        _ = event_listener_token.cancelled() => {
                            return Err(ConnectionError::ClientCancelled)
                        }
                    };
                }
            });

        let timeout_handler_server = self.clone();
        let timeout_handler_token = token.clone();

        let timeout_handler_handle: JoinHandle<Result<Option<()>, ConnectionError>> =
            task::spawn(async move {
                let client = timeout_handler_server
                    .player_list
                    .get(&uuid)
                    .ok_or(ConnectionError::ClientAlreadyConnected)?;

                client.to_client.send(ClientCommand::Ping);

                loop {
                    select! {
                        _ = sleep(Duration::from_secs(15)) => {
                            client.to_client.send(ClientCommand::Ping);
                        }
                        _ = timeout_handler_token.cancelled() => {
                            return Err(ConnectionError::ClientCancelled)
                        }
                    };
                }
            });

        // WARNING BAD CODE
        #[allow(unused_must_use)]
        let result = {
            tokio::pin!(
                command_listener_handle,
                event_listener_handle,
                timeout_handler_handle
            );
            select! {
                result = command_listener_handle.as_mut() => {
                    token.cancel();
                    event_listener_handle.await;
                    timeout_handler_handle.await;
                    result
                }
                result = event_listener_handle.as_mut() => {
                    token.cancel();
                    command_listener_handle.await;
                    timeout_handler_handle.await;
                    result
                }
                result = timeout_handler_handle.as_mut() => {
                    token.cancel();
                    command_listener_handle.await;
                    event_listener_handle.await;
                    result
                }
            }
        };

        if let Ok(err) = result {
            err
        } else {
            Err(ConnectionError::ClientCancelled)
        }
    }
    pub async fn handle_chat(
        &self,
        client: &Client,
        message: String,
    ) -> Result<(), ConnectionError> {
        let message = self.handler.on_chat(client, message).await?;
        if let Some(message) = message {
            self.broadcast_chat(message).await;
        }
        Ok(())
    }
    pub async fn handle_position_update(
        &self,
        client: &Client,
        on_ground: bool,
        position: Option<Vec3>,
        rotation: Option<Rotation>,
    ) -> Result<(), ConnectionError> {
        let previous_pos = client.player.read().await.position.clone();
        let pos = if let Some(pos) = position {
            let pos = self.handler.on_player_move(client, pos).await?;
            if let Some(pos) = &pos {
                client.player.write().await.position = pos.clone();
            };
            pos
        } else {
            None
        };
        let previous_rot = client.player.read().await.rotation.clone();
        let rot = if let Some(rot) = rotation {
            let rot = self.handler.on_player_look(client, rot).await?;
            if let Some(rot) = &rot {
                client.player.write().await.rotation = rot.clone();
            };
            rot
        } else {
            None
        };

        self.broadcast_entity_move(
            client.entity_id,
            pos,
            previous_pos,
            rot,
            previous_rot,
            on_ground,
        )
        .await;

        Ok(())
    }
    pub async fn handle_sneaking(
        &self,
        client: &Client,
        sneaking: bool,
    ) -> Result<(), ConnectionError> {
        if let Some(sneaking) = self.handler.on_player_sneak(client, sneaking).await? {
            client.player.write().await.sneaking = sneaking;
            let metadata = vec![
                EntityMetadata::EntityPose(if sneaking {
                    Pose::Sneaking
                } else {
                    Pose::Standing
                }),
                EntityMetadata::EntityFlags(EntityDataFlags::new().with_is_crouching(sneaking)),
            ];
            self.broadcast_entity_metadata_update(client.entity_id, metadata)
                .await;
        }
        Ok(())
    }
    pub async fn handle_sprinting(
        &self,
        client: &Client,
        sprinting: bool,
    ) -> Result<(), ConnectionError> {
        if let Some(sprinting) = self.handler.on_player_sprint(client, sprinting).await? {
            client.player.write().await.sprinting = sprinting;
            let metadata = vec![EntityMetadata::EntityFlags(
                EntityDataFlags::new().with_is_sprinting(sprinting),
            )];
            self.broadcast_entity_metadata_update(client.entity_id, metadata)
                .await;
        }
        Ok(())
    }
    pub async fn broadcast_chat(&self, message: String) {
        // TODO: join futures
        for client in self.player_list.iter() {
            client.to_client.send(ClientCommand::ChatMessage {
                message: message.clone(),
            });
        }
    }
    pub async fn broadcast_entity_move(
        &self,
        entity_id: i32,
        position: Option<Vec3>,
        previous_position: Vec3,
        rotation: Option<Rotation>,
        previous_rotation: Rotation,
        on_ground: bool,
    ) {
        for client in self.player_list.iter() {
            if entity_id == client.entity_id {
                continue;
            }
            client.to_client.send(ClientCommand::MoveEntity {
                entity_id,
                position: position.clone(),
                previous_position: previous_position.clone(),
                rotation: rotation.clone(),
                previous_rotation: previous_rotation.clone(),
                on_ground,
            });
        }
    }
    pub async fn broadcast_entity_metadata_update(
        &self,
        entity_id: i32,
        metadata: Vec<EntityMetadata>,
    ) {
        for client in self.player_list.iter() {
            if entity_id == client.entity_id {
                continue;
            }
            client.to_client.send(ClientCommand::UpdateEntityMetadata {
                entity_id,
                metadata: metadata.clone(),
            });
        }
    }
    pub async fn broadcast_player_leave(&self, client: &Client) {
        for other_client in self.player_list.iter() {
            if other_client.entity_id == client.entity_id {
                continue;
            }
            other_client.to_client.send(ClientCommand::RemovePlayer {
                entity_id: client.entity_id,
                uuid: client.uuid,
            });
        }
    }
    pub async fn broadcast_player_join(&self, client: &Client) {
        for other_client in self.player_list.iter() {
            if other_client.entity_id == client.entity_id {
                continue;
            }
            let add_player = {
                let player = client.player.read().await;
                ClientCommand::AddPlayer {
                    entity_id: client.entity_id,
                    uuid: client.uuid,
                    position: player.position.clone(),
                    rotation: player.rotation.clone(),
                    name: player.profile.name.clone(),
                    properties: player.profile.properties.clone(),
                    gamemode: player.gamemode,
                }
            };
            other_client.to_client.send(add_player);
        }
    }
}
