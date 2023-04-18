use async_trait::async_trait;
use reqwest::{Response, RequestBuilder};
use s2rs_derive::Forwarder;
use serde::{de::DeserializeOwned, Serialize};
use crate::cursor::Cursor;
use super::{NetworkError, GeneralError, ParsingError, general_parser::{GeneralParsable, GeneralParser}};

#[derive(Forwarder, Debug)]
pub enum AsJsonError {
    #[forward] Parsing(serde_json::Error),
    #[forward] Decoding(reqwest::Error),
}

impl From<AsJsonError> for GeneralError {
    fn from(value: AsJsonError) -> Self {
        match value {
            AsJsonError::Decoding(error) => Self::Network(NetworkError::Request(error)),
            AsJsonError::Parsing(error) => Self::Parsing(ParsingError::Auto(error))
        }
    }
}

#[async_trait]
pub trait ResponseUtils where Self: Sized {
    fn only_success(self) -> Result<Self, NetworkError>;
    async fn json<'a, T: DeserializeOwned>(self) -> Result<T, AsJsonError>;
    async fn general_parser<T: GeneralParsable>(self) -> Result<T, GeneralError>
        where GeneralError: From<<T as GeneralParsable>::Error>;
    async fn general_parser_vec<T: GeneralParsable>(self) -> Result<Vec<T>, GeneralError>
        where GeneralError: From<<T as GeneralParsable>::Error>;
}

#[async_trait]
impl ResponseUtils for Response {
    fn only_success(self) -> Result<Self, NetworkError> {
        if self.status().is_success() {
            Ok(self)
        } else {
            Err(NetworkError::Status(self.status()))
        }
    }

    async fn json<'a, T: DeserializeOwned>(self) -> Result<T, AsJsonError> {
        let text = self.text().await?;
        Ok(serde_json::from_str::<T>(&text)?)
    }

    async fn general_parser<T: GeneralParsable>(self) -> Result<T, GeneralError> where GeneralError: From<<T as GeneralParsable>::Error> {
        Ok(T::parse(&self.json::<GeneralParser>().await?)?)
    }

    async fn general_parser_vec<T: GeneralParsable>(self) -> Result<Vec<T>, GeneralError> where GeneralError: From<<T as GeneralParsable>::Error> {
        Ok(T::parse_vec(&self.json::<Vec<GeneralParser>>().await?)?)
    }
}

#[async_trait]
pub trait RequestBuilderUtils where Self: Sized {
    async fn send_success(self) -> Result<Response, NetworkError>;
    async fn project_send_success(self, id: u64) -> Result<Response, NetworkError>;
    fn cursor(self, cursor: impl Into<Cursor>) -> Self;
    fn cursor_2(self, cursor: impl Into<Cursor>) -> Self;
    fn json<T: Serialize>(self, data: T) -> Result<Self, serde_json::Error>;
    fn project_referer(self, id: u64) -> Self;
}

#[async_trait]
impl RequestBuilderUtils for RequestBuilder {
    async fn send_success(self) -> Result<Response, NetworkError> {
        let idk = self.send().await;
        idk?.only_success()
    }

    async fn project_send_success(self, id: u64) -> Result<Response, NetworkError> {
        self.project_referer(id).send_success().await
    }

    fn project_referer(self, id: u64) -> Self {
        self.header("referer", format!("https://scratch.mit.edu/projects/{id}"))
    }

    fn cursor(self, cursor: impl Into<Cursor>) -> Self {
        let cursor: Cursor = cursor.into();
        self.query(&[
            ("limit", cursor.get_limit().map(|v| v.min(40))),
            ("offset", Some(cursor.start))
        ])
    }

    fn cursor_2(self, cursor: impl Into<Cursor>) -> Self {
        let cursor: Cursor = cursor.into();
        self.query(&[
            ("limit", cursor.get_limit().map(|l| l.min(40))),
            ("offset", Some(cursor.start))
        ])
    }

    fn json<T: Serialize>(self, data: T) -> Result<Self, serde_json::Error> {
        Ok(
            self.header("Content-Type", "application/json")
            .body(serde_json::to_string(&data)?)
        )
    }
}