use std::collections::HashMap;
use s2rs_derive::Forwarder;
use serde::Deserialize;
use super::{Api, utils::ResponseUtils};

#[derive(Deserialize, Debug)]
pub struct Project {
    pub id: u64,
    pub sys_id: u64,
    #[serde( rename = "username" )]
    pub author_name: String,
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub public: bool,
    pub comments_allowed: bool,
    #[serde( rename = "times" )]
    pub history: ProjectHistory,
    pub remix: s2rs::api::ProjectRemix,
    #[serde( rename = "statistics" )]
    pub stats: ProjectStats,
    #[serde( rename = "metadata" )]
    pub meta: ProjectMeta
}

#[derive(Deserialize, Debug)]
pub struct ProjectHistory {
    pub created: String,
    pub modified: String,
    pub shared: String,
    pub last_check: String,
    pub last_metadata_check: String,
}

#[derive(Deserialize, Debug)]
pub struct ProjectStats {
    pub ranks: ProjectRanks,
    pub views: u32,
    pub loves: u32,
    pub favorites: u32,
    pub comments: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectRanks {
    pub views: u32,
    pub loves: u32,
    pub favorites: u32,
}

#[derive(Deserialize, Debug)]
pub struct ProjectMeta {
    pub version: u32,
    pub costumes: u32,
    pub variables: u32,
    pub assets: u32,
    pub hash: String,
    pub user_agent: Option<String>,
    pub history: Option<HashMap<String, String>>
}

#[derive(Forwarder, Debug)]
pub enum GetProjectError {
    #[forward(reqwest::Error, super::utils::AsJsonError)]
    This(super::Error),
    Invalid,
    NotFound
}

impl Api {
    pub async fn get_project_sdb(&self, id: u64) -> Result<Project, GetProjectError> {
        let response = self.get(&format!["project/info/{id}/"]).send().await?;
        Ok(response.json().await?)
    }
}