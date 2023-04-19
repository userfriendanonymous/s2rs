use s2rs::{Api, api::GeneralResult};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
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
    pub statistics: UserStatistics
}

#[derive(Clone)]
pub enum UserStatus {
    Scratcher,
    NewScratcher,
}

impl Deserialize for UserStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        
    }
}

#[derive(Deserialize, Clone)]
pub struct UserStatistics {
    pub ranks: UserRanks,
    pub loves: u32,
    pub favorites: u32,
    pub comments: u32,
    pub views: u32,
    pub followers: u32,
    pub following: u32
}

#[derive(Deserialize, Clone)]
pub struct UserRanks {
    pub country: UserRanksCountry,
    pub loves: u32,
    pub favorites: u32,
    pub comments: u32,
    pub views: u32,
    pub followers: u32,
    pub following: u32
}

#[derive(Deserialize, Clone)]
pub struct UserRanksCountry {
    pub loves: u32,
    pub favorites: u32,
    pub views: u32,
    pub followers: u32,
    pub following: u32,
}

