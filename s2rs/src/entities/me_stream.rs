use async_trait::async_trait;
use std::sync::Arc;
use crate::{api, Cursor, Api};
use super::{stream::{GeneralStreamGen, GeneralStreamResult}, Project2, Me};

// region: MeProjectsLovedByFollowing
#[derive(Clone)] pub struct MeProjectsLovedByFollowing;
#[async_trait] impl GeneralStreamGen for MeProjectsLovedByFollowing {
    type Data = Project2;
    type Error = api::Error;
    type This = Me;
    async fn gen(&self, cursor: Cursor, _this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(Project2::vec_new(api.projects_loved_by_following(cursor).await?, api.clone()))
    }
}
// endregion: MeProjectsLovedByFollowing

// region: MeViewedProjects
#[derive(Clone)] pub struct MeViewedProjects;
#[async_trait] impl GeneralStreamGen for MeViewedProjects {
    type Data = Project2;
    type Error = api::Error;
    type This = Me;
    async fn gen(&self, cursor: Cursor, _this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(Project2::vec_new(api.viewed_projects(cursor).await?, api.clone()))
    }
}
// endregion: MeViewedProjects

// region: MeProjectsSharedByFollowing
#[derive(Clone)] pub struct MeProjectsSharedByFollowing;
#[async_trait] impl GeneralStreamGen for MeProjectsSharedByFollowing {
    type Data = Project2;
    type Error = api::Error;
    type This = Me;
    async fn gen(&self, cursor: Cursor, _this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(Project2::vec_new(api.projects_shared_by_following(cursor).await?, api.clone()))
    }
}
// endregion: MeProjectsSharedByFollowing