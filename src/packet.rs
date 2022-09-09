pub mod c2s_packet {
    use crate::{
        client::client_data::State,
        packet_parser::parser::{self, IndexedBuffer, PacketBuffer},
    };

    use super::s2c_packet::{PingResponse, S2CPacket};

    pub fn get_packet(
        packet_buffer: PacketBuffer,
        packet_id: &i32,
        state: &State,
    ) -> Result<C2S, String> {
        match state {
            State::HandShaking => get_handshake_packet(packet_buffer, packet_id),
            State::Login => get_login_packet(packet_buffer, packet_id),
            State::Status => get_status_packet(packet_buffer, packet_id),
            State::Play => get_play_packet(packet_buffer, packet_id),
        }
    }

    fn get_handshake_packet(buf: PacketBuffer, packet_id: &i32) -> Result<C2S, String> {
        match packet_id {
            0 => Ok(C2S::Handshake(Handshake::parse(buf))),
            //0xFE: legacy server list ping
            _ => Err(format!(
                "Could not find packet with ID: {}, state: Handshake",
                packet_id
            )),
        }
    }

    fn get_login_packet(buf: PacketBuffer, packet_id: &i32) -> Result<C2S, String> {
        match packet_id {
            0 => Ok(C2S::LoginStart(LoginStart::parse(buf))), //TODO two more login packets
            1 => Ok(C2S::EncryptionResponse(EncryptionResponse::parse(buf))),
            //2 => Login Plugin Response
            _ => Err(format!(
                "Could not find packet with ID: {}, state: Login",
                packet_id
            )),
        }
    }

    fn get_status_packet(buf: PacketBuffer, packet_id: &i32) -> Result<C2S, String> {
        match packet_id {
            0 => Ok(C2S::StatusRequest(StatusRequest::parse(buf))),
            1 => Ok(C2S::PingRequest(PingRequest::parse(buf))),
            _ => Err(format!(
                "Could not find packet with ID: {}, state: Status",
                packet_id
            )),
        }
    }

    fn get_play_packet(_buf: PacketBuffer, packet_id: &i32) -> Result<C2S, String> {
        match packet_id {
            _ => Err(format!(
                "Could not find packet with ID: {}, state: Handshake",
                packet_id
            )),
        }
    }

    pub trait Packet<T> {
        fn parse(indexed_buffer: PacketBuffer) -> T;
    }

    #[derive(Debug)]
    pub enum C2S {
        //Handshaking
        Handshake(Handshake),
        LoginStart(LoginStart),
        //Status
        StatusRequest(StatusRequest),
        PingRequest(PingRequest),
        EncryptionResponse(EncryptionResponse),
    }

    #[derive(Debug)]
    pub struct Handshake {
        pub protocol_version: i32,
        pub server_address: String,
        pub server_port: u16,
        pub next_state: i32,
    }

    impl Packet<Handshake> for Handshake {
        fn parse(buf: PacketBuffer) -> Handshake {
            Handshake {
                protocol_version: buf.parse_var_int(),
                server_address: buf.parse_string(),
                server_port: buf.parse_unsigned_short(),
                next_state: buf.parse_var_int(),
            }
        }
    }

    #[derive(Debug)]
    pub struct LoginStart {
        pub player_name: String,
    }

    impl Packet<LoginStart> for LoginStart {
        fn parse(buf: PacketBuffer) -> LoginStart {
            LoginStart {
                player_name: buf.parse_string(),
            }
        }
    }
    #[derive(Debug)]
    pub struct StatusRequest {}

    impl Packet<StatusRequest> for StatusRequest {
        fn parse(_buf: PacketBuffer) -> StatusRequest {
            StatusRequest {}
        }
    }
    #[derive(Debug)]
    pub struct PingRequest {
        pub payload: i64,
    }

    impl Packet<PingRequest> for PingRequest {
        fn parse(buf: PacketBuffer) -> PingRequest {
            PingRequest {
                payload: buf.parse_signed_long(),
            }
        }
    }

    #[derive(Debug)]
    pub struct EncryptionResponse {
        pub shared_secret: Vec<u8>,
        pub verify_token: Option<Vec<u8>>,
        pub salt: Option<i64>,
    }

    impl Packet<EncryptionResponse> for EncryptionResponse {
        fn parse(buf: PacketBuffer) -> EncryptionResponse {
            EncryptionResponse {
                shared_secret: buf.parse_byte_array(),
                verify_token: None,
                salt: None,
            }
        }
    }
}

pub mod s2c_packet {
    use aes::cipher::{AsyncStreamCipher, KeyIvInit};
    use std::{io::Write, net::TcpStream, rc::Rc, sync::Mutex};

    type Aes128Cfb8Enc = cfb8::Encryptor<aes::Aes128>;

    use crate::{
        cfb8::CarrotCakeCipher,
        packet_serializer::serializer::{
            serialize_byte_array, serialize_signed_long, serialize_string, serialize_uuid,
            serialize_var_int,
        },
        server::server_data,
    };

    use super::cfb8::CBF8Cipher;

    pub trait S2CPacket {
        fn write(&mut self) -> Vec<u8>;
        fn write_packet(&mut self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
            let packet_data = &self.write();

            stream.write_all(&serialize_var_int(Vec::new(), packet_data.len() as i32))?;
            stream.write_all(&packet_data)?;

            Ok(())
        }
        fn write_encrypted_packet(
            &mut self,
            stream: &mut TcpStream,
            cipher: &mut CarrotCakeCipher,
        ) -> Result<(), std::io::Error> {
            let mut packet_data = self.write();
            let mut full_packet_data = [
                serialize_var_int(Vec::new(), packet_data.len() as i32),
                packet_data,
            ]
            .concat();

            cipher.encrypt(&mut full_packet_data);

            stream.write_all(&full_packet_data)?;

            // stream.write_all(&serialize_var_int(Vec::new(), packet_data.len() as i32))?;
            // stream.write_all(&packet_data)?;
            Ok(())
        }
    }

    #[derive(Debug)]
    pub struct ServerStatus {
        pub server_data: String,
    }

    impl S2CPacket for ServerStatus {
        fn write(&mut self) -> Vec<u8> {
            let mut data = Vec::new();
            data = serialize_var_int(data, 0);
            data = serialize_string(data, &mut self.server_data);
            data
        }
    }

    #[derive(Debug)]
    pub struct PingResponse {
        pub payload: i64,
    }

    impl S2CPacket for PingResponse {
        fn write(&mut self) -> Vec<u8> {
            let mut data = Vec::new();
            data = serialize_var_int(data, 1);
            data = serialize_signed_long(data, self.payload);
            data
        }
    }

    #[derive(Debug)]
    pub struct EncryptionRequest {
        pub server_id: String,
        pub public_key_length: i32,
        pub public_key: Vec<u8>,
        pub verify_token_length: i32,
        pub verify_token: Vec<u8>,
    }

    impl S2CPacket for EncryptionRequest {
        fn write(&mut self) -> Vec<u8> {
            let mut data = Vec::new();
            data = serialize_var_int(data, 1);
            data = serialize_string(data, &self.server_id);
            data = serialize_var_int(data, self.public_key_length);
            data = serialize_byte_array(data, &mut self.public_key);
            data = serialize_var_int(data, self.verify_token_length);
            data = serialize_byte_array(data, &mut self.verify_token);
            data
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

    impl S2CPacket for LoginSuccess {
        fn write(&mut self) -> Vec<u8> {
            let mut data = Vec::new();
            data = serialize_var_int(data, 2);
            data = serialize_uuid(data, self.id);
            data = serialize_string(data, &self.username);
            data = serialize_var_int(data, 0 as i32);
            data
        }
    }

    #[derive(Debug)]
    pub struct LoginPlay {
        pub entity_id: i32, //entity id of the player
        pub hardcore: bool,
        pub gamemode: u8, //0 = survival, 1 = creative, 2 = adventure, 3 = spectator
        pub previous_gamemode: u8, //0 = survival, 1 = creative, 2 = adventure, 3 = spectator
        pub dimension_count: i32,
        pub dimension_names: Vec<String>,
        pub registry_codec: String, //NBT Tag Compound (idk what that means but it's a string and probably nbt format)
        pub dimension_type: String,
        pub dimension_name: String,
        pub hashed_seed: i64,  //first 8 bytes of sha256 of world seed
        pub max_players: i32,  //unused
        pub view_distance: u8, //2-32
        pub simulation_distance: u8,
        pub reduced_debug_info: bool,    //should be true
        pub enable_respawn_screen: bool, //true: show respawn screen, false: instantly respawn
        pub is_debug: bool,              //debug world
        pub is_flat: bool,               //superflat world
        pub has_death_location: bool,
        pub death_dimension_name: Option<String>,
        pub death_position: Option<[f64; 3]>,
    }

    impl S2CPacket for LoginPlay {
        fn write(&mut self) -> Vec<u8> {
            let mut data = Vec::new();
            data = serialize_var_int(data, 2);
            //can you add these will i have to not be late to school
            data
        }
    }
}

pub mod cfb8 {

    use aes::cipher::consts::U16;
    use aes::cipher::generic_array::sequence::Lengthen;
    use aes::cipher::KeyInit;
    use aes::{
        cipher::{generic_array::GenericArray, BlockEncrypt},
        Aes128,
    };

    pub struct CBF8Cipher {
        iv: GenericArray<u8, U16>,
        tmp: GenericArray<u8, U16>,
        cipher: Aes128,
    }

    const SIZE: usize = 16;

    impl CBF8Cipher {
        pub fn new(key: &[u8], iv: &[u8]) -> Result<CBF8Cipher, String> {
            if key.len() != SIZE {
                return Err(format!("key is not {} bytes", SIZE));
            }
            if iv.len() != SIZE {
                return Err(format!("key is not {} bytes", SIZE));
            }

            let mut iv_fixed = [0u8; SIZE];
            iv_fixed.copy_from_slice(key);

            let mut key_fixed = [0u8; SIZE];
            key_fixed.copy_from_slice(key);

            let tmp = [0u8; SIZE];

            Ok(CBF8Cipher {
                iv: GenericArray::from(iv_fixed),
                tmp: GenericArray::from(tmp),
                cipher: Aes128::new(&GenericArray::from(key_fixed)),
            })
        }
        pub fn encrypt(&mut self, data: &mut [u8]) {
            // iterate through the data encrypting it
            for i in 0..data.len() {
                // encrypt the iv
                self.cipher.encrypt_block(&mut self.iv);

                // xor the data with the iv
                data[i] ^= self.iv[0];

                // shift the iv left
                self.iv.copy_within(1..SIZE, 0);
                self.iv[SIZE - 1] = data[i];
            }
        }
    }
}
