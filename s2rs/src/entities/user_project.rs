use std::sync::Arc;
use s2rs_derive::deref;
use super::{Project, User, UserWithId};
use derivative::Derivative;
use crate::api::{self, Api};
#[cfg(feature = "stream")] use super::{stream::GeneralStream, UserProjectCommentReplies, UserProjectComments};
#[cfg(feature = "stream")] use crate::cursor::Cursor;

// region: UserProject
#[deref(this)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct UserProject {
    pub this: Arc<Project>,
    pub author: Arc<User>,
    #[derivative(Debug = "ignore")]
    pub api: Arc<Api>
}

impl UserProject {
    pub fn with_author(id: u64, author: Arc<User>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            api: api.clone(),
            author,
            this: Project::new(id, api),
        })
    }

    #[cfg(feature = "stream")]
    pub fn comments(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<UserProjectComments> {
        GeneralStream::with_this(UserProjectComments, cursor.into(), self.clone(), self.api.clone())
    }
}
// endregion: UserProject

// region: CommentAuthor
#[derive(Debug)]
#[deref(this)]
pub struct CommentAuthor {
    pub this: Arc<UserWithId>,
    pub scratch_team: bool,
    pub image: String
}

impl CommentAuthor {
    pub fn new(data: api::CommentAuthor, api: Arc<Api>) -> Self {
        Self {
            image: data.image,
            scratch_team: data.scratch_team,
            this: UserWithId::new(data.id, data.name, api)
        }
    }
}
// endregion: CommentAuthor

// region: UserProjectComment
#[derive(Derivative)]
#[derivative(Debug)]
pub struct UserProjectComment {
    pub id: u64,
    pub at: Arc<UserProject>,
    #[derivative(Debug = "ignore")]
    api: Arc<Api>,
}

impl UserProjectComment {
    pub fn with_at(id: u64, at: Arc<UserProject>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            at,
            id,
            api
        })
    }

    #[cfg(feature = "stream")]
    pub fn replies(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<UserProjectCommentReplies> {
        GeneralStream::with_this(UserProjectCommentReplies, cursor.into(), self.clone(), self.api.clone())
    }
}
// endregion: UserProjectComment

// region: UserProjectCommentMeta
#[derive(Debug)]
#[deref(this)]
pub struct UserProjectCommentMeta {
    pub this: Arc<UserProjectComment>,
    pub author: CommentAuthor,
    pub parent: Option<Arc<UserProjectComment>>,
    pub to_user_id: Option<u64>,
    pub content: String,
    pub created_at: String,
    pub modified_at: String,
    pub reply_count: u64,
}

impl UserProjectCommentMeta {
    pub fn with_this(data: api::Comment, this: Arc<UserProjectComment>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            content: data.content,
            created_at: data.created_at,
            modified_at: data.modified_at,
            parent: data.parent_id.map(|parent_id| UserProjectComment::with_at(parent_id, this.at.clone(), api.clone())),
            author: CommentAuthor::new(data.author, api),
            reply_count: data.reply_count,
            to_user_id: data.to_user_id,
            this,
        })
    }

    pub fn new(data: api::Comment, at: Arc<UserProject>, api: Arc<Api>) -> Arc<Self> {
        let id = data.id;
        Self::with_this(data, UserProjectComment::with_at(id, at, api.clone()), api)
    }

    pub fn vec_new(data: Vec<api::Comment>, at: Arc<UserProject>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, at.clone(), api.clone())).collect()
    }
}
// endregion: UserProjectCommentMeta
