use dashmap::DashMap;
use rsa::{PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

use crate::player::Player;

use self::game_settings::GameSettings;

pub struct Server {
    pub player_list: DashMap<i32, Player>,
    pub encryption: Encryption,
    pub game_settings: GameSettings,
}

impl Server {
    pub fn new() -> Server {
        Server {
            player_list: DashMap::new(),
            encryption: Encryption::new(),
            game_settings: GameSettings::from("game_settings.json".to_string()),
        }
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

        let key_bytes = &pub_key.n().to_bytes_be();

        let pub_encoded_bytes = rsa_der::public_key_to_der(&key_bytes, &key_bytes);

        Encryption {
            pub_key: pub_key,
            priv_key: priv_key,
            encoded_pub: pub_encoded_bytes,
        }
    }
}

pub mod game_settings {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GameSettings {
        pub hardcore: bool,
        pub difficulty: u8,
    }

    impl GameSettings {
        pub fn from(path: String) -> GameSettings {
            match std::fs::read_to_string(&path) {
                Ok(file) => match serde_json::from_str::<GameSettings>(&file) {
                    Ok(settings) => settings,
                    Err(e) => panic!("Failed to parse settings file: {}", e),
                },
                Err(e) => match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        std::fs::File::create(&path).unwrap();
                        let default_settings = GameSettings::default();
                        std::fs::write(path, serde_json::to_string(&default_settings).unwrap())
                            .unwrap();

                        default_settings
                    }
                    _ => panic!("Error opening file: {:?}", e),
                },
            }
        }
    }

    impl Default for GameSettings {
        fn default() -> Self {
            GameSettings {
                hardcore: false,
                difficulty: 0,
            }
        }
    }
}
