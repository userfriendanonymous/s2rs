use async_trait::async_trait;
use reqwest::{Response, RequestBuilder, StatusCode};
// use serde::{de::DeserializeOwned, Serialize};
use crate::{cursor::Cursor, json};

#[async_trait]
pub trait ResponseUtils where Self: Sized {
    fn only_success(self) -> Result<Self, StatusCode>;
    // async fn json<'a, T: DeserializeOwned>(self) -> Result<T, super::Error>;
    async fn json_parser<T: json::Parsable, E: From<T::Error> + From<reqwest::Error>>(self) -> Result<T, E>;
    async fn json_parser_vec<T: json::Parsable, E: From<T::Error> + From<reqwest::Error>>(self) -> Result<Vec<T>, E>;
}

#[async_trait]
impl ResponseUtils for Response {
    fn only_success(self) -> Result<Self, StatusCode> {
        if self.status().is_success() {
            Ok(self)
        } else {
            Err(self.status())
        }
    }

    // async fn json<'a, T: DeserializeOwned>(self) -> Result<T, super::Error> {
    //     let text = self.text().await?;
    //     Ok(serde_json::from_str::<T>(&text)?)
    // }

    async fn json_parser<T: json::Parsable, E: From<T::Error> + From<reqwest::Error>>(self) -> Result<T, E> {
        Ok(T::parse(&self.json::<json::Parser>().await?)?)
    }

    async fn json_parser_vec<T: json::Parsable, E: From<T::Error> + From<reqwest::Error>>(self) -> Result<Vec<T>, E> {
        Ok(T::parse_vec(&self.json::<Vec<json::Parser>>().await?)?)
    }
}

#[async_trait]
pub trait RequestBuilderUtils where Self: Sized {
    async fn send_success(self) -> Result<Response, super::Error>;
    async fn project_send_success(self, id: u64) -> Result<Response, super::Error>;
    fn cursor(self, cursor: impl Into<Cursor>) -> Self;
    fn cursor_2(self, cursor: impl Into<Cursor>) -> Self;
    // fn json<T: Serialize>(self, data: T) -> Result<Self, serde_json::Error>;
    fn project_referer(self, id: u64) -> Self;
}

#[async_trait]
impl RequestBuilderUtils for RequestBuilder {
    async fn send_success(self) -> Result<Response, super::Error> {
        let idk = self.send().await;
        Ok(idk?.only_success()?)
    }

    async fn project_send_success(self, id: u64) -> Result<Response, super::Error> {
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

    // fn json<T: Serialize>(self, data: T) -> Result<Self, serde_json::Error> {
    //     Ok(
    //         self.header("Content-Type", "application/json")
    //         .body(serde_json::to_string(&data)?)
    //     )
    // }
}