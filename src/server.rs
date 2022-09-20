use dashmap::DashMap;
use rsa::{PublicKeyParts, RsaPrivateKey, RsaPublicKey};

use crate::player::Player;

use self::{game_settings::GameSettings, registry::Registry};

pub struct Server {
    pub player_list: DashMap<i32, Player>,
    pub encryption: Encryption,
    pub game_settings: GameSettings,
    pub registry: Registry,
}

impl Server {
    pub fn new() -> Server {
        Server {
            player_list: DashMap::new(),
            encryption: Encryption::new(),
            game_settings: GameSettings::from("game_settings.json".to_string()),
            registry: Registry::new(),
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

        let pub_encoded_bytes =
            rsa_der::public_key_to_der(&pub_key.n().to_bytes_be(), &pub_key.e().to_bytes_be());

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

pub mod registry {
    use std::{
        collections::{hash_map::DefaultHasher, HashMap},
        fs,
        hash::{Hash, Hasher},
    };

    use serde::Deserialize;

    pub struct Registry {
        pub global_block_palette: HashMap<u64, i32>,
        pub global_biome_palette: HashMap<u64, i32>,
    }

    impl Registry {
        pub fn new() -> Registry {
            Registry {
                global_block_palette: create_global_block_palette(),
                global_biome_palette: create_global_biome_palette(),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    struct JsonPaletteElement {
        pub value: String,
        pub key: i32,
        pub properties: Option<Vec<JsonPaletteProperty>>,
    }

    #[derive(Debug, Hash, Deserialize)]
    struct JsonPaletteProperty {
        pub name: String,
        pub value: String,
    }

    impl Hash for JsonPaletteElement {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
            if let Some(properties) = &self.properties {
                properties.hash(state);
            }
        }
    }

    fn create_global_block_palette() -> HashMap<u64, i32> {
        let file_content = fs::read_to_string("./block-palette.json").unwrap();

        let mut hashmap = HashMap::new();
        let json: Vec<JsonPaletteElement> = serde_json::from_str(&file_content).unwrap();

        for element in json.iter() {
            let mut hasher = DefaultHasher::new();
            element.hash(&mut hasher);
            hashmap.insert(hasher.finish(), element.key);
        }
        hashmap
    }

    fn create_global_biome_palette() -> HashMap<u64, i32> {
        let file_content = fs::read_to_string("./biome-palette.json").unwrap();

        let mut hashmap = HashMap::new();
        let json: Vec<JsonPaletteElement> = serde_json::from_str(&file_content).unwrap();

        for element in json.iter() {
            let mut hasher = DefaultHasher::new();
            element.hash(&mut hasher);
            hashmap.insert(hasher.finish(), element.key);
        }
        hashmap
    }
}
