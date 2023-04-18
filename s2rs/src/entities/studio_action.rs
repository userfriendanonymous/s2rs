use super::{UserWithId, User, ProjectWithTitle};
use std::sync::Arc;
use crate::api::{Api, self};

// region: StudioAction
#[derive(Debug)]
pub struct StudioAction {
    pub id: u64,
    pub actor: Arc<UserWithId>,
    pub created_at: String,
    pub event: StudioActionEvent,
}

impl StudioAction {
    pub fn new(data: api::StudioAction, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            actor: UserWithId::new(data.actor_id, data.actor_name, api.clone()),
            created_at: data.created_at,
            event: StudioActionEvent::new(data.event, api),
            id: data.id
        })
    }

    pub fn vec_new(data: Vec<api::StudioAction>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}
// endregion: StudioAction

// region: StudioActionEvent
#[derive(Debug, PartialEq, Eq)]
pub enum StudioActionEvent {
    Update,
    AddProject(Arc<ProjectWithTitle>),
    RemoveProject(Arc<ProjectWithTitle>),
    AcceptInvite(Arc<User>),
    Promote(Arc<User>),
}

impl StudioActionEvent {
    fn new(data: api::StudioActionEvent, api: Arc<Api>) -> Self {
        match data {
            api::StudioActionEvent::AddProject { id, title } => Self::AddProject(
                ProjectWithTitle::new(title, id, api)
            ),
            api::StudioActionEvent::AcceptInvite { from_name } => Self::AcceptInvite(
                User::new(from_name, api)
            ),
            api::StudioActionEvent::Promote { name } => Self::Promote(
                User::new(name, api)
            ),
            api::StudioActionEvent::RemoveProject { id, title } => Self::RemoveProject(
                ProjectWithTitle::new(title, id, api)
            ),
            api::StudioActionEvent::Update => Self::Update
        }
    }
}
// endregion: StudioActionEvent
