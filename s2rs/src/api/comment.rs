use serde::{Deserialize, Serialize};
use super::{Api, utils::RequestBuilderUtils};
use crate::cursor::Cursor;

#[derive(Deserialize, Debug)]
pub struct Comment {
    pub id: u64,
    pub author: CommentAuthor,
    pub parent_id: Option<u64>,
    #[serde( rename = "commentee_id" )]
    pub to_user_id: Option<u64>,
    pub content: String,
    #[serde( rename = "datetime_created" )]
    pub created_at: String,
    #[serde( rename = "datetime_modified" )]
    pub modified_at: String,
    pub reply_count: u64,
}

#[derive(Deserialize, Debug)]
pub struct CommentAuthor {
    pub id: u64,
    #[serde( rename = "username" )]
    pub name: String,
    #[serde( rename = "scratchteam" )]
    pub scratch_team: bool,
    pub image: String
}

// region: SendComment
#[derive(Serialize)]
pub struct SendComment {
    pub content: String,
    pub parent_id: Option<u64>,
    pub to_id: Option<u64>,
}

impl From<String> for SendComment {
    fn from(content: String) -> Self {
        Self {
            content,
            parent_id: None,
            to_id: None,
        }
    }
}

impl From<&str> for SendComment {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}
// endregion: SendComment

impl Api {
    pub async fn user_project_comments(&self, name: &str, id: u64, cursor: impl Into<Cursor>) -> super::Result<Vec<Comment>> {
        let response = self.get(&format!("users/{name}/projects/{id}/comments/")).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn user_project_comment_replies(&self, name: &str, id: u64, comment_id: u64, cursor: impl Into<Cursor>) -> super::Result<Vec<Comment>> {
        let response = self.get(&format!("users/{name}/projects/{id}/comments/{comment_id}/replies/")).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn studio_comments(&self, id: u64, cursor: impl Into<Cursor>) -> super::Result<Vec<Comment>> {
        let response = self.get(&format!("studios/{id}/comments/")).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn studio_comment_replies(&self, id: u64, comment_id: u64, cursor: impl Into<Cursor>) -> super::Result<Vec<Comment>> {
        let response = self.get(&format!("studios/{id}/comments/{comment_id}/")).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }
}
