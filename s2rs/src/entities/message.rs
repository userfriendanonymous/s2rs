use super::{User, UserWithId, StudioWithTitle, ProjectWithTitle};
use std::sync::Arc;
use crate::api::{Api, self};

// region: Message
#[derive(Debug)]
pub struct Message {
    pub id: u64,
    pub created_at: String,
    pub actor: Arc<UserWithId>,
    pub event: MessageEvent,
}

impl Message {
    pub fn new(data: api::Message, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            actor: UserWithId::new(data.actor_id, data.actor_name, api.clone()),
            created_at: data.created_at,
            event: MessageEvent::new(data.event, api),
            id: data.id
        })
    }

    pub fn vec_new(data: Vec<api::Message>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}
// endregion: Message

// region: MessageCommentLocation
#[derive(Debug, PartialEq, Eq)]
pub enum MessageCommentLocation {
    Profile(Arc<UserWithId>),
    Studio(Arc<StudioWithTitle>),
    Project(Arc<ProjectWithTitle>),
}

impl MessageCommentLocation {
    fn new(variant: api::MessageCommentLocation, id: u64, title: String, api: Arc<Api>) -> Self {
        match variant {
            api::MessageCommentLocation::Profile => Self::Profile(
                UserWithId::new(id, title, api)
            ),
            api::MessageCommentLocation::Project => Self::Project(
                ProjectWithTitle::new(title, id, api)
            ),
            api::MessageCommentLocation::Studio => Self::Studio(
                StudioWithTitle::new(title, id, api)
            )
        }
    }
}
// endregion: MessageCommentLocation

// region: MessageEvent
#[derive(Debug, PartialEq, Eq)]
pub enum MessageEvent {
    FollowUser(Arc<UserWithId>),
    LoveProject(Arc<ProjectWithTitle>),
    FavoriteProject(Arc<ProjectWithTitle>),
    RemixProject {
        this: Arc<ProjectWithTitle>,
        parent: Arc<ProjectWithTitle>,
    },
    Comment {
        location: MessageCommentLocation,
        id: u64,
        fragment: String,
        to: Option<Arc<User>>,
    },
    InviteCurator(Arc<StudioWithTitle>),
    PromoteStudio(Arc<StudioWithTitle>),
    StudioActivity(Arc<StudioWithTitle>),
    ForumPost {
        id: u64,
        title: String,
    },
    Welcome
}

impl MessageEvent {
    fn new(data: api::MessageEvent, api: Arc<Api>) -> Self {
        match data {
            api::MessageEvent::AddComment {
                location_type: _, location, location_id, location_title, id,
                fragment, to_name
            } => Self::Comment {
                to: to_name.map(|name| User::new(name, api.clone())),
                location: MessageCommentLocation::new(location, location_id, location_title, api),
                id,
                fragment,
            },

            api::MessageEvent::InviteCurator { id, title } => Self::InviteCurator(
                StudioWithTitle::new(title, id, api)
            ),

            api::MessageEvent::FavoriteProject { id, title } => Self::FavoriteProject(
                ProjectWithTitle::new(title, id, api)
            ),

            api::MessageEvent::FollowUser { to_id, to_name } => Self::FollowUser(
                UserWithId::new(to_id, to_name, api)
            ),

            api::MessageEvent::ForumPost { id, title } => Self::ForumPost { id, title },

            api::MessageEvent::LoveProject { id, title } => Self::LoveProject(
                ProjectWithTitle::new(title, id, api)
            ),

            api::MessageEvent::RemixProject { id, title, parent_id, parent_title } => Self::RemixProject {
                this: ProjectWithTitle::new(title, id, api.clone()),
                parent: ProjectWithTitle::new(parent_title, parent_id, api)
            },

            api::MessageEvent::StudioActivity { id, title } => Self::StudioActivity(
                StudioWithTitle::new(title, id, api)
            ),

            api::MessageEvent::PromoteStudio { id, title } => Self::PromoteStudio(
                StudioWithTitle::new(title, id, api)
            ),

            api::MessageEvent::Welcome => Self::Welcome
        }
    }
}
// endregion: MessageEvent
