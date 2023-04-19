use crate::{Api, cursor::Cursor};
use super::{ExploreMode, Project2, utils::{RequestBuilderUtils, ResponseUtils}, Studio2};

pub type SearchMode = ExploreMode;

#[derive(Clone, Debug)]
pub struct SearchQuery {
    pub mode: Option<SearchMode>,
    pub query: Option<String>,
}

impl SearchQuery {
    pub fn as_query(&self) -> Vec<(&str, &str)> {
        let mut result = Vec::new();
        if let Some(query) = &self.query {
            result.push(("q", query.as_str()))
        }
        if let Some(mode) = &self.mode {
            result.push(("mode", mode.as_ref()))
        }
        result
    }
}

impl Api {
    pub async fn search_projects(&self, query: &SearchQuery, cursor: impl Into<Cursor>) -> super::Result<Vec<Project2>> {
        let response = self.get("search/projects/").query(&query.as_query()).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn search_studios(&self, query: &SearchQuery, cursor: impl Into<Cursor>) -> super::Result<Vec<Studio2>> {
        let response = self.get("search/studios/").query(&query.as_query()).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }
}