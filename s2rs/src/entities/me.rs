use std::sync::Arc;
use crate::{Api, api::{UserInfo, FeaturedLabel, self}};
use super::{User, FrontPage};
use derivative::Derivative;
use s2rs_derive::deref;
#[cfg(feature = "stream")] use super::{stream::GeneralStream, MeProjectsLovedByFollowing, MeProjectsSharedByFollowing, MeViewedProjects};
#[cfg(feature = "stream")] use crate::Cursor;

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
        self.api.set_profile_info(info).await?;
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
        Ok(FrontPage::new(self.api.front_page().await?, self.api.clone()))
    }

    pub async fn news(&self) -> api::Result<Vec<api::News>> {
        self.api.news().await
    }

    pub async fn global_projects_count(&self) -> Result<u64, api::GetProjectsCountError> {
        self.api.projects_count().await
    }

    #[cfg(feature = "file")]
    pub async fn set_icon<B>(&self, buffer: B) -> api::Result<()>
    where B: Into<std::borrow::Cow<'static, [u8]>> {
        self.api.set_user_icon(buffer).await
    }

    #[cfg(feature = "cookie")]
    pub async fn login(&self, name: &str, password: &str) -> Result<super::Login, api::LoginError> {
        Ok(super::Login::new(self.api.login(name, password).await?, self.api.clone()))
    }

    #[cfg(feature = "stream")]
    pub fn projects_loved_by_following(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<MeProjectsLovedByFollowing> {
        GeneralStream::with_this(MeProjectsLovedByFollowing, cursor.into(), self.clone(), self.api.clone())
    }

    #[cfg(feature = "stream")]
    pub fn projects_shared_by_following(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<MeProjectsSharedByFollowing> {
        GeneralStream::with_this(MeProjectsSharedByFollowing, cursor.into(), self.clone(), self.api.clone())
    }

    #[cfg(feature = "stream")]
    pub fn viewed_projects(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<MeViewedProjects> {
        GeneralStream::with_this(MeViewedProjects, cursor.into(), self.clone(), self.api.clone())
    }
}