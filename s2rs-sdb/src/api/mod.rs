use std::sync::Arc;
use reqwest::{RequestBuilder, Method, Client};
use s2rs_derive::{deref, Forwarder};
use crate::json;

pub use user::*;
pub use project::*;
pub use forum_user::*;

pub mod user;
pub mod project;
pub mod forum_user;
mod utils;

const BASE_URL: &str = "https://scratchdb.lefty.one/v3/";

#[derive(Forwarder, Debug)]
pub enum Error {
    #[forward] Network(reqwest::Error),
    #[forward] Parsing(ParsingError)
}

#[derive(Forwarder, Debug)]
pub enum ParsingError {
    #[forward] Json(json::Error),
    #[forward] Serde(serde_json::Error)
}

#[deref(this)]
pub struct Api {
    client: Arc<Client>,
    #[allow(unused)]
    name: Arc<String>,
    pub this: Arc<s2rs::Api>,
}

impl s2rs::api::Extension for Api {
    
    fn extended(pipe: s2rs::api::ExtensionPipe, this: Arc<s2rs::Api>) -> Arc<Self> {
        Arc::new(Self {
            client: pipe.client,
            name: pipe.name,
            this
        })
    }
}

impl Api {
}

impl Api {
    fn request(&self, method: Method, url: &str) -> RequestBuilder {
        self.client.request(method, format!["{BASE_URL}{url}"])
    }

    fn get(&self, url: &str) -> RequestBuilder {
        self.request(Method::GET, url)
    }
}