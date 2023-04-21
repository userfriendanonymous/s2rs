use s2rs_derive::Forwarder;
use serde::Deserialize;

use super::Api;

#[derive(Deserialize, Clone, Debug)]
pub struct LoginResponse {
    #[serde( rename = "username" )]
    pub name: String,
    #[serde( rename = "token" )]
    pub x_token: String,
    #[serde( rename = "num_tries" )]
    pub tries_count: u16,
    #[serde( rename = "msg" )]
    pub message: String,
    pub success: u8,
    pub messages: Vec<String>,
    pub id: u64,
}

pub struct Login {
    pub name: String,
    pub x_token: String,
    pub session_token: String,
    pub tries_count: u16,
    pub message: String,
    pub success: u8,
    pub messages: Vec<String>,
    pub id: u64,
}

impl Login {
    pub fn from_response(data: LoginResponse, session_token: String) -> Self {
        Self {
            id: data.id,
            message: data.message,
            messages: data.messages,
            name: data.name,
            session_token,
            success: data.success,
            tries_count: data.tries_count,
            x_token: data.x_token,
        }
    }
}

#[derive(Debug, Forwarder)]
pub enum LoginError {
    #[forward(serde_json::Error)]
    This(super::Error),
    SetCookieHeaderNotFound,
    SessionIdCookieNotFound,
    #[forward] HeaderParsing(reqwest::header::ToStrError)
}

impl Api {
    #[cfg(feature = "cookie")]
    pub async fn login(&self, name: &str, password: &str) -> Result<Login, LoginError> {
        use super::utils::{RequestBuilderUtils, ResponseUtils};
        use serde_json::json;
        use crate::cookies::Cookie;

        let response = self.post_base("accounts/login/").json(json!({
            "username": name,
            "password": password,
            "useMessages": true
        }))?.send_success().await?;

        let header = response.headers().get("Set-Cookie").ok_or(LoginError::SetCookieHeaderNotFound)?;
        let header = header.to_str()?;

        let session_token = Cookie::from_header(header, "scratchsessionsid").ok_or(LoginError::SessionIdCookieNotFound)?.value;
        Ok(Login::from_response(response.json().await?, session_token))
    }
}