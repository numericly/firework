use crate::client::{Client, ClientCommand, Player};
use crate::entities::{EntityDataFlags, EntityMetadata, Pose};
use async_trait::async_trait;
use dashmap::{DashMap, DashSet};
use firework_authentication::{authenticate, AuthenticationError, Profile};
use firework_protocol::client_bound::{
    EncryptionRequest, LoginDisconnect, LoginSuccess, Pong, ServerStatus, SetCompression,
};
use firework_protocol::server_bound::{ClientInformation, Ping, ServerBoundPacket};
use firework_protocol::{read_specific_packet, ConnectionState, Protocol, ProtocolError};
use firework_protocol_core::VarInt;
use firework_world::World;
use rsa::{PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use std::collections::HashSet;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::num::ParseIntError;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio::{select, task};
use tokio_util::sync::CancellationToken;

pub mod client;
pub mod entities;
pub mod gui;

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("protocol error {0}")]
    ProtocolError(#[from] ProtocolError),
    #[error("unexpected next connection state {0:?}")]
    UnexpectedNextState(ConnectionState),
    #[error("invalid shared secret length, expected 16, got {0}")]
    InvalidSharedSecretLength(usize),
    #[error("authentication error {0}")]
    AuthenticationError(#[from] AuthenticationError),
    #[error("rsa error {0}")]
    RsaError(#[from] rsa::errors::Error),
    #[error("uuid parse error {0}")]
    UuidParseError(#[from] ParseIntError),
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
    #[error("world error {0}")]
    WorldError(#[from] firework_world::WorldError),
}

#[derive(Clone, Debug)]
pub enum Entities {}

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
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

#[derive(Debug)]
pub struct ClientData {
    pub profile: Profile,
    pub uuid: u128,
    pub entity_id: i32,
    pub has_connected: RwLock<bool>,
    pub loaded_chunks: Mutex<HashSet<(i32, i32)>>,
    pub settings: RwLock<Option<ClientInformation>>,
    pub brand: RwLock<Option<String>>,
    pub socket_address: SocketAddr,
}

pub struct ServerManager<T: Sized, const PLAYER_RESERVED_ENTITY_IDS: i32 = 1_000_000> {
    encryption: Arc<Encryption>,
    proxy: Arc<T>,
    player_entity_ids: DashSet<i32>,
    lowest_free_entity_id: Mutex<i32>,
}

#[async_trait]
pub trait ServerProxy {
    type TransferData: Clone + Send + Sync + 'static;
    async fn new() -> Self
    where
        Self: Sized;
    async fn run(self: Arc<Self>);
    async fn handle_connection(self: Arc<Self>, connection: Protocol, client_data: ClientData);
    async fn motd(&self) -> Result<String, ConnectionError>;
}

impl<T: ServerProxy + std::marker::Send + std::marker::Sync + 'static> ServerManager<T> {
    pub async fn run(port: u16) {
        let encryption = Arc::new(Encryption::new());

        let server = Arc::new(ServerManager {
            encryption,
            proxy: Arc::new(T::new().await),
            lowest_free_entity_id: Mutex::new(0),
            player_entity_ids: DashSet::new(),
        });

        let cloned_server = server.clone();

        tokio::task::spawn(async move {
            let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
                .await
                .expect("Failed to bind server to port");
            println!("Server started listening on port: {}", port);
            loop {
                let stream = listener.accept().await;

                if let Ok((stream, socket_addr)) = stream {
                    let connection = Protocol::new(stream);
                    let server = cloned_server.clone();
                    #[allow(unused_must_use)]
                    tokio::task::spawn(async move {
                        server.handle_connection(socket_addr, connection).await;
                    });
                }
            }
        });

        server.proxy.clone().run().await;
    }
    async fn handle_connection(
        self: Arc<Self>,
        ip_address: SocketAddr,
        mut connection: Protocol,
    ) -> Result<(), ConnectionError> {
        let handshake = read_specific_packet!(&connection, Handshake).await?;

        match handshake.next_state {
            // Handle server ping
            ConnectionState::Status => {
                {
                    *connection.connection_state.write().await = ConnectionState::Status;
                }
                read_specific_packet!(&connection, StatusRequest).await?;

                // Fix me
                let motd = self.proxy.motd().await?;

                let server_status = ServerStatus {
                    motd: motd.to_string(),
                };

                connection.write_packet(server_status).await?;

                let Ping { payload, .. } = read_specific_packet!(&connection, Ping).await?;

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
                    let login_start = read_specific_packet!(connection, LoginStart).await?;

                    let client_username = login_start.name;

                    let encryption_request = EncryptionRequest {
                        server_id: "".to_string(), // deprecated after 1.7
                        public_key: encryption.encoded_pub.clone(),
                        verify_token: Vec::new(),
                    };
                    connection.write_packet(encryption_request).await?;

                    let encryption_response =
                        read_specific_packet!(connection, EncryptionResponse).await?;

                    let shared_secret = encryption.priv_key.decrypt(
                        rsa::PaddingScheme::PKCS1v15Encrypt,
                        encryption_response.shared_secret.as_slice(),
                    )?;

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
                            connection.write_packet(disconnect).await;
                            return Err(ConnectionError::AuthenticationError(err));
                        }
                    };

                    connection
                        .enable_encryption(shared_secret.as_slice(), shared_secret.as_slice())
                        .await;

                    let set_compression = SetCompression {
                        threshold: VarInt(0),
                    };

                    connection.write_packet(set_compression).await?;

                    connection.enable_compression();

                    let uuid = u128::from_str_radix(&profile.id, 16)?;

                    let login_success = LoginSuccess {
                        uuid: uuid.clone(),
                        username: profile.name.clone(),
                        properties: profile.properties.clone(),
                    };

                    connection.write_packet(login_success).await?;

                    Ok((uuid, profile))
                }

                {
                    *connection.connection_state.write().await = ConnectionState::Login;
                }

                let (uuid, profile) =
                    handle_login(&mut connection, self.encryption.clone()).await?;

                let entity_id = self.get_entity_id().await;

                let client_data = ClientData {
                    loaded_chunks: Mutex::new(HashSet::new()),
                    has_connected: RwLock::new(false),
                    entity_id: entity_id.clone(),
                    uuid,
                    profile,
                    settings: RwLock::new(None),
                    brand: RwLock::new(None),
                    socket_address: ip_address,
                };

                self.proxy
                    .clone()
                    .handle_connection(connection, client_data)
                    .await;

                self.remove_entity_id(entity_id).await;
                Ok(())
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
}

pub struct Server<Handler, Proxy: ServerProxy>
where
    Proxy: ServerProxy + Send + Sync + 'static,
{
    pub world: Arc<World>,
    pub entities: Arc<DashMap<i32, Entities>>,
    pub player_list: Arc<DashMap<u128, Client<Proxy>>>,
    pub difficulty: RwLock<u8>,
    pub difficulty_locked: RwLock<bool>,
    pub handler: Handler,
    pub brand: String,
    _lowest_free_id: Mutex<i32>,

    proxy: PhantomData<Proxy>,
}

#[async_trait]
#[allow(unused_variables)]
pub trait ServerHandler<Proxy>
where
    Self: Sized + Send + Sync + 'static,
    Proxy: ServerProxy + Send + Sync + 'static,
{
    fn new() -> Self;
    async fn on_load(&self, server: &Server<Self, Proxy>, proxy: &Proxy) {}
    async fn on_tick(&self, server: &Server<Self, Proxy>, proxy: &Proxy) {}
    async fn load_player(&self, profile: Profile, uuid: u128) -> Result<Player, ConnectionError>;
    async fn on_client_connected(
        &self,
        server: &Server<Self, Proxy>,
        proxy: &Proxy,
        client: &Client<Proxy>,
    ) -> Result<(), ConnectionError> {
        Ok(())
    }
    async fn on_client_post_world_load(
        &self,
        server: &Server<Self, Proxy>,
        proxy: &Proxy,
        client: &Client<Proxy>,
    ) -> Result<(), ConnectionError> {
        Ok(())
    }
    async fn on_client_command(
        &self,
        server: &Server<Self, Proxy>,
        proxy: &Proxy,
        client: &Client<Proxy>,
        command: ClientCommand<Proxy::TransferData>,
    ) -> Result<Option<ClientCommand<Proxy::TransferData>>, ConnectionError> {
        Ok(Some(command))
    }
    async fn on_client_packet(
        &self,
        server: &Server<Self, Proxy>,
        proxy: &Proxy,
        client: &Client<Proxy>,
        packet: ServerBoundPacket,
    ) -> Result<Option<ServerBoundPacket>, ConnectionError> {
        Ok(Some(packet))
    }
    async fn on_chat(
        &self,
        server: &Server<Self, Proxy>,
        proxy: &Proxy,
        client: &Client<Proxy>,
        chat: String,
    ) -> Result<Option<String>, ConnectionError> {
        let name = &client.player.read().await.profile.name;
        Ok(Some(format!(r#"{{ "text": "<{}> {}"}}"#, name, chat)))
    }
    async fn on_chat_command(
        &self,
        server: &Server<Self, Proxy>,
        proxy: &Proxy,
        client: &Client<Proxy>,
        command: String,
    ) -> Result<Option<String>, ConnectionError> {
        Ok(Some(command))
    }
    async fn on_player_move(
        &self,
        server: &Server<Self, Proxy>,
        proxy: &Proxy,
        client: &Client<Proxy>,
        pos: Vec3,
    ) -> Result<Option<Vec3>, ConnectionError> {
        Ok(Some(pos))
    }
    async fn on_player_look(
        &self,
        server: &Server<Self, Proxy>,
        proxy: &Proxy,
        client: &Client<Proxy>,
        rotation: Rotation,
    ) -> Result<Option<Rotation>, ConnectionError> {
        Ok(Some(rotation))
    }
    async fn on_player_sneak(
        &self,
        server: &Server<Self, Proxy>,
        proxy: &Proxy,
        client: &Client<Proxy>,
        sneaking: bool,
    ) -> Result<Option<bool>, ConnectionError> {
        Ok(Some(sneaking))
    }
    async fn on_player_sprint(
        &self,
        server: &Server<Self, Proxy>,
        proxy: &Proxy,
        client: &Client<Proxy>,
        sprinting: bool,
    ) -> Result<Option<bool>, ConnectionError> {
        Ok(Some(sprinting))
    }
}

impl<Handler: ServerHandler<Proxy>, Proxy: ServerProxy> Server<Handler, Proxy>
where
    Proxy: Send + Sync + 'static,
    Handler: Send + Sync + 'static,
    Proxy::TransferData: Clone,
{
    pub async fn new(world: World, brand: String) -> Arc<Self> {
        Arc::new(Self {
            difficulty: RwLock::new(0),
            difficulty_locked: RwLock::new(false),
            world: Arc::new(world),
            player_list: Arc::new(DashMap::new()),
            entities: Arc::new(DashMap::new()),
            _lowest_free_id: Mutex::new(0),
            handler: Handler::new(),
            proxy: PhantomData {},
            brand,
        })
    }
    pub async fn run(self: Arc<Self>, proxy: Arc<Proxy>) {
        let mut interval = tokio::time::interval(Duration::from_millis(50));
        self.handler.on_load(&self, &proxy).await;
        loop {
            interval.tick().await;

            self.handle_tick(proxy.clone()).await;
        }
    }
    pub async fn handle_tick(&self, proxy: Arc<Proxy>) {
        self.handler.on_tick(&self, &proxy).await;
    }
    pub async fn handle_connection(
        self: Arc<Self>,
        proxy: Arc<Proxy>,
        connection: Arc<Protocol>,
        client_data: Arc<ClientData>,
    ) -> Result<Proxy::TransferData, ConnectionError> {
        let player = self
            .handler
            .load_player(client_data.profile.clone(), client_data.uuid)
            .await?;

        let (to_client_sender, to_client_receiver) =
            broadcast::channel::<ClientCommand<Proxy::TransferData>>(10);

        // generate client
        let uuid = player.uuid.clone();
        let client = {
            let client = Client::new(
                connection.clone(),
                client_data.clone(),
                player,
                to_client_sender,
            );

            #[allow(unused_must_use)]
            if let Some(other_client) = self.player_list.get(&uuid) {
                other_client.to_client.send(ClientCommand::Disconnect {
                    reason: r#"{"text": "You logged in from another location"}"#.to_string(),
                });
                client
                    .handle_command(
                        &self,
                        &proxy,
                        ClientCommand::Disconnect {
                            reason: r#"{"text": "You are already logged in"}"#.to_string(),
                        },
                    )
                    .await;
                return Err(ConnectionError::ClientAlreadyConnected);
            }

            self.player_list
                .insert(client.client_data.uuid.clone(), client);

            let client = self
                .player_list
                .get(&uuid)
                .ok_or(ConnectionError::ClientNotInPlayerList)?;

            client
        };

        self.broadcast_chat(format!(
            r#"{{"text": "{} joined the game","color":"yellow"}}"#,
            client.player.read().await.profile.name
        ))
        .await;

        let status = self
            .clone()
            .handle_player(proxy, &client, uuid, to_client_receiver)
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
        proxy: Arc<Proxy>,
        client: &Client<Proxy>,
        uuid: u128,
        mut to_client_receiver: broadcast::Receiver<ClientCommand<Proxy::TransferData>>,
    ) -> Result<Proxy::TransferData, ConnectionError> {
        if client.connection_state().await != ConnectionState::Play {
            client.change_to_play(&self).await?;
        } else {
            client.transfer_world(&self).await?;
        }
        client.load_world(&self).await?;
        self.handler
            .on_client_post_world_load(&self, &proxy, client)
            .await?;
        self.broadcast_player_join(&client).await;

        self.handler
            .on_client_connected(&self, &proxy, client)
            .await?;

        let token = CancellationToken::new();

        let command_listener_server = self.clone();
        let command_listener_proxy = proxy.clone();
        let command_listener_token = token.clone();

        let command_listener_handle: JoinHandle<Result<Proxy::TransferData, ConnectionError>> =
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
                                .on_client_command(
                                    &command_listener_server,
                                    &command_listener_proxy,
                                    client.value(),
                                    command
                                )
                                .await?
                            {
                                if let Some(data) = client
                                    .handle_command(
                                        &command_listener_server,
                                        &command_listener_proxy,
                                        command
                                    )
                                    .await? {
                                    return Ok(data);
                                }
                            }
                        }
                        _ = command_listener_token.cancelled() => {
                            return Err(ConnectionError::ClientCancelled)
                        }
                    };
                }
            });

        let event_listener_server = self.clone();
        let event_listener_proxy = proxy.clone();
        let event_listener_token = token.clone();

        let event_listener_handle: JoinHandle<Result<Proxy::TransferData, ConnectionError>> =
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
                                .on_client_packet(
                                    &event_listener_server,
                                    &event_listener_proxy,
                                    client.value(), event
                                )
                                .await?
                            {
                                client
                                    .handle_packet(
                                        &event_listener_server,
                                        &event_listener_proxy,
                                        event
                                    )
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

        #[allow(unused_must_use)]
        let timeout_handler_handle: JoinHandle<
            Result<Proxy::TransferData, ConnectionError>,
        > = task::spawn(async move {
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
        server: &Server<Handler, Proxy>,
        proxy: &Proxy,
        client: &Client<Proxy>,
        message: String,
    ) -> Result<(), ConnectionError> {
        let message = self.handler.on_chat(server, proxy, client, message).await?;
        if let Some(message) = message {
            self.broadcast_chat(message).await;
        }
        Ok(())
    }
    pub async fn handle_position_update(
        &self,
        server: &Server<Handler, Proxy>,
        proxy: &Proxy,
        client: &Client<Proxy>,
        on_ground: bool,
        position: Option<Vec3>,
        rotation: Option<Rotation>,
    ) -> Result<(), ConnectionError> {
        let previous_pos = client.player.read().await.position.clone();
        let pos = if let Some(pos) = position {
            let pos = self
                .handler
                .on_player_move(server, proxy, client, pos)
                .await?;
            if let Some(pos) = &pos {
                client.player.write().await.position = pos.clone();
                let previous_chunk_x = previous_pos.x as i32 >> 4;
                let previous_chunk_z = previous_pos.z as i32 >> 4;

                let chunk_x = pos.x as i32 >> 4;
                let chunk_z = pos.z as i32 >> 4;

                if previous_chunk_x != chunk_x || previous_chunk_z != chunk_z {
                    client.move_chunk(self, chunk_x, chunk_z).await?;
                }
            };
            pos
        } else {
            None
        };
        let previous_rot = client.player.read().await.rotation.clone();
        let rot = if let Some(rot) = rotation {
            let rot = self
                .handler
                .on_player_look(server, proxy, client, rot)
                .await?;
            if let Some(rot) = &rot {
                client.player.write().await.rotation = rot.clone();
            };
            rot
        } else {
            None
        };

        self.broadcast_entity_move(
            client.client_data.entity_id,
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
        server: &Server<Handler, Proxy>,
        proxy: &Proxy,
        client: &Client<Proxy>,
        sneaking: bool,
    ) -> Result<(), ConnectionError> {
        if let Some(sneaking) = self
            .handler
            .on_player_sneak(server, proxy, client, sneaking)
            .await?
        {
            client.player.write().await.sneaking = sneaking;
            let metadata = vec![
                EntityMetadata::EntityPose(if sneaking {
                    Pose::Sneaking
                } else {
                    Pose::Standing
                }),
                EntityMetadata::EntityFlags(EntityDataFlags::new().with_is_crouching(sneaking)),
            ];
            self.broadcast_entity_metadata_update(client.client_data.entity_id, metadata)
                .await;
        }
        Ok(())
    }
    pub async fn handle_sprinting(
        &self,
        server: &Server<Handler, Proxy>,
        proxy: &Proxy,
        client: &Client<Proxy>,
        sprinting: bool,
    ) -> Result<(), ConnectionError> {
        if let Some(sprinting) = self
            .handler
            .on_player_sprint(server, proxy, client, sprinting)
            .await?
        {
            client.player.write().await.sprinting = sprinting;
            let metadata = vec![EntityMetadata::EntityFlags(
                EntityDataFlags::new().with_is_sprinting(sprinting),
            )];
            self.broadcast_entity_metadata_update(client.client_data.entity_id, metadata)
                .await;
        }
        Ok(())
    }
    #[allow(unused_must_use)]
    pub async fn broadcast_chat(&self, message: String) {
        // TODO: join futures
        for client in self.player_list.iter() {
            client.to_client.send(ClientCommand::ChatMessage {
                message: message.clone(),
            });
        }
    }
    #[allow(unused_must_use)]
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
            if entity_id == client.client_data.entity_id {
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
    #[allow(unused_must_use)]
    pub async fn broadcast_entity_metadata_update(
        &self,
        entity_id: i32,
        metadata: Vec<EntityMetadata>,
    ) {
        for client in self.player_list.iter() {
            if entity_id == client.client_data.entity_id {
                continue;
            }
            client.to_client.send(ClientCommand::UpdateEntityMetadata {
                entity_id,
                metadata: metadata.clone(),
            });
        }
    }
    #[allow(unused_must_use)]
    pub async fn broadcast_player_leave(&self, client: &Client<Proxy>) {
        for other_client in self.player_list.iter() {
            if other_client.client_data.entity_id == client.client_data.entity_id {
                continue;
            }
            other_client.to_client.send(ClientCommand::RemovePlayer {
                entity_id: client.client_data.entity_id,
                uuid: client.client_data.uuid,
            });
        }
    }
    #[allow(unused_must_use)]
    pub async fn broadcast_player_join(&self, client: &Client<Proxy>) {
        for other_client in self.player_list.iter() {
            if other_client.client_data.entity_id == client.client_data.entity_id {
                continue;
            }
            let add_player = {
                let player = client.player.read().await;
                ClientCommand::AddPlayer {
                    client_data: client.client_data.clone(),
                    position: player.position.clone(),
                    rotation: player.rotation.clone(),
                    gamemode: player.gamemode.clone() as u8,
                }
            };
            other_client.to_client.send(add_player);
        }
    }
}