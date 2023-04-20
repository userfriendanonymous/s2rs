use s2rs_derive::Forwarder;

use super::{Api, utils::RequestBuilderUtils};
use crate::json;
use crate::cursor::Cursor;

#[derive(Debug)]
pub struct FollowingAction {
    pub id: u64,
    pub actor_name: String,
    pub actor_id: u64,
    pub created_at: String,
    pub event: FollowingActionEvent,
    pub event_type: String,
}

impl json::Parsable for FollowingAction {
    type Error = json::ExpectedError;
    fn parse(data: &json::Parser) -> Result<Self, Self::Error> {
        Ok(FollowingAction {
            actor_name: data.i("actor_username").string()?,
            actor_id: data.i("actor_id").u64()?,
            created_at: data.i("datetime_created").string()?,
            event: data.parse()?,
            id: data.i("id").u64()?,
            event_type: data.i("type").string()?
        })
    }
}

#[derive(Debug)]
pub enum FollowingActionEvent {
    FollowUser {
        to_name: String,
        to_id: u64,
    },
    FollowStudio {
        title: String,
        id: u64,
    },
    LoveProject {
        title: String,
        id: u64,
    },
    FavoriteProject {
        title: String,
        id: u64,
    },
    AcceptStudioInvite {
        id: u64,
        title: String,
        to_name: String,
    },
    ShareProject {
        title: String,
        id: u64,
    },
    RemixProject {
        parent_id: u64,
        parent_title: String,
        title: String,
        id: u64,
    },
    PromoteStudio {
        id: u64,
        title: String,
        to_name: String,
        to_id: u64,
    }
}

#[derive(Forwarder)]
pub enum FollowingActionEventParseError {
    InvalidEventType(String),
    Json(json::ExpectedError)
}

impl json::Parsable for FollowingActionEvent {
    type Error = FollowingActionEventParseError;
    fn parse(data: &json::Parser) -> Result<Self, Self::Error> {
        Ok(match data.i("type").string()?.as_str() {
            "followuser" => Self::FollowUser {
                to_id: data.i("followed_user_id").u64()?,
                to_name: data.i("followed_username").string()?
            },
            "followstudio" => Self::FollowStudio {
                title: data.i("title").string()?,
                id: data.i("gallery_id").u64()?
            },
            "loveproject" => Self::LoveProject {
                title: data.i("project_title").string()?,
                id: data.i("project_id").u64()?
            },
            "favoriteproject" => Self::FavoriteProject {
                title: data.i("project_title").string()?,
                id: data.i("project_id").u64()?
            },
            "becomecurator" => Self::AcceptStudioInvite {
                to_name: data.i("username").string()?,
                id: data.i("gallery_id").u64()?,
                title: data.i("title").string()?,
            },
            "shareproject" => Self::ShareProject {
                title: data.i("project_title").string()?,
                id: data.i("project_id").u64()?
            },
            "remixproject" => Self::RemixProject {
                parent_id: data.i("parent_id").u64()?,
                parent_title: data.i("parent_title").string()?,
                title: data.i("title").string()?,
                id: data.i("project_id").u64()?
            },
            "becomeownerstudio" => Self::PromoteStudio {
                id: data.i("gallery_id").u64()?,
                to_id: data.i("recipient_id").u64()?,
                to_name: data.i("recipient_username").string()?,
                title: data.i("gallery_title").string()?,
            },
            event_type => Err(FollowingActionEventParseError::InvalidEventType(event_type.to_owned()))?
        })
    }
}

impl Api {
    pub async fn get_following_users_activity(&self, name: &str, cursor: impl Into<Cursor>) -> super::Result<Vec<FollowingAction>> {
        let response = self.get(&format!["users/{name}/following/users/activity/"]).cursor(cursor).send_success().await?;
        response.general_parser_vec().await
    }
}