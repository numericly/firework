use crate::client::{Client, ClientCommand, Player};
use crate::entities::{EntityMetadata, Pose};
use async_trait::async_trait;
use client::{InventorySlot, PreviousPosition};
use commands::CommandTree;
use dashmap::{DashMap, DashSet};
use firework_authentication::{authenticate, AuthenticationError, Profile};
use firework_protocol::data_types::{EntityAnimationType, Hand};
use firework_protocol::server_bound::{
    ClickContainer, ClientInformation, EncryptionResponse, Handshake, Interact, LoginStart, Ping,
    ServerBoundPacket,
};
use firework_protocol::{
    client_bound::{EncryptionRequest, LoginDisconnect, LoginSuccess, Pong, ServerStatus},
    data_types::Slot,
};
use firework_protocol::{read_specific_packet, ConnectionState, Protocol, ProtocolError};
use firework_world::World;
use rsa::{PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use sha1::{Digest, Sha1};
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashSet, time::Instant};
use std::{fmt::Debug, ops::Div};
use std::{marker::PhantomData, ops::Sub};
use std::{net::SocketAddr, ops::Mul};
use std::{num::ParseIntError, ops::Add};
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio::{select, task};
use tokio_util::sync::CancellationToken;

use rustrict::{Censor, Type};

pub use firework_authentication as authentication;
pub use firework_data as data;
pub use firework_protocol as protocol;
pub use firework_world as world;

pub mod client;
pub mod commands;
pub mod entities;
pub mod gui;

pub const TICKS_PER_SECOND: usize = 20;

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
    #[error("server not found {0}")]
    ServerNotFound(u128),
}

#[derive(Debug, Clone, Default)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub const fn new(x: i32, y: i32, z: i32) -> BlockPos {
        BlockPos { x, y, z }
    }
}

impl From<Vec3> for BlockPos {
    fn from(vec: Vec3) -> BlockPos {
        BlockPos {
            x: vec.x as i32,
            y: vec.y as i32,
            z: vec.z as i32,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AxisAlignedBB {
    pub min: BlockPos,
    pub max: BlockPos,
}

impl AxisAlignedBB {
    pub const fn new(min: BlockPos, max: BlockPos) -> AxisAlignedBB {
        AxisAlignedBB { min, max }
    }
    pub fn within(&self, pos: Vec3) -> bool {
        pos.x >= self.min.x as f64
            && pos.x <= self.max.x as f64
            && pos.y >= self.min.y as f64
            && pos.y <= self.max.y as f64
            && pos.z >= self.min.z as f64
            && pos.z <= self.max.z as f64
    }
}

pub enum AxisAlignedPlane {
    X { min: Vec3, max: Vec3 },
    Y { min: Vec3, max: Vec3 },
    Z { min: Vec3, max: Vec3 },
}

impl AxisAlignedPlane {
    pub fn intersects(&self, starting_position: &Vec3, end: &Vec3) -> bool {
        let delta = end.clone() - starting_position.clone();

        let time = match self {
            Self::X { min, .. } => {
                if delta.x == 0.0 {
                    return false;
                }
                (min.x - starting_position.x) / delta.x
            }
            Self::Y { min, .. } => {
                if delta.y == 0.0 {
                    return false;
                }
                (min.y - starting_position.y) / delta.y
            }
            Self::Z { min, .. } => {
                if delta.z == 0.0 {
                    return false;
                }
                (min.z - starting_position.z) / delta.z
            }
        };

        // if the intersection is behind the starting position, it doesn't count
        // likewise, if the intersection is beyond the end of the ray, it doesn't count
        if time < 0.0 || time > 1.0 {
            return false;
        }
        let intersection = starting_position.clone() + delta * Vec3::scalar(time);
        self.within(intersection)
    }

    fn within(&self, position: Vec3) -> bool {
        match self {
            Self::X { min, max } => {
                position.y >= min.y
                    && position.y <= max.y
                    && position.z >= min.z
                    && position.z <= max.z
            }
            Self::Y { min, max } => {
                position.x >= min.x
                    && position.x <= max.x
                    && position.z >= min.z
                    && position.z <= max.z
            }
            Self::Z { min, max } => {
                position.x >= min.x
                    && position.x <= max.x
                    && position.y >= min.y
                    && position.y <= max.y
            }
        }
    }

    pub fn center(&self) -> Vec3 {
        match self {
            Self::X { min, max } => Vec3::new(min.x, (min.y + max.y) / 2.0, (min.z + max.z) / 2.0),
            Self::Y { min, max } => Vec3::new((min.x + max.x) / 2.0, min.y, (min.z + max.z) / 2.0),
            Self::Z { min, max } => Vec3::new((min.x + max.x) / 2.0, (min.y + max.y) / 2.0, min.z),
        }
    }

    pub fn to_cartesian_pair(&self) -> (Vec3, Vec3) {
        match self {
            Self::X { min, max } => (min.clone(), max.clone()),
            Self::Y { min, max } => (min.clone(), max.clone()),
            Self::Z { min, max } => (min.clone(), max.clone()),
        }
    }
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
    pub const fn scalar(s: f64) -> Vec3 {
        Vec3 { x: s, y: s, z: s }
    }
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn distance(&self, other: &Vec3) -> f64 {
        (self.clone() - other.clone()).length()
    }
    pub fn normalize(&self) -> Vec3 {
        let length = self.length();

        Vec3::new(self.x / length, self.y / length, self.z / length)
    }
    pub fn sum(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn lerp(&self, other: &Vec3, t: f64) -> Vec3 {
        Vec3::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
            self.z + (other.z - self.z) * t,
        )
    }
    pub fn clamp(&self, min: f64, max: f64) -> Vec3 {
        // clamp the length of the vector between min and max
        let length = self.length();
        let length = if length < min { min } else { length };
        let length = if length > max { max } else { length };

        self.normalize() * Vec3::scalar(length)
    }
    pub fn yaw(&self) -> f64 {
        self.z.atan2(self.x)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Rotation {
    pub const fn new(yaw: f32, pitch: f32) -> Rotation {
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
    pub fn direction(&self) -> Vec3 {
        let yaw = (-self.yaw).to_radians();
        let pitch = self.pitch.to_radians();

        let x = yaw.sin() * pitch.cos();
        let y = -pitch.sin();
        let z = yaw.cos() * pitch.cos();

        Vec3::new(x as f64, y as f64, z as f64)
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
    encryption: Option<Arc<Encryption>>,
    proxy: Arc<T>,
    player_entity_ids: DashSet<i32>,
    lowest_free_entity_id: Mutex<i32>,
}

#[async_trait]
pub trait ServerProxy {
    type TransferData: Debug + Clone + Send + Sync + 'static;
    type Roles: Clone + Send + Sync + 'static;
    type Permissions: Clone + Send + Sync + 'static;
    async fn new() -> Self
    where
        Self: Sized;
    async fn run(self: Arc<Self>);
    async fn handle_connection(self: Arc<Self>, connection: Protocol, client_data: ClientData);
    async fn motd(&self) -> Result<String, ConnectionError>;
}

#[derive(Debug, Clone)]
pub struct ServerOptions {
    pub port: u16,
    pub host: bool,
    pub encryption: bool,
}

impl Default for ServerOptions {
    fn default() -> Self {
        ServerOptions {
            port: 25565,
            host: false,
            encryption: true,
        }
    }
}

impl<T: ServerProxy + std::marker::Send + std::marker::Sync + 'static> ServerManager<T> {
    pub async fn run(opts: ServerOptions) {
        let ServerOptions {
            port,
            host,
            encryption: encryption_enabled,
        } = opts;

        let encryption = if encryption_enabled {
            Some(Arc::new(Encryption::new()))
        } else {
            None
        };

        let server = Arc::new(ServerManager {
            encryption,
            proxy: Arc::new(T::new().await),
            lowest_free_entity_id: Mutex::new(0),
            player_entity_ids: DashSet::new(),
        });

        let cloned_server = server.clone();

        let address = if host { "0.0.0.0" } else { "127.0.0.1" };

        tokio::task::spawn(async move {
            let listener = TcpListener::bind(format!("{}:{}", address, port))
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
        let handshake: Handshake = read_specific_packet!(&connection, Handshake).await?;

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
                    encryption: Option<Arc<Encryption>>,
                ) -> Result<(u128, Profile), ConnectionError> {
                    let login_start: LoginStart =
                        read_specific_packet!(connection, LoginStart).await?;

                    let client_username = login_start.name;

                    let profile = if let Some(encryption) = encryption {
                        let encryption_request = EncryptionRequest {
                            server_id: "".to_string(), // deprecated after 1.7
                            public_key: encryption.encoded_pub.clone(),
                            verify_token: Vec::new(),
                        };
                        connection.write_packet(encryption_request).await?;

                        let encryption_response: EncryptionResponse =
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

                        #[allow(unused_must_use)]
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

                        profile
                    } else {
                        let uuid = {
                            let mut hash = Sha1::new();

                            hash.update(&client_username);

                            let mut uuid_bytes = [0u8; 16];

                            uuid_bytes.copy_from_slice(&hash.finalize().as_slice()[0..16]);

                            u128::from_be_bytes(uuid_bytes)
                        };

                        let profile = Profile {
                            id: format!("{:x}", uuid),
                            name: client_username,
                            ..Default::default()
                        };
                        profile
                    };

                    connection.enable_compression(0).await?;

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
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    pub entities: Arc<DashMap<i32, Entities>>,
    pub player_list: Arc<DashMap<u128, Client<Handler, Proxy>>>,
    pub difficulty: RwLock<u8>,
    pub difficulty_locked: RwLock<bool>,
    pub handler: Handler,
    pub brand: String,
    pub id: u128,
    _lowest_free_id: Mutex<i32>,

    proxy: PhantomData<Proxy>,
}
#[async_trait]
#[allow(unused_variables)]
pub trait PlayerHandler<Handler, Proxy>
where
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
    Proxy: ServerProxy + Send + Sync + 'static,
{
    fn new(server: Arc<Server<Handler, Proxy>>, proxy: Arc<Proxy>) -> Self;
    async fn on_pre_load(&self, client: &Client<Handler, Proxy>) -> Result<(), ConnectionError> {
        Ok(())
    }
    async fn on_post_load(&self, client: &Client<Handler, Proxy>) -> Result<(), ConnectionError> {
        Ok(())
    }
    async fn on_leave(&self, client: &Client<Handler, Proxy>) -> Result<(), ConnectionError> {
        Ok(())
    }
    async fn on_transfer(&self, client: &Client<Handler, Proxy>) -> Result<(), ConnectionError> {
        Ok(())
    }
    async fn on_tick(&self, client: &Client<Handler, Proxy>) {}
    async fn on_client_bound_packet(
        &self,
        client: &Client<Handler, Proxy>,
    ) -> Result<(), ConnectionError> {
        Ok(())
    }
    async fn on_server_bound_packet(
        &self,
        client: &Client<Handler, Proxy>,
    ) -> Result<(), ConnectionError> {
        Ok(())
    }
    async fn on_client_command(
        &self,
        client: &Client<Handler, Proxy>,
        command: ClientCommand<Proxy::TransferData>,
    ) -> Result<Option<ClientCommand<Proxy::TransferData>>, ConnectionError> {
        Ok(Some(command))
    }
    async fn on_chat(
        &self,
        client: &Client<Handler, Proxy>,
        chat: String,
    ) -> Result<Option<String>, ConnectionError> {
        let name = &client.player.read().await.profile.name;

        let (censored, analysis) = Censor::from_str(&chat)
            .with_censor_threshold(Type::SEVERE | Type::OFFENSIVE)
            .with_censor_replacement('*')
            .censor_and_analyze();

        Ok(Some(format!(r#"{{ "text": "<{}> {}"}}"#, name, censored)))
    }
    async fn on_use_item(
        &self,
        client: &Client<Handler, Proxy>,
        item: Slot,
        slot_id: InventorySlot,
    ) -> Result<(), ConnectionError> {
        Ok(())
    }
    async fn on_click_container(
        &self,
        client: &Client<Handler, Proxy>,
        click: ClickContainer,
    ) -> Result<(), ConnectionError> {
        Ok(())
    }
    async fn on_swap_item(&self, client: &Client<Handler, Proxy>) -> Result<(), ConnectionError> {
        Ok(())
    }
    async fn on_drop_item(
        &self,
        client: &Client<Handler, Proxy>,
        is_stack: bool,
    ) -> Result<(), ConnectionError> {
        Ok(())
    }
    async fn on_death(&self, client: &Client<Handler, Proxy>) -> Result<bool, ConnectionError> {
        Ok(true)
    }
    async fn on_chat_command(
        &self,
        client: &Client<Handler, Proxy>,
        command: String,
    ) -> Result<Option<String>, ConnectionError> {
        Ok(Some(command))
    }
    async fn on_move(
        &self,
        client: &Client<Handler, Proxy>,
        pos: Vec3,
    ) -> Result<Option<Vec3>, ConnectionError> {
        Ok(Some(pos))
    }
    async fn on_look(
        &self,
        client: &Client<Handler, Proxy>,
        rotation: Rotation,
    ) -> Result<Option<Rotation>, ConnectionError> {
        Ok(Some(rotation))
    }
    async fn on_on_ground(
        &self,
        client: &Client<Handler, Proxy>,
        on_ground: bool,
    ) -> Result<bool, ConnectionError> {
        Ok(on_ground)
    }
    async fn on_sneak(
        &self,
        client: &Client<Handler, Proxy>,
        sneaking: bool,
    ) -> Result<Option<bool>, ConnectionError> {
        Ok(Some(sneaking))
    }
    async fn on_sprint(
        &self,
        client: &Client<Handler, Proxy>,
        sprinting: bool,
    ) -> Result<Option<bool>, ConnectionError> {
        Ok(Some(sprinting))
    }
    async fn on_interact(
        &self,
        client: &Client<Handler, Proxy>,
        interact: Interact,
    ) -> Result<(), ConnectionError> {
        Ok(())
    }
    async fn on_swing_hand(
        &self,
        client: &Client<Handler, Proxy>,
        hand: Hand,
    ) -> Result<(), ConnectionError> {
        Ok(())
    }
}

#[async_trait]
#[allow(unused_variables)]
pub trait ServerHandler<Proxy>
where
    Self: Sized + Send + Sync + 'static,
    Proxy: ServerProxy + Send + Sync + 'static,
{
    type PlayerHandler: PlayerHandler<Self, Proxy> + Send + Sync + 'static;
    fn new() -> Self;
    async fn get_commands(
        &self,
        server: &Server<Self, Proxy>,
        proxy: &Proxy,
    ) -> Result<&CommandTree<Self, Proxy>, ConnectionError>;
    fn get_world(&self) -> &'static World;
    async fn on_load(&self, server: &Server<Self, Proxy>, proxy: Arc<Proxy>) {}
    async fn on_tick(&self, server: &Server<Self, Proxy>, proxy: Arc<Proxy>) {}
    async fn load_player(&self, profile: Profile, uuid: u128) -> Result<Player, ConnectionError>;
    async fn on_client_connected(
        &self,
        server: &Server<Self, Proxy>,
        proxy: &Proxy,
        client: &Client<Self, Proxy>,
    ) -> Result<(), ConnectionError> {
        Ok(())
    }
    async fn on_client_post_world_load(
        &self,
        server: &Server<Self, Proxy>,
        proxy: &Proxy,
        client: &Client<Self, Proxy>,
    ) -> Result<(), ConnectionError> {
        Ok(())
    }
}

impl<Handler: ServerHandler<Proxy>, Proxy: ServerProxy> Server<Handler, Proxy>
where
    Proxy: Send + Sync + 'static,
    Handler: Send + Sync + 'static,
    Proxy::TransferData: Clone,
{
    pub fn new(brand: String, id: u128) -> Arc<Self> {
        Arc::new(Self {
            difficulty: RwLock::new(0),
            difficulty_locked: RwLock::new(false),
            player_list: Arc::new(DashMap::new()),
            entities: Arc::new(DashMap::new()),
            _lowest_free_id: Mutex::new(0),
            handler: Handler::new(),
            proxy: PhantomData {},
            brand,
            id,
        })
    }
    pub fn get_world(&self) -> &'static World {
        self.handler.get_world()
    }
    pub async fn run(self: Arc<Self>, proxy: Arc<Proxy>, token: CancellationToken) {
        let mut interval = tokio::time::interval(Duration::from_millis(50));
        self.handler.on_load(&self, proxy.clone()).await;
        loop {
            interval.tick().await;

            self.handle_tick(proxy.clone()).await;

            if token.is_cancelled() {
                break;
            }
        }
    }
    pub async fn handle_tick(&self, proxy: Arc<Proxy>) {
        self.handler.on_tick(&self, proxy.clone()).await;
        for client in self.player_list.iter() {
            if client.player.read().await.invulnerable_time > 0 {
                client.player.write().await.invulnerable_time -= 1;
            }
            client.player.write().await.attack_strength_ticker += 1;
            client.handler.on_tick(&client).await;
        }
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
            broadcast::channel::<ClientCommand<Proxy::TransferData>>(100);

        let (to_client_visual_sender, to_client_visual_receiver) =
            broadcast::channel::<ClientCommand<Proxy::TransferData>>(100);

        // generate client
        let uuid = player.uuid.clone();
        let client = {
            let client = Client::new(
                self.clone(),
                proxy.clone(),
                connection.clone(),
                client_data.clone(),
                player,
                to_client_sender,
                to_client_visual_sender,
            );

            #[allow(unused_must_use)]
            if let Some(other_client) = self.player_list.get(&uuid) {
                other_client
                    .disconnect(r#"{"text": "You logged in from another location"}"#.to_string());
                client
                    .handle_command(ClientCommand::Disconnect {
                        reason: r#"{"text": "You are already logged in"}"#.to_string(),
                    })
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

        let status = self
            .clone()
            .handle_player(
                proxy,
                &client,
                uuid,
                to_client_receiver,
                to_client_visual_receiver,
            )
            .await;

        self.broadcast_player_leave(&client).await;

        drop(client);

        self.player_list.remove(&uuid);
        status
    }
    pub async fn handle_player(
        self: Arc<Self>,
        proxy: Arc<Proxy>,
        client: &Client<Handler, Proxy>,
        uuid: u128,
        mut to_client_receiver: broadcast::Receiver<ClientCommand<Proxy::TransferData>>,
        mut to_client_visual_receiver: broadcast::Receiver<ClientCommand<Proxy::TransferData>>,
    ) -> Result<Proxy::TransferData, ConnectionError> {
        if client.connection_state().await != ConnectionState::Play {
            client.change_to_play().await?;
        } else {
            client.transfer_world().await?;
        }
        client.handler.on_pre_load(client).await?;
        client.load_world().await?;
        client.handler.on_post_load(client).await?;
        self.broadcast_player_join(&client).await;

        self.handler
            .on_client_connected(&self, &proxy, client)
            .await?;

        let token = CancellationToken::new();

        let command_listener_server = self.clone();
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
                            let command = command.expect("Unable to recv command");
                            if let Some(data) = client.handle_command(command).await? {

                                client.handler.on_transfer(&client).await?;

                                for _ in 0..to_client_receiver.len() {
                                    let command = to_client_receiver.recv().await;
                                    #[allow(unused_must_use)]
                                    if let Ok(command) = command {
                                        client.handle_command(command).await;
                                    }
                                }
                                return Ok(data);
                            }
                        }
                        command = to_client_visual_receiver.recv() => {
                            // it's ok to throw away packets here, they are visual only
                            // if it throws a lagged error we can just ignore it
                            if let Ok(command) = command {
                                if let Some(_) = client.handle_command(command).await? {
                                    panic!("Visual command cannot return data")
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
                            client.handle_packet(event).await?;
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

            client.ping().await?;

            let mut missed_pings = 0;

            loop {
                select! {
                    _ = sleep(Duration::from_secs(1)) => {
                        if !client.ping().await? {
                            missed_pings += 1;
                            if missed_pings >= 15 {
                                return Err(ConnectionError::ClientTimedOut)
                            }
                            println!("Missed ping: {}", missed_pings);
                        } else {
                            missed_pings = 0;
                        }
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

        client.handler.on_leave(&client).await?;

        if let Ok(err) = result {
            err
        } else {
            Err(ConnectionError::ClientCancelled)
        }
    }
    pub async fn handle_death(
        &self,
        _server: &Server<Handler, Proxy>,
        _proxy: &Proxy,
        client: &Client<Handler, Proxy>,
    ) -> Result<(), ConnectionError> {
        if client.handler.on_death(client).await? {
            // self.broadcast_player_death(&client).await;
            // Set health 0
            const SHOW_RESPAWN_SCREEN: bool = false;
            if !SHOW_RESPAWN_SCREEN {
                let max_health = client.player.read().await.max_health.clone();
                client.set_health(max_health);
            }
        }
        Ok(())
    }
    pub async fn handle_chat(
        &self,
        _proxy: &Proxy,
        client: &Client<Handler, Proxy>,
        message: String,
    ) -> Result<(), ConnectionError> {
        let message = client.handler.on_chat(client, message).await?;
        if let Some(message) = message {
            self.broadcast_chat(message);
        }
        Ok(())
    }
    pub async fn handle_chat_command(
        &self,
        _proxy: &Proxy,
        client: &Client<Handler, Proxy>,
        command: String,
    ) -> Result<(), ConnectionError> {
        client.handler.on_chat_command(client, command).await?;
        Ok(())
    }
    pub async fn handle_position_update(
        &self,
        _proxy: &Proxy,
        client: &Client<Handler, Proxy>,
        on_ground: bool,
        position: Option<Vec3>,
        rotation: Option<Rotation>,
    ) -> Result<(), ConnectionError> {
        {
            let mut player = client.player.write().await;
            if let Some(previous_position) = &player.previous_position {
                if let Some(position) = position.clone() {
                    let multiplier = previous_position.time.elapsed().as_secs_f64()
                        / Duration::from_millis(50).as_secs_f64();

                    let delta = position.clone() - previous_position.position.clone();

                    player.velocity = delta * Vec3::scalar(multiplier);
                }
            }
        }
        let previous_pos = client.player.read().await.position.clone();
        let position = if let Some(position) = position {
            let position = client.handler.on_move(client, position).await?;
            if let Some(position) = &position {
                let mut player = client.player.write().await;
                if let Some(previous_position) = &player.previous_position {
                    let multiplier = previous_position.time.elapsed().as_secs_f64()
                        / Duration::from_millis(50).as_secs_f64();

                    let delta = position.clone() - previous_position.position.clone();
                    if delta.y < 0. {
                        player.fall_distance -= delta.y;
                    }
                    player.velocity = delta * Vec3::scalar(multiplier);
                }
                player.previous_position = Some(PreviousPosition {
                    position: position.clone(),
                    time: Instant::now(),
                });
                player.position = position.clone();

                let previous_chunk_x = previous_pos.x as i32 >> 4;
                let previous_chunk_z = previous_pos.z as i32 >> 4;

                let chunk_x = position.x as i32 >> 4;
                let chunk_z = position.z as i32 >> 4;

                if previous_chunk_x != chunk_x || previous_chunk_z != chunk_z {
                    client.move_chunk(chunk_x, chunk_z).await?;
                }
            };
            position
        } else {
            None
        };
        let previous_rot = client.player.read().await.rotation.clone();
        let rot = if let Some(rot) = rotation {
            let rot = client.handler.on_look(client, rot).await?;
            if let Some(rot) = &rot {
                client.player.write().await.rotation = rot.clone();
            };
            rot
        } else {
            None
        };

        let on_ground = client.handler.on_on_ground(client, on_ground).await?;

        if on_ground {
            client.player.write().await.fall_distance = 0.;
        }

        self.broadcast_entity_move(client, position, previous_pos, rot, previous_rot, on_ground)
            .await;

        Ok(())
    }
    pub async fn handle_elytra_flying(
        &self,
        _proxy: &Proxy,
        client: &Client<Handler, Proxy>,
        flying: bool,
    ) -> Result<(), ConnectionError> {
        let mut player = client.player.write().await;
        player.elytra_flying = flying;
        let metadata = vec![EntityMetadata::EntityFlags(player.entity_flags())];
        self.broadcast_entity_metadata_update(client, metadata, false);
        Ok(())
    }
    pub async fn handle_sneaking(
        &self,
        _proxy: &Proxy,
        client: &Client<Handler, Proxy>,
        sneaking: bool,
    ) -> Result<(), ConnectionError> {
        if let Some(sneaking) = client.handler.on_sneak(client, sneaking).await? {
            let mut player = client.player.write().await;
            player.sneaking = sneaking;
            let metadata = vec![
                EntityMetadata::EntityPose(if sneaking {
                    Pose::Sneaking
                } else {
                    Pose::Standing
                }),
                EntityMetadata::EntityFlags(player.entity_flags()),
            ];
            self.broadcast_entity_metadata_update(client, metadata, false);
        }
        Ok(())
    }
    pub async fn handle_sprinting(
        &self,
        _proxy: &Proxy,
        client: &Client<Handler, Proxy>,
        sprinting: bool,
    ) -> Result<(), ConnectionError> {
        if let Some(sprinting) = client.handler.on_sprint(client, sprinting).await? {
            let mut player = client.player.write().await;
            player.sprinting = sprinting;
            player.first_sprinting_hit = sprinting;
            let metadata = vec![EntityMetadata::EntityFlags(player.entity_flags())];
            self.broadcast_entity_metadata_update(client, metadata, false);
        }
        Ok(())
    }
    #[allow(unused_must_use)]
    pub async fn handle_interact(
        &self,
        _proxy: &Proxy,
        client: &Client<Handler, Proxy>,
        interact: Interact,
    ) -> Result<(), ConnectionError> {
        client.handler.on_interact(client, interact).await?;
        Ok(())
    }
    #[allow(unused_must_use)]
    pub async fn handle_swing(
        &self,
        _proxy: &Proxy,
        client: &Client<Handler, Proxy>,
        hand: Hand,
    ) -> Result<(), ConnectionError> {
        client.player.write().await.attack_strength_ticker = 0;
        for other_client in self.player_list.iter() {
            if other_client.client_data.uuid == client.client_data.uuid {
                continue;
            }
            other_client.send_entity_animation(
                client.client_data.entity_id,
                match hand {
                    Hand::MainHand => EntityAnimationType::SwingMainArm,
                    Hand::OffHand => EntityAnimationType::SwingOffhand,
                },
            );
        }
        client.handler.on_swing_hand(client, hand).await?;
        Ok(())
    }
    #[allow(unused_must_use)]
    pub fn broadcast_chat(&self, message: String) {
        // TODO: join futures
        for client in self.player_list.iter() {
            client.show_chat_message(message.clone());
        }
    }
    #[allow(unused_must_use)]
    pub async fn broadcast_entity_move(
        &self,
        client: &Client<Handler, Proxy>,
        position: Option<Vec3>,
        previous_position: Vec3,
        rotation: Option<Rotation>,
        previous_rotation: Rotation,
        on_ground: bool,
    ) {
        for other_client in self.player_list.iter() {
            if client.client_data.entity_id == other_client.client_data.entity_id {
                continue;
            }
            other_client.__move_entity(
                client.client_data.entity_id,
                position.clone(),
                previous_position.clone(),
                rotation.clone(),
                previous_rotation.clone(),
                on_ground,
            );
        }
    }
    #[allow(unused_must_use)]
    pub fn broadcast_entity_metadata_update(
        &self,
        client: &Client<Handler, Proxy>,
        metadata: Vec<EntityMetadata>,
        update_all: bool,
    ) {
        for other_client in self.player_list.iter() {
            if client.client_data.entity_id == other_client.client_data.entity_id && !update_all {
                continue;
            }
            other_client.__update_entity_metadata(client.client_data.entity_id, metadata.clone());
        }
    }
    #[allow(unused_must_use)]
    pub async fn broadcast_player_leave(&self, client: &Client<Handler, Proxy>) {
        for other_client in self.player_list.iter() {
            if other_client.client_data.entity_id == client.client_data.entity_id {
                continue;
            }
            other_client.__remove_player(client.client_data.entity_id, client.client_data.uuid);
        }
    }
    #[allow(unused_must_use)]
    pub async fn broadcast_player_join(&self, client: &Client<Handler, Proxy>) {
        for other_client in self.player_list.iter() {
            if other_client.client_data.entity_id == client.client_data.entity_id {
                continue;
            }
            let player = client.player.read().await;
            other_client.__add_player(
                client.client_data.clone(),
                player.gamemode.clone() as u8,
                player.position.clone(),
                player.rotation.clone(),
            );
        }
    }
}
