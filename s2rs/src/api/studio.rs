use serde::Deserialize;
use serde_json::json;
use super::{Api, utils::{RequestBuilderUtils, ResponseUtils}, GeneralResult};
use crate::cursor::Cursor;

#[derive(Deserialize, Debug)]
pub struct Studio {
    pub id: u64,
    pub title: String,
    pub host: u64,
    pub description: String,
    pub visibility: String,
    pub public: bool,
    pub open_to_all: bool,
    pub comments_allowed: bool,
    pub image: String,
    pub history: StudioHistory,
    pub stats: StudioStats
}

#[derive(Deserialize, Debug)]
pub struct StudioHistory {
    pub created: String,
    pub modified: String,
}

#[derive(Deserialize, Debug)]
pub struct StudioStats {
    pub comments: u64,
    pub followers: u64,
    pub managers: u64,
    pub projects: u64,
}

#[derive(Deserialize, Debug)]
pub struct AddStudioProject {
    #[serde( rename = "actorId" )]
    pub actor_id: u64,
    #[serde( rename = "datetimeCreated" )]
    pub created_at: String,
    #[serde( rename = "projectId", deserialize_with = "crate::utils::serde::de::string_to_u64" )]
    pub project_id: u64,
    #[serde( rename = "studioId", deserialize_with = "crate::utils::serde::de::string_to_u64" )]
    pub id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioInfo {
    pub title: Option<String>,
    pub description: Option<String>
}

impl Api {
    pub async fn get_studio_meta(&self, id: u64) -> GeneralResult<Studio> {
        let response = self.get(&format!["studios/{id}/"]).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn get_user_curating_studios(&self, name: &str, cursor: impl Into<Cursor>) -> GeneralResult<Vec<Studio>> {
        let response = self.get(&format!["users/{name}/studios/curate/"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn get_project_studios(&self, id: u64, cursor: impl Into<Cursor>) -> GeneralResult<Vec<Studio>> {
        let response = self.get(&format!["projects/{id}/studios/"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn add_studio_project(&self, id: u64, project_id: u64) -> GeneralResult<()> {
        let _ = self.post(&format!["studios/{id}/project/{project_id}/"]).send_success().await?;
        Ok(())
    }

    pub async fn remove_studio_project(&self, id: u64, project_id: u64) -> GeneralResult<()> {
        let _ = self.post(&format!["studios/{id}/project/{project_id}/"]).send_success().await?;
        Ok(())
    }

    pub async fn open_studio(&self, id: u64) -> GeneralResult<()> {
        let _ = self.put_site_api(&format!["galleries/{id}/mark/open/"]).send_success().await?;
        Ok(())
    }

    pub async fn close_studio(&self, id: u64) -> GeneralResult<()> {
        let _ = self.put_site_api(&format!["galleries/{id}/mark/closed/"]).send_success().await?;
        Ok(())
    }

    pub async fn follow_studio(&self, id: u64) -> GeneralResult<()> {
        let _ = self.put_site_api(&format!["users/bookmarkers/{id}/add/"])
        .query(&[("usernames", self.name())])
        .send_success().await?;
        Ok(())
    }

    pub async fn unfollow_studio(&self, id: u64) -> GeneralResult<()> {
        let _ = self.put_site_api(&format!["users/bookmarkers/{id}/remove/"])
        .query(&[("usernames", self.name())])
        .send_success().await?;
        Ok(())
    }

    pub async fn toggle_studio_commenting(&self, id: u64) -> GeneralResult<()> {
        let _ = self.post_site_api(&format!["comments/gallery/{id}/toggle-comments/"]).send_success().await?;
        Ok(())
    }

    pub async fn send_studio_comment(&self, id: u64, content: &str, parent_id: Option<u64>, to_id: Option<u64>) -> GeneralResult<()> {
        let _ = self.post_site_api(&format!["comments/gallery/{id}/add/"])
        .json(json!({
            "content": content,
            "parent_id": parent_id,
            "commentee_id": to_id
        }))?
        .send_success().await?;
        Ok(())
    }

    pub async fn invite_studio_curator(&self, id: u64, name: &str) -> GeneralResult<()> {
        let _ = self.put_site_api(&format!["users/curators-in/{id}/invite_curator/"])
        .query(&[("usernames", name)])
        .send_success().await?;
        Ok(())
    }

    pub async fn remove_studio_curator(&self, id: u64, name: &str) -> GeneralResult<()> {
        let _ = self.put_site_api(&format!["users/curators-in/{id}/remove/"])
        .query(&[("usernames", name)])
        .send_success().await?;
        Ok(())
    }

    pub async fn accept_studio_invite(&self, id: u64) -> GeneralResult<()> {
        let _ = self.put_site_api(&format!["users/curators-in/{id}/add/"])
        .query(&[("usernames", self.name())])
        .send_success().await?;
        Ok(())
    }

    pub async fn promote_studio_curator(&self, id: u64, name: &str) -> GeneralResult<()> {
        let _ = self.put_site_api(&format!["users/curators-in/{id}/promote/"])
        .query(&[("usernames", name)])
        .send_success().await?;
        Ok(())
    }

    pub async fn set_studio_info(&self, id: u64, info: &StudioInfo) -> GeneralResult<()> {
        let _ = self.put_site_api(&format!["galleries/all/{id}/"])
        .json(json!({
            "title": info.title,
            "description": info.description
        }))?
        .send_success().await?;
        Ok(())
    }
}