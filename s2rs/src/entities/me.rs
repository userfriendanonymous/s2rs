use std::{sync::Arc, borrow::Cow};
use crate::{Api, api::{UserInfo, FeaturedLabel, self}};
use super::{User, FrontPage};
use derivative::Derivative;
use s2rs_derive::deref;

#[deref(this)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Me {
    pub this: Arc<User>,
    #[derivative(Debug = "ignore")]
    pub api: Arc<Api>,
}

impl Me {
    pub fn with_this(this: Arc<User>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            api,
            this
        })
    }
}

impl Me {
    pub async fn set_info(&self, info: &UserInfo) -> api::Result<()> {
        self.api.set_user_info(info).await?;
        Ok(())
    }

    pub async fn set_bio(&self, content: impl Into<String>) -> api::Result<()> {
        self.set_info(&UserInfo { bio: Some(content.into()), ..Default::default() }).await
    }

    pub async fn set_status(&self, content: impl Into<String>) -> api::Result<()> {
        self.set_info(&UserInfo { status: Some(content.into()), ..Default::default() }).await
    }

    pub async fn set_featured(&self, id: u64, label: Option<FeaturedLabel>) -> api::Result<()> {
        self.set_info(&UserInfo { featured_id: Some(id), featured_label: label, ..Default::default() }).await
    }

    pub async fn front_page(&self) -> api::Result<FrontPage> {
        Ok(FrontPage::new(self.api.get_front_page().await?, self.api.clone()))
    }

    pub async fn news(&self) -> api::Result<Vec<api::News>> {
        self.api.get_news().await
    }

    pub async fn global_projects_count(&self) -> Result<u64, api::GetProjectsCountError> {
        self.api.get_projects_count().await
    }

    pub async fn set_icon<B, FN>(&self, buffer: B, file_name: FN) -> api::Result<()>
    where B: Into<Cow<'static, [u8]>>, FN: Into<Cow<'static, str>> {
        self.api.set_user_icon(buffer, file_name).await
    }
}