use std::sync::Arc;
use crate::{api::{Api, Tokens, self}, entities::{User, Project, Studio, Me, ForumTopic, ForumPost}};

pub struct ExtensionPipe {
    pub me: Arc<Me>,
    pub api: Arc<Api>,
}

pub trait Extension {
    fn extended(pipe: ExtensionPipe, this: Arc<Session>) -> Arc<Self>;
}

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
    pub fn extend<T: Extension>(self: &Arc<Self>) -> Arc<T> {
        T::extended(ExtensionPipe {
            api: self.api.clone(),
            me: self.me.clone()
        }, self.clone())
    }

    pub fn new(name: impl Into<Arc<String>>) -> Arc<Self> {
        let name: Arc<_> = name.into();
        let api = Api::new(name.clone());
        Arc::new(Self {
            me: Me::with_this(User::new(name, api.clone()), api.clone()),
            api,
        })
    }

    pub fn with_auth(name: impl Into<Arc<String>>, tokens: &Tokens) -> Result<Arc<Self>, api::WithAuthError> {
        let name: Arc<_> = name.into();
        let api = Api::with_auth(name.clone(), tokens)?;
        Ok(Arc::new(Self {
            me: Me::with_this(User::new(name, api.clone()), api.clone()),
            api
        }))
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

    pub fn forum_topic(&self, id: u64) -> Arc<ForumTopic> {
        ForumTopic::new(id, self.api.clone())
    }

    pub fn forum_topic_post(&self, id: u64) -> Arc<ForumPost> {
        ForumPost::new(id, self.api.clone())
    }

    pub fn me(&self) -> Arc<Me> {
        self.me.clone()
    }
}