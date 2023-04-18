use std::{collections::HashMap, str::FromStr};
use s2rs_derive::{Forwarder, deref};

#[derive(Forwarder, Debug)]
pub enum TryIntoReqwestHeadersError {
    #[forward] Name(reqwest::header::InvalidHeaderName),
    #[forward] Value(reqwest::header::InvalidHeaderValue)
}

#[deref(items)]
#[derive(Clone, Default, Debug)]
pub struct Headers {
    items: HashMap<String, String>
}

impl Headers {
    pub fn new() -> Self {
        Self {
            items: HashMap::new()
        }
    }

    pub fn add(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.items.insert(key.into(), value.into());
    }

    pub fn unwrap(self) -> HashMap<String, String> {
        self.items
    }
}

impl TryFrom<Headers> for reqwest::header::HeaderMap {
    type Error = TryIntoReqwestHeadersError;
    fn try_from(value: Headers) -> Result<Self, Self::Error> {
        let data = value.unwrap();
        let mut result = reqwest::header::HeaderMap::new();
        for (name, value) in data {
            result.insert(reqwest::header::HeaderName::from_str(name.as_str())?, reqwest::header::HeaderValue::from_str(&value)?);
        }
        Ok(result)
    }
}

// impl<'a> From<&'a Headers> for Vec<httparse::Header<'a>> {
//     fn from(value: &'a Headers) -> Self {
//         let mut result = Vec::new();
//         for (name, value) in (*value).iter() {
//             result.push(httparse::Header { name: &name.clone(), value: value.clone().as_ref() });
//         }
//         result
//     }
// }