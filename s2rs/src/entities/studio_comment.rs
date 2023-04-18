use std::sync::Arc;
use s2rs_derive::deref;

use crate::api::{self, Api};
use super::{Studio, CommentAuthor};

// region: StudioComment
#[derive(Debug, PartialEq, Eq)]
pub struct StudioComment {
    pub id: u64,
    pub at: Arc<Studio>,
}

impl StudioComment {
    pub fn with_at(id: u64, at: Arc<Studio>) -> Arc<Self> {
        Arc::new(Self {
            at,
            id
        })
    }
}
// endregion: StudioComment

// region: StudioCommentMeta
#[derive(Debug)]
#[deref(this)]
pub struct StudioCommentMeta {
    pub this: Arc<StudioComment>,
    pub author: CommentAuthor,
    pub parent: Option<Arc<StudioComment>>,
    pub to_user_id: Option<u64>,
    pub content: String,
    pub created_at: String,
    pub modified_at: String,
    pub reply_count: u64,
}

impl StudioCommentMeta {
    pub fn with_this(data: api::Comment, this: Arc<StudioComment>, at: Arc<Studio>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            author: CommentAuthor::new(data.author, api),
            content: data.content,
            created_at: data.created_at,
            modified_at: data.modified_at,
            parent: data.parent_id.map(|parent_id| StudioComment::with_at(parent_id, at)),
            reply_count: data.reply_count,
            to_user_id: data.to_user_id,
            this,
        })
    }

    pub fn new(data: api::Comment, at: Arc<Studio>, api: Arc<Api>) -> Arc<Self> {
        let id = data.id;
        Self::with_this(data, StudioComment::with_at(id, at.clone()), at, api)
    }

    pub fn vec_new(data: Vec<api::Comment>, at: Arc<Studio>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, at.clone(), api.clone())).collect()
    }
}
// endregion: ProjectCommentMeta
