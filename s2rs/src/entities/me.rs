use std::sync::Arc;
use super::User;
use crate::{Api, api::{UserInfo, self, FeaturedLabel}};
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
    pub async fn set_info(&self, info: &UserInfo) -> Result<(), api::GeneralError> {
        self.api.set_user_info(info).await?;
        Ok(())
    }

    pub async fn set_bio(&self, content: impl Into<String>) -> Result<(), api::GeneralError> {
        self.set_info(&UserInfo { bio: Some(content.into()), ..Default::default() }).await
    }

    pub async fn set_status(&self, content: impl Into<String>) -> Result<(), api::GeneralError> {
        self.set_info(&UserInfo { status: Some(content.into()), ..Default::default() }).await
    }

    pub async fn set_featured(&self, id: u64, label: Option<FeaturedLabel>) -> Result<(), api::GeneralError> {
        self.set_info(&UserInfo { featured_id: Some(id), featured_label: label, ..Default::default() }).await
    }
}