use std::sync::Arc;
use crate::api::{self, Api};
use crate::cursor::Cursor;
use super::{stream::{GeneralStreamGen, GeneralStreamResult}, Project, Project3, CloudAction};
use async_trait::async_trait;

// region: ProjectRemixes
#[derive(Clone)] pub struct ProjectRemixes;
#[async_trait] impl GeneralStreamGen for ProjectRemixes {
    type Data = Project3;
    type Error = api::Error;
    type This = Project;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(Project3::vec_new(api.project_remixes(this.id, cursor).await?, api.clone()))
    }
}
// endregion: ProjectRemixes

// region: ProjectCloudActivity
#[derive(Clone)] pub struct ProjectCloudActivity;
#[async_trait] impl GeneralStreamGen for ProjectCloudActivity {
    type Data = CloudAction;
    type Error = api::GetProjectCloudActivityError;
    type This = Project;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(CloudAction::vec_new(api.project_cloud_activity(this.id, cursor).await?, api.clone()))
    }
}
// endregion: ProjectCloudActivity