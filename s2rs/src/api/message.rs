use super::{general_parser::{GeneralParser, GeneralParsable}, Api, utils::{ResponseUtils, RequestBuilderUtils}, GeneralResult, ParsingCustomError};
use crate::cursor::Cursor;

pub struct Message {
    pub id: u64,
    pub created_at: String,
    pub actor_name: String,
    pub actor_id: u64,
    pub event: MessageEvent,
    pub event_type: String,
}

impl GeneralParsable for Message {
    type Error = ParsingCustomError;
    fn parse(data: &GeneralParser) -> Result<Self, Self::Error> {
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

pub enum MessageCommentLocation {
    Profile,
    Project,
    Studio
}

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

impl GeneralParsable for MessageEvent {
    type Error = ParsingCustomError;
    fn parse(data: &GeneralParser) -> Result<Self, Self::Error> {
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
                    location: match location_type {
                        0 => MessageCommentLocation::Project,
                        1 => MessageCommentLocation::Profile,
                        2 => MessageCommentLocation::Studio,
                        _ => Err(ParsingCustomError)?
                    },
                    location_type,
                    location_id: data.i("comment_obj_id").u64()?,
                    location_title: data.i("comment_obj_title").string()?,
                    id: data.i("comment_id").u64()?,
                    fragment: data.i("comment_fragment").string()?,
                    to_name: data.i("commentee_username").option(|v| v.string()).transpose()?,
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
            _ => Err(ParsingCustomError)?
        })
    }
}

impl Api {
    pub async fn get_user_messages(&self, name: &str, cursor: impl Into<Cursor>) -> GeneralResult<Vec<Message>> {
        let response = self.get(&format!["users/{name}/messages/"]).cursor(cursor).send_success().await?;
        response.general_parser_vec().await
    }
}