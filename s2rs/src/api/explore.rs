use crate::{Api, cursor::Cursor};
use super::{Language, PartialProject2, GeneralResult, utils::{RequestBuilderUtils, ResponseUtils}, Studio2};

#[derive(Debug)]
pub struct ExploreQuery<'a> {
    pub query: Option<&'a str>,
    pub mode: Option<&'a ExploreMode>,
    pub language: Option<&'a Language>,
}

impl<'a> ExploreQuery<'a> {
    pub fn as_query(&self) -> Vec<(&str, &str)> {
        let mut result = Vec::new();
        if let Some(query) = self.query {
            result.push(("q", query))
        }
        if let Some(mode) = self.mode {
            result.push(("mode", mode.as_ref()))
        }
        if let Some(language) = self.language {
            result.push(("language", language.as_code()))
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExploreMode {
    Popular,
    Trending,
}

impl AsRef<str> for ExploreMode {
    fn as_ref(&self) -> &str {
        match self {
            Self::Popular => "popular",
            Self::Trending => "trending"
        }
    }
}

impl Api {
    pub async fn explore_projects<'q>(&self, query: ExploreQuery<'q>, cursor: impl Into<Cursor>) -> GeneralResult<Vec<PartialProject2>> {
        let response = self.get("explore/projects/").query(&query.as_query()).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn explore_studios<'q>(&self, query: ExploreQuery<'q>, cursor: impl Into<Cursor>) -> GeneralResult<Vec<Studio2>> {
        let response = self.get("explore/studios/").query(&query.as_query()).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }
}
