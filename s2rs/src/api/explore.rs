use crate::{Api, cursor::Cursor, Language};
use super::{Project2, utils::RequestBuilderUtils, Studio2};

#[derive(Debug, Clone)]
pub struct ExploreQuery {
    pub query: Option<String>,
    pub mode: Option<ExploreMode>,
    pub language: Option<Language>,
}

impl ExploreQuery {
    pub fn as_query(&self) -> Vec<(&str, &str)> {
        let mut result = Vec::new();
        if let Some(query) = &self.query {
            result.push(("q", query.as_str()))
        }
        if let Some(mode) = &self.mode {
            result.push(("mode", mode.as_ref()))
        }
        if let Some(language) = &self.language {
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
    pub async fn explore_projects(&self, query: &ExploreQuery, cursor: impl Into<Cursor>) -> super::Result<Vec<Project2>> {
        let response = self.get("explore/projects/").query(&query.as_query()).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }

    pub async fn explore_studios(&self, query: &ExploreQuery, cursor: impl Into<Cursor>) -> super::Result<Vec<Studio2>> {
        let response = self.get("explore/studios/").query(&query.as_query()).cursor(cursor).send_success().await?;
        Ok(response.json().await?)
    }
}
