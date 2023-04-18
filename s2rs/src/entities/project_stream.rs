use std::sync::Arc;
use crate::api::{self, Api};
use crate::cursor::Cursor;
use super::{stream::{GeneralStreamGen, GeneralStreamResult}, Project, PartialProject, ProjectCommentMeta, CloudAction};
use async_trait::async_trait;

// region: ProjectRemixes
#[derive(Clone)] pub struct ProjectRemixes;
#[async_trait] impl GeneralStreamGen for ProjectRemixes {
    type Data = PartialProject;
    type Error = api::GeneralError;
    type This = Project;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(PartialProject::vec_new(api.get_project_remixes(this.id, cursor).await?, api.clone()))
    }
}
// endregion: ProjectRemixes

// region: ProjectComments
#[derive(Clone)] pub struct ProjectComments;
#[async_trait] impl GeneralStreamGen for ProjectComments {
    type Data = ProjectCommentMeta;
    type Error = api::GeneralError;
    type This = Project;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(ProjectCommentMeta::vec_new(api.get_project_comments(this.id, cursor).await?, this.clone(), api.clone()))
    }
}
// endregion: ProjectComments

// region: ProjectCloudActivity
#[derive(Clone)] pub struct ProjectCloudActivity;
#[async_trait] impl GeneralStreamGen for ProjectCloudActivity {
    type Data = CloudAction;
    type Error = api::GeneralError;
    type This = Project;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(CloudAction::vec_new(api.get_project_cloud_activity(this.id, cursor).await?, api.clone()))
    }
}
// endregion: ProjectCloudActivity