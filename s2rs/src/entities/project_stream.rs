use std::sync::Arc;
use crate::ProjectComment;
use crate::api::{self, Api};
use crate::cursor::Cursor;
use super::{stream::{GeneralStreamGen, GeneralStreamResult}, Project, Project3, ProjectCommentMeta, CloudAction};
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

// region: ProjectComments
#[derive(Clone)] pub struct ProjectComments;
#[async_trait] impl GeneralStreamGen for ProjectComments {
    type Data = ProjectCommentMeta;
    type Error = api::Error;
    type This = Project;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(ProjectCommentMeta::vec_new(api.project_comments(this.id, cursor).await?, this.clone(), api.clone()))
    }
}
// endregion: ProjectComments

// region: ProjectCommentReplies
#[derive(Clone)] pub struct ProjectCommentReplies;
#[async_trait] impl GeneralStreamGen for ProjectCommentReplies {
    type Data = ProjectCommentMeta;
    type Error = api::Error;
    type This = ProjectComment;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(ProjectCommentMeta::vec_new(api.project_comment_replies(this.at.id, this.id, cursor).await?, this.at.clone(), api.clone()))
    }
}
// endregion: ProjectCommentReplies

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