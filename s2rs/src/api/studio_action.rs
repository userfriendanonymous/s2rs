use std::str::FromStr;

use s2rs_derive::Forwarder;

use super::{Api, utils::RequestBuilderUtils};
use crate::cursor::Cursor;
use crate::json;

#[derive(Debug)]
pub struct StudioAction {
    pub id: u64,
    pub actor_name: String,
    pub actor_id: u64,
    pub created_at: String,
    pub event: StudioActionEvent,
    pub event_type: String,
}

#[derive(Debug, Clone, Forwarder)]
pub enum StudioActionParseIdError {
    NoContent,
    Parsing(<u64 as FromStr>::Err)
}

#[derive(Forwarder, Clone, Debug)]
pub enum StudioActionParseError {
    #[forward(<u64 as FromStr>::Err)]
    Id(StudioActionParseIdError),
    #[forward] Event(StudioActionEventParseError)
}

impl json::Parsable for StudioAction {
    type Error = StudioActionParseError;
    fn parse(data: &json::Parser) -> Result<Self, Self::Error> {
        let id = data.i("id").string()?;
        let id = id.split('-').last().ok_or(StudioActionParseIdError::NoContent)?.parse()?;

        Ok(StudioAction {
            actor_name: data.i("actor_username").string()?,
            actor_id: data.i("actor_id").u64()?,
            created_at: data.i("datetime_created").string()?,
            event: data.parse()?,
            id,
            event_type: data.i("type").string()?
        })
    }
}

#[derive(Debug)]
pub enum StudioActionEvent {
    Update,
    AddProject {
        id: u64, // project_id
        title: String // project_title
    },
    RemoveProject {
        id: u64, // both same as on add project
        title: String,
    },
    AcceptInvite {
        from_name: String, // username
    },
    Promote {
        name: String // recipient_username
    },
}

#[derive(Forwarder)]
pub enum StudioActionEventParseError {
    #[forward] Expected(json::ExpectedError),
    InvalidType(String)
}

impl json::Parsable for StudioActionEvent {
    type Error = StudioActionEventParseError;
    fn parse(data: &json::Parser) -> Result<Self, Self::Error> {
        Ok(match data.i("type").string()?.as_str() {
            "updatestudio" => Self::Update,
            "addprojecttostudio" => Self::AddProject {
                id: data.i("project_id").u64()?,
                title: data.i("project_title").string()?,
            },
            "removeprojectstudio" => Self::RemoveProject {
                id: data.i("project_id").u64()?,
                title: data.i("project_title").string()?,
            },
            "becomecurator" => Self::AcceptInvite {
                from_name: data.i("username").string()?,
            },
            "becomeownerstudio" => Self::Promote {
                name: data.i("recipient_username").string()?
            },
            t => Err(StudioActionEventParseError::InvalidType(t.to_owned()))?
        })
    }
}

impl Api {
    pub async fn get_studio_activity(&self, id: u64, cursor: impl Into<Cursor>) -> super::Result<Vec<StudioAction>> {
        let response = self.get(&format!["studios/{id}/activity/"]).cursor(cursor).send_success().await?;
        response.general_parser_vec().await
    }
}