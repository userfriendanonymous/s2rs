use std::sync::Arc;
mod user;

pub enum Error {
    Network(s2rs::api::NetworkError)
}

pub struct Api {
    #[allow(unused)]
    name: Arc<String>
}

impl s2rs::api::Extension for Api {
    fn extended(pipe: s2rs::api::ExtensionPipe) -> Arc<Self> {
        Self::new(pipe.name)
    }
}

impl Api {
    pub fn new(name: impl Into<Arc<String>>) -> Arc<Self> {
        Arc::new(Self {
            name: name.into()
        })
    }
}