use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq)]
#[repr(transparent)]
#[serde(transparent)]
pub struct VarIntList(pub Vec<i32>);

lazy_static! {
    pub static ref TAGS: HashMap<String, HashMap<String, VarIntList>> = {
        let mut map = HashMap::new();
        nbt::from_reader::<_, HashMap<String, HashMap<String, VarIntList>>>(TAGS_BYTES.as_slice())
            .unwrap()
            .into_iter()
            .for_each(|(k, v)| {
                map.insert(k, v);
            });

        map
    };
}

lazy_static! {
    pub static ref REGISTRY: Vec<u8> = include_bytes!("../registry.nbt").to_vec();
    pub static ref TAGS_BYTES: Vec<u8> = include_bytes!("../tags.nbt").to_vec();
}
