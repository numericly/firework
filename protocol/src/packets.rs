pub mod server_bound {

    use crate::{deserializer::IncomingPacketData, protocol::ConnectionState};

    #[derive(Debug)]
    pub enum ServerBoundPacket {
        Handshake(Handshake),
        LoginStart(LoginStart),

        StatusRequest(StatusRequest),
        PingRequest(PingRequest),
        EncryptionResponse(EncryptionResponse),

        ClientInformation(ClientInformation),
        CloseContainer(CloseContainer),
        PluginMessage(PluginMessage),
        SetPlayerPosition(SetPlayerPosition),
        SetPlayerAndRotationPosition(SetPlayerAndRotationPosition),
        PlayerCommand(PlayerCommand),
        SetHeldItem(SetHeldItem),
    }

    impl ServerBoundPacket {
        pub fn from(
            state: &ConnectionState,
            mut packet_data: IncomingPacketData,
        ) -> Result<ServerBoundPacket, String> {
            let packet_id = packet_data.read_var_int()?;

            match state {
                ConnectionState::HandShaking => match packet_id {
                    0 => Handshake::deserialize(packet_data),
                    _ => Err(format!(
                        "Unknown packet id {} for state {:?}",
                        packet_id, state
                    )),
                },
                ConnectionState::Status => match packet_id {
                    0 => StatusRequest::deserialize(packet_data),
                    1 => PingRequest::deserialize(packet_data),
                    _ => Err(format!(
                        "Unknown packet id {} for state {:?}",
                        packet_id, state
                    )),
                },
                ConnectionState::Login => match packet_id {
                    0 => LoginStart::deserialize(packet_data),
                    1 => EncryptionResponse::deserialize(packet_data),
                    _ => Err(format!(
                        "Unknown packet id {} for state {:?}",
                        packet_id, state
                    )),
                },
                ConnectionState::Play => match packet_id {
                    8 => ClientInformation::deserialize(packet_data),
                    12 => CloseContainer::deserialize(packet_data),
                    13 => PluginMessage::deserialize(packet_data),
                    20 => SetPlayerPosition::deserialize(packet_data),
                    21 => SetPlayerAndRotationPosition::deserialize(packet_data),
                    30 => PlayerCommand::deserialize(packet_data),
                    40 => SetHeldItem::deserialize(packet_data),
                    _ => Err(format!(
                        "Unknown packet id {} for state {:?}",
                        packet_id, state
                    )),
                },
            }
        }
    }

    pub trait Deserialize {
        fn deserialize(packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String>;
    }

    #[derive(Debug)]
    pub struct Handshake {
        pub protocol_version: i32,
        pub server_address: String,
        pub server_port: u16,
        pub next_state: ConnectionState,
    }

    impl Deserialize for Handshake {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let protocol_version = packet_data.read_var_int()?;
            let server_address = packet_data.read_string()?;
            let server_port = packet_data.read_u16()?;
            let next_state = match packet_data.read_var_int()? {
                1 => ConnectionState::Status,
                2 => ConnectionState::Login,
                _ => return Err("Invalid next state".to_string()),
            };

            Ok(ServerBoundPacket::Handshake(Handshake {
                protocol_version: protocol_version,
                server_address: server_address,
                server_port: server_port,
                next_state: next_state,
            }))
        }
    }

    #[derive(Debug)]
    pub struct LoginStart {
        pub username: String,
    }

    impl Deserialize for LoginStart {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let username = packet_data.read_string()?;

            Ok(ServerBoundPacket::LoginStart(LoginStart { username }))
        }
    }

    #[derive(Debug)]
    pub struct StatusRequest {}

    impl Deserialize for StatusRequest {
        fn deserialize(_packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            Ok(ServerBoundPacket::StatusRequest(StatusRequest {}))
        }
    }

    #[derive(Debug)]
    pub struct PingRequest {
        pub payload: i64,
    }

    impl Deserialize for PingRequest {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let payload = packet_data.read_long()?;

            Ok(ServerBoundPacket::PingRequest(PingRequest { payload }))
        }
    }

    #[derive(Debug)]
    pub struct EncryptionResponse {
        pub shared_secret: Vec<u8>,
        pub verify_token: Option<Vec<u8>>,
    }

    impl Deserialize for EncryptionResponse {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let shared_secret_length = packet_data.read_var_int()?;
            let shared_secret = packet_data.read_bytes(shared_secret_length as usize)?;
            let verify_token_length = packet_data.read_var_int()?;
            let verify_token = packet_data.read_bytes(verify_token_length as usize)?;

            Ok(ServerBoundPacket::EncryptionResponse(EncryptionResponse {
                shared_secret: shared_secret,
                verify_token: Some(verify_token),
            }))
        }
    }

    #[derive(Debug)]
    pub struct ClientInformation {
        pub locale: String,
        pub view_distance: u8,
        pub chat_mode: ChatMode,
        pub chat_colors: bool,
        pub displayed_skin_parts: u8,
        pub main_hand: MainHand,
        pub enable_text_filtering: bool,
        pub allow_server_listings: bool,
    }

    #[derive(Debug)]
    pub enum ChatMode {
        Enabled,
        CommandsOnly,
        Hidden,
    }

    #[derive(Debug)]
    pub enum MainHand {
        Left,
        Right,
    }

    impl Deserialize for ClientInformation {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let locale = packet_data.read_string()?;
            let view_distance = packet_data.read_unsigned_byte()?;
            let chat_mode = match packet_data.read_var_int()? {
                0 => ChatMode::Enabled,
                1 => ChatMode::CommandsOnly,
                2 => ChatMode::Hidden,
                _ => return Err("Invalid chat mode".to_string()),
            };
            let chat_colors = packet_data.read_boolean()?;
            let displayed_skin_parts = packet_data.read_unsigned_byte()?;
            let main_hand = match packet_data.read_var_int()? {
                0 => MainHand::Left,
                1 => MainHand::Right,
                _ => return Err("Invalid main hand".to_string()),
            };
            let enable_text_filtering = packet_data.read_boolean()?;
            let allow_server_listings = packet_data.read_boolean()?;

            Ok(ServerBoundPacket::ClientInformation(ClientInformation {
                locale: locale,
                view_distance: view_distance,
                chat_mode: chat_mode,
                chat_colors: chat_colors,
                displayed_skin_parts: displayed_skin_parts,
                main_hand: main_hand,
                enable_text_filtering: enable_text_filtering,
                allow_server_listings: allow_server_listings,
            }))
        }
    }

    #[derive(Debug)]
    pub struct CloseContainer {
        pub window_id: u8,
    }

    impl Deserialize for CloseContainer {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let window_id = packet_data.read_unsigned_byte()?;

            Ok(ServerBoundPacket::CloseContainer(CloseContainer {
                window_id,
            }))
        }
    }

    #[derive(Debug)]
    pub struct PluginMessage {
        pub channel: String,
        pub data: Vec<u8>,
    }

    impl Deserialize for PluginMessage {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let channel = packet_data.read_string()?;
            let data_length = packet_data.read_var_int()?;
            let data = packet_data.read_bytes(data_length as usize)?;

            Ok(ServerBoundPacket::PluginMessage(PluginMessage {
                channel: channel,
                data: data,
            }))
        }
    }

    #[derive(Debug)]
    pub struct SetPlayerPosition {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub on_ground: bool,
    }

    impl Deserialize for SetPlayerPosition {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let x = packet_data.read_double()?;
            let y = packet_data.read_double()?;
            let z = packet_data.read_double()?;
            let on_ground = packet_data.read_boolean()?;

            Ok(ServerBoundPacket::SetPlayerPosition(SetPlayerPosition {
                x: x,
                y: y,
                z: z,
                on_ground: on_ground,
            }))
        }
    }

    #[derive(Debug)]
    pub struct SetPlayerAndRotationPosition {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
        pub on_ground: bool,
    }

    impl Deserialize for SetPlayerAndRotationPosition {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let x = packet_data.read_double()?;
            let y = packet_data.read_double()?;
            let z = packet_data.read_double()?;
            let yaw = packet_data.read_float()?;
            let pitch = packet_data.read_float()?;
            let on_ground = packet_data.read_boolean()?;

            Ok(ServerBoundPacket::SetPlayerAndRotationPosition(
                SetPlayerAndRotationPosition {
                    x: x,
                    y: y,
                    z: z,
                    yaw: yaw,
                    pitch: pitch,
                    on_ground: on_ground,
                },
            ))
        }
    }

    #[derive(Debug)]
    pub struct SetHeldItem {
        pub slot: u16,
    }

    impl Deserialize for SetHeldItem {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let slot = packet_data.read_short()?;

            Ok(ServerBoundPacket::SetHeldItem(SetHeldItem {
                slot: slot as u16,
            }))
        }
    }

    #[derive(Debug)]
    pub struct PlayerCommand {
        pub entity_id: i32,
        pub action: PlayerCommandAction,
        pub jump_boost: i32,
    }

    #[derive(Debug)]
    pub enum PlayerCommandAction {
        StartSneaking,
        StopSneaking,
        LeaveBed,
        StartSprinting,
        StopSprinting,
        StartJumpWithHorse,
        StopJumpWithHorse,
        OpenHorseInventory,
        StartFlyingWithElytra,
    }

    impl Deserialize for PlayerCommand {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let entity_id = packet_data.read_var_int()?;
            let action = match packet_data.read_var_int()? {
                0 => PlayerCommandAction::StartSneaking,
                1 => PlayerCommandAction::StopSneaking,
                2 => PlayerCommandAction::LeaveBed,
                3 => PlayerCommandAction::StartSprinting,
                4 => PlayerCommandAction::StopSprinting,
                5 => PlayerCommandAction::StartJumpWithHorse,
                6 => PlayerCommandAction::StopJumpWithHorse,
                7 => PlayerCommandAction::OpenHorseInventory,
                8 => PlayerCommandAction::StartFlyingWithElytra,
                _ => return Err("Invalid player command action".to_string()),
            };
            let jump_boost = packet_data.read_var_int()?;

            Ok(ServerBoundPacket::PlayerCommand(PlayerCommand {
                entity_id: entity_id,
                action: action,
                jump_boost: jump_boost,
            }))
        }
    }
}

pub mod client_bound {
    use quartz_nbt::NbtCompound;

    use crate::serializer::OutboundPacketData;

    pub trait Serialize {
        fn serialize(&self) -> OutboundPacketData {
            let mut packet_data = OutboundPacketData::new();

            // Write packet with packet id
            packet_data.write_var_int(self.packet_id());
            self.serialize_into(&mut packet_data);

            packet_data
        }
        fn serialize_into(&self, packet_data: &mut OutboundPacketData);
        fn packet_id(&self) -> i32;
    }

    #[derive(Debug)]
    pub struct ServerStatus {
        pub server_data: String,
    }

    impl Serialize for ServerStatus {
        fn serialize_into(&self, packet_data: &mut OutboundPacketData) {
            packet_data.write_string(&self.server_data);
        }
        fn packet_id(&self) -> i32 {
            0
        }
    }

    #[derive(Debug)]
    pub struct PingResponse {
        pub payload: i64,
    }

    impl Serialize for PingResponse {
        fn serialize_into(&self, packet_data: &mut OutboundPacketData) {
            packet_data.write_signed_long(self.payload);
        }
        fn packet_id(&self) -> i32 {
            1
        }
    }

    #[derive(Debug)]
    pub struct EncryptionRequest {
        pub server_id: String,
        pub public_key: Vec<u8>,
        pub verify_token: Vec<u8>,
    }

    impl Serialize for EncryptionRequest {
        fn serialize_into(&self, packet_data: &mut OutboundPacketData) {
            packet_data.write_string(&self.server_id);
            packet_data.write_var_int(self.public_key.len() as i32);
            packet_data.write_bytes(&self.public_key);
            packet_data.write_var_int(self.verify_token.len() as i32);
            packet_data.write_bytes(&self.verify_token);
        }
        fn packet_id(&self) -> i32 {
            1
        }
    }

    #[derive(Debug)]
    pub struct LoginSuccess {
        pub id: u128,
        pub username: String,
        pub properties: Vec<LoginSuccessProperty>,
    }

    #[derive(Debug)]
    pub struct LoginSuccessProperty {
        pub name: String,
        pub value: String,
        pub signature: Option<String>,
    }

    impl Serialize for LoginSuccess {
        fn serialize_into(&self, packet_data: &mut OutboundPacketData) {
            packet_data.write_uuid(self.id);
            packet_data.write_string(&self.username);
            packet_data.write_var_int(self.properties.len() as i32);
            for property in &self.properties {
                packet_data.write_string(&property.name);
                packet_data.write_string(&property.value);
                match &property.signature {
                    Some(signature) => {
                        packet_data.write_bool(true);
                        packet_data.write_string(signature);
                    }
                    None => packet_data.write_bool(false),
                }
            }
        }
        fn packet_id(&self) -> i32 {
            2
        }
    }

    #[derive(Debug)]
    pub struct Disconnect {
        pub reason: String,
    }

    impl Serialize for Disconnect {
        fn serialize_into(&self, packet_data: &mut OutboundPacketData) {
            packet_data.write_string(&self.reason);
        }
        fn packet_id(&self) -> i32 {
            25
        }
    }
    #[derive(Debug)]
    pub struct WorldLogin {
        pub entity_id: i32,
        pub is_hardcore: bool,
        pub game_mode: u8,
        pub previous_game_mode: i8,
        pub dimensions: Vec<String>,
        pub registry_codec: NbtCompound,
        pub dimension_type: String,
        pub dimension_name: String,
        pub hashed_seed: i64,
        pub max_players: i32,
        pub view_distance: i32,
        pub simulation_distance: i32,
        pub reduced_debug_info: bool,
        pub enable_respawn_screen: bool,
        pub is_debug: bool,
        pub is_flat: bool,
        pub has_death_location: bool,
        pub death_dimension_name: Option<String>,
        pub death_position: Option<(i32, i32, i32)>,
    }

    impl Serialize for WorldLogin {
        fn serialize_into(&self, packet_data: &mut OutboundPacketData) {
            packet_data.write_signed_int(self.entity_id);
            packet_data.write_bool(self.is_hardcore);
            packet_data.write_unsigned_byte(self.game_mode);
            packet_data.write_signed_byte(self.previous_game_mode);
            packet_data.write_var_int(self.dimensions.len() as i32);
            for dimension in &self.dimensions {
                packet_data.write_string(dimension);
            }
            packet_data.write_nbt(&self.registry_codec);
            packet_data.write_string(&self.dimension_type);
            packet_data.write_string(&self.dimension_name);
            packet_data.write_signed_long(self.hashed_seed);
            packet_data.write_var_int(self.max_players);
            packet_data.write_var_int(self.view_distance);
            packet_data.write_var_int(self.simulation_distance);
            packet_data.write_bool(self.reduced_debug_info);
            packet_data.write_bool(self.enable_respawn_screen);
            packet_data.write_bool(self.is_debug);
            packet_data.write_bool(self.is_flat);
            packet_data.write_bool(self.has_death_location);
        }
        fn packet_id(&self) -> i32 {
            37
        }
    }
}