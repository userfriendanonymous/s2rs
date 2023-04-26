use serde::Deserialize;
use crate::Api;
use super::utils::RequestBuilderUtils;

// region: FeaturedLabel
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeaturedLabel {
    FeaturedProject,
    FeaturedTutorial,
    WorkInProgress,
    RemixThis,
    MyFavoriteThings,
    WhyIScratch,
}

impl FeaturedLabel {
    pub fn from_label_str(data: &str) -> Option<Self> {
        Some(match data {
            "Featured Project" => Self::FeaturedProject,
            "Featured Tutorial" => Self::FeaturedTutorial,
            "Work In Progress" => Self::WorkInProgress,
            "Remix This!" => Self::RemixThis,
            "My Favorite Things" => Self::MyFavoriteThings,
            "Why I Scratch" => Self::WhyIScratch,
            _ => None?
        })
    }

    pub fn as_id(&self) -> &str {
        match self {
            Self::FeaturedProject => "",
            Self::FeaturedTutorial => "0",
            Self::WorkInProgress => "1",
            Self::RemixThis => "2",
            Self::MyFavoriteThings => "3",
            Self::WhyIScratch => "4"
        }
    }

    pub fn deserialize<'de, D: serde::de::Deserializer<'de>>(d: D) -> Result<FeaturedLabel, D::Error> {
        use serde::de::Error;
        let label = serde_json::Value::deserialize(d)?;
        Self::from_label_str(label.as_str().ok_or(D::Error::custom("expected string"))?)
        .ok_or(D::Error::custom("invalid value"))
    }
}
// endregion: FeaturedLabel

#[derive(Deserialize, Debug, Clone)]
pub struct UserFeatured {
    pub id: u64, // not sure what this field is for
    #[serde( rename = "featured_project_label_name", deserialize_with = "FeaturedLabel::deserialize" )]
    pub label: FeaturedLabel,
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
    pub async fn user_featured(&self, name: &str) -> super::Result<UserFeatured> {
        let response = self.get_site_api(&format!["users/all/{name}/"]).send_success().await?;
        Ok(response.json().await?)
    }
}