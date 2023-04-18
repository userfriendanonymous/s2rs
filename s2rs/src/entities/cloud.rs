#[cfg(feature = "web_socket")] use std::sync::Arc;
#[cfg(feature = "web_socket")] use crate::web_socket::{WebSocket, self};
#[cfg(feature = "web_socket")] use crate::headers::Headers;

pub trait CloudListener {
    fn receive(&self, data: String);
}

// region: Listener
#[allow(unused)]
#[cfg(feature = "web_socket")]
struct Listener<L: CloudListener> {
    cloud: Arc<Cloud>,
    this: Arc<L>,
    id: u64,
    headers: Arc<Headers>,
}

#[cfg(feature = "web_socket")]
impl<L: CloudListener> Listener<L> {
    pub fn new(id: u64, headers: Arc<Headers>, this: Arc<L>, cloud: Arc<Cloud>) -> Arc<Self> {
        Arc::new(Self {
            cloud,
            this,
            id,
            headers
        })
    }
}

#[cfg(feature = "web_socket")]
impl<L: CloudListener> web_socket::Listener for Listener<L> {
    fn receive_text(&self, content: String) {
        dbg!(content);
    }
}
// endregion: Listener

// region: Cloud
#[cfg(feature = "web_socket")]
pub struct Cloud {
    socket: Arc<WebSocket>,
    id: u64,
}

#[cfg(feature = "web_socket")]
impl Cloud {
    pub fn new(id: u64, socket: Arc<WebSocket>) -> Arc<Self> {
        Arc::new(Self {
            socket,
            id
        })
    }

    pub async fn listen<L: CloudListener>(self: Arc<Self>, headers: Arc<Headers>, listener: Arc<L>) {
        self.socket.listen(Listener::new(self.id, headers, listener, self.clone())).await;
    }
}
// endregion: Cloud

