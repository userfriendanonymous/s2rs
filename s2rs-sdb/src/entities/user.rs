use std::sync::Arc;

use derivative::Derivative;
use s2rs_derive::deref;

use crate::api::Api;

#[deref(this)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct User {
    #[derivative(Debug = "ignore")]
    pub api: Arc<Api>,
    pub this: Arc<s2rs::api::User>
}

impl User {
    pub async fn sdb_meta(&self) -> Self {}
}