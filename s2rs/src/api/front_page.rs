use serde::Deserialize;
use crate::Api;
use super::{GeneralResult, utils::{RequestBuilderUtils, ResponseUtils}};

#[derive(Debug, Deserialize)]
pub struct FrontPage {
    pub new_projects: Vec<FrontPageProject>,
    pub featured_studios: Vec<FrontPageFeaturedStudio>,
    pub featured_projects: Vec<FrontPageFeaturedProject>,
    pub curated_projects: Vec<FrontPageCuratedProject>,
    pub most_remixed_projects: Vec<FrontPageMostRemixedProject>,
    pub most_loved_projects: Vec<FrontPageMostLovedProject>,
    pub design_studio_projects: Vec<FrontPageDesignStudioProject>,
}

// region: structures
#[derive(Deserialize, Debug)]
pub struct FrontPageProject {
    pub thumbnail_url: String,
    pub title: String,
    #[serde( rename = "creator" )]
    pub author_name: String,
    pub id: u64,
    pub love_count: u32,
}

#[derive(Deserialize, Debug)]
pub struct FrontPageMostRemixedProject {
    pub id: u64,
    pub title: String,
    #[serde( rename = "remixers_count" )]
    pub remix_count: u32,
    pub love_count: u32,
    pub thumbnail_url: String,
    #[serde( rename = "creator" )]
    pub author_name: String,
}

#[derive(Deserialize, Debug)]
pub struct FrontPageDesignStudioProject {
    pub id: u64,
    #[serde( rename = "gallery_id" )]
    pub studio_id: u64,
    #[serde( rename = "gallery_title" )]
    pub studio_title: String,
    pub title: String,
    #[serde( rename = "remixers_count" )]
    pub remix_count: u32,
    pub love_count: u32,
    pub thumbnail_url: String,
    #[serde( rename = "creator" )]
    pub author_name: String,
}

#[derive(Deserialize, Debug)]
pub struct FrontPageCuratedProject {
    pub id: u64,
    pub title: String,
    pub love_count: u32,
    pub thumbnail_url: String,
    #[serde( rename = "creator" )]
    pub author_name: String,
    pub curator_name: String,
}

#[derive(Deserialize, Debug)]
pub struct FrontPageFeaturedStudio {
    pub id: u64,
    pub title: String,
    pub thumbnail_url: String,
}

#[derive(Deserialize, Debug)]
pub struct FrontPageMostLovedProject {
    pub id: u64,
    pub title: String,
    pub thumbnail_url: String,
    #[serde( rename = "creator" )]
    pub author_name: String,
    pub love_count: u32,
}

#[derive(Deserialize, Debug)]
pub struct FrontPageFeaturedProject {
    pub id: u64,
    pub title: String,
    pub thumbnail_url: String,
    #[serde( rename = "creator" )]
    pub author_name: String,
    pub love_count: u32,
}
// endregion: structures

impl Api {
    pub async fn get_front_page(&self) -> GeneralResult<FrontPage> {
        let response = self.get_proxy("featured/").send_success().await?;
        Ok(response.json().await?)
    }
}