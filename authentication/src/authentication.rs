use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "id")]
    pub id: String,
    pub name: String,
    pub properties: Vec<ProfileProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: String,
}

pub async fn authenticate(
    shared_secret: &[u8],
    pub_key: &Vec<u8>,
    username: &String,
    ip: &String,
) -> Profile {
    let server_hash = hash_server("", shared_secret, pub_key);
    let formatted_url = format!("https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}&ip={}", username, server_hash, ip);
    let text = reqwest::get(&formatted_url).await.unwrap().text().await;

    serde_json::from_str::<Profile>(&text.unwrap()).unwrap()
}

fn hash_server(server_id: &str, shared_secret: &[u8], pub_key: &Vec<u8>) -> String {
    let mut hash = Sha1::new();

    hash.update(server_id.as_bytes());
    hash.update(&shared_secret);
    hash.update(pub_key);

    let formatted = BigInt::from_signed_bytes_be(hash.finalize().as_slice());

    format!("{:x}", formatted)
}
