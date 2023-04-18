use crate::Api;
use super::{GeneralResult, utils::RequestBuilderUtils};
#[cfg(feature = "rss")] use super::ParsingCustomError;
#[cfg(feature = "time")] use chrono::{DateTime, Utc};

#[cfg(feature = "time")]
pub struct ForumRssTopic {
    pub title: String,
    pub id: u64,
    pub updated_at: DateTime<Utc>,
    pub posts: Vec<ForumRssTopicPost>,
}

#[cfg(feature = "rss")]
impl ForumRssTopic {
    pub fn try_from_rss(data: feed_rs::model::Feed) -> Result<Self, ParsingCustomError> {
        let mut posts = Vec::new();
        for entry in data.entries {
            posts.push(ForumRssTopicPost::try_from_rss(entry)?);
        }
        Ok(Self {
            id: data.id.split('/').rev().nth(1).ok_or(())?.parse().ok().ok_or(())?,
            title: data.title.ok_or(())?.content,
            updated_at: data.updated.ok_or(())?,
            posts
        })
    }
}

#[cfg(feature = "time")]
pub struct ForumRssTopicPost {
    pub id: u64,
    pub created_at: DateTime<Utc>,
    pub author_name: String,
    pub content: String,
}

#[cfg(feature = "rss")]
impl ForumRssTopicPost {
    pub fn try_from_rss(mut data: feed_rs::model::Entry) -> Result<Self, ParsingCustomError> {
        if data.authors.get(0).is_none() {
            Err(())?
        }
        Ok(Self {
            author_name: data.authors.swap_remove(0).name,
            content: data.content.ok_or(())?.body.ok_or(())?,
            created_at: data.published.ok_or(())?,
            id: data.id.parse().ok().ok_or(())?
        })
    }
}

impl Api {
    pub async fn get_forum_post_content(&self, id: u64) -> GeneralResult<String> {
        let response = self.get_base(&format!["discuss/post/{id}/source/"]).send_success().await?;
        Ok(response.text().await?)
    }

    #[cfg(feature = "rss")]
    pub async fn get_forum_rss_topic(&self, id: u64) -> GeneralResult<ForumRssTopic> {
        let response = self.get_base(&format!["discuss/feeds/topic/{id}/"]).send_success().await?;
        let feed = feed_rs::parser::parse(response.text().await?.as_bytes()).map_err(|_| ParsingCustomError)?;
        Ok(ForumRssTopic::try_from_rss(feed)?)
    }
}