use s2rs_derive::Forwarder;
use reqwest::StatusCode;
use super::{Api, utils::RequestBuilderUtils};
use crate::json::{self, Parsable};
use crate::cursor::Cursor;

#[derive(Debug)]
pub struct CloudAction {
    pub by_name: String,
    pub event: CloudActionEvent,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Forwarder)]
pub enum CloudActionParseError {
    #[forward] Event(CloudActionEventParseError),
    #[forward] Expected(json::ExpectedError)
}

impl json::Parsable for CloudAction {
    type Error = CloudActionParseError;
    fn parse(data: &json::Parser) -> Result<Self, Self::Error> {
        Ok(Self {
            by_name: data.i("user").string()?,
            event: data.parse()?,
            timestamp: data.i("timestamp").u64()?,
        })
    }
}

#[derive(Debug)]
pub enum CloudActionEvent {
    Create(String),
    Delete(String),
    Set {
        name: String,
        value: String,
    },
}

#[derive(Debug, Clone, Forwarder)]
pub enum CloudActionEventParseError {
    #[forward] Expected(json::ExpectedError),
    InvalidType(String)
}

impl json::Parsable for CloudActionEvent {
    type Error = CloudActionEventParseError;
    fn parse(data: &json::Parser) -> Result<Self, Self::Error> {
        Ok(match data.i("verb").str()? {
            "set_var" => Self::Set {
                name: data.i("name").string()?,
                value: if let Ok(value) = data.i("value").string() {
                    value
                } else {
                    data.i("value").u64()?.to_string()
                }
            },
            "create_var" => Self::Create(data.i("name").string()?),
            "del_var" => Self::Delete(data.i("name").string()?),
            t => Err(CloudActionEventParseError::InvalidType(t.to_owned()))?
        })
    }
}

#[derive(Forwarder, Debug)]
pub enum GetProjectCloudActivityError {
    #[forward(StatusCode, reqwest::Error)]
    This(super::Error),
    #[forward] Parsing(CloudActionParseError),
}

impl Api {
    pub async fn project_cloud_activity(&self, id: u64, cursor: impl Into<Cursor>) -> Result<Vec<CloudAction>, GetProjectCloudActivityError> {
        let response = self.get_cloud("logs").cursor(cursor)
        .query(&[("projectid", id)]).send_success().await?;
        Ok(CloudAction::parse_vec(&response.json::<Vec<_>>().await?)?)
    }
}