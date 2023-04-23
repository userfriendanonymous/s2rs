use serde::Deserialize;
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

impl Api {
    pub async fn project_comments(&self, id: u64, cursor: impl Into<Cursor>) -> super::Result<Vec<Comment>> {
        let response = self.get(&format!("comments/project/{id}")).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn project_comment_replies(&self, id: u64, comment_id: u64, cursor: impl Into<Cursor>) -> super::Result<Vec<Comment>> {
        let response = self.get(&format!("comments/project/{id}/{comment_id}")).cursor(cursor).send_success().await?;
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
