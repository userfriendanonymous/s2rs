use std::str::FromStr;
use crate::Api;
use super::utils::RequestBuilderUtils;
use crate::json;
#[cfg(feature = "time")] use chrono::{DateTime, Utc};
use s2rs_derive::Forwarder;

// region: ForumCategory
pub enum ForumCategory {
    Announcements,
    NewScratchers,
    HelpWithScripts,
    ShowAndTell,
    ProjectIdeas,
    Collaboration,
    Requests,
    ProjectSaveLevelCodes,
    QuestionsAboutScratch,
    Suggestions,
    BugsAndGlitches,
    AdvancedTopics,
    ConnectingToThePhysicalWorld,
    DevelopingScratchExtensions,
    OpenSourceProjects,
    ThingsIAmMakingAndCreating,
    ThingsIAmReadingAndPlaying,
}

impl AsRef<str> for ForumCategory {
    fn as_ref(&self) -> &str {
        match self {
            Self::AdvancedTopics => "AdvancedTopics",
            Self::Announcements => "Announcements",
            Self::BugsAndGlitches => "BugsAndGlitches",
            _ => todo!()
        }
    }
}
// endregion: ForumCategory

// region: ForumTopicRss
#[cfg(feature = "time")]
pub struct ForumTopicRss {
    pub title: String,
    pub id: u64,
    pub updated_at: DateTime<Utc>,
    pub posts: Vec<ForumTopicRssPost>,
}

#[derive(Forwarder, Clone, Debug)]
pub enum ForumTopicRssParseIdError {
    NoContent,
    Parsing(<u64 as FromStr>::Err)
}

#[derive(Forwarder, Clone, Debug)]
pub enum ForumTopicRssParseError {
    #[forward] Expected(json::ExpectedError),
    #[forward(<u64 as FromStr>::Err)]
    Id(ForumTopicRssParseIdError),
    UpdatedAtNotFound,
    TitleNotFound,
    #[forward] Post(ForumTopicRssPostParseError)
}

#[cfg(feature = "rss")]
impl ForumTopicRss {
    pub fn try_from_rss(data: feed_rs::model::Feed) -> Result<Self, ForumTopicRssParseError> {
        type Error = ForumTopicRssParseError;
        type IdError = ForumTopicRssParseIdError;

        let mut posts = Vec::new();
        for entry in data.entries {
            posts.push(ForumTopicRssPost::try_from_rss(entry)?);
        }
        Ok(Self {
            id: data.id.split('/').rev().nth(1).ok_or(IdError::NoContent)?.parse().map_err(IdError::Parsing)?,
            title: data.title.ok_or(Error::TitleNotFound)?.content,
            updated_at: data.updated.ok_or(Error::UpdatedAtNotFound)?,
            posts
        })
    }
}
// endregion: ForumTopicRss

// region: ForumTopicRssPost
#[cfg(feature = "time")]
pub struct ForumTopicRssPost {
    pub id: u64,
    pub created_at: DateTime<Utc>,
    pub author_name: String,
    pub content: String,
}

#[derive(Forwarder, Debug, Clone)]
pub enum ForumTopicRssPostParseError {
    #[forward] Expected(json::ExpectedError),
    Id(<u64 as FromStr>::Err),
    AuthorNotFound,
    ContentNotFound,
    CreatedAtNotFound,
}

#[cfg(feature = "rss")]
impl ForumTopicRssPost {
    pub fn try_from_rss(mut data: feed_rs::model::Entry) -> Result<Self, ForumTopicRssPostParseError> {
        if data.authors.get(0).is_none() {
            Err(ForumTopicRssPostParseError::AuthorNotFound)?
        }
        Ok(Self {
            author_name: data.authors.swap_remove(0).name,
            content: data.summary.ok_or(ForumTopicRssPostParseError::ContentNotFound)?.content,
            created_at: data.published.ok_or(ForumTopicRssPostParseError::CreatedAtNotFound)?,
            id: data.id.parse().map_err(ForumTopicRssPostParseError::Id)?
        })
    }
}
// endregion: ForumTopicRssPost

#[derive(Forwarder)]
pub enum GetForumTopicRssError {
    #[forward] Parsing(ForumTopicRssParseError),
    #[forward] Rss(feed_rs::parser::ParseFeedError),
    #[forward(reqwest::Error)]
    This(super::Error)
}

impl Api {
    pub async fn get_forum_post_content(&self, id: u64) -> super::Result<String> {
        let response = self.get_base(&format!["discuss/post/{id}/source/"]).send_success().await?;
        Ok(response.text().await?)
    }

    #[cfg(feature = "rss")]
    pub async fn get_forum_topic_rss(&self, id: u64) -> Result<ForumTopicRss, GetForumTopicRssError> {
        let response = self.get_base(&format!["discuss/feeds/topic/{id}/"]).send_success().await?;
        let feed = feed_rs::parser::parse(response.text().await?.as_bytes())?;
        Ok(ForumTopicRss::try_from_rss(feed)?)
    }
}