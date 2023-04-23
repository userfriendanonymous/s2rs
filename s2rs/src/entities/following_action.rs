use super::{UserWithId, User, ProjectWithTitle, StudioWithTitle};
use std::sync::Arc;
use crate::api::{Api, self};

// region: FollowingAction
#[derive(Debug)]
pub struct FollowingAction {
    pub id: u64,
    pub actor: Arc<UserWithId>,
    pub created_at: String,
    pub event: FollowingActionEvent,
}

impl FollowingAction {
    pub fn new(data: api::FollowingAction, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            actor: UserWithId::new(data.actor_id, data.actor_name, api.clone()),
            created_at: data.created_at,
            event: FollowingActionEvent::new(data.event, api),
            id: data.id
        })
    }

    pub fn vec_new(data: Vec<api::FollowingAction>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}
// endregion: FollowingAction

// region: FollowingActionEvent
#[derive(Debug)]
pub enum FollowingActionEvent {
    FollowUser(Arc<UserWithId>),
    FollowStudio(Arc<StudioWithTitle>),
    LoveProject(Arc<ProjectWithTitle>),
    FavoriteProject(Arc<ProjectWithTitle>),
    AcceptStudioInvite {
        this: Arc<StudioWithTitle>,
        to: Arc<User>
    },
    ShareProject(Arc<ProjectWithTitle>),
    RemixProject {
        parent: Arc<ProjectWithTitle>,
        this: Arc<ProjectWithTitle>,
    },
    PromoteStudio {
        to: Arc<UserWithId>,
        this: Arc<StudioWithTitle>
    }
}

impl FollowingActionEvent {
    fn new(data: api::FollowingActionEvent, api: Arc<Api>) -> Self {
        match data {
            api::FollowingActionEvent::FavoriteProject { title, id } => Self::FavoriteProject(
                ProjectWithTitle::new(title, id, api)
            ),

            api::FollowingActionEvent::LoveProject { title, id } => Self::LoveProject(
                ProjectWithTitle::new(title, id, api)
            ),

            api::FollowingActionEvent::RemixProject { parent_id, parent_title, title, id } => Self::RemixProject {
                parent: ProjectWithTitle::new(parent_title, parent_id, api.clone()),
                this: ProjectWithTitle::new(title, id, api)
            },

            api::FollowingActionEvent::AcceptStudioInvite { id, title, to_name } => Self::AcceptStudioInvite {
                this: StudioWithTitle::new(title, id, api.clone()),
                to: User::new(to_name, api)
            },

            api::FollowingActionEvent::FollowStudio { title, id } => Self::FollowStudio(
                StudioWithTitle::new(title, id, api)
            ),

            api::FollowingActionEvent::FollowUser { to_name, to_id } => Self::FollowUser(
                UserWithId::new(to_id, to_name, api)
            ),

            api::FollowingActionEvent::PromoteStudio { id, title, to_name, to_id } => Self::PromoteStudio {
                to: UserWithId::new(to_id, to_name, api.clone()),
                this: StudioWithTitle::new(title, id, api)
            },

            api::FollowingActionEvent::ShareProject { title, id } => Self::ShareProject(
                ProjectWithTitle::new(title, id, api)
            )
        }
    }
}
// endregion: FollowingActionEvent
