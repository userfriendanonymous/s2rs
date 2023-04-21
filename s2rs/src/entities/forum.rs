
use std::sync::Arc;
use crate::{Api, api};
#[cfg(feature = "time")] use chrono::{DateTime, Utc};
use derivative::Derivative;
use super::User;

// region: ForumTopicRss
#[derive(Debug)]
#[cfg(feature = "time")]
pub struct ForumTopicRss {
    pub this: Arc<ForumTopic>,
    pub title: String,
    pub updated_at: DateTime<Utc>,
    pub posts: Vec<Arc<ForumTopicRssPost>>,
}

#[cfg(feature = "rss")]
impl ForumTopicRss {
    pub fn with_this(data: api::ForumTopicRss, this: Arc<ForumTopic>, api: Arc<Api>) -> Self {
        Self {
            this,
            posts: ForumTopicRssPost::vec_new(data.posts, api),
            title: data.title,
            updated_at: data.updated_at
        }
    }
}
// endregion: ForumTopicRss

// region: ForumTopicRssPost
#[derive(Debug)]
#[cfg(feature = "time")]
pub struct ForumTopicRssPost {
    pub this: Arc<ForumPost>,
    pub created_at: DateTime<Utc>,
    pub author: Arc<User>,
    pub content: String,
}

#[cfg(feature = "rss")]
impl ForumTopicRssPost {
    pub fn new(data: api::ForumTopicRssPost, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            this: ForumPost::new(data.id, api.clone()),
            author: User::new(data.author_name, api),
            content: data.content,
            created_at: data.created_at
        })
    }

    pub fn vec_new(data: Vec<api::ForumTopicRssPost>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}
// endregion: ForumTopicRssPost

// region: ForumTopic
#[derive(Derivative)]
#[derivative(Debug)]
pub struct ForumTopic {
    pub id: u64,
    #[derivative(Debug = "ignore")]
    api: Arc<Api>
}

impl ForumTopic {
    pub fn new(id: u64, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            api,
            id
        })
    }

    #[cfg(feature = "rss")]
    pub async fn rss(self: &Arc<Self>) -> Result<ForumTopicRss, api::GetForumTopicRssError> {
        Ok(ForumTopicRss::with_this(self.api.forum_topic_rss(self.id).await?, self.clone(), self.api.clone()))
    }
}
// endregion: ForumTopic

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ForumPost {
    pub id: u64,
    #[derivative(Debug = "ignore")]
    api: Arc<Api>
}

impl ForumPost {
    pub fn new(id: u64, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            api,
            id
        })
    }

    pub async fn content(&self) -> api::Result<String> {
        self.api.forum_post_content(self.id).await
    }
}

