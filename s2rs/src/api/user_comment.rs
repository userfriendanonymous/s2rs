use super::Api;
#[cfg(feature = "html")] use html_parser::{Dom, Element};
use s2rs_derive::Forwarder;
use serde_json::json;
#[cfg(feature = "html")] use crate::cursor::Cursor;
use super::utils::RequestBuilderUtils;
// const NUMBERS: &str = "1234567890";

#[derive(Debug)]
pub enum CommentContentFragment {
    Link {
        to: String,
        content: String,
    },
    Text(String),
    Emoji(String)
}

#[derive(Debug)]
pub struct CommentContent(pub Vec<CommentContentFragment>);

impl CommentContent {
    #[cfg(feature = "html")]
    fn try_from_html(element: &Element) -> Option<Self> {
        use crate::html::ElementUtils;

        fn get_link_data(element: &Element) -> Option<(String, String)> {
            if let Some(text) = element.get_text() {
                Some((text.to_owned(), element.get_attribute("href")?))
            } else {
                get_link_data(element.children.first()?.element()?)
            }
        }

        let mut fragments = Vec::new();

        for node in &element.children {
            if let Some(text) = node.text() {
                fragments.push(CommentContentFragment::Text(text.to_owned()));

            } else if let Some(element) = node.element() {
                match element.name.as_str() {
                    "a" => {
                        let (content, to) = get_link_data(element)?;
                            fragments.push(CommentContentFragment::Link { content, to })
                    }
                    "img" => fragments.push(CommentContentFragment::Emoji(element.get_attribute("src")?)),
                    _ => None?
                }
            }
        }
        Some(Self(fragments))
    }
}

// region: UserReply
#[derive(Debug)]
pub struct UserReply {
    pub id: u64,
    pub profile_name: String,
    pub author_name: String,
    pub author_id: u64,
    pub avatar_url: String,
    pub content: CommentContent,
    pub created_at: String,
}

impl UserReply {
    #[cfg(feature = "html")]
    fn try_from_html(element: &Element, profile_name: String) -> Option<Self> {
        use crate::html::ElementUtils;

        let info = element.child_by_class("comment")?;
        let inner_info = info.child_by_class("info")?;
        let extra_info = inner_info.children.get(2)?.element()?;

        Some(Self {
            avatar_url: info.child_by_id("comment-user")?.child_by_class("avatar")?.get_attribute("src")?,
            author_name: inner_info.child_by_class("name")?.child_by_name("a")?.get_text()?.to_owned(),
            content: CommentContent::try_from_html(inner_info.child_by_class("content")?)?,
            created_at: extra_info.child_by_class("time")?.get_attribute("title")?,
            author_id: extra_info.child_by_name("a")?.get_attribute("data-commentee-id")?.parse().ok()?,
            id: info.get_attribute("data-comment-id")?.parse().ok()?,
            profile_name
        })
    }
}
// endregion: UserReply

// region: UserComment
#[derive(Debug)]
pub struct UserComment {
    pub id: u64,
    pub profile_name: String,
    pub author_name: String,
    pub author_id: u64,
    pub avatar_url: String,
    pub content: CommentContent,
    pub created_at: String,
    pub replies: Vec<UserReply>,
    // pub reply_count: u32,
}

impl UserComment {
    #[cfg(feature = "html")]
    fn try_from_html(element: &Element, profile_name: String) -> Option<Self> {
        use crate::html::ElementUtils;

        let info = element.child_by_class("comment")?;
        let inner_info = info.child_by_class("info")?;
        let extra_info = inner_info.children.get(2)?.element()?;
        let mut replies = Vec::new();
        for node in &element.child_by_class("replies")?.children {
            if let Some(element) = node.element() {
                if element.classes.contains(&"reply".to_string()) {
                    replies.push(UserReply::try_from_html(element, profile_name.clone())?);
                }
            }
        }

        Some(Self {
            avatar_url: info.child_by_id("comment-user")?.child_by_class("avatar")?.get_attribute("src")?,
            author_name: inner_info.child_by_class("name")?.child_by_name("a")?.get_text()?.to_owned(),
            content: CommentContent::try_from_html(inner_info.child_by_class("content")?)?,
            created_at: extra_info.child_by_class("time")?.get_attribute("title")?,
            author_id: extra_info.child_by_name("a")?.get_attribute("data-commentee-id")?.parse().ok()?,
            id: info.get_attribute("data-comment-id")?.parse().ok()?,
            profile_name,
            replies
            // reply_count: match element.child_by_class("more-replies") {
            //     Some(element) => {
            //         dbg!(element.child_by_class("pulldown"));
            //         let text = element.child_by_class("pulldown")?.get_text()?;
            //         let mut num = "".to_string();
            //         let mut iter = text.chars().skip(text.find_tail("See all ")?);
            //         loop {
            //             let character = iter.next()?;
            //             if NUMBERS.contains(character) {
            //                 num.push(character.to_owned());
            //             } else {
            //                 break;
            //             }

            //         }
            //         num.parse().ok()
            //     },

            //     None => replies.len().try_into().ok()
            // }?,
        })
    }
}
// endregion: UserComment

#[derive(Debug, Forwarder)]
pub enum GetUserCommentsError {
    Parsing,
    #[forward(reqwest::Error)]
    This(super::Error)
}

impl Api {
    #[cfg(feature = "html")]
    pub async fn user_comments(&self, name: &str, cursor: impl Into<Cursor>) -> Result<Vec<UserComment>, GetUserCommentsError> {
        let response = self.get_site_api(&format!["comments/user/{name}/"]).cursor(cursor).send_success().await?;
        let data = response.text().await?;

        let dom = Dom::parse(&data).ok().ok_or(GetUserCommentsError::Parsing)?;

        let mut result = Vec::new();

        for node in &dom.children {
            if let Some(element) = node.element() {
                if element.classes.contains(&"top-level-reply".to_string()) {
                    result.push(UserComment::try_from_html(element, name.to_owned()).ok_or(GetUserCommentsError::Parsing)?);
                }
            }
        }
        
        Ok(result)
    }
}

impl Api {
    pub async fn report_user_comment(&self, id: u64) -> super::Result<()> {
        self.post_site_api(&format!["comments/user/{}/rep/", &self.name]).json(&json!({
            "id": id.to_string()
        })).send_success().await?;
        Ok(())
    }
}