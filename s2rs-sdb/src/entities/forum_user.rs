use std::sync::Arc;
use derivative::Derivative;
use s2rs::api::ForumCategory;
use s2rs_derive::deref;
use crate::api::{Api, self};
use super::User;

#[deref(this)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct ForumUser {
    pub this: Arc<User>,
    #[derivative(Debug = "ignore")]
    pub api: Arc<Api>
}

impl ForumUser {
    pub async fn posts_history(&self, category: &ForumCategory) -> Result<Vec<api::ForumUserPostHistory>, api::GetForumUserPostsHistoryError> {
        self.api.get_forum_user_posts_history(&self.name, category).await
    }
}