use serde::{Deserialize, Serialize};
use serde_json::json;
use super::{Api, utils::RequestBuilderUtils};
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
pub struct Studio2 {
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

#[derive(Debug, Clone, Serialize, Default)]
pub struct StudioInfo {
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub title: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub description: Option<String>
}

impl Api {
    pub async fn studio_meta(&self, id: u64) -> super::Result<Studio> {
        let response = self.get(&format!["studios/{id}/"]).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn user_curating_studios(&self, name: &str, cursor: impl Into<Cursor>) -> super::Result<Vec<Studio2>> {
        let response = self.get(&format!["users/{name}/studios/curate/"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn project_studios(&self, id: u64, cursor: impl Into<Cursor>) -> super::Result<Vec<Studio>> {
        let response = self.get(&format!["projects/{id}/studios/"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn add_studio_project(&self, id: u64, project_id: u64) -> super::Result<()> {
        let _ = self.post(&format!["studios/{id}/project/{project_id}/"]).send_success().await?;
        Ok(())
    }

    pub async fn remove_studio_project(&self, id: u64, project_id: u64) -> super::Result<()> {
        let _ = self.post(&format!["studios/{id}/project/{project_id}/"]).send_success().await?;
        Ok(())
    }

    pub async fn open_studio(&self, id: u64) -> super::Result<()> {
        let _ = self.put_site_api(&format!["galleries/{id}/mark/open/"]).send_success().await?;
        Ok(())
    }

    pub async fn close_studio(&self, id: u64) -> super::Result<()> {
        let _ = self.put_site_api(&format!["galleries/{id}/mark/closed/"]).send_success().await?;
        Ok(())
    }

    pub async fn follow_studio(&self, id: u64) -> super::Result<()> {
        let _ = self.put_site_api(&format!["users/bookmarkers/{id}/add/"])
        .query(&[("usernames", self.name())])
        .send_success().await?;
        Ok(())
    }

    pub async fn unfollow_studio(&self, id: u64) -> super::Result<()> {
        let _ = self.put_site_api(&format!["users/bookmarkers/{id}/remove/"])
        .query(&[("usernames", self.name())])
        .send_success().await?;
        Ok(())
    }

    pub async fn toggle_studio_commenting(&self, id: u64) -> super::Result<()> {
        let _ = self.post_site_api(&format!["comments/gallery/{id}/toggle-comments/"]).send_success().await?;
        Ok(())
    }

    pub async fn send_studio_comment(&self, id: u64, content: &str, parent_id: Option<u64>, to_id: Option<u64>) -> super::Result<()> {
        let _ = self.post_site_api(&format!["comments/gallery/{id}/add/"])
        .json(&json!({
            "content": content,
            "parent_id": parent_id,
            "commentee_id": to_id
        }))
        .send_success().await?;
        Ok(())
    }

    pub async fn invite_studio_curator(&self, id: u64, name: &str) -> super::Result<()> {
        let _ = self.put_site_api(&format!["users/curators-in/{id}/invite_curator/"])
        .query(&[("usernames", name)])
        .send_success().await?;
        Ok(())
    }

    pub async fn remove_studio_curator(&self, id: u64, name: &str) -> super::Result<()> {
        let _ = self.put_site_api(&format!["users/curators-in/{id}/remove/"])
        .query(&[("usernames", name)])
        .send_success().await?;
        Ok(())
    }

    pub async fn accept_studio_invite(&self, id: u64) -> super::Result<()> {
        let _ = self.put_site_api(&format!["users/curators-in/{id}/add/"])
        .query(&[("usernames", self.name())])
        .send_success().await?;
        Ok(())
    }

    pub async fn promote_studio_curator(&self, id: u64, name: &str) -> super::Result<()> {
        let _ = self.put_site_api(&format!["users/curators-in/{id}/promote/"])
        .query(&[("usernames", name)])
        .send_success().await?;
        Ok(())
    }

    pub async fn set_studio_info(&self, id: u64, info: &StudioInfo) -> super::Result<()> {
        let _ = self.put_site_api(&format!["galleries/all/{id}/"])
        .json(info)
        .send_success().await?;
        Ok(())
    }

    pub async fn studio_thumbnail(&self, id: u64, width: u16, height: u16) -> super::Result<Vec<u8>> {
        let response = self.get_uploads(&format!["get_image/gallery/{id}_{width}x{height}.png"]).send().await?;
        let status = response.status();
        if status.is_success() || status.as_u16() == 302 {
            Ok(response.bytes().await?.into())
        } else {
            Err(status)?
        }
    }

    #[cfg(feature = "file")]
    pub async fn set_studio_thumbnail<B>(&self, id: u64, buffer: B) -> super::Result<()>
    where B: Into<std::borrow::Cow<'static, [u8]>> {
        use reqwest::multipart::{Form, Part};

        let form = Form::new()
        .part("file", Part::bytes(buffer).file_name(""));
        let _ = self.post_site_api(&format!["galleries/all/{id}/"]).multipart(form).send_success().await?;
        Ok(())
    }
}