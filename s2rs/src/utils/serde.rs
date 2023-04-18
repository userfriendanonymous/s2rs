
pub mod de {
    use serde::{Deserializer, Deserialize};
    use serde_json::Value;
    pub fn string_to_u64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        Ok(match Value::deserialize(deserializer)? {
            Value::String(s) => s.parse().map_err(serde::de::Error::custom)?,
            _ => return Err(serde::de::Error::custom("expected string"))
        })
    }
}

pub mod ser {
    use serde::Serializer;
    pub fn _u64_to_string<S>(data: &u64, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        s.serialize_str(&data.to_string())
    }
}

