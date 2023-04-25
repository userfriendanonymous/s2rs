use std::sync::Arc;
use crate::api::{self, Api};
use super::{User, stream::{GeneralStreamResult, GeneralStreamGen}, Project3, UserProject, Studio2, UserMeta, FollowingAction, UserProjectCommentMeta, UserProjectComment};
#[cfg(feature = "html")] use super::UserCommentMeta;
use async_trait::async_trait;
use crate::cursor::Cursor;

// region: UserProjects
#[derive(Clone)] pub struct UserProjects;
#[async_trait] impl GeneralStreamGen for UserProjects {
    type Data = Project3;
    type Error = api::Error;
    type This = User;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(Project3::vec_new(api.user_projects(&this.name, cursor).await?, api.clone()))
    }
}
// endregion: UserProjects

// region: UserFavorites
#[derive(Clone)] pub struct UserFavorites;
#[async_trait] impl GeneralStreamGen for UserFavorites {
    type Data = Project3;
    type Error = api::Error;
    type This = User;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(Project3::vec_new(api.user_favorites(&this.name, cursor).await?, api.clone()))
    }
}
// endregion: UserFavorites

// region: UserCuratingStudios
#[derive(Clone)] pub struct UserCuratingStudios;
#[async_trait] impl GeneralStreamGen for UserCuratingStudios {
    type Data = Studio2;
    type Error = api::Error;
    type This = User;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(Studio2::vec_new(api.user_curating_studios(&this.name, cursor).await?, api.clone()))
    }
}
// endregion: UserCuratingStudios

// region: UserFollowers
#[derive(Clone)] pub struct UserFollowers;
#[async_trait] impl GeneralStreamGen for UserFollowers {
    type Data = UserMeta;
    type Error = api::Error;
    type This = User;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(UserMeta::vec_new(api.user_followers(&this.name, cursor).await?, api.clone()))
    }
}
// endregion: UserFollowers

// region: UserFollowing
#[derive(Clone)] pub struct UserFollowing;
#[async_trait] impl GeneralStreamGen for UserFollowing {
    type Data = UserMeta;
    type Error = api::Error;
    type This = User;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(UserMeta::vec_new(api.user_following(&this.name, cursor).await?, api.clone()))
    }
}
// endregion: UserFollowing

// region: UserMessages
#[derive(Clone)] pub struct UserMessages;
#[async_trait] impl GeneralStreamGen for UserMessages {
    type Data = UserMeta;
    type Error = api::Error;
    type This = User;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(UserMeta::vec_new(api.user_following(&this.name, cursor).await?, api.clone()))
    }
}
// endregion: UserMessages

// region: UserFollowingActivity
#[derive(Clone)] pub struct UserFollowingActivity;
#[async_trait] impl GeneralStreamGen for UserFollowingActivity {
    type Data = FollowingAction;
    type Error = api::GetFollowingUsersActivityError;
    type This = User;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(FollowingAction::vec_new(api.following_users_activity(&this.name, cursor).await?, api.clone()))
    }
}
// endregion: UserFollowingActivity

// region: UserComments
#[cfg(feature = "html")]
#[derive(Clone)] pub struct UserComments;
#[cfg(feature = "html")]
#[async_trait] impl GeneralStreamGen for UserComments {
    type Data = UserCommentMeta;
    type Error = api::GetUserCommentsError;
    type This = User;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(UserCommentMeta::vec_with_profile(api.user_comments(&this.name, cursor).await?, this.clone(), api.clone()))
    }
}
// endregion: UserComments

// region: UserProjectComments
#[derive(Clone)] pub struct UserProjectComments;
#[async_trait] impl GeneralStreamGen for UserProjectComments {
    type Data = UserProjectCommentMeta;
    type Error = api::Error;
    type This = UserProject;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(UserProjectCommentMeta::vec_new(api.user_project_comments(&this.author.name, this.id, cursor).await?, this.clone(), api.clone()))
    }
}
// endregion: UserProjectComments

// region: UserProjectCommentReplies
#[derive(Clone)] pub struct UserProjectCommentReplies;
#[async_trait] impl GeneralStreamGen for UserProjectCommentReplies {
    type Data = UserProjectCommentMeta;
    type Error = api::Error;
    type This = UserProjectComment;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(UserProjectCommentMeta::vec_new(api.user_project_comment_replies(&this.at.author.name, this.at.id, this.id, cursor).await?, this.at.clone(), api.clone()))
    }
}
// endregion: UserProjectCommentReplies