use super::{Api, user::{UserProfileImages, UserHistory}, utils::RequestBuilderUtils, SendComment};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::cursor::Cursor;

// region: Project
#[derive(Deserialize, Debug)]
pub struct Project2 {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub visibility: String,
    pub public: bool,
    pub comments_allowed: bool,
    pub is_published: bool,
    pub author: ProjectAuthor,
    pub image: String,
    pub images: ProjectImages,
    pub stats: ProjectStats,
    pub remix: ProjectRemix,
    pub history: ProjectHistory,
}

#[derive(Deserialize, Debug)]
pub struct Project3 {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub instructions: String, 
    pub visibility: String,
    pub public: bool,
    pub comments_allowed: bool,
    pub is_published: bool,
    pub author: Project3Author,
    pub image: String,
    pub images: ProjectImages,
    pub stats: ProjectStats,
    pub remix: ProjectRemix,
    pub history: ProjectHistory,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub visibility: String,
    pub public: bool,
    pub comments_allowed: bool,
    pub is_published: bool,
    pub author: ProjectAuthor,
    pub image: String,
    pub images: ProjectImages,
    pub stats: ProjectStats,
    pub remix: ProjectRemix,
    pub history: ProjectHistory,
    #[serde( rename = "project_token" )]
    pub token: String
}
// endregion: Project

// region: ProjectAuthor
#[derive(Deserialize, Debug)]
pub struct ProjectAuthor {
    pub id: u64,
    #[serde( rename = "username" )]
    pub name: String,
    #[serde( rename = "scratchteam" )]
    pub scratch_team: bool,
    pub history: UserHistory,
    pub profile: ProjectAuthorProfile,
}

#[derive(Deserialize, Debug)]
pub struct Project3Author {
    pub id: u64,
    #[serde( rename = "scratchteam" )]
    pub scratch_team: bool,
    pub history: UserHistory,
    pub profile: ProjectAuthorProfile,
}

#[derive(Deserialize, Debug)]
pub struct ProjectAuthorProfile {
    pub id: (),
    pub images: UserProfileImages,
}
// endregion: ProjectAuthor

// region: Project extra
#[derive(Deserialize, Debug)]
pub struct ProjectImages {
    #[serde( rename = "282x218" )]
    pub x282_218: String,
    #[serde( rename = "216x163" )]
    pub x216_163: String,
    #[serde( rename = "200x200" )]
    pub x200_200: String,
    #[serde( rename = "144x108" )]
    pub x144_108: String,
    #[serde( rename = "135x102" )]
    pub x135_102: String,
    #[serde( rename = "100x80" )]
    pub x100_80: String,
}

#[derive(Deserialize, Debug)]
pub struct ProjectStats {
    pub views: u64,
    pub loves: u64,
    pub favorites: u64,
    pub remixes: u64,
}

#[derive(Deserialize, Debug)]
pub struct ProjectHistory {
    pub created: String,
    pub modified: String,
    pub shared: String,
}

#[derive(Deserialize, Debug)]
pub struct ProjectRemix {
    pub parent: Option<u64>,
    pub root: Option<u64>,
}
// endregion: Project extra


#[derive(Debug, Clone, Serialize, Default)]
pub struct ProjectInfo {
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub title: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub instructions: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub description: Option<String>,
}

impl Api {
    pub async fn project_meta(&self, id: u64) -> super::Result<Project> {
        let response = self.get(&format!("projects/{id}/")).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn user_projects(&self, name: &str, cursor: impl Into<Cursor>) -> super::Result<Vec<Project3>> {
        let response = self.get(&format!("users/{name}/projects/")).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn user_favorites(&self, name: &str, cursor: impl Into<Cursor>) -> super::Result<Vec<Project3>> {
        let response = self.get(&format!("users/{name}/favorites/")).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn viewed_projects(&self, cursor: impl Into<Cursor>) -> super::Result<Vec<Project2>> {
        let response = self.get(&format!("users/{}/projects/recentlyviewed/", &self.name)).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn projects_loved_by_following(&self, cursor: impl Into<Cursor>) -> super::Result<Vec<Project2>> {
        let response = self.get(&format!["users/{}/following/users/loves/", &self.name]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn projects_shared_by_following(&self, cursor: impl Into<Cursor>) -> super::Result<Vec<Project2>> {
        let response = self.get(&format!["users/{}/following/users/projects/", &self.name]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn project_remixes(&self, id: u64, cursor: impl Into<Cursor>) -> super::Result<Vec<Project3>> {
        let response = self.get(&format!("projects/{id}/remixes/")).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn send_project_comment(&self, id: u64, data: &SendComment) -> super::Result<()> {
        let _ = self.post_proxy(&format!["comments/project/{id}/"])
        .json(&serde_json::to_string(data)?)
        .project_send_success(id).await?;
        Ok(())
    }

    pub async fn set_project_commenting(&self, id: u64, allowed: bool) -> super::Result<()> {
        let _ = self.put(&format!["projects/{id}/"])
        .json(&json!({
            "comments_allowed": allowed
        }))
        .project_send_success(id).await?;
        Ok(())
    }

    pub async fn love_project(&self, id: u64) -> super::Result<()> {
        let _ = self.post_proxy(&format!("projects/{id}/loves/user/{}", self.name())).project_send_success(id).await?;
        Ok(())
    }

    pub async fn unlove_project(&self, id: u64) -> super::Result<()> {
        let _ = self.delete_proxy(&format!("projects/{id}/loves/user/{}", self.name())).project_send_success(id).await?;
        Ok(())
    }

    pub async fn favorite_project(&self, id: u64) -> super::Result<()> {
        let _ = self.post_proxy(&format!("projects/{id}/favorites/user/{}", self.name())).send().await?;
        Ok(())
    }

    pub async fn unfavorite_project(&self, id: u64) -> super::Result<()> {
        let _ = self.delete_proxy(&format!("projects/{id}/favorites/user/{}/", self.name())).project_send_success(id).await?;
        Ok(())
    }

    pub async fn unshare_project(&self, id: u64) -> super::Result<()> {
        let _response = self.put_proxy(&format!("projects/{id}/unshare/")).project_send_success(id).await?;
        Ok(())
    }

    pub async fn view_project(&self, id: u64) -> super::Result<()> {
        let _ = self.post(&format!("users/{}/projects/{id}/views", self.name())).project_send_success(id).await?;
        Ok(())
    }

    pub async fn delete_project_comment(&self, id: u64, comment_id: u64) -> super::Result<()> {
        let _ = self.delete_proxy(&format!("comments/project/{id}/comment/{comment_id}/")).project_send_success(id).await?;
        Ok(())
    }

    pub async fn set_project_info(&self, id: u64, info: &ProjectInfo) -> super::Result<()> {
        let _ = self.put(&format!("projects/{id}/"))
        .json(info)
        .project_send_success(id).await?;
        Ok(())
    }

    pub async fn project_thumbnail(&self, id: u64, width: u16, height: u16) -> super::Result<Vec<u8>> {
        let response = self.get_uploads(&format!["get_image/project/{id}_{width}x{height}.png"]).send().await?;
        let status = response.status();
        if status.is_success() || status.as_u16() == 302 {
            Ok(response.bytes().await?.to_vec())
        } else {
            Err(status)?
        }
    }

    pub async fn set_project_thumbnail(&self, id: u64, buffer: Vec<u8>) -> super::Result<()> {
        let _ = self.post_internal_api(&format!["project/thumbnail/{id}/set/"]).body(buffer).send_success().await?;
        Ok(())
    }
}