use async_trait::async_trait;
use crate::api::{self, SearchQuery};
use crate::{Api, cursor::Cursor};
use super::{Project2, Me, Studio2};
use super::stream::{GeneralStreamResult, GeneralStreamGen, GeneralStream};
use std::sync::Arc;

// region: streams
#[derive(Clone)] pub struct SearchProjects {
    pub query: SearchQuery
}
#[async_trait] impl GeneralStreamGen for SearchProjects {
    type Data = Project2;
    type Error = api::GeneralError;
    type This = Me;
    async fn gen(&self, cursor: Cursor, _: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(Project2::vec_new(api.search_projects(&self.query, cursor).await?, api.clone()))
    }
}

#[derive(Clone)] pub struct SearchStudios {
    pub query: SearchQuery
}
#[async_trait] impl GeneralStreamGen for SearchStudios {
    type Data = Studio2;
    type Error = api::GeneralError;
    type This = Me;
    async fn gen(&self, cursor: Cursor, _: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self> {
        Ok(Studio2::vec_new(api.search_studios(&self.query, cursor).await?, api.clone()))
    }
}
// endregion: streams

impl Me {
    pub async fn search_projects(self: &Arc<Self>, query: SearchQuery, cursor: impl Into<Cursor>) -> GeneralStream<SearchProjects> {
        GeneralStream::with_this(SearchProjects { query }, cursor.into(), self.clone(), self.api.clone())
    }

    pub async fn search_studios(self: &Arc<Self>, query: SearchQuery, cursor: impl Into<Cursor>) -> GeneralStream<SearchStudios> {
        GeneralStream::with_this(SearchStudios { query }, cursor.into(), self.clone(), self.api.clone())
    }
}