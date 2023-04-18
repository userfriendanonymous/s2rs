//! General project related entities

use std::sync::Arc;
use derivative::Derivative;
use crate::api::{Api, self};
#[cfg(feature = "stream")] use crate::cursor::Cursor;
use super::User;
#[cfg(feature = "web_socket")] use super::Cloud;
#[cfg(feature = "stream")] use super::{project_stream::*, stream::GeneralStream};
use s2rs_derive::deref;

// region: ProjectWithTitle
/// Extends [`Project`] with it's title
/// # Examples
/// ```
/// # tokio_test::block_on(async {
/// # use s2rs::session::Session;
/// # let session = Session::new("YourUsername");
/// let project = session.project(823872487);
/// let project_with_title = project.meta().await.unwrap().this; // `this` field stores ProjectWithTitle
/// dbg![meta.title, meta.id];
/// # })
/// ```
#[derive(Debug, PartialEq, Eq)]
#[deref(this)]
pub struct ProjectWithTitle {
    pub title: String,
    pub this: Arc<Project>
}

impl ProjectWithTitle {
    pub fn new(title: String, id: u64, api: Arc<Api>) -> Arc<Self> {
        Self::with_this(title, Project::new(id, api))
    }

    pub fn with_this(title: String, this: Arc<Project>) -> Arc<Self> {
        Arc::new(Self {
            this,
            title
        })
    }
}
// endregion: ProjectWithTitle

// region: ProjectCore
#[derive(Debug)]
pub struct ProjectCoreRaw {
    pub description: String,
    pub instructions: String,
    pub visibility: String,
    pub public: bool,
    pub comments_allowed: bool,
    pub is_published: bool,
    pub image: String,
    pub images: api::ProjectImages,
    pub stats: api::ProjectStats,
    pub remix: api::ProjectRemix,
    pub history: api::ProjectHistory,
}

#[derive(Debug)]
#[deref(this)]
pub struct ProjectCore {
    pub this: Arc<ProjectWithTitle>,
    pub description: String,
    pub instructions: String,
    pub visibility: String,
    pub public: bool,
    pub comments_allowed: bool,
    pub is_published: bool,
    pub image: String,
    pub images: api::ProjectImages,
    pub stats: api::ProjectStats,
    pub remix: api::ProjectRemix,
    pub history: api::ProjectHistory,
}

impl ProjectCore {
    pub fn with_this(data: ProjectCoreRaw, this: Arc<ProjectWithTitle>) -> Arc<Self> {
        Arc::new(Self {
            this,
            image: data.image,
            images: data.images,
            instructions: data.instructions,
            is_published: data.is_published,
            public: data.public,
            remix: data.remix,
            stats: data.stats,
            visibility: data.visibility,
            comments_allowed: data.comments_allowed,
            description: data.description,
            history: data.history
        })
    }

    pub fn new(data: ProjectCoreRaw, id: u64, title: String, api: Arc<Api>) -> Arc<Self> {
        Self::with_this(data, ProjectWithTitle::new(title, id, api))
    }
}
// endregion: ProjectCore

// region: ProjectMeta
/// Project metadata
/// - Mapping for <https://api.scratch.mit.edu/projects/PROJECT-ID>
/// # Examples
/// ```
/// # tokio_test::block_on(async {
/// # use s2rs::session::Session;
/// # let session = Session::new("YourUsername");
/// let project = session.project(823872487);
/// let meta = project.meta().await.unwrap();
/// dbg![
///     &meta.author.history.joined,
///     &meta.token
///     // ...
/// ];
/// # })
/// ```
#[derive(Debug)]
#[deref(this)]
pub struct ProjectMeta {
    pub this: Arc<ProjectCore>,
    pub author: ProjectAuthor,
    pub token: String
}

impl ProjectMeta {
    pub fn with_this_this(data: api::Project, this: Arc<ProjectWithTitle>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            this: ProjectCore::with_this(
                ProjectCoreRaw {
                    comments_allowed: data.comments_allowed,
                    description: data.description,
                    history: data.history,
                    image: data.image,
                    images: data.images,
                    instructions: data.instructions,
                    is_published: data.is_published,
                    public: data.public,
                    remix: data.remix,
                    stats: data.stats,
                    visibility: data.visibility
                },
            this),
            author: ProjectAuthor::new(data.author, api),
            token: data.token
        })
    }

    pub fn with_this_this_this(data: api::Project, this: Arc<Project>, api: Arc<Api>) -> Arc<Self> {
        let title = data.title.clone();
        Self::with_this_this(data, ProjectWithTitle::with_this(title, this), api)
    }

    pub fn new(data: api::Project, api: Arc<Api>) -> Arc<Self> {
        let id = data.id;
        let title = data.title.clone();
        Self::with_this_this(data, ProjectWithTitle::new(title, id, api.clone()), api)
    }

    pub fn vec_new(data: Vec<api::Project>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}
// endregion: ProjectMeta

// region: ProjectAuthor
/// Project author
/// # Examples
/// ```
/// # tokio_test::block_on(async {
/// # use s2rs::session::Session;
/// # let session = Session::new("YourUsername");
/// let project = session.project(823872487);
/// let author = &project.meta().await.unwrap().author;
/// dbg![&author.scratch_team];
/// # })
/// ```
#[derive(Debug)]
#[deref(this)]
pub struct ProjectAuthor {
    pub this: Arc<User>,
    pub scratch_team: bool,
    pub history: api::UserHistory,
    pub profile: api::ProjectAuthorProfile,
}

impl ProjectAuthor {
    pub fn new(data: api::ProjectAuthor, api: Arc<Api>) -> Self {
        Self {
            this: User::new(data.name, api),
            history: data.history,
            profile: data.profile,
            scratch_team: data.scratch_team,
        }
    }

    pub fn with_this(data: api::ProjectAuthor, this: Arc<User>) -> Self {
        Self {
            this,
            history: data.history,
            profile: data.profile,
            scratch_team: data.scratch_team,
        }
    }
}
// endregion: ProjectAuthor

// region: PartialProject
/// Partial project
/// - Used to map some parts of API that return project metadata which is not the same as [`ProjectMeta`]
/// # Examples TODO
/// ```
/// # tokio_test::block_on(async {
/// # use s2rs::session::Session;
/// # let session = Session::new("YourUsername");
/// let project = session.project(823872487);
/// let author = &project.meta().await.unwrap().author;
/// dbg![&author.scratch_team];
/// # })
/// ```
#[derive(Debug)]
#[deref(this)]
pub struct PartialProject {
    pub this: Arc<ProjectCore>,
    pub author: api::PartialProjectAuthor,
}

impl PartialProject {
    pub fn with_this_this(data: api::PartialProject, this: Arc<ProjectWithTitle>) -> Arc<Self> {
        Arc::new(Self {
            author: data.author,
            this: ProjectCore::with_this(
                ProjectCoreRaw {
                    comments_allowed: data.comments_allowed,
                    description: data.description,
                    history: data.history,
                    image: data.image,
                    images: data.images,
                    instructions: data.instructions,
                    is_published: data.is_published,
                    public: data.public,
                    remix: data.remix,
                    stats: data.stats,
                    visibility: data.visibility
                },
            this),
        })
    }

    pub fn with_this_this_this(data: api::PartialProject, this: Arc<Project>) -> Arc<Self> {
        let title = data.title.clone();
        Self::with_this_this(data, ProjectWithTitle::with_this(title, this))
    }

    pub fn new(data: api::PartialProject, api: Arc<Api>) -> Arc<Self> {
        let id = data.id;
        let title = data.title.clone();
        Self::with_this_this(data, ProjectWithTitle::new(title, id, api))
    }

    pub fn vec_new(data: Vec<api::PartialProject>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}
// endregion: PartialProject

// region: PartialProject2
#[deref(this)]
#[derive(Debug)]
pub struct PartialProject2 {
    pub this: Arc<ProjectCore>,
    pub author: ProjectAuthor,
}

impl PartialProject2 {
    pub fn with_this_this(data: api::PartialProject2, this: Arc<ProjectWithTitle>, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            author: ProjectAuthor::new(data.author, api),
            this: ProjectCore::with_this(
                ProjectCoreRaw {
                    comments_allowed: data.comments_allowed,
                    description: data.description,
                    history: data.history,
                    image: data.image,
                    images: data.images,
                    instructions: data.instructions,
                    is_published: data.is_published,
                    public: data.public,
                    remix: data.remix,
                    stats: data.stats,
                    visibility: data.visibility
                },
            this),
        })
    }

    pub fn with_this_this_this(data: api::PartialProject2, this: Arc<Project>, api: Arc<Api>) -> Arc<Self> {
        let title = data.title.clone();
        Self::with_this_this(data, ProjectWithTitle::with_this(title, this), api)
    }

    pub fn new(data: api::PartialProject2, api: Arc<Api>) -> Arc<Self> {
        let id = data.id;
        let title = data.title.clone();
        Self::with_this_this(data, ProjectWithTitle::new(title, id, api.clone()), api)
    }

    pub fn vec_new(data: Vec<api::PartialProject2>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}
// endregion: PartialProject2

// region: Project

/// Project identifier
/// # Examples
/// ```
/// # tokio_test::block_on(async {
/// # use s2rs::session::Session;
/// # let session = Session::new("YourUsername");
/// let project = session.project(823872487);
/// project.love().await.unwrap();
/// // ...
/// # })
/// ```
/// ```
/// # tokio_test::block_on(async {
/// # use s2rs::session::Session;
/// # let session = Session::new("YourUsername");
/// let project = session.project(823872487);
/// project.love().await.unwrap(); // Love project
/// dbg![studio.meta().await.unwrap()]; // Gets project metadata
/// project.set_title("This is a cool project!").await.unwrap();
/// project.send_comment("epic comment").await.unwrap();
/// for project in project.remixes((0, 13)).all().await.unwrap() {
///     project.love().await.unwrap();
/// }
/// // ...
/// # })
/// ```
#[derive(Derivative)]
#[derivative(Debug, PartialEq, Eq)]
pub struct Project {
    #[derivative(Debug="ignore", PartialEq="ignore")]
    api: Arc<Api>,
    pub id: u64,
}

impl Project {
    pub fn new(id: u64, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            api,
            id
        })
    }
}

impl Project {
    pub async fn meta(self: &Arc<Self>) -> Result<Arc<ProjectMeta>, api::GeneralError> {
        Ok(ProjectMeta::with_this_this_this(self.api.get_project_meta(self.id).await?, self.clone(), self.api.clone()))
    }

    #[cfg(feature = "stream")]
    pub fn remixes(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<ProjectRemixes> {
        GeneralStream::with_this(ProjectRemixes, cursor.into(), self.clone(), self.api.clone())
    }

    #[cfg(feature = "stream")]
    pub fn comments(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<ProjectComments> {
        GeneralStream::with_this(ProjectComments, cursor.into(), self.clone(), self.api.clone())
    }

    #[cfg(feature = "stream")]
    pub fn cloud_activity(self: &Arc<Self>, cursor: impl Into<Cursor>) -> GeneralStream<ProjectCloudActivity> {
        GeneralStream::with_this(ProjectCloudActivity, cursor.into(), self.clone(), self.api.clone())
    }

    #[cfg(feature = "web_socket")]
    pub async fn cloud(self: &Arc<Self>, id: u64) -> Result<Arc<Cloud>, tokio_tungstenite::tungstenite::Error> {
        Ok(Cloud::new(5, self.api.get_project_cloud(id).await?))
    }

    pub async fn love(&self) -> Result<(), api::GeneralError> {
        self.api.love_project(self.id).await
    }

    pub async fn unlove(&self) -> Result<(), api::GeneralError> {
        self.api.unlove_project(self.id).await
    }

    pub async fn favorite(&self) -> Result<(), api::GeneralError> {
        self.api.favorite_project(self.id).await
    }

    pub async fn unfavorite(&self) -> Result<(), api::GeneralError> {
        self.api.unfavorite_project(self.id).await
    }

    pub async fn unshare(&self) -> Result<(), api::GeneralError> {
        self.api.unshare_project(self.id).await
    }

    pub async fn send_comment(&self, content: &str) -> Result<(), api::GeneralError> {
        self.api.send_project_comment(self.id, content, None, None).await
    }

    pub async fn delete_comment(&self, id: u64) -> Result<(), api::GeneralError> {
        self.api.delete_project_comment(self.id, id).await
    }

    pub async fn view(&self) -> Result<(), api::GeneralError> {
        self.api.view_project(self.id).await
    }

    pub async fn set_commenting(&self, allowed: bool) -> Result<(), api::GeneralError> {
        self.api.set_project_commenting(self.id, allowed).await
    }

    pub async fn report(&self) -> Result<(), api::GeneralError> {
        self.api.report_user_comment(self.id).await
    }
}
// endregion: Project