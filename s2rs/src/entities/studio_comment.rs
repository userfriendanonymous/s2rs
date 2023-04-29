use std::sync::Arc;
use derivative::Derivative;
use s2rs_derive::deref;

use crate::api::{self, Api, SendComment};
use super::{Studio, CommentAuthor};
#[cfg(feature = "stream")] use super::{stream::GeneralStream, StudioCommentReplies};
#[cfg(feature = "stream")] use crate::cursor::Cursor;

// region: StudioComment
#[allow(unused)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct StudioComment {
    pub id: u64,
    pub at: Arc<Studio>,
    #[derivative(Debug = "ignore")]
    api: Arc<Api>
}

impl StudioComment {
    pub fn with_at(id: u64, at: Arc<Studio>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            at,
            id,
            api
        })
    }

    #[cfg(feature = "stream")]
    pub async fn replies(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<StudioCommentReplies> {
        GeneralStream::with_this(StudioCommentReplies, cursor.into(), self.clone(), self.api.clone())
    }

    pub async fn reply(&self, content: impl Into<String>, to_id: Option<u64>) -> api::Result<()> {
        self.api.send_studio_comment(self.at.id, &SendComment {
            content: content.into(),
            parent_id: Some(self.id),
            to_id
        }).await
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
            content: data.content,
            created_at: data.created_at,
            modified_at: data.modified_at,
            parent: data.parent_id.map(|parent_id| StudioComment::with_at(parent_id, at, api.clone())),
            author: CommentAuthor::new(data.author, api),
            reply_count: data.reply_count,
            to_user_id: data.to_user_id,
            this,
        })
    }

    pub fn new(data: api::Comment, at: Arc<Studio>, api: Arc<Api>) -> Arc<Self> {
        let id = data.id;
        Self::with_this(data, StudioComment::with_at(id, at.clone(), api.clone()), at, api)
    }

    pub fn vec_new(data: Vec<api::Comment>, at: Arc<Studio>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, at.clone(), api.clone())).collect()
    }
}
// endregion: ProjectCommentMeta
