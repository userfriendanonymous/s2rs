use super::{Api, utils::{RequestBuilderUtils, ResponseUtils}, ParsingCustomError};
use super::general_parser::{GeneralParser, GeneralParsable};
use crate::cursor::Cursor;

#[derive(Debug)]
pub struct CloudAction {
    pub by_name: String,
    pub event: CloudActionEvent,
    pub timestamp: u64,
}

impl GeneralParsable for CloudAction {
    type Error = ParsingCustomError;
    fn parse(data: &GeneralParser) -> Result<Self, Self::Error> {
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

impl GeneralParsable for CloudActionEvent {
    type Error = ParsingCustomError;
    fn parse(data: &GeneralParser) -> Result<Self, Self::Error> {
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
            _ => Err(())?
        })
    }
}

impl Api {
    pub async fn get_project_cloud_activity(&self, id: u64, cursor: impl Into<Cursor>) -> super::Result<Vec<CloudAction>> {
        let response = self.get_cloud("logs").cursor(cursor)
        .query(&[("projectid", id)]).send_success().await?;
        response.general_parser_vec().await
    }
}