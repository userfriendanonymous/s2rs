use std::sync::Arc;
use crate::{api::{Api, Tokens, self}, entities::{User, Project, Studio, Me}};

/// Session abstracts away plain and flat api requests and makes library usage very intuitive.
/// # Example
/// ```
/// use s2rs::session::Session;
/// # tokio_test::block_on(async {
/// let session = Session::new("griffpatch");
/// let user = session.user("TimMcCool");
/// let meta_data = user.meta().await.unwrap();
/// # })
/// ```
pub struct Session {
    api: Arc<Api>,
    me: Arc<Me>,
}

impl Session {
    pub fn new(name: impl Into<String>) -> Self {
        let name: String = name.into();
        let api = Arc::new(Api::new(name.clone()));
        Self {
            me: Me::with_this(User::new(name, api.clone()), api.clone()),
            api,
        }
    }

    pub fn with_auth(name: impl Into<String>, tokens: &Tokens) -> Result<Self, api::Error> {
        let name: String = name.into();
        let api = Arc::new(Api::with_auth(name.clone(), tokens)?);
        Ok(Self {
            me: Me::with_this(User::new(name, api.clone()), api.clone()),
            api
        })
    }

    pub fn user(&self, name: impl Into<String>) -> Arc<User> {
        User::new(name.into(), self.api.clone())
    }

    pub fn project(&self, id: u64) -> Arc<Project> {
        Project::new(id, self.api.clone())
    }

    pub fn studio(&self, id: u64) -> Arc<Studio> {
        Studio::new(id, self.api.clone())
    }

    pub fn me(&self) -> Arc<Me> {
        self.me.clone()
    }
}