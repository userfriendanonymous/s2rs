use std::sync::Arc;
use crate::api::{self, Api};
use super::User;

#[derive(Debug, PartialEq, Eq)]
pub struct CloudAction {
    pub by: Arc<User>,
    pub timestamp: u64,
    pub event: CloudActionEvent,
}

impl CloudAction {
    pub fn new(data: api::CloudAction, api: Arc<Api>) -> Arc<Self> {
        Arc::new(Self {
            by: User::new(data.by_name, api),
            timestamp: data.timestamp,
            event: CloudActionEvent::new(data.event)
        })
    }

    pub fn vec_new(data: Vec<api::CloudAction>, api: Arc<Api>) -> Vec<Arc<Self>> {
        data.into_iter().map(|data| Self::new(data, api.clone())).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloudActionEvent {
    Set {
        name: String,
        value: String,
    },
    Create(String),
    Delete(String),
}

impl CloudActionEvent {
    pub fn new(data: api::CloudActionEvent) -> Self {
        match data {
            api::CloudActionEvent::Create(name) => Self::Create(name),
            api::CloudActionEvent::Delete(name) => Self::Delete(name),
            api::CloudActionEvent::Set { name, value } => Self::Set { name, value }
        }
    }
}