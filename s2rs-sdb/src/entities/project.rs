use std::sync::Arc;
use derivative::Derivative;
use s2rs_derive::deref;
use crate::api::{Api, self};
use super::User;

// region: ProjectMeta
#[derive(Debug)]
pub struct ProjectMeta {
    pub this: Arc<ProjectWithTitle>,
    pub sys_id: u64,
    pub author: Arc<User>,
    pub description: String,
    pub instructions: String,
    pub public: bool,
    pub comments_allowed: bool,
    pub history: api::ProjectHistory,
    pub remix: s2rs::api::ProjectRemix,
    pub stats: api::ProjectStats,
    pub meta: api::ProjectMeta
}

impl ProjectMeta {
    pub fn with_this_this(data: api::Project, this_this: Arc<Project>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            author: User::new(data.author_name, api),
            comments_allowed: data.comments_allowed,
            description: data.description,
            history: data.history,
            instructions: data.instructions,
            meta: data.meta,
            public: data.public,
            remix: data.remix,
            stats: data.stats,
            sys_id: data.sys_id,
            this: ProjectWithTitle::with_this(data.title, this_this)
        })
    }
}
// endregion: ProjectMeta

// region: ProjectWithTitle
#[deref(this)]
#[derive(Debug)]
pub struct ProjectWithTitle {
    pub this: Arc<Project>,
    pub title: String,
}

impl ProjectWithTitle {
    pub fn new(title: String, id: u64, api: Arc<Api>) -> Arc<Self> {
        Self::with_this(title, Project::new(id, api))
    }

    pub fn with_this(title: String, this: Arc<Project>) -> Arc<Self> {
        Arc::new(Self {
            this,
            title
        })
    }
}
// endregion: ProjectWithTitle

#[deref(this)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Project {
    #[derivative(Debug = "ignore")]
    api: Arc<Api>,
    pub this: Arc<s2rs::Project>,
}

impl Project {
    pub fn with_this(this: Arc<s2rs::Project>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            api,
            this
        }) 
    }

    pub fn new(id: u64, api: Arc<Api>) -> Arc<Self> {
        Self::with_this(s2rs::Project::new(id, api.this.clone()), api)
    }

    pub async fn sdb_meta(self: &Arc<Self>) -> Result<Arc<ProjectMeta>, api::GetProjectError> {
        Ok(ProjectMeta::with_this_this(self.api.get_project_sdb(self.id).await?, self.clone(), self.api.clone()))
    }
}