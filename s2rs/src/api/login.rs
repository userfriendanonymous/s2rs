use serde::Deserialize;
use serde_json::json;
use super::{Api, utils::{RequestBuilderUtils, ResponseUtils}};

#[derive(Deserialize, Clone, Debug)]
pub struct Login {
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

impl Api {
    pub async fn login(&self, name: &str, password: &str) -> super::Result<Login> {
        let response = self.post_base("accounts/login/").json(json!({
            "username": name,
            "password": password,
            "useMessages": true
        }))?.send_success().await?;

        response.json().await
    }
}