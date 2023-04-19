use std::sync::Arc;
use crate::{api::Api, entities::{User, Project}};

pub struct Session {
    api: Arc<Api>,
    this: Arc<s2rs::Session>,
}

impl s2rs::session::Extension for Session {
    fn extended(pipe: s2rs::session::ExtensionPipe, this: Arc<s2rs::Session>) -> Arc<Self> {
        Arc::new(Self {
            api: pipe.api.extend::<Api>(),
            this
        })
    }
}

impl Session {
    pub fn user(&self, name: impl Into<String>) -> Arc<User> {
        User::new(Arc::new(name.into()), self.api.clone())
    }

    pub fn project(&self, id: u64) -> Arc<Project> {
        Project::new(id, self.api.clone())
    }
}