use s2rs_derive::Forwarder;
use s2rs::api::ForumCategory;
use serde::Deserialize;
use super::{Api, utils::ResponseUtils};

#[derive(Deserialize, Debug)]
pub struct ForumUserPostHistory {
    #[serde( rename = "date" )]
    pub at: String,
    pub value: u32,
}

#[derive(Forwarder)]
pub enum GetForumUserPostsHistoryError {
    #[forward(reqwest::Error, super::utils::AsJsonError)]
    This(super::Error)
}

impl Api {
    pub async fn get_forum_user_posts_history(&self, name: &str, category: &ForumCategory) -> Result<Vec<ForumUserPostHistory>, GetForumUserPostsHistoryError> {
        let response = self.get(&format!["forum/user/graph/{name}/{}/", category.as_ref()]).send().await?;
        Ok(response.json().await?)
    }
}