use serde::{Deserialize, Deserializer};
use serde::de;

pub fn deserialize_string_as_u32<D: Deserializer>(deserializer: D) -> Result<u32, D::Error> {
    String::deserialize(deserializer).and_then(|s| {
        s.parse::<u32>().map_err(|_| de::Error::custom(de::Unexpected::Other("non-numeric string")))
    })
}
