use serde::Deserialize;
use crate::Api;
use super::utils::{RequestBuilderUtils, ResponseUtils};

#[derive(Deserialize, Debug, Clone)]
pub struct UserFeatured {
    pub id: u64, // not sure what this field is for
    #[serde( rename = "featured_project_label_name" )]
    pub label: String,
    #[serde( rename = "featured_project_label_id" )]
    pub label_id: Option<u64>,
    #[serde( rename = "featured_project_data" )]
    pub project: UserFeaturedProject,
    #[serde( rename = "thumbnail_url" )]
    pub profile_image_url: String,
    #[serde( rename = "user" )]
    pub profile: UserFeaturedProfile,

}

#[derive(Deserialize, Debug, Clone)]
pub struct UserFeaturedProject {
    #[serde( rename = "creator" )]
    pub author_name: String,
    pub thumbnail_url: String,
    pub id: u64,
    pub title: String,
    #[serde( rename = "datetime_modified" )]
    pub modified_at: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserFeaturedProfile {
    #[serde( rename = "username" )]
    pub name: String,
    #[serde( rename = "pk" )]
    pub id: u64,
}

impl Api {
    pub async fn get_user_featured(&self, name: &str) -> super::Result<UserFeatured> {
        let response = self.get_site_api(&format!["users/all/{name}/"]).send_success().await?;
        Ok(response.json().await?)
    }
}