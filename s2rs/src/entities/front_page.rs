
use std::sync::Arc;
use crate::{api, Api};
use super::{ProjectWithTitle, User, StudioWithTitle};

#[derive(Debug)]
pub struct FrontPage {
    pub new_projects: Vec<FrontPageProject>,
    pub featured_studios: Vec<FrontPageFeaturedStudio>,
    pub featured_projects: Vec<FrontPageProject>,
    pub curated_projects: Vec<FrontPageCuratedProject>,
    pub most_remixed_projects: Vec<FrontPageMostRemixedProject>,
    pub most_loved_projects: Vec<FrontPageProject>,
    pub design_studio_projects: Vec<FrontPageDesignStudioProject>,
}

impl FrontPage {
    pub fn new(data: api::FrontPage, api: Arc<Api>) -> Self {
        Self {
            new_projects: FrontPageProject::vec_new(data.new_projects, api.clone()),
            featured_studios: FrontPageFeaturedStudio::vec_new(data.featured_studios, api.clone()),
            curated_projects: FrontPageCuratedProject::vec_new(data.curated_projects, api.clone()),
            design_studio_projects: FrontPageDesignStudioProject::vec_new(data.design_studio_projects, api.clone()),
            featured_projects: FrontPageProject::vec_new(data.featured_projects, api.clone()),
            most_loved_projects: FrontPageProject::vec_new(data.most_loved_projects, api.clone()),
            most_remixed_projects: FrontPageMostRemixedProject::vec_new(data.most_remixed_projects, api),
        }
    }
}

// #[derive(Debug)]
// pub struct News {
//     pub id: u64,
//     pub at: String,
//     pub title: String,
//     pub url: String,
//     pub image: String,
//     pub description: String,
// }

// region: structures
#[derive(Debug)]
pub struct FrontPageProject {
    pub this: Arc<ProjectWithTitle>,
    pub author: Arc<User>,
    pub thumbnail_url: String,
    pub love_count: u32,
}

impl FrontPageProject {
    pub fn new(data: api::FrontPageProject, api: Arc<Api>) -> Self {
        Self {
            this: ProjectWithTitle::new(data.title, data.id, api.clone()),
            author: User::new(data.author_name, api),
            love_count: data.love_count,
            thumbnail_url: data.thumbnail_url
        }
    }

    pub fn vec_new(data: Vec<api::FrontPageProject>, api: Arc<Api>) -> Vec<Self> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}

#[derive(Debug)]
pub struct FrontPageMostRemixedProject {
    pub this: Arc<ProjectWithTitle>,
    pub author: Arc<User>,
    pub remix_count: u32,
    pub love_count: u32,
    pub thumbnail_url: String,
}

impl FrontPageMostRemixedProject {
    pub fn new(data: api::FrontPageMostRemixedProject, api: Arc<Api>) -> Self {
        Self {
            this: ProjectWithTitle::new(data.title, data.id, api.clone()),
            author: User::new(data.author_name, api),
            love_count: data.love_count,
            thumbnail_url: data.thumbnail_url,
            remix_count: data.remix_count,
        }
    }

    pub fn vec_new(data: Vec<api::FrontPageMostRemixedProject>, api: Arc<Api>) -> Vec<Self> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}

#[derive(Debug)]
pub struct FrontPageDesignStudioProject {
    pub this: Arc<ProjectWithTitle>,
    pub author: Arc<User>,
    pub studio: Arc<StudioWithTitle>,
    pub remix_count: u32,
    pub love_count: u32,
    pub thumbnail_url: String,
}

impl FrontPageDesignStudioProject {
    pub fn new(data: api::FrontPageDesignStudioProject, api: Arc<Api>) -> Self {
        Self {
            this: ProjectWithTitle::new(data.title, data.id, api.clone()),
            author: User::new(data.author_name, api.clone()),
            love_count: data.love_count,
            thumbnail_url: data.thumbnail_url,
            remix_count: data.remix_count,
            studio: StudioWithTitle::new(data.studio_title, data.studio_id, api)
        }
    }

    pub fn vec_new(data: Vec<api::FrontPageDesignStudioProject>, api: Arc<Api>) -> Vec<Self> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}

#[derive(Debug)]
pub struct FrontPageCuratedProject {
    pub this: Arc<ProjectWithTitle>,
    pub author: Arc<User>,
    pub curated_by: Arc<User>,
    pub love_count: u32,
    pub thumbnail_url: String,
}

impl FrontPageCuratedProject {
    pub fn new(data: api::FrontPageCuratedProject, api: Arc<Api>) -> Self {
        Self {
            this: ProjectWithTitle::new(data.title, data.id, api.clone()),
            author: User::new(data.author_name, api.clone()),
            love_count: data.love_count,
            thumbnail_url: data.thumbnail_url,
            curated_by: User::new(data.curated_by_name, api)
        }
    }

    pub fn vec_new(data: Vec<api::FrontPageCuratedProject>, api: Arc<Api>) -> Vec<Self> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}

#[derive(Debug)]
pub struct FrontPageFeaturedStudio {
    pub this: Arc<StudioWithTitle>,
    pub thumbnail_url: String,
}

impl FrontPageFeaturedStudio {
    pub fn new(data: api::FrontPageFeaturedStudio, api: Arc<Api>) -> Self {
        Self {
            this: StudioWithTitle::new(data.title, data.id, api),
            thumbnail_url: data.thumbnail_url
        }
    }

    pub fn vec_new(data: Vec<api::FrontPageFeaturedStudio>, api: Arc<Api>) -> Vec<Self> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}
// endregion: structures
