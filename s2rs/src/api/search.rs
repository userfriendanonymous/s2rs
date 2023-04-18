use crate::{Api, cursor::Cursor};
use super::{ExploreMode, PartialProject2, GeneralResult, utils::{RequestBuilderUtils, ResponseUtils}, Studio2};

#[derive(Clone, Debug)]
pub struct SearchQuery<'a> {
    pub mode: Option<&'a ExploreMode>,
    pub query: Option<&'a str>,
}

impl<'a> SearchQuery<'a> {
    pub fn as_query(&self) -> Vec<(&str, &str)> {
        let mut result = Vec::new();
        if let Some(query) = self.query {
            result.push(("q", query))
        }
        if let Some(mode) = self.mode {
            result.push(("mode", mode.as_ref()))
        }
        result
    }
}

impl Api {
    pub async fn search_projects<'q>(&self, query: SearchQuery<'q>, cursor: impl Into<Cursor>) -> GeneralResult<Vec<PartialProject2>> {
        let response = self.get("search/projects/").query(&query.as_query()).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn search_studios<'q>(&self, query: SearchQuery<'q>, cursor: impl Into<Cursor>) -> GeneralResult<Vec<Studio2>> {
        let response = self.get("search/studios/").query(&query.as_query()).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }
}