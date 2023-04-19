
use async_trait::async_trait;
use crate::{Api, cursor::Cursor, api::{ExploreQuery, self}};
use std::sync::Arc;
use super::{stream::{GeneralStreamGen, GeneralStreamResult, GeneralStream}, Project2, Me, Studio2};

// region: streams
#[derive(Clone)] pub struct ExploreProjects {
    pub query: ExploreQuery
}
#[async_trait] impl GeneralStreamGen for ExploreProjects {
    type Data = Project2;
    type Error = api::Error;
    type This = Me;
    async fn gen(&self, cursor: Cursor, _: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(Project2::vec_new(api.explore_projects(&self.query, cursor).await?, api.clone()))
    }
}

#[derive(Clone)] pub struct ExploreStudios {
    pub query: ExploreQuery
}
#[async_trait] impl GeneralStreamGen for ExploreStudios {
    type Data = Studio2;
    type Error = api::Error;
    type This = Me;
    async fn gen(&self, cursor: Cursor, _: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(Studio2::vec_new(api.explore_studios(&self.query, cursor).await?, api.clone()))
    }
}
// endregion: streams

impl Me {
    pub fn explore_projects(self: &Arc<Self>, query: ExploreQuery, cursor: impl Into<Cursor>) -> GeneralStream<ExploreProjects> {
        GeneralStream::with_this(ExploreProjects { query }, cursor.into(), self.clone(), self.api.clone())
    }

    pub fn explore_studios(self: &Arc<Self>, query: ExploreQuery, cursor: impl Into<Cursor>) -> GeneralStream<ExploreStudios> {
        GeneralStream::with_this(ExploreStudios { query }, cursor.into(), self.clone(), self.api.clone())
    }
}