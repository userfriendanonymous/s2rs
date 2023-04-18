#[cfg(feature = "web_socket")] use std::sync::Arc;
#[cfg(feature = "web_socket")] use crate::web_socket::WebSocket;
use super::Api;

impl Api {
    #[cfg(feature = "web_socket")]
    pub async fn get_project_cloud(&self, _id: u64) -> Result<Arc<WebSocket>, tokio_tungstenite::tungstenite::Error> {
        WebSocket::new(
            "wss://clouddata.scratch.mit.edu"
        ).await
    }
}