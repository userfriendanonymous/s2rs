use serde::Deserialize;
use super::{user::UserProfileImages, Api, utils::{RequestBuilderUtils, ResponseUtils}, GeneralResult};
use crate::cursor::Cursor;

#[derive(Deserialize, Debug)]
pub struct StudioProject {
    pub id: u64,
    pub title: String,
    pub image: String,
    #[serde( rename = "username" )]
    pub name: String,
    pub avatar: UserProfileImages,
    pub actor_id: u64,
    pub creator_id: u64,
}

impl Api {
    pub async fn get_studio_projects(&self, id: u64, cursor: impl Into<Cursor>) -> GeneralResult<Vec<StudioProject>> {
        let response = self.get(&format!["studios/{id}/projects"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }
}