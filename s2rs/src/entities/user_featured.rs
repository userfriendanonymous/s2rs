use std::sync::Arc;
use super::{UserWithId, User, ProjectWithTitle};
use crate::{api, Api};

#[derive(Debug)]
pub struct UserFeatured {
    pub id: u64, // not sure what this field is for
    pub label: String,
    pub label_id: Option<u64>,
    pub project: UserFeaturedProject,
    pub profile_image_url: String,
    pub profile: Arc<UserWithId>,
}

impl UserFeatured {
    pub fn with_profile_this(data: api::UserFeatured, profile_this: Arc<User>, api: Arc<Api>) -> Self {
        Self {
            id: data.id,
            label: data.label,
            label_id: data.label_id,
            profile: UserWithId::with_this(data.profile.id, profile_this, api.clone()),
            profile_image_url: data.profile_image_url,
            project: UserFeaturedProject::new(data.project, api)
        }
    }
}

#[derive(Debug)]
pub struct UserFeaturedProject {
    pub this: Arc<ProjectWithTitle>,
    pub thumbnail_url: String,
    pub modified_at: String,
}

impl UserFeaturedProject {
    pub fn new(data: api::UserFeaturedProject, api: Arc<Api>) -> Self {
        Self {
            this: ProjectWithTitle::new(data.title, data.id, api),
            thumbnail_url: data.thumbnail_url,
            modified_at: data.modified_at
        }
    }
}