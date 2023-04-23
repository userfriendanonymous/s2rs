use s2rs_derive::Forwarder;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{cursor::Cursor, json};
use reqwest::StatusCode;
use super::{Api, utils::RequestBuilderUtils, Project2};

// region: User
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
// endregion: User

// region: UserNameCheck
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
// endregion: UserNameCheck

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

#[derive(Forwarder, Debug)]
pub enum SetUserIconError {
    #[forward(StatusCode)]
    This(super::Error),
    TooLarge, // thumbnail-too-large
    Invalid // image-invalid
}

#[derive(Forwarder, Debug)]
pub enum GetUserMessagesCountError {
    #[forward(reqwest::Error)]
    This(super::Error),
    #[forward] Parsing(json::ExpectedError),
}

impl Api {
    pub async fn user_meta(&self, name: &str) -> super::Result<User> {
        let response = self.get(&format!["users/{name}"]).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn user_messages_count(&self, name: &str) -> Result<u64, GetUserMessagesCountError> {
        let response = self.get(&format!["users/{name}/messages/count"]).send_success().await?;

        let data: json::Parser = response.json().await?;
        let count = data.i("count").u64()?;
        Ok(count)
    }

    pub async fn user_followers(&self, name: &str, cursor: impl Into<Cursor>) -> super::Result<Vec<User>> {
        let response = self.get(&format!["users/{name}/followers/"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn user_following(&self, name: &str, cursor: impl Into<Cursor>) -> super::Result<Vec<User>> {
        let response = self.get(&format!["users/{name}/following"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn projects_loved_by_following(&self, name: &str, cursor: impl Into<Cursor>) -> super::Result<Vec<Project2>> {
        let response = self.get(&format!["users/{name}/following/users/loves/"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn projects_shared_by_following(&self, name: &str, cursor: impl Into<Cursor>) -> super::Result<Vec<Project2>> {
        let response = self.get(&format!["users/{name}/following/users/projects/"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn studio_managers(&self, id: u64, cursor: impl Into<Cursor>) -> super::Result<Vec<User>> {
        let response = self.get(&format!["studios/{id}/managers"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn studio_curators(&self, id: u64, cursor: impl Into<Cursor>) -> super::Result<Vec<User>> {
        let response = self.get(&format!["studios/{id}/curators/"]).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn follow_user(&self, name: &str) -> super::Result<Value> {
        let response = self.put_site_api(&format!["users/followers/{name}/add/"])
        .query(&[("usernames", self.name())])
        .send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn unfollow_user(&self, name: &str) -> super::Result<Value> {
        let response = self.put_site_api(&format!["users/followers/{name}/remove/"])
        .query(&[("usernames", self.name())])
        .send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn send_user_comment(&self, name: &str, content: String, parent_id: Option<u64>, to_id: Option<u64>,) -> super::Result<()> {
        let _ = self.post_site_api(&format!["comments/user/{name}/add/"])
        .json(&json!({
            "commentee_id": to_id,
            "content": content,
            "parent_id": parent_id
        }))
        .send_success().await?;
        Ok(())
    }

    pub async fn toggle_user_commenting(&self, name: &str) -> super::Result<()> {
        let _ = self.post_site_api(&format!["comments/user/{name}/toggle-comments/"]).send_success().await?;
        Ok(())
    }

    pub async fn set_user_info(&self, info: &UserInfo) -> super::Result<()> {
        let _ = self.put_site_api(&format!["users/all/{}/", &self.name])
        .json(&info)
        .send_success().await?;
        Ok(())
    }

    pub async fn check_user_name(&self, name: &str) -> super::Result<UserNameCheck> {
        let response = self.get(&format!["accounts/checkusername/{name}/"]).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn user_icon(&self, id: u64, width: u16, height: u16) -> super::Result<Vec<u8>> {
        let response = self.get_uploads(&format!["get_image/user/{id}_{width}x{height}.png"]).send().await?;
        let status = response.status();
        if status.is_success() || status.as_u16() == 302 {
            Ok(response.bytes().await?.into())
        } else {
            Err(status)?
        }
    }

    #[cfg(feature = "file")]
    pub async fn set_user_icon<B>(&self, buffer: B) -> super::Result<()>
    where B: Into<std::borrow::Cow<'static, [u8]>> {
        use reqwest::multipart::{Form, Part};

        let form = Form::new()
        .part("file", Part::bytes(buffer).file_name(""));
        let _ = self.post_site_api(&format!["users/all/{}/", &self.name]).multipart(form).send_success().await?;
        Ok(())
    }
}
