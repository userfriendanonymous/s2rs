use s2rs_derive::Forwarder;
use serde::Deserialize;
use crate::Api;
use super::utils::RequestBuilderUtils;
use crate::json;

#[derive(Debug, Deserialize, Clone)]
pub struct News {
    pub id: u64,
    #[serde( rename = "stamp" )]
    pub at: String,
    #[serde( rename = "headline" )]
    pub title: String,
    pub url: String,
    pub image: String,
    #[serde( rename = "copy" )]
    pub description: String,
}

// region: FrontPage
#[derive(Debug, Deserialize, Clone)]
pub struct FrontPage {
    #[serde( rename = "community_newest_projects" )]
    pub new_projects: Vec<FrontPageProject>,
    #[serde( rename = "community_featured_studios" )]
    pub featured_studios: Vec<FrontPageFeaturedStudio>,
    #[serde( rename = "community_featured_projects" )]
    pub featured_projects: Vec<FrontPageProject>,
    #[serde( rename = "curator_top_projects" )]
    pub curated_projects: Vec<FrontPageCuratedProject>,
    #[serde( rename = "community_most_remixed_projects" )]
    pub most_remixed_projects: Vec<FrontPageMostRemixedProject>,
    #[serde( rename = "community_most_loved_projects" )]
    pub most_loved_projects: Vec<FrontPageProject>,
    #[serde( rename = "scratch_design_studio" )]
    pub design_studio_projects: Vec<FrontPageDesignStudioProject>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FrontPageProject {
    pub thumbnail_url: String,
    pub title: String,
    #[serde( rename = "creator" )]
    pub author_name: String,
    pub id: u64,
    pub love_count: u32,
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
pub struct FrontPageCuratedProject {
    pub id: u64,
    pub title: String,
    pub love_count: u32,
    pub thumbnail_url: String,
    #[serde( rename = "creator" )]
    pub author_name: String,
    #[serde( rename = "curator_name" )]
    pub curated_by_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FrontPageFeaturedStudio {
    pub id: u64,
    pub title: String,
    pub thumbnail_url: String,
}
// endregion: FrontPage

// region: Health
#[derive(Deserialize, Debug, Clone)]
pub struct Health {
    pub version: String,
    pub uptime: f32,
    pub load: Vec<f32>,
    pub sql: HealthSql,
    pub time_stamp: u64,
    pub cache: HealthCache,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HealthCache {
    pub connected: bool,
    pub ready: bool
}

#[derive(Deserialize, Debug, Clone)]
pub struct HealthSql {
    pub main: HealthSqlItem,
    pub project_comments: HealthSqlItem,
    #[serde( rename = "gallery_comments" )]
    pub studio_comments: HealthSqlItem,
    #[serde( rename = "userprofile_comments" )]
    pub profile_comments: HealthSqlItem
}

#[derive(Deserialize, Debug, Clone)]
pub struct HealthSqlItem {
    pub primary: HealthSqlItemItem,
    pub replica: HealthSqlItemItem
}

#[derive(Deserialize, Debug, Clone)]
pub struct HealthSqlItemItem {
    pub ssl: bool,
    pub destroyed: bool,
    pub min: u32,
    pub max: u32,
    #[serde( rename = "numUsed" )]
    pub used_count: u32,
    #[serde( rename = "numFree" )]
    pub free_count: u32,
    #[serde( rename = "pendingAcquires" )]
    pub pending_acquires: u32,
    #[serde( rename = "pendingCreates" )]
    pub pending_creates: u32,
}
// endregion: Health

#[derive(Forwarder, Debug)]
pub enum GetProjectsCountError {
    #[forward] Expected(json::ExpectedError),
    #[forward(reqwest::Error)]
    This(super::Error)
}

impl Api {
    pub async fn front_page(&self) -> super::Result<FrontPage> {
        let response = self.get_proxy("featured/").send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn news(&self) -> super::Result<Vec<News>> {
        let response = self.get("news").send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn projects_count(&self) -> Result<u64, GetProjectsCountError> {
        let response = self.get("projects/count/all/").send_success().await?;
        let data: json::Parser = response.json().await?;
        Ok(data.i("count").u64()?)
    }

    pub async fn clear_messages(&self) -> super::Result<()> {
        let _ = self.post_site_api("messages/messages-clear/").send_success().await?;
        Ok(())
    }

    pub async fn health(&self) -> super::Result<Health> {
        let response = self.get("health/").send_success().await?;
        Ok(response.json().await?)
    }
}