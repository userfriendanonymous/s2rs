use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::cursor::Cursor;
use super::{Api, utils::{ResponseUtils, RequestBuilderUtils}, ParsingCustomError, GeneralResult, PartialProject2};

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: u64,
    #[serde( rename = "username" )]
    pub name: String,
    #[serde( rename = "scratchteam" )]
    pub scratch_team: bool,
    pub history: UserHistory,
    pub profile: UserProfile,
}

#[derive(Deserialize, Debug)]
pub struct UserProfile {
    pub id: u64,
    pub images: UserProfileImages,
    pub status: String,
    pub bio: String,
    pub country: String,
}

#[derive(Deserialize, Debug)]
pub struct UserProfileImages {
    #[serde( rename = "90x90" )]
    pub x90: String,
    #[serde( rename = "60x60" )]
    pub x60: String,
    #[serde( rename = "55x55" )]
    pub x55: String,
    #[serde( rename = "50x50" )]
    pub x50: String,
    #[serde( rename = "32x32" )]
    pub x32: String,
}

#[derive(Deserialize, Debug)]
pub struct UserHistory {
    pub joined: String
}

#[derive(Debug, PartialEq, Eq)]
pub enum UserNameCheck {
    Valid,
    Invalid,
    Bad,
    Taken,
}

impl<'de> Deserialize<'de> for UserNameCheck {
    fn deserialize<D>(d: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        use serde::de::Error;
        Ok(match Value::deserialize(d)?.as_object().ok_or(D::Error::custom("expected object"))?
        .get("msg").ok_or(D::Error::custom("'msg' field not found"))?
        .as_str().ok_or(D::Error::custom("'msg' field must be string"))? {
            "invalid username" => Self::Invalid,
            "valid username" => Self::Valid,
            "username exists" => Self::Taken,
            "bad username" => Self::Bad,
            msg => Err(D::Error::custom(format!["invalid 'msg' field value: `{msg}`"]))?
        })
    }
}

// region: UserInfo
#[derive(Debug, Default, Serialize)]
pub struct UserInfo {
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub bio: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none" )]
    pub status: Option<String>,
    #[serde( rename = "featured_project", skip_serializing_if = "Option::is_none" )]
    pub featured_id: Option<u64>,
    #[serde( rename = "featured_project_label", skip_serializing_if = "Option::is_none" )]
    pub featured_label: Option<FeaturedLabel>
}

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
}

impl Serialize for FeaturedLabel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_id())
    }
}
// endregion: UserInfo

impl Api {
    pub async fn get_user_meta(&self, name: &str) -> GeneralResult<User> {
        let response = self.get(&format!["users/{name}"]).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn get_user_messages_count(&self, name: &str) -> GeneralResult<u64> {
        let response = self.get(&format!["users/{name}/messages/count"]).send_success().await?;

        let data: Value = response.json().await?;
        let count = data["count"].as_u64().ok_or(ParsingCustomError)?;
        Ok(count)
    }

    pub async fn get_user_followers(&self, name: &str, cursor: impl Into<Cursor>) -> GeneralResult<Vec<User>> {
        let response = self.get(&format!["users/{name}/followers/"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn get_user_following(&self, name: &str, cursor: impl Into<Cursor>) -> GeneralResult<Vec<User>> {
        let response = self.get(&format!["users/{name}/following"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn get_projects_loved_by_following(&self, name: &str, cursor: impl Into<Cursor>) -> GeneralResult<Vec<PartialProject2>> {
        let response = self.get(&format!["users/{name}/following/users/loves/"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn get_projects_shared_by_following(&self, name: &str, cursor: impl Into<Cursor>) -> GeneralResult<Vec<PartialProject2>> {
        let response = self.get(&format!["users/{name}/following/users/projects/"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn get_studio_managers(&self, id: u64, cursor: impl Into<Cursor>) -> GeneralResult<Vec<User>> {
        let response = self.get(&format!["studios/{id}/managers"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn get_studio_curators(&self, id: u64, cursor: impl Into<Cursor>) -> GeneralResult<Vec<User>> {
        let response = self.get(&format!["studios/{id}/curators/"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn follow_user(&self, name: &str) -> GeneralResult<Value> {
        let response = self.put_site_api(&format!["users/followers/{name}/add/"])
        .query(&[("usernames", self.name())])
        .send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn unfollow_user(&self, name: &str) -> GeneralResult<Value> {
        let response = self.put_site_api(&format!["users/followers/{name}/remove/"])
        .query(&[("usernames", self.name())])
        .send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn send_user_comment(&self, name: &str, content: String, parent_id: Option<u64>, to_id: Option<u64>,) -> GeneralResult<()> {
        let _ = self.post_site_api(&format!["comments/user/{name}/add/"])
        .json(json!({
            "commentee_id": to_id,
            "content": content,
            "parent_id": parent_id
        }))?
        .send_success().await?;
        Ok(())
    }

    pub async fn toggle_user_commenting(&self, name: &str) -> GeneralResult<()> {
        let _ = self.post_site_api(&format!["comments/user/{name}/toggle-comments/"]).send_success().await?;
        Ok(())
    }

    pub async fn set_user_info(&self, info: &UserInfo) -> GeneralResult<()> {
        let _ = self.put_site_api(&format!["users/all/{}/", &self.name])
        .json(info)?
        .send_success().await?;
        Ok(())
    }

    pub async fn check_user_name(&self, name: &str) -> GeneralResult<UserNameCheck> {
        let response = self.get(&format!["accounts/checkusername/{name}/"]).send_success().await?;
        Ok(response.json().await?)
    }
}
