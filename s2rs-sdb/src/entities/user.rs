use std::sync::Arc;
use derivative::Derivative;
use s2rs_derive::deref;
use crate::api::{Api, self};

// region: UserMeta
#[derive(Debug)]
pub struct UserMeta {
    pub this: Arc<UserWithId>,
    pub sys_id: u64,
    pub joined: String,
    pub country: String,
    pub bio: String,
    pub work: String,
    pub status: api::UserStatus,
    pub school: Option<String>,
    pub stats: api::UserStats
}

impl UserMeta {
    pub fn with_this_this(data: api::UserMeta, this: Arc<User>) -> Self {
        Self {
            work: data.work,
            bio: data.bio,
            country: data.country,
            joined: data.joined,
            school: data.school,
            stats: data.stats,
            status: data.status,
            sys_id: data.sys_id,
            this: UserWithId::with_this(data.id, this)
        }
    }
}
// endregion: UserMeta

#[deref(this)]
#[derive(Debug)]
pub struct UserWithId {
    pub this: Arc<User>,
    pub id: u64
}

impl UserWithId {
    pub fn with_this(id: u64, this: Arc<User>) -> Arc<Self> {
        Arc::new(Self {
            id,
            this
        })
    }
    
    pub fn new(id: u64, name: String, api: Arc<Api>) -> Arc<Self> {
        Self::with_this(id, User::new(name, api))
    }
}

#[deref(this)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct User {
    #[derivative(Debug = "ignore")]
    pub api: Arc<Api>,
    pub this: Arc<s2rs::User>
}

impl User {
    pub fn with_this(this: Arc<s2rs::User>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            api,
            this
        }) 
    }

    pub fn new(name: impl Into<Arc<String>>, api: Arc<Api>) -> Arc<Self> {
        Self::with_this(s2rs::User::new(name, api.this.clone()), api)
    }

    pub async fn sdb_meta(self: &Arc<Self>) -> Result<UserMeta, api::GetUserError> {
        Ok(UserMeta::with_this_this(self.api.get_user_sdb(&self.name).await?, self.clone()))
    }
    
    // pub async fn l(&self) -> Result<UserMeta, api::GetUserError> {
    //     Ok(UserMeta::new(self.api.get_user_sdb(&self.name).await?, self.api.clone()))
    // }
}