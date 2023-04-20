use derivative::Derivative;
use s2rs_derive::deref;
use crate::api::{self, CommentContent};
use super::{Api, UserWithId, User};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmojiVariant {
    Meow,
    Taco,
    Sushi,
    Apple,
    Broccoli,
    Pizza,
    Gobo,
    Waffle,
    Candycorn,
    Confetti,
    Map,
    Suitcase,
    Camera,
    Compass,
    Binoculars,
    Ksllxmfcai,
    Pride,
    Blm,
    Cat,
    AwwCat,
    CoolCat,
    TongueOutCat,
    WinkCat,
    LolCat,
    UpsideDownCat,
    HuhCat,
    LoveItCat,
    FavItCat,
    RainbowCat,
    PizzaCat,
}

// region: UserComment
#[derive(Derivative)]
#[derivative(Debug, PartialEq, Eq)]
pub struct UserComment {
    pub id: u64,
    #[derivative(PartialEq = "ignore")]
    pub profile: Arc<User>,
    #[derivative(Debug = "ignore", PartialEq = "ignore")]
    pub api: Arc<Api>,
}

impl UserComment {
    pub fn with_profile(id: u64, profile: Arc<User>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            id,
            profile,
            api
        })
    }

    pub fn new(id: u64, user_name: String, api: Arc<Api>) -> Arc<Self> {
        Self::with_profile(id, User::new(user_name, api.clone()), api)
    }

    pub async fn report(&self) -> api::Result<()> {
        self.api.report_user_comment(self.id).await
    }
}
// endregion: UserComment

// region: UserCommentMeta
#[derive(Debug)]
#[deref(this)]
pub struct UserCommentMeta {
    pub this: Arc<UserComment>,
    pub author: Arc<UserWithId>,
    pub avatar_url: String,
    pub created_at: String,
    pub content: CommentContent,
    pub replies: Vec<Arc<UserReplyMeta>>,
}

impl UserCommentMeta {
    pub fn with_this(data: api::UserComment, this: Arc<UserComment>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            replies: UserReplyMeta::vec_with_profile(data.replies, this.profile.clone(), api.clone()),
            this,
            author: UserWithId::new(data.author_id, data.author_name, api),
            avatar_url: data.avatar_url,
            content: data.content,
            created_at: data.created_at,
        })
    }

    pub fn with_profile(data: api::UserComment, profile: Arc<User>, api: Arc<Api>) -> Arc<Self> {
        let id = data.id;
        Self::with_this(data, UserComment::with_profile(id, profile, api.clone()), api)
    }

    pub fn vec_with_profile(data: Vec<api::UserComment>, profile: Arc<User>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::with_profile(data, profile.clone(), api.clone())).collect()
    }
}
// endregion: UserCommentMeta

// region: UserReply
#[derive(Debug, PartialEq, Eq)]
pub struct UserReply {
    pub id: u64,
    pub profile: Arc<User>,
}

impl UserReply {
    pub fn with_profile(id: u64, profile: Arc<User>) -> Arc<Self> {
        Arc::new(Self {
            id,
            profile
        })
    }
}
// endregion: UserReply

// region: UserReplyMeta
#[derive(Debug)]
#[deref(this)]
pub struct UserReplyMeta {
    pub this: Arc<UserReply>,
    pub author: Arc<UserWithId>,
    pub avatar_url: String,
    pub content: CommentContent,
    pub created_at: String,
}

impl UserReplyMeta {
    pub fn with_this(data: api::UserReply, this: Arc<UserReply>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            this,
            author: UserWithId::new(data.author_id, data.author_name, api),
            avatar_url: data.avatar_url,
            content: data.content,
            created_at: data.created_at,
        })
    }

    pub fn with_profile(data: api::UserReply, profile: Arc<User>, api: Arc<Api>) -> Arc<Self> {
        let id = data.id;
        Self::with_this(data, UserReply::with_profile(id, profile), api)
    }

    pub fn vec_with_profile(data: Vec<api::UserReply>, profile: Arc<User>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::with_profile(data, profile.clone(), api.clone())).collect()
    }
}
// endregion: UserReplyMeta

