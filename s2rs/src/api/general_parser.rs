use serde::Deserialize;
use serde_json::Value;
use super::ParsingCustomError;

pub trait GeneralParsable where Self: Sized {
    type Error;
    fn parse(data: &GeneralParser) -> Result<Self, Self::Error>;
    fn parse_vec(data: &[GeneralParser]) -> Result<Vec<Self>, Self::Error> {
        let mut result = Vec::new();
        for item in data {
            result.push(Self::parse(item)?);
        }
        Ok(result)
    }
}

#[derive(Clone)]
pub struct GeneralParser {
    value: Value
}

impl GeneralParser {
    pub fn parse<T: GeneralParsable>(&self) -> Result<T, T::Error> {
        T::parse(self)
    }
}

impl<'de> Deserialize<'de> for GeneralParser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        let value = Value::deserialize(deserializer)?;
        Ok(Self::from(value))
    }
}

impl From<Value> for GeneralParser {
    fn from(value: Value) -> Self {
        Self {
            value
        }
    }
}

type ParsingResult<T> = Result<T, ParsingCustomError>;
impl GeneralParser {
    #[allow(unused)]
    pub fn bool(&self) -> ParsingResult<bool> {
        self.value.as_bool().ok_or(ParsingCustomError)
    }

    pub fn string(&self) -> ParsingResult<String> {
        Ok(self.value.as_str().ok_or(ParsingCustomError)?.to_owned())
    }

    pub fn str(&self) -> ParsingResult<&str> {
        self.value.as_str().ok_or(ParsingCustomError)
    }

    pub fn u64(&self) -> ParsingResult<u64> {
        self.value.as_u64().ok_or(ParsingCustomError)
    }

    pub fn u8(&self) -> ParsingResult<u8> {
        self.value.as_u64().ok_or(ParsingCustomError)?.try_into().ok().ok_or(ParsingCustomError)
    }

    pub fn option<T>(&self, change: impl FnOnce(Self) -> T) -> Option<T> {
        if self.value.is_null() {
            None
        } else {
            Some(change(self.clone()))
        }
    }

    #[allow(unused)]
    pub fn array(&self) -> ParsingResult<Vec<Self>> {
        let mut result = Vec::new();
        for value in self.value.as_array().ok_or(ParsingCustomError)?.iter().cloned() {
            result.push(Self::from(value))
        }
        Ok(result)
    }

    #[allow(unused)]
    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn i<I: serde_json::value::Index>(&self, index: I) -> Self {
        Self::from(self.value[index].to_owned())
    }
}
