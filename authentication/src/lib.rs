use std::io::Write;

use num_bigint::BigInt;
use protocol_core::SerializeField;
use protocol_derive::DeserializeField;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub properties: Vec<ProfileProperty>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, DeserializeField, Clone)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl SerializeField for ProfileProperty {
    fn serialize<W: Write>(&self, mut writer: W) {
        SerializeField::serialize(&self.name, &mut writer);
        SerializeField::serialize(&self.value, &mut writer);
        SerializeField::serialize(&self.signature, &mut writer);
    }
}

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("Failed to make request")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse response")]
    ParseError(#[from] serde_json::Error),
}

pub async fn authenticate(
    shared_secret: &[u8],
    pub_key: &[u8],
    username: String,
) -> Result<Profile, AuthenticationError> {
    let server_hash = hash_server("", shared_secret, pub_key);
    let formatted_url = format!(
        "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}",
        username, server_hash
    );
    let text = reqwest::get(&formatted_url).await?.text().await?;

    Ok(serde_json::from_str::<Profile>(&text)?)
}

fn hash_server(server_id: &str, shared_secret: &[u8], pub_key: &[u8]) -> String {
    let mut hash = Sha1::new();

    hash.update(server_id.as_bytes());
    hash.update(&shared_secret);
    hash.update(pub_key);

    let formatted = BigInt::from_signed_bytes_be(hash.finalize().as_slice());

    format!("{:x}", formatted)
}
