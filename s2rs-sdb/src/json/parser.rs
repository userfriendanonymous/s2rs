use serde::Deserialize;
use serde_json::Value;

#[derive(Debug)]
pub enum Error {
    Expected(ErrorExpected)
}

#[derive(Debug)]
pub struct ErrorExpected {
    pub found: Value,
    pub expected: ErrorExpectedVariant
}

#[derive(Debug)]
pub enum ErrorExpectedVariant {
    Bool,
    String,
    U64,
    U8,
    Option,
    Array,
}

pub trait Parsable where Self: Sized {
    type Error;
    fn parse(data: &Parser) -> Result<Self, Self::Error>;
    fn parse_vec(data: &[Parser]) -> Result<Vec<Self>, Self::Error> {
        let mut result = Vec::new();
        for item in data {
            result.push(Self::parse(item)?);
        }
        Ok(result)
    }
}

#[derive(Clone)]
pub struct Parser {
    value: Value
}

impl Parser {
    pub fn parse<T: Parsable>(&self) -> Result<T, T::Error> {
        T::parse(self)
    }
}

impl<'de> Deserialize<'de> for Parser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        let value = Value::deserialize(deserializer)?;
        Ok(Self::from(value))
    }
}

impl From<Value> for Parser {
    fn from(value: Value) -> Self {
        Self {
            value
        }
    }
}

type ParsingResult<T> = Result<T, Error>;
impl Parser {
    fn error_expected(&self, expected: ErrorExpectedVariant) -> Error {
        Error::Expected(ErrorExpected { found: self.value.clone(), expected })
    }

    #[allow(unused)]
    pub fn bool(&self) -> ParsingResult<bool> {
        self.value.as_bool().ok_or_else(|| self.error_expected(ErrorExpectedVariant::Bool))
    }

    pub fn string(&self) -> ParsingResult<String> {
        Ok(self.value.as_str().ok_or_else(|| self.error_expected(ErrorExpectedVariant::String))?.to_owned())
    }

    pub fn str(&self) -> ParsingResult<&str> {
        self.value.as_str().ok_or_else(|| self.error_expected(ErrorExpectedVariant::String))
    }

    pub fn u64(&self) -> ParsingResult<u64> {
        self.value.as_u64().ok_or(self.error_expected(ErrorExpectedVariant::U64))
    }

    pub fn u8(&self) -> ParsingResult<u8> {
        let error = || self.error_expected(ErrorExpectedVariant::U8);
        self.value.as_u64().ok_or_else(error)?.try_into().ok().ok_or_else(error)
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
        let error = Error::Expected(ErrorExpected { found: self.value.clone(), expected: ErrorExpectedVariant::Bool });
        let mut result = Vec::new();
        for value in self.value.as_array().ok_or_else(|| self.error_expected(ErrorExpectedVariant::Array))?.iter().cloned() {
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
