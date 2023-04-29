use std::sync::Arc;
use crate::{api, Api};
use super::UserWithId;

#[derive(Clone, Debug)]
pub struct Login {
    pub this: Arc<UserWithId>,
    pub x_token: String,
    pub session_token: String,
    pub tries_count: u16,
    pub message: String,
    pub success: u8,
    pub messages: Vec<String>,
}

impl Login {
    // pub fn with_this_this(data: api::Login, this: Arc<User>, api: Arc<Api>) -> Self {
    //     Self {
    //         message: data.message,
    //         messages: data.messages,
    //         success: data.success,
    //         tries_count: data.tries_count,
    //         x_token: data.x_token,
    //         session_token: data.session_token,
    //         this: UserWithId::with_this(data.id, this, api)
    //     }
    // }

    pub fn new(data: api::Login, api: Arc<Api>) -> Self {
        Self {
            message: data.message,
            messages: data.messages,
            success: data.success,
            tries_count: data.tries_count,
            x_token: data.x_token,
            session_token: data.session_token,
            this: UserWithId::new(data.id, data.name, api),
        }
    }
}