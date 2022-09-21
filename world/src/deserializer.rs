use std::io::Read;

use serde::{
    de::{self, Visitor},
    Deserialize,
};

pub struct World {}

impl<'de> Deserialize<'de> for World {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HeaderVisitor;

        impl<'de> Visitor<'de> for HeaderVisitor {
            fn expecting(
                &self,
                formatter: &mut serde::__private::fmt::Formatter,
            ) -> serde::__private::fmt::Result {
                todo!()
            }
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
            }
        }
        deserializer.deserialize_byte_buf(visitor)
    }
}
