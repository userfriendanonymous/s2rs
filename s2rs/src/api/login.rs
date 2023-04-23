use s2rs_derive::Forwarder;
use serde::Deserialize;
use crate::json;
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

#[derive(Clone, Debug, Forwarder)]
pub enum LoginParseError {
    #[forward] Expected(json::ExpectedError),
    EmptyArray,
    
}

impl Login {
    pub fn from_parser(data: json::Parser, session_token: String) -> Result<Self, LoginParseError> {
        let data = data.array()?.get(0).ok_or(LoginParseError::EmptyArray)?.to_owned();

        Ok(Self {
            id: data.try_i("id")?,
            message: data.try_i("msg")?,
            messages: data.try_i("messages")?,
            name: data.try_i("username")?,
            session_token,
            success: data.try_i("success")?,
            tries_count: data.try_i("num_tries")?,
            x_token: data.try_i("token")?
        })

    }
}

#[cfg(feature = "cookie")]
#[derive(Debug, Forwarder)]
pub enum LoginError {
    #[forward(serde_json::Error, reqwest::Error)]
    This(super::Error),
    SetCookieHeaderNotFound,
    SessionIdCookieNotFound,
    #[forward] HeaderParsing(reqwest::header::ToStrError),
    #[forward] HeadersConverting(crate::headers::TryIntoReqwestHeadersError),
    #[forward] CookiesParsing(crate::cookies::CookiesFromHeaderError),
    #[forward] Parsing(LoginParseError)
}

impl Api {
    #[cfg(feature = "cookie")]
    pub async fn login(&self, name: &str, password: &str) -> Result<Login, LoginError> {
        use super::utils::{RequestBuilderUtils};
        use serde_json::json;
        use crate::cookies::Cookies;

        let mut cookies = Cookies::default();
        cookies.add("scratchcsrftoken", "a");

        let mut headers = Self::core_headers();
        headers.add("X-CSRFToken", "a");
        headers.add("Cookie", cookies);

        let response = self.post_base("login/")
        .headers(headers.try_into()?)
        .json(&json!({
            "username": name,
            "password": password,
        })).send_success().await?;

        let header = response.headers().get("Set-Cookie").ok_or(LoginError::SetCookieHeaderNotFound)?;
        let header = header.to_str()?;

        let cookies = Cookies::from_header(header)?;
        let session_token = cookies.get("scratchsessionsid").ok_or(LoginError::SessionIdCookieNotFound)?.value.to_owned();
        Ok(Login::from_parser(response.json().await?, session_token)?)
    }
}