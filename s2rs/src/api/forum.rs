use crate::Api;
use super::utils::RequestBuilderUtils;
#[cfg(feature = "rss")] use super::ParsingCustomError;
#[cfg(feature = "time")] use chrono::{DateTime, Utc};

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

#[cfg(feature = "time")]
pub struct ForumTopicRss {
    pub title: String,
    pub id: u64,
    pub updated_at: DateTime<Utc>,
    pub posts: Vec<ForumTopicRssPost>,
}

#[cfg(feature = "rss")]
impl ForumTopicRss {
    pub fn try_from_rss(data: feed_rs::model::Feed) -> Result<Self, ParsingCustomError> {
        let mut posts = Vec::new();
        dbg!();
        for entry in data.entries {
            posts.push(ForumTopicRssPost::try_from_rss(entry)?);
        }
        dbg!();
        data.id.split('/').rev().nth(1).ok_or(())?.parse::<u64>().ok().ok_or(())?;
        dbg!();
        Ok(Self {
            id: data.id.split('/').rev().nth(1).ok_or(())?.parse().ok().ok_or(())?,
            title: data.title.ok_or(())?.content,
            updated_at: data.updated.ok_or(())?,
            posts
        })
    }
}

#[cfg(feature = "time")]
pub struct ForumTopicRssPost {
    pub id: u64,
    pub created_at: DateTime<Utc>,
    pub author_name: String,
    pub content: String,
}

#[cfg(feature = "rss")]
impl ForumTopicRssPost {
    pub fn try_from_rss(mut data: feed_rs::model::Entry) -> Result<Self, ParsingCustomError> {
        dbg!();
        if data.authors.get(0).is_none() {
            Err(())?
        }
        dbg!(&data.authors);
        Ok(Self {
            author_name: data.authors.swap_remove(0).name,
            content: data.summary.ok_or(())?.content,
            created_at: data.published.ok_or(())?,
            id: data.id.parse().ok().ok_or(())?
        })
    }
}

impl Api {
    pub async fn get_forum_post_content(&self, id: u64) -> super::Result<String> {
        let response = self.get_base(&format!["discuss/post/{id}/source/"]).send_success().await?;
        Ok(response.text().await?)
    }

    #[cfg(feature = "rss")]
    pub async fn get_forum_topic_rss(&self, id: u64) -> super::Result<ForumTopicRss> {
        let response = self.get_base(&format!["discuss/feeds/topic/{id}/"]).send_success().await?;
        let feed = feed_rs::parser::parse(response.text().await?.as_bytes()).map_err(|_| ParsingCustomError)?;
        Ok(ForumTopicRss::try_from_rss(feed)?)
    }
}