use dashmap::DashMap;
use rsa::{PublicKeyParts, RsaPrivateKey, RsaPublicKey};

use crate::{game_settings::GameSettings, player::Player};

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

        let pub_encoded_bytes =
            rsa_der::public_key_to_der(&pub_key.n().to_bytes_be(), &pub_key.e().to_bytes_be());

        Encryption {
            pub_key: pub_key,
            priv_key: priv_key,
            encoded_pub: pub_encoded_bytes,
        }
    }
}
