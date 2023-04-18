use std::sync::Arc;
use lazy_static::lazy_static;
use reqwest::{StatusCode, Url, Client, Method, RequestBuilder};
use s2rs_derive::Forwarder;
use serde_json::Value;
use general_parser::GeneralParser;
use utils::{RequestBuilderUtils, ResponseUtils};
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
mod utils;
mod url_path;
mod general_parser;


lazy_static! {
    static ref URLS: Urls = Urls {
        api: Url::construct(protocols::HTTPS, domains::API).unwrap(),
        projects: Url::construct(protocols::HTTPS, domains::PROJECTS).unwrap(),
        base: Url::construct(protocols::HTTPS, domains::BASE).unwrap(),
        cloud: Url::construct(protocols::HTTPS, domains::CLOUD).unwrap(),
    };
}

trait UrlUtils where Self: Sized {
    fn construct(protocol: &str, domain: &str) -> Result<Self, url::ParseError>;
}

impl UrlUtils for Url {
    fn construct(protocol: &str, domain: &str) -> Result<Self, url::ParseError> {
        Self::parse(&format!("{}{}", protocol, domain))
    }
}

pub mod protocols {
    pub const HTTPS: &str = "https://";
    pub const HTTP: &str = "http://";
}

pub mod domains {
    pub const API: &str = "api.scratch.mit.edu";
    pub const PROJECTS: &str = "projects.scratch.mit.edu";
    pub const BASE: &str = "scratch.mit.edu";
    pub const CLOUD: &str = "clouddata.scratch.mit.edu";
}

#[derive(Debug, Clone)]
struct Urls {
    pub api: Url,
    #[allow(unused)]
    pub projects: Url,
    pub base: Url,
    pub cloud: Url,
}

type GeneralResult<T> = Result<T, GeneralError>;

#[derive(Forwarder, Debug)]
pub enum NetworkError {
    #[forward] Request(reqwest::Error),
    #[forward] Status(StatusCode),
}

#[derive(Forwarder, Debug)]
pub enum GeneralError {
    #[forward(StatusCode, reqwest::Error)]
    Network(NetworkError),
    #[forward(serde_json::Error, ParsingCustomError)]
    Parsing(ParsingError)
}

#[derive(Debug)]
pub struct ParsingCustomError;

impl<T> From<Option<T>> for ParsingCustomError {
    fn from(_: Option<T>) -> Self {
        Self
    }
}

impl From<()> for ParsingCustomError {
    fn from(_: ()) -> Self {
        Self
    }
}

impl From<ParsingCustomError> for ParsingError {
    fn from(_: ParsingCustomError) -> Self {
        Self::Custom
    }
}

#[derive(Forwarder, Debug)]
pub enum ParsingError {
    Custom,
    #[forward] Auto(serde_json::Error),
}

#[derive(Debug, Forwarder)]
pub enum Error {
    #[forward] Client(reqwest::Error),
    #[forward] Headers(HeadersError),
}

pub struct Tokens {
    pub session: String,
    pub x: String,
    pub csrf: String,
}

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
    fn try_from(local: Arc<headers::Headers>) -> Result<Self, Self::Error> {
        Ok(Self {
            reqwest: (*local).clone().try_into()?,
            local,
        })
    }
}

#[derive(Debug)]
pub struct Api {
    client: Arc<Client>,
    urls: Urls,
    name: String,
    headers: Headers
}

impl Api {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            client: Arc::new(Client::new()),
            urls: URLS.clone(),
            name: name.into(),
            headers: Headers::default()
        }
    }

    pub fn with_auth(name: impl Into<String>, tokens: &Tokens) -> Result<Self, Error> {
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
        
        Ok(Self {
            client: Arc::new(Client::new()),
            urls: URLS.clone(),
            name: name.into(),
            headers: Arc::new(headers).try_into()?,
        })
    }
    #[allow(unused)]
    async fn get_session(&self) { // not used
        let response = self.post_base("session/").send_success().await.unwrap();
        response.cookies().for_each(|cookie| {
            dbg!(cookie);
        });
        let res = response.json::<Value>().await.unwrap();
        dbg!(&res["user"]["token"]);
        dbg!(&res["user"]["banned"]);
        dbg!(res);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // region: request
    fn request(&self, method: Method, url: &Url, path: &str) -> RequestBuilder {
        let mut url = url.clone();
        url.set_path(path);
        self.client.request(method, url).headers(self.headers.reqwest.clone())
    }

    fn get_request(&self, url: &Url, path: &str) -> RequestBuilder {
        self.request(Method::GET, url, path)
    }

    fn post_request(&self, url: &Url, path: &str) -> RequestBuilder {
        self.request(Method::POST, url, path)
    }

    fn put_request(&self, url: &Url, path: &str) -> RequestBuilder {
        self.request(Method::PUT, url, path)
    }
    #[allow(unused)]
    fn delete_request(&self, url: &Url, path: &str) -> RequestBuilder {
        self.request(Method::DELETE, url, path)
    }
    #[allow(unused)]
    fn options_request(&self, url: &Url, path: &str) -> RequestBuilder {
        self.request(Method::DELETE, url, path)
    }
    // endregion: request

    // region: api
    fn get(&self, path: &str) -> RequestBuilder {
        self.get_request(&self.urls.api, path)
    }

    fn put(&self, path: &str) -> RequestBuilder {
        self.put_request(&self.urls.api, path)
    }

    fn post(&self, path: &str) -> RequestBuilder {
        self.post_request(&self.urls.api, path)
    }
    // endregion: api

    // region: base
    fn get_base(&self, path: &str) -> RequestBuilder {
        self.get_request(&self.urls.base, path)
    }
    #[allow(unused)]
    fn put_base(&self, path: &str) -> RequestBuilder {
        self.put_request(&self.urls.base, path)
    }
    #[allow(unused)]
    fn post_base(&self, path: &str) -> RequestBuilder {
        self.post_request(&self.urls.base, path)
    }
    // endregion: base

    // region: site-api
    fn request_site_api(&self, method: Method, path: &str) -> RequestBuilder {
        self.request(method, &self.urls.base, &format!("site-api/{path}"))
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
        self.request(method, &self.urls.api, &format!("proxy/{path}"))
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
        self.request(method, &self.urls.cloud, path)
    }

    fn get_cloud(&self, path: &str) -> RequestBuilder {
        self.request(Method::GET, &self.urls.cloud, path)
    }
    // endregion: cloud
}