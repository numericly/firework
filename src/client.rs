pub mod client_data {
    use crate::authentication::Profile;

    pub struct Client {
        pub state: State,
        pub packet_encryption: PacketEncryption,
        pub username: Option<String>,
        pub profile: Option<Profile>,
    }

    type Aes128Cfb8Enc = cfb8::Encryptor<aes::Aes128>;
    type Aes128Cfb8Dec = cfb8::Decryptor<aes::Aes128>;

    impl Client {
        pub fn new() -> Client {
            Client {
                state: State::HandShaking,
                packet_encryption: PacketEncryption::new(),
                username: None,
                profile: None,
            }
        }
    }

    #[derive(Debug)]
    pub enum State {
        HandShaking,
        Status,
        Login,
        Play,
    }

    pub struct PacketEncryption {
        pub encryptor: Option<Aes128Cfb8Enc>,
        pub decryptor: Option<Aes128Cfb8Dec>,
    }

    impl PacketEncryption {
        pub fn new() -> PacketEncryption {
            PacketEncryption {
                encryptor: None,
                decryptor: None,
            }
        }
    }
}
