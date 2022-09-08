pub mod server_data {
    use rsa::{PublicKeyParts, RsaPrivateKey, RsaPublicKey};

    pub struct Server {
        pub encryption: Encryption,
    }

    impl Server {
        pub fn new() -> Server {
            Server {
                encryption: Encryption::new(),
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
            // Key size
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
}
