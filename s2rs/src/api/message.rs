use s2rs_derive::Forwarder;
use super::utils::ResponseUtils;
use super::{Api, utils::RequestBuilderUtils};
use crate::cursor::Cursor;
use crate::json;

// region: Message
pub struct Message {
    pub id: u64,
    pub created_at: String,
    pub actor_name: String,
    pub actor_id: u64,
    pub event: MessageEvent,
    pub event_type: String,
}

#[derive(Forwarder, Debug, Clone)]
pub enum MessageParseError {
    #[forward] Expected(json::ExpectedError),
    #[forward] Event(MessageEventParseError)
}

impl json::Parsable for Message {
    type Error = MessageParseError;
    fn parse(data: &json::Parser) -> Result<Self, Self::Error> {
        Ok(Self {
            id: data.i("id").u64()?,
            created_at: data.i("datetime_created").string()?,
            actor_name: data.i("actor_username").string()?,
            actor_id: data.i("actor_id").u64()?,
            event: data.parse()?,
            event_type: data.i("type").string()?,
        })
    }
}
// endregion: Message

// region: MessageCommentLocation
pub enum MessageCommentLocation {
    Profile,
    Project,
    Studio
}

impl MessageCommentLocation {
    pub fn from_u8(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::Project,
            1 => Self::Profile,
            2 => Self::Studio,
            _ => None?
        })
    }
}
// endregion: MessageCommentLocation

// region: MessageEvent
pub enum MessageEvent {
    FollowUser {
        to_id: u64,
        to_name: String,
    },
    LoveProject {
        id: u64,
        title: String,
    },
    FavoriteProject {
        id: u64,
        title: String,
    },
    RemixProject {
        id: u64,
        title: String,
        parent_id: u64,
        parent_title: String,
    },
    AddComment {
        location: MessageCommentLocation,
        location_type: u8,
        location_id: u64,
        location_title: String,
        id: u64,
        fragment: String,
        to_name: Option<String>,
    },
    InviteCurator {
        id: u64,
        title: String,
    },
    PromoteStudio {
        id: u64,
        title: String,
    },
    StudioActivity {
        id: u64,
        title: String,
    },
    ForumPost {
        id: u64,
        title: String,
    },
    Welcome
}

#[derive(Forwarder, Debug, Clone)]
pub enum MessageEventParseError {
    #[forward] Expected(json::ExpectedError),
    #[forward] CommentLocation(u8),
    InvalidType(String)
}

impl json::Parsable for MessageEvent {
    type Error = MessageEventParseError;
    fn parse(data: &json::Parser) -> Result<Self, Self::Error> {
        Ok(match data.i("type").string()?.as_str() {
            "followuser" => Self::FollowUser {
                to_id: data.i("followed_user_id").u64()?,
                to_name: data.i("followed_username").string()?,
            },
            "loveproject" => Self::LoveProject {
                id: data.i("project_id").u64()?,
                title: data.i("title").string()?
            },
            "favoriteproject" => Self::FavoriteProject {
                id: data.i("project_id").u64()?,
                title: data.i("project_title").string()?,
            },
            "remixproject" => Self::RemixProject {
                id: data.i("project_id").u64()?,
                title: data.i("title").string()?,
                parent_id: data.i("parent_id").u64()?,
                parent_title: data.i("parent_title").string()?
            },
            "addcomment" => {
                let location_type = data.i("comment_type").u8()?;
                Self::AddComment {
                    location: MessageCommentLocation::from_u8(location_type).ok_or(MessageEventParseError::CommentLocation(location_type))?,
                    location_type,
                    location_id: data.i("comment_obj_id").u64()?,
                    location_title: data.i("comment_obj_title").string()?,
                    id: data.i("comment_id").u64()?,
                    fragment: data.i("comment_fragment").string()?,
                    to_name: data.try_i("commentee_username")?,
                }
            },
            "curatorinvite" => Self::InviteCurator {
                id: data.i("gallery_id").u64()?,
                title: data.i("title").string()?,
            },
            "becomeownerstudio" => Self::PromoteStudio {
                id: data.i("gallery_id").u64()?,
                title: data.i("title").string()?,
            },
            "studioactivity" => Self::StudioActivity {
                id: data.i("gallery_id").u64()?,
                title: data.i("title").string()?,
            },
            "forumpost" => Self::ForumPost {
                id: data.i("topic_id").u64()?,
                title: data.i("topic_title").string()?,
            },
            t => Err(MessageEventParseError::InvalidType(t.to_owned()))?
        })
    }
}
// endregion: MessageEvent

#[derive(Forwarder, Debug)]
pub enum GetUserMessagesError {
    #[forward] This(super::Error),
    #[forward] Parsing(MessageParseError)
}

impl Api {
    pub async fn user_messages(&self, name: &str, cursor: impl Into<Cursor>) -> Result<Vec<Message>, GetUserMessagesError> {
        let response = self.get(&format!["users/{name}/messages/"]).cursor(cursor).send_success().await?;
        response.json_parser_vec().await
    }
}