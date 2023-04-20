use std::sync::Arc;
use crate::api::{self, Api};
use crate::cursor::Cursor;
use super::{stream::{GeneralStreamGen, GeneralStreamResult}, UserMeta, Studio, StudioProject, StudioCommentMeta, StudioAction};
use async_trait::async_trait;

// region: StudioCurators
#[derive(Clone)] pub struct StudioCurators;
#[async_trait] impl GeneralStreamGen for StudioCurators {
    type Data = UserMeta;
    type Error = api::Error;
    type This = Studio;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(UserMeta::vec_new(api.get_studio_curators(this.id, cursor).await?, api.clone()))
    }
}
// endregion: StudioCurators

// region: StudioManagers
#[derive(Clone)] pub struct StudioManagers;
#[async_trait] impl GeneralStreamGen for StudioManagers {
    type Data = UserMeta;
    type Error = api::Error;
    type This = Studio;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(UserMeta::vec_new(api.get_studio_managers(this.id, cursor).await?, api.clone()))
    }
}
// endregion: StudioManagers

// region: StudioProjects
#[derive(Clone)] pub struct StudioProjects;
#[async_trait] impl GeneralStreamGen for StudioProjects {
    type Data = StudioProject;
    type Error = api::Error;
    type This = Studio;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(StudioProject::vec_new(api.get_studio_projects(this.id, cursor).await?, api.clone()))
    }
}
// endregion: StudioProjects

// region: StudioComments
#[derive(Clone)] pub struct StudioComments;
#[async_trait] impl GeneralStreamGen for StudioComments {
    type Data = StudioCommentMeta;
    type Error = api::Error;
    type This = Studio;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(StudioCommentMeta::vec_new(api.get_studio_comments(this.id, cursor).await?, this.clone(), api.clone()))
    }
}
// endregion: StudioComments

// region: StudioActivity
#[derive(Clone)] pub struct StudioActivity;
#[async_trait] impl GeneralStreamGen for StudioActivity {
    type Data = StudioAction;
    type Error = api::GetStudioActivityError;
    type This = Studio;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(StudioAction::vec_new(api.get_studio_activity(this.id, cursor).await?, api.clone()))
    }
}
// endregion: StudioActivity
