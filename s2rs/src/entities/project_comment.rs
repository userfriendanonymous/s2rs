use std::sync::Arc;
use s2rs_derive::deref;
use crate::api::{self, Api};
use super::{UserWithId, Project};

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

// region: ProjectComment
#[derive(Debug, PartialEq, Eq)]
pub struct ProjectComment {
    pub id: u64,
    pub at: Arc<Project>,
}

impl ProjectComment {
    pub fn with_at(id: u64, at: Arc<Project>) -> Arc<Self> {
        Arc::new(Self {
            at,
            id
        })
    }
}
// endregion: ProjectComment

// region: ProjectCommentMeta
#[derive(Debug)]
#[deref(this)]
pub struct ProjectCommentMeta {
    pub this: Arc<ProjectComment>,
    pub author: CommentAuthor,
    pub parent: Option<Arc<ProjectComment>>,
    pub to_user_id: Option<u64>,
    pub content: String,
    pub created_at: String,
    pub modified_at: String,
    pub reply_count: u64,
}

impl ProjectCommentMeta {
    pub fn with_this(data: api::Comment, this: Arc<ProjectComment>, at: Arc<Project>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            author: CommentAuthor::new(data.author, api),
            content: data.content,
            created_at: data.created_at,
            modified_at: data.modified_at,
            parent: data.parent_id.map(|parent_id| ProjectComment::with_at(parent_id, at)),
            reply_count: data.reply_count,
            to_user_id: data.to_user_id,
            this,
        })
    }

    pub fn new(data: api::Comment, at: Arc<Project>, api: Arc<Api>) -> Arc<Self> {
        let id = data.id;
        Self::with_this(data, ProjectComment::with_at(id, at.clone()), at, api)
    }

    pub fn vec_new(data: Vec<api::Comment>, at: Arc<Project>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, at.clone(), api.clone())).collect()
    }
}
// endregion: ProjectCommentMeta
