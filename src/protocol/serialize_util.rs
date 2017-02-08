use serde::de;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize_i32_as_str<S: Serializer>(n: &i32, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str((*n).to_string().as_str())
}

pub fn deserialize_string_as_i32<D: Deserializer>(deserializer: D) -> Result<i32, D::Error> {
    String::deserialize(deserializer).and_then(|s| {
        s.parse::<i32>().map_err(|_| de::Error::custom(de::Unexpected::Other("non-numeric string")))
    })
}
