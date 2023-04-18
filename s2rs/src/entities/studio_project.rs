use std::sync::Arc;
use crate::api::{Api, self};
use super::{User, ProjectWithTitle};

#[derive(Debug)]
pub struct StudioProject {
    pub this: Arc<ProjectWithTitle>,
    pub image: String,
    // avatar: ProfileImages,
    pub creator: Arc<User>, // name
    pub actor_id: u64,
    pub creator_id: u64,
}

impl StudioProject {
    pub fn new(data: api::StudioProject, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            this: ProjectWithTitle::new(data.title, data.id, api.clone()),
            creator: User::new(data.name, api),
            actor_id: data.actor_id,
            creator_id: data.creator_id,
            image: data.image,
        })
    }

    pub fn vec_new(data: Vec<api::StudioProject>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}