//! General studio related entities

use std::sync::Arc;
use derivative::Derivative;
use s2rs_derive::deref;
use crate::api::{self, Api, StudioInfo};
#[cfg(feature = "stream")] use crate::cursor::Cursor;
use super::Project;
#[cfg(feature = "stream")] use super::{studio_stream::*, stream::GeneralStream};

// region: StudioWithTitle
/// Extends [`Studio`] with it's title
/// # Examples
/// ```
/// # tokio_test::block_on(async {
/// # use s2rs::session::Session;
/// # let session = Session::new("YourUsername");
/// let studio = session.studio(32774157);
/// let studio_with_title = studio.meta().await.unwrap().this; // `this` field stores StudioWithTitle
/// dbg![meta.title, meta.id];
/// # })
/// ```
#[derive(Debug, PartialEq, Eq)]
#[deref(this)]
pub struct StudioWithTitle {
    pub title: String,
    pub this: Arc<Studio>,
}

impl StudioWithTitle {
    pub fn with_this(title: String, this: Arc<Studio>) -> Arc<Self> {
        Arc::new(Self {
            this,
            title
        })
    }

    pub fn new(title: String, id: u64, api: Arc<Api>) -> Arc<Self> {
        Self::with_this(title, Studio::new(id, api))
    }
}
// endregion: StudioWithTitle

// region: StudioMeta
/// Studio metadata
/// - Mapping for <https://api.scratch.mit.edu/studios/STUDIO-ID>
/// # Examples
/// ```
/// # tokio_test::block_on(async {
/// # use s2rs::session::Session;
/// # let session = Session::new("YourUsername");
/// let studio = session.studio(32774157);
/// let meta = studio.meta().await.unwrap();
/// dbg![
///     &meta.history.modified,
///     &meta.stats.projects,
///     &meta.description
///     // ...
/// ];
/// # })
/// ```
#[derive(Debug)]
#[deref(this)]
pub struct StudioMeta {
    pub this: Arc<StudioWithTitle>,
    pub host: u64,
    pub description: String,
    pub visibility: String,
    pub public: bool,
    pub open_to_all: bool,
    pub comments_allowed: bool,
    pub image: String,
    pub history: api::StudioHistory,
    pub stats: api::StudioStats
}

impl StudioMeta {
    pub fn with_this(data: api::Studio, this: Arc<StudioWithTitle>) -> Arc<Self> {
        Arc::new(Self {
            this,
            comments_allowed: data.comments_allowed,
            description: data.description,
            history: data.history,
            host: data.host,
            image: data.image,
            open_to_all: data.open_to_all,
            public: data.public,
            stats: data.stats,
            visibility: data.visibility,
        })
    }

    pub fn with_this_this(data: api::Studio, this: Arc<Studio>) -> Arc<Self> {
        let title = data.title.clone();
        Self::with_this(data, StudioWithTitle::with_this(title, this))
    }

    pub fn new(data: api::Studio, api: Arc<Api>) -> Arc<Self> {
        let title = data.title.clone();
        let id = data.id;
        Self::with_this(data, StudioWithTitle::new(title, id, api))
    }

    pub fn vec_new(data: Vec<api::Studio>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}
// endregion: StudioMeta

// region: AddStudioProject
#[derive(Debug)]
pub struct AddStudioProject {
    pub this: Arc<Studio>,
    pub created_at: String,
    pub project: Project,
}
// endregion: AddStudioProject

// region: Studio
/// Studio identifier
/// # Examples
/// ```
/// # tokio_test::block_on(async {
/// # use s2rs::session::Session;
/// # let session = Session::new("YourUsername");
/// let studio = session.studio(32774157);
/// studio.follow().await.unwrap();
/// // ...
/// # })
/// ```
/// ```
/// # tokio_test::block_on(async {
/// # use s2rs::session::Session;
/// # let session = Session::new("YourUsername");
/// let studio = session.studio(32774157);
/// dbg![studio.meta().await.unwrap()]; // Gets studio metadata
/// studio.follow().await.unwrap();
/// studio.set_title("This is a cool studio!").await.unwrap();
/// studio.send_comment("epic comment").await.unwrap();
/// for project in studio.projects((0, 13)) { // 0 - 13 projects in studio
///     project.love().await.unwrap();
/// }
/// // ...
/// # })
/// ```
#[derive(Derivative)]
#[derivative(Debug, PartialEq, Eq)]
pub struct Studio {
    #[derivative(Debug="ignore", PartialEq="ignore")]
    api: Arc<Api>,
    pub id: u64,
}

impl Studio {
    pub fn new(id: u64, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            api,
            id
        })
    }
}

impl Studio {
    /// Get studio metadata
    /// - Results in [`StudioMeta`]
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// // ...
    /// studio.follow().await.unwrap();
    /// // ...
    /// # })
    /// ```
    pub async fn meta(self: &Arc<Self>) -> Result<Arc<StudioMeta>, api::GeneralError> {
        Ok(StudioMeta::with_this_this(self.api.get_studio_meta(self.id).await?, self.clone()))
    }

    /// Get studio curators
    /// - Results in [`GeneralStream`]
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// let some_curators = studio.curators((0, 67)).next().await.unwrap();
    /// dbg![some_curators.len()] // 40
    /// # })
    /// ```
    /// - To get all of them:
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(33004041);
    /// let all_curators = studio.curators((0, 67)).all().await.unwrap();
    /// dbg![all_curators.len()] // 67 (Or less if studio has less curators than 67)
    /// for curator in all_curators {
    ///     dbg![curator];
    /// }
    /// # })
    /// ```
    #[cfg(feature = "stream")]
    pub fn curators(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<StudioCurators> {
        GeneralStream::with_this(StudioCurators, cursor.into(), self.clone(), self.api.clone())
    }

    /// Get studio managers
    /// - Results in [`GeneralStream<StudioManagers>`]
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// for manager in studio.managers((0, 12)).all().await.unwrap() {
    ///     dbg![manager];
    /// }
    /// # })
    /// ```
    #[cfg(feature = "stream")]
    pub fn managers(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<StudioManagers> {
        GeneralStream::with_this(StudioManagers, cursor.into(), self.clone(), self.api.clone())
    }

    /// Get studio projects
    /// - Results in [`GeneralStream<StudioProjects>`]
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// for project in studio.projects((0, 14)).all().await.unwrap() {
    ///     dbg![project];
    /// }
    /// # })
    /// ```
    #[cfg(feature = "stream")]
    pub fn projects(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<StudioProjects> {
        GeneralStream::with_this(StudioProjects, cursor.into(), self.clone(), self.api.clone())
    }

    /// Get studio comments
    /// - Results in [`GeneralStream<StudioComments>`]
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// for comment in studio.comments((5, 20)).all().await.unwrap() {
    ///     dbg![comment];
    /// }
    /// # })
    /// ```
    #[cfg(feature = "stream")]
    pub fn comments(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<StudioComments> {
        GeneralStream::with_this(StudioComments, cursor.into(), self.clone(), self.api.clone())
    }

    /// Get studio activity
    /// - Results in [`GeneralStream<StudioActivity>`]
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// for action in studio.activity((0, 30)).all().await.unwrap() {
    ///     dbg![action];
    /// }
    /// # })
    /// ```
    #[cfg(feature = "stream")]
    pub fn activity(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<StudioActivity> {
        GeneralStream::with_this(StudioActivity, cursor.into(), self.clone(), self.api.clone())
    }

    /// Invite someone to the studio
    /// - Requires authentication
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// studio.invite("griffpatch").await.unwrap();
    /// # })
    /// ```
    pub async fn invite(&self, name: &str) -> Result<(), api::GeneralError> {
        self.api.invite_studio_curator(self.id, name).await
    }

    /// Add project to the studio
    /// - Requires authentication
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// studio.add_project(823872487).await.unwrap();
    /// # })
    /// ```
    pub async fn add_project(&self, id: u64) -> Result<(), api::GeneralError> {
        self.api.add_studio_project(self.id, id).await
    }

    /// Open studio to public
    /// - Requires authentication
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// studio.open().await.unwrap();
    /// # })
    /// ```
    pub async fn open(&self) -> Result<(), api::GeneralError> {
        self.api.open_studio(self.id).await
    }

    /// Close studio from public
    /// - Requires authentication
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// studio.close().await.unwrap();
    /// # })
    /// ```
    pub async fn close(&self) -> Result<(), api::GeneralError> {
        self.api.close_studio(self.id).await
    }

    /// Send comment in studio
    /// - Requires authentication
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// studio.send_comment("Hello everyone!").await.unwrap();
    /// # })
    /// ```
    pub async fn send_comment(&self, content: &str) -> Result<(), api::GeneralError> {
        self.api.send_studio_comment(self.id, content, None, None).await
    }

    /// Follow studio
    /// - Requires authentication
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// studio.follow().await.unwrap();
    /// # })
    /// ```
    pub async fn follow(&self) -> Result<(), api::GeneralError> {
        self.api.follow_studio(self.id).await
    }

    /// Unfollow studio
    /// - Requires authentication
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// studio.unfollow().await.unwrap();
    /// # })
    /// ```
    pub async fn unfollow(&self) -> Result<(), api::GeneralError> {
        self.api.unfollow_studio(self.id).await
    }


    /// Accept studio invite
    /// - Requires authentication
    /// - Works only if you have been invited to the studio
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// studio.accept_invite().await.unwrap();
    /// # })
    /// ```
    pub async fn accept_invite(&self) -> Result<(), api::GeneralError> {
        self.api.accept_studio_invite(self.id).await
    }

    /// Toggle studio commenting
    /// - Requires authentication
    /// - Works only if you have right permissions
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// studio.toggle_commenting().await.unwrap();
    /// # })
    /// ```
    pub async fn toggle_commenting(&self) -> Result<(), api::GeneralError> {
        self.api.toggle_studio_commenting(self.id).await
    }

    /// Promote user to a manager
    /// - Requires authentication
    /// - Works if user which is being promoted is a curator of that studio
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// studio.promote("griffpatch").await.unwrap();
    /// # })
    /// ```
    pub async fn promote(&self, name: &str) -> Result<(), api::GeneralError> {
        self.api.promote_studio_curator(self.id, name).await
    }

    /// Set studio info
    /// - Requires authentication
    /// - Works only if you have enough permissions
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// use s2rs::api::StudioInfo;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// studio.set_info(&StudioInfo {
    ///     title: "Cool studio title".to_string(),
    ///     description: "Cool studio description".to_string(),
    /// }).await.unwrwap();
    /// # })
    /// ```
    pub async fn set_info(&self, info: &StudioInfo) -> Result<(), api::GeneralError> {
        self.api.set_studio_info(self.id, info).await
    }

    /// Set studio title
    /// - Requires authentication
    /// - Works only if you have enough permissions
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// studio.set_title("Some cool studio").await.unwrap();
    /// # })
    /// ```
    pub async fn set_title(&self, content: impl Into<String>) -> Result<(), api::GeneralError> {
        self.set_info(&StudioInfo {
            description: None,
            title: Some(content.into())
        }).await
    }

    /// Set studio description
    /// - Requires authentication
    /// - Works only if you have enough permissions
    /// # Examples
    /// ```
    /// # tokio_test::block_on(async {
    /// # use s2rs::session::Session;
    /// # let session = Session::new("YourUsername");
    /// let studio = session.studio(32774157);
    /// studio.set_title("Some cool description").await.unwrap();
    /// # })
    /// ```
    pub async fn set_description(&self, content: impl Into<String>) -> Result<(), api::GeneralError> {
        self.set_info(&StudioInfo {
            description: Some(content.into()),
            title: None
        }).await
    }
}
// endregion: Studio

