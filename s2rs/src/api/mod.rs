use std::sync::Arc;
use reqwest::{StatusCode, Client, Method, RequestBuilder};
use s2rs_derive::Forwarder;
use crate::{cookies::Cookies, headers};

pub use studio::*;
pub use user::*;
pub use project::*;
pub use following_action::*;
pub use studio_action::*;
pub use message::*;
pub use comment::*;
pub use studio_project::*;
pub use user_comment::*;
pub use cloud_action::*;
pub use user_featured::*;
pub use front_page::*;
pub use explore::*;
pub use search::*;
pub use forum::*;

pub mod user;
pub mod project;
pub mod studio;
pub mod following_action;
pub mod message;
pub mod comment;
pub mod studio_project;
pub mod user_comment;
pub mod user_action;
pub mod studio_action;
pub mod cloud_action;
pub mod cloud;
pub mod forum;
pub mod user_featured;
pub mod front_page;
pub mod explore;
pub mod search;
mod utils;

pub mod protocols {
    pub const HTTPS: &str = "https://";
    pub const HTTP: &str = "http://";
}

pub mod domains {
    pub const API: &str = "api.scratch.mit.edu/";
    pub const PROJECTS: &str = "projects.scratch.mit.edu/";
    pub const BASE: &str = "scratch.mit.edu/";
    pub const CLOUD: &str = "clouddata.scratch.mit.edu/";
    pub const UPLOADS: &str = "uploads.scratch.mit.edu/";
}

pub struct Tokens {
    pub session: String,
    pub x: String,
    pub csrf: String,
}

// region: errors
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Forwarder, Debug)]
pub enum Error {
    #[forward] Status(StatusCode),
    #[forward] Network(reqwest::Error),
    #[forward] Parsing(serde_json::Error)
}

#[derive(Debug, Forwarder)]
pub enum WithAuthError {
    #[forward] Client(reqwest::Error),
    #[forward] Headers(HeadersError),
}
// endregion: errors

// region: Headers
#[derive(Forwarder, Debug)]
pub enum HeadersError {
    #[forward] Reqwest(headers::TryIntoReqwestHeadersError),
}

#[derive(Default, Debug)]
pub struct Headers {
    pub reqwest: reqwest::header::HeaderMap,
    pub local: Arc<headers::Headers>
}

impl TryFrom<Arc<headers::Headers>> for Headers {
    type Error = HeadersError;
    fn try_from(local: Arc<headers::Headers>) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            reqwest: (*local).clone().try_into()?,
            local,
        })
    }
}
// endregion: Headers

pub struct ExtensionPipe {
    pub client: Arc<Client>,
    pub name: Arc<String>,
    pub headers: Arc<headers::Headers>,
}

pub trait Extension {
    fn extended(pipe: ExtensionPipe, this: Arc<Api>) -> Arc<Self>;
}

#[derive(Debug)]
pub struct Api {
    client: Arc<Client>,
    name: Arc<String>,
    headers: Headers,
}

impl Api {
    pub fn extend<T: Extension>(self: &Arc<Self>) -> Arc<T> {
        T::extended(ExtensionPipe {
            client: self.client.clone(),
            name: self.name.clone(),
            headers: self.headers.local.clone()
        }, self.clone())
    }

    pub fn new(name: impl Into<Arc<String>>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::new(Client::new()),
            name: name.into(),
            headers: Headers::default()
        })
    }

    pub fn with_auth(name: impl Into<Arc<String>>, tokens: &Tokens) -> std::result::Result<Arc<Self>, WithAuthError> {
        let mut cookies = Cookies::default();
        cookies.add("scratchcsrftoken", tokens.csrf.as_str());
        cookies.add("scratchsessionsid", tokens.session.as_str());
        cookies.add("scratchlanguage", "en");

        let mut headers = headers::Headers::new();
        headers.add("cookie", Into::<String>::into(cookies));
        headers.add("x-csrftoken", &tokens.csrf);
        headers.add("x-token", &tokens.x);
        headers.add("referer", "https://scratch.mit.edu");
        headers.add("x-requested-with", "XMLHttpRequest");
        headers.add("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.101 Safari/537.36");
        
        Ok(Arc::new(Self {
            client: Arc::new(Client::new()),
            name: name.into(),
            headers: Arc::new(headers).try_into()?,
        }))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn request(&self, method: Method, url: &str) -> RequestBuilder {
        self.client.request(method, url).headers(self.headers.reqwest.clone())
    }
    fn https_request(&self, method: Method, url: &str) -> RequestBuilder {
        self.request(method, &format!["{}{url}", protocols::HTTPS])
    }

    // region: api
    fn request_api(&self, method: Method, path: &str) -> RequestBuilder {
        self.https_request(method, &format!["{}{path}", domains::API])
    }
    fn get(&self, path: &str) -> RequestBuilder {
        self.request_api(Method::GET, path)
    }
    fn put(&self, path: &str) -> RequestBuilder {
        self.request_api(Method::PUT, path)
    }

    fn post(&self, path: &str) -> RequestBuilder {
        self.request_api(Method::POST, path)
    }
    // endregion: api

    // region: base
    fn get_base(&self, path: &str) -> RequestBuilder {
        self.https_request(Method::GET, &format!["{}{path}", domains::BASE])
    }
    // endregion: base

    // region: site-api
    fn request_site_api(&self, method: Method, path: &str) -> RequestBuilder {
        self.https_request(method, &format!["{}site-api/{path}", domains::BASE])
    }
    #[allow(unused)]
    fn get_site_api(&self, path: &str) -> RequestBuilder {
        self.request_site_api(Method::GET, path)
    }
    fn put_site_api(&self, path: &str) -> RequestBuilder {
        self.request_site_api(Method::PUT, path)
    }
    fn post_site_api(&self, path: &str) -> RequestBuilder {
        self.request_site_api(Method::POST, path)
    }
    // endregion: site-api

    // region: proxy
    fn request_proxy(&self, method: Method, path: &str) -> RequestBuilder {
        self.request_site_api(method, &format!["{}proxy/{path}", domains::API])
    }
    fn get_proxy(&self, path: &str) -> RequestBuilder {
        self.request_proxy(Method::GET, path)
    }
    fn post_proxy(&self, path: &str) -> RequestBuilder {
        self.request_proxy(Method::POST, path)
    }
    fn put_proxy(&self, path: &str) -> RequestBuilder {
        self.request_proxy(Method::PUT, path)
    }
    fn delete_proxy(&self, path: &str) -> RequestBuilder {
        self.request_proxy(Method::DELETE, path)
    }
    // endregion: proxy

    // region: cloud
    #[allow(unused)]
    fn request_cloud(&self, method: Method, path: &str) -> RequestBuilder {
        self.https_request(method, &format!["{}{path}", domains::CLOUD])
    }
    fn get_cloud(&self, path: &str) -> RequestBuilder {
        self.request_cloud(Method::GET, path)
    }
    // endregion: cloud

    // region: uploads
    fn request_uploads(&self, method: Method, path: &str) -> RequestBuilder {
        self.https_request(method, &format!["{}{path}", domains::UPLOADS])
    }
    fn get_uploads(&self, path: &str) -> RequestBuilder {
        self.request_uploads(Method::GET, path)
    }
    // endregion: uploads
}