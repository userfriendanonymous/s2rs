use serde::Deserialize;
use serde_json::Value;

use crate::utils::TryAs;

// #[derive(Debug)]
// pub enum Error {
//     Expected(ExpectedError)
// }

#[derive(Debug, Clone)]
pub struct ExpectedError {
    pub found: Value,
    pub expected: ExpectedErrorVariant
}

#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Clone, Copy)]
pub enum ExpectedErrorVariant {
    Bool,
    String,
    U64,
    U8,
    Array,
    U16
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

// region: TryAs

impl TryAs<u64, ExpectedError> for Parser {
    fn try_as(&self) -> ExpectedResult<u64> {
        self.u64()
    }
}

impl TryAs<u16, ExpectedError> for Parser {
    fn try_as(&self) -> ExpectedResult<u16> {
        self.u16()
    }
}

impl TryAs<String, ExpectedError> for Parser {
    fn try_as(&self) -> ExpectedResult<String> {
        self.string()
    }
}

impl TryAs<u8, ExpectedError> for Parser {
    fn try_as(&self) -> ExpectedResult<u8> {
        self.u8()
    }
}

impl TryAs<Vec<Self>, ExpectedError> for Parser {
    fn try_as(&self) -> ExpectedResult<Vec<Self>> {
        self.array()
    }
}

impl TryAs<bool, ExpectedError> for Parser {
    fn try_as(&self) -> ExpectedResult<bool> {
        self.bool()
    }
}

impl<T, E> TryAs<Option<T>, E> for Parser where Self: TryAs<T, E> {
    fn try_as(&self) -> Result<Option<T>, E> {
        self.typed_option()
    }
}

impl<T> TryAs<Vec<T>, ExpectedError> for Parser where Self: TryAs<T, ExpectedError> {
    fn try_as(&self) -> ExpectedResult<Vec<T>> {
        self.typed_array()
    }
}
// endregion: ParserAs

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

type ExpectedResult<T> = Result<T, ExpectedError>;
impl Parser {
    fn error_expected(&self, expected: ExpectedErrorVariant) -> ExpectedError {
        ExpectedError { found: self.value.clone(), expected }
    }

    #[allow(unused)]
    pub fn bool(&self) -> ExpectedResult<bool> {
        self.value.as_bool().ok_or_else(|| self.error_expected(ExpectedErrorVariant::Bool))
    }

    pub fn string(&self) -> ExpectedResult<String> {
        Ok(self.value.as_str().ok_or_else(|| self.error_expected(ExpectedErrorVariant::String))?.to_owned())
    }

    pub fn str(&self) -> ExpectedResult<&str> {
        self.value.as_str().ok_or_else(|| self.error_expected(ExpectedErrorVariant::String))
    }

    pub fn u64(&self) -> ExpectedResult<u64> {
        self.value.as_u64().ok_or(self.error_expected(ExpectedErrorVariant::U64))
    }

    pub fn u8(&self) -> ExpectedResult<u8> {
        let error = || self.error_expected(ExpectedErrorVariant::U8);
        self.value.as_u64().ok_or_else(error)?.try_into().ok().ok_or_else(error)
    }

    pub fn u16(&self) -> ExpectedResult<u16> {
        self.u64()?.try_into().map_err(|_| self.error_expected(ExpectedErrorVariant::U16))
    }

    pub fn typed_option<T, E>(&self) -> Result<Option<T>, E> where Self: TryAs<T, E> {
        match self.option() {
            Some(v) => Ok(Some(v.try_as()?)),
            None => Ok(None),
        }
    }

    pub fn option(&self) -> Option<Self> {
        if self.value.is_null() {
            None
        } else {
            Some(self.clone())
        }
    }

    #[allow(unused)]
    pub fn array(&self) -> ExpectedResult<Vec<Self>> {
        let mut result = Vec::new();
        for value in self.value.as_array().ok_or_else(|| self.error_expected(ExpectedErrorVariant::Array))?.iter().cloned() {
            result.push(Self::from(value))
        }
        Ok(result)
    }

    pub fn typed_array<T>(&self) -> ExpectedResult<Vec<T>> where Self: TryAs<T, ExpectedError> {
        let mut result = Vec::new();
        for value in self.value.as_array().ok_or_else(|| self.error_expected(ExpectedErrorVariant::Array))?.iter().cloned() {
            result.push(Self::from(value).try_as()?)
        }
        Ok(result)
    }

    #[allow(unused)]
    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn try_i<I: serde_json::value::Index, T, E>(&self, index: I) -> Result<T, E> where Self: TryAs<T, E> {
        Self::from(self.value[index].to_owned()).try_as()
    }

    pub fn i<I: serde_json::value::Index>(&self, index: I) -> Self {
        Self::from(self.value[index].to_owned())
    }
}
