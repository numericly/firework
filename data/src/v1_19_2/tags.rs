use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;

#[derive(Deserialize, Debug, PartialEq)]
#[repr(transparent)]
#[serde(transparent)]
pub struct VarIntList(pub Vec<i32>);

lazy_static! {
    pub static ref TAGS: HashMap<String, HashMap<String, VarIntList>> = {
        let mut map = HashMap::new();
        nbt::from_reader::<_, HashMap<String, HashMap<String, VarIntList>>>(
            File::open("tags.nbt").unwrap(),
        )
        .unwrap()
        .into_iter()
        .for_each(|(k, v)| {
            map.insert(k, v);
        });

        map
    };
}
