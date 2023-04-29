//! General user related entities
use std::sync::Arc;
use derivative::Derivative;
use s2rs_derive::deref;
use crate::{api::{Api, self, UserNameCheck, SendComment}, utils::into_arc::IntoArc};
use super::{UserFeatured, UserProject, UserComment};
#[cfg(feature = "stream")] use super::{stream::GeneralStream, user_stream::*};
#[cfg(feature = "stream")] use crate::cursor::Cursor;

// region: UserMeta
/// General user metadata
/// - Mapping for <https://api.scratch.mit.edu/users/USERNAME/>
/// # Examples
/// ```
/// # tokio_test::block_on(async {
/// # use s2rs::session::Session;
/// # let session = Session::new("YourUsername");
/// let user = session.user("griffpatch");
/// let meta_data = user.meta().await.unwrap();
/// dbg!(&meta_data.profile.bio);
/// # })
/// ```
#[derive(Debug)]
#[deref(this)]
pub struct UserMeta {
    pub this: Arc<UserWithId>,
    pub scratch_team: bool,
    pub history: api::UserHistory,
    pub profile: api::UserProfile,
}

impl UserMeta {
    pub fn with_this(data: api::User, this: Arc<UserWithId>) -> Arc<Self> {
        Arc::new(Self {
            this,
            history: data.history,
            scratch_team: data.scratch_team,
            profile: data.profile,
        })
    }

    pub fn with_this_this(data: api::User, this: Arc<User>, api: Arc<Api>) -> Arc<Self> {
        let id = data.id;
        Self::with_this(data, UserWithId::with_this(id, this, api))
    }

    pub fn new(data: api::User, api: Arc<Api>) -> Arc<Self> {
        let name = data.name.clone();
        let id = data.id;
        Self::with_this(data, UserWithId::new(id, name, api))
    }

    pub fn vec_new(data: Vec<api::User>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}
// endregion: UserMeta

// region: UserWithId
/// Extends [`User`] with user ID
/// # Examples
/// ```
/// # tokio_test::block_on(async {
/// # use s2rs::session::Session;
/// # let session = Session::new("YourUsername");
/// let user = session.user("griffpatch");
/// let meta_data = user.meta().await.unwrap();
/// dbg!(&meta_data.profile.bio);
/// # })
/// ```
#[derive(Derivative)]
#[derivative(Debug)]
#[deref(this)]
pub struct UserWithId {
    pub this: Arc<User>,
    pub id: u64,
    #[derivative(Debug = "ignore")]
    api: Arc<Api>
}

impl UserWithId {
    pub fn with_this(id: u64, this: Arc<User>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            id,
            this,
            api
        })
    }
    
    pub fn new(id: u64, name: String, api: Arc<Api>) -> Arc<Self> {
        Self::with_this(id, User::new(name, api.clone()), api)
    }

    pub async fn icon(&self, width: u16, height: u16) -> api::Result<Vec<u8>> {
        self.api.user_icon(self.id, width, height).await
    }
}
// endregion: UserWithId

// region: User
/// This struct only identifies user (by username) without storing any other data
/// # Examples
/// ```
/// # tokio_test::block_on(async {
/// # use s2rs::session::Session;
/// # let session = Session::new("YourUsername");
/// let user = session.user("griffpatch");
/// user.follow().await.unwrap();
/// // ...
/// # })
/// ```
#[derive(Derivative)]
#[derivative(Debug, PartialEq, Eq)]
pub struct User {
    #[derivative(Debug="ignore", PartialEq="ignore")]
    api: Arc<Api>,
    pub name: Arc<String>,
}

impl User {
    pub fn new(name: impl IntoArc<String>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            api,
            name: name.into_arc()
        })
    }
}

impl User {
    /// User's meta-data
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let user = session.user("griffpatch");
    /// let data = user.meta().await.unwrap();
    /// dbg![ &data.history.joined ];
    /// # })
    /// ```
    pub async fn meta(self: &Arc<Self>) -> Result<Arc<UserMeta>, api::Error> {
        Ok(UserMeta::with_this_this(self.api.user_meta(&self.name).await?, self.clone(), self.api.clone()))
    }

    pub fn comment(self: &Arc<Self>, id: u64) -> Arc<UserComment> {
        UserComment::with_profile(id, self.clone(), self.api.clone())
    }

    /// User's messages count
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let user = session.user("griffpatch");
    /// let msg_count = user.message_count().await.unwrap();
    /// dbg![ msg_count ];
    /// # })
    /// ```
    pub async fn message_count(&self) -> Result<u64, api::GetUserMessagesCountError> {
        self.api.user_messages_count(&self.name).await
    }

    /// User's featured project data
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let user = session.user("griffpatch");
    /// let featured = user.featured().await.unwrap();
    /// dbg![ &featured.project.modified_at ];
    /// # })
    /// ```
    pub async fn featured(self: &Arc<Self>) -> Result<UserFeatured, api::Error> {
        Ok(UserFeatured::with_profile_this(self.api.user_featured(&self.name).await?, self.clone(), self.api.clone()))
    }

    /// User's projects
    /// - `Requires crate feature: 'stream'`
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let user = session.user("griffpatch");
    /// for project in user.projects((0, 50)).collect().await.unwrap() {
    ///     dbg![ &project ];
    /// }
    /// # })
    /// ```
    #[cfg(feature = "stream")]
    pub fn projects(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<UserProjects> {
        GeneralStream::with_this(UserProjects, cursor.into(), self.clone(), self.api.clone())
    }

    /// User's favorites
    /// - `Requires crate feature: 'stream'`
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let user = session.user("griffpatch");
    /// for project in user.favorites((0, 50)).collect().await.unwrap() {
    ///     dbg![ &project ];
    /// }
    /// # })
    /// ```
    #[cfg(feature = "stream")]
    pub fn favorites(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<UserFavorites> {
        GeneralStream::with_this(UserFavorites, cursor.into(), self.clone(), self.api.clone())
    }

    /// User's curating studios
    /// - `Requires crate feature: 'stream'`
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let user = session.user("griffpatch");
    /// for studio in user.curating_studios((0, 50)).collect().await.unwrap() {
    ///     dbg![ &studio ];
    /// }
    /// # })
    /// ```
    #[cfg(feature = "stream")]
    pub fn curating_studios(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<UserCuratingStudios> {
        GeneralStream::with_this(UserCuratingStudios, cursor.into(), self.clone(), self.api.clone())
    }

    /// User's followers
    /// - `Requires crate feature: 'stream'`
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let user = session.user("griffpatch");
    /// for user in user.followers((0, 50)).collect().await.unwrap() {
    ///     dbg![ &user ];
    /// }
    /// # })
    /// ```
    #[cfg(feature = "stream")]
    pub fn followers(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<UserFollowers> {
        GeneralStream::with_this(UserFollowers, cursor.into(), self.clone(), self.api.clone())
    }

    /// User's followers
    /// - `Requires crate feature: 'stream'`
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let user = session.user("griffpatch");
    /// for user in user.followers((0, 50)).collect().await.unwrap() {
    ///     dbg![ &user ];
    /// }
    /// # })
    /// ```
    #[cfg(feature = "stream")]
    pub fn following(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<UserFollowing> {
        GeneralStream::with_this(UserFollowing, cursor.into(), self.clone(), self.api.clone())
    }

    #[cfg(feature = "stream")]
    pub async fn messages(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<UserMessages> {
        GeneralStream::with_this(UserMessages, cursor.into(), self.clone(), self.api.clone())
    }

    #[cfg(feature = "stream")]
    pub async fn following_activity(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<UserFollowingActivity> {
        GeneralStream::with_this(UserFollowingActivity, cursor.into(), self.clone(), self.api.clone())
    }

    #[cfg(feature = "stream")]
    #[cfg(feature = "html")]
    pub fn comments(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<UserComments> {
        GeneralStream::with_this(UserComments, cursor.into(), self.clone(), self.api.clone())
    }

    pub async fn follow(&self) -> Result<(), api::Error> {
        self.api.follow_user(&self.name).await?;
        Ok(())
    }

    pub async fn unfollow(&self) -> Result<(), api::Error> {
        self.api.unfollow_user(&self.name).await?;
        Ok(())
    }

    pub async fn send_comment(&self, data: impl Into<SendComment>) -> Result<(), api::Error> {
        self.api.send_user_comment(&self.name, &data.into()).await?;
        Ok(())
    }

    pub async fn check(&self) -> api::Result<UserNameCheck> {
        self.api.check_user_name(&self.name).await
    }

    pub fn project(self: &Arc<Self>, id: u64) -> Arc<UserProject> {
        UserProject::with_author(id, self.clone(), self.api.clone())
    }
}
// endregion: User
