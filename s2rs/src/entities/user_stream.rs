use std::sync::Arc;
use crate::api::{self, Api};
use super::{User, stream::{GeneralStreamResult, GeneralStreamGen}, Project3, StudioMeta, UserMeta, Project2, FollowingAction};
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
    type Data = StudioMeta;
    type Error = api::Error;
    type This = User;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(StudioMeta::vec_new(api.user_curating_studios(&this.name, cursor).await?, api.clone()))
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

// region: UserProjectsLovedByFollowing
#[derive(Clone)] pub struct UserProjectsLovedByFollowing;
#[async_trait] impl GeneralStreamGen for UserProjectsLovedByFollowing {
    type Data = Project2;
    type Error = api::Error;
    type This = User;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(Project2::vec_new(api.projects_loved_by_following(&this.name, cursor).await?, api.clone()))
    }
}
// endregion: UserProjectsLovedByFollowing

// region: UserViews
#[derive(Clone)] pub struct UserViews;
#[async_trait] impl GeneralStreamGen for UserViews {
    type Data = Project2;
    type Error = api::Error;
    type This = User;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(Project2::vec_new(api.projects_loved_by_following(&this.name, cursor).await?, api.clone()))
    }
}
// endregion: UserViews

// region: UserProjectsSharedByFollowing
#[derive(Clone)] pub struct UserProjectsSharedByFollowing;
#[async_trait] impl GeneralStreamGen for UserProjectsSharedByFollowing {
    type Data = Project2;
    type Error = api::Error;
    type This = User;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(Project2::vec_new(api.projects_shared_by_following(&this.name, cursor).await?, api.clone()))
    }
}
// endregion: UserProjectsSharedByFollowing

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
