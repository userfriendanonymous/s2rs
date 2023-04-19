use async_trait::async_trait;
use reqwest::{Response, RequestBuilder};
use s2rs_derive::Forwarder;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Forwarder, Debug)]
pub enum AsJsonError {
    #[forward] Parsing(serde_json::Error),
    #[forward] Decoding(reqwest::Error),
}

impl From<AsJsonError> for super::Error {
    fn from(value: AsJsonError) -> Self {
        match value {
            AsJsonError::Decoding(error) => Self::Network(error),
            AsJsonError::Parsing(error) => Self::Parsing(super::ParsingError::Serde(error))
        }
    }
}

#[async_trait]
pub trait ResponseUtils where Self: Sized {
    async fn json<'a, T: DeserializeOwned>(self) -> Result<T, AsJsonError>;
}

#[async_trait]
impl ResponseUtils for Response {
    async fn json<'a, T: DeserializeOwned>(self) -> Result<T, AsJsonError> {
        let text = self.text().await?;
        Ok(serde_json::from_str::<T>(&text)?)
    }
}

#[async_trait]
pub trait RequestBuilderUtils where Self: Sized {
    // fn cursor(self, cursor: impl Into<Cursor>) -> Self;
    fn json<T: Serialize>(self, data: T) -> Result<Self, serde_json::Error>;
}

#[async_trait]
impl RequestBuilderUtils for RequestBuilder {
    fn json<T: Serialize>(self, data: T) -> Result<Self, serde_json::Error> {
        Ok(
            self.header("Content-Type", "application/json")
            .body(serde_json::to_string(&data)?)
        )
    }
}