use std::sync::Arc;
use derivative::Derivative;
use crate::{Api, api::{SendComment, self}};
use super::Project;

#[allow(unused)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct ProjectComment {
    pub id: u64,
    pub at: Arc<Project>,
    #[derivative(Debug = "ignore")]
    api: Arc<Api>,
}

impl ProjectComment {
    pub fn with_at(id: u64, at: Arc<Project>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            api,
            at,
            id
        })
    }

    pub async fn reply(&self, content: impl Into<String>, to_id: Option<u64>) -> api::Result<()> {
        self.api.send_project_comment(self.at.id, &SendComment {
            content: content.into(),
            parent_id: Some(self.id),
            to_id
        }).await
    }
}
