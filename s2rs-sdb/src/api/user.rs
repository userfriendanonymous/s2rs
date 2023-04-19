use s2rs_derive::Forwarder;
use serde::Deserialize;
use super::{Api, utils::ResponseUtils};

#[derive(Deserialize, Debug, Clone)]
pub struct UserMeta {
    pub id: u64,
    pub sys_id: u64,
    #[serde( rename = "username" )]
    pub name: String,
    pub joined: String,
    pub country: String,
    pub bio: String,
    pub work: String,
    pub status: UserStatus,
    pub school: Option<String>,
    #[serde( rename = "statistics" )]
    pub stats: UserStats
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum UserStatus {
    Scratcher,
    NewScratcher,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserStats {
    pub ranks: UserRanks,
    pub loves: u32,
    pub favorites: u32,
    pub comments: u32,
    pub views: u32,
    pub followers: u32,
    pub following: u32
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserRanks {
    pub country: UserRanksCountry,
    pub loves: u32,
    pub favorites: u32,
    pub comments: u32,
    pub views: u32,
    pub followers: u32,
    pub following: u32
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserRanksCountry {
    pub loves: u32,
    pub favorites: u32,
    pub views: u32,
    pub followers: u32,
    pub following: u32,
}

#[derive(Forwarder, Debug)]
pub enum GetUserError {
    #[forward(reqwest::Error, super::utils::AsJsonError)]
    This(super::Error),
    Invalid,
    NotFound
}

impl Api {
    pub async fn get_user_sdb(&self, name: &str) -> Result<UserMeta, GetUserError> {
        let response = self.get(&format!["user/info/{name}/"]).send().await?;
        Ok(response.json().await?)
    }

    pub async fn get_users_leader_board(&self, country: &str, page: u32) -> Result<Vec<UserMeta>, GetUserError> {
        let response = self.get(&format!["user/rank/{country}/followers/{page}/"]).send().await?;
        Ok(response.json().await?)
    }
}