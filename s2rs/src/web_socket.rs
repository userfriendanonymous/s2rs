use std::sync::Arc;

use futures_util::{StreamExt, stream::{SplitSink, SplitStream}, SinkExt};
use s2rs_derive::Forwarder;
use serde::Serialize;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::{self, Message}, connect_async, WebSocketStream, MaybeTlsStream};

type Write = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Message>;
type Read = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

#[derive(Forwarder)]
pub enum SendJsonError {
    #[forward] Serde(serde_json::Error),
    #[forward] Tungstenite(tungstenite::Error)
}

pub trait Listener {
    fn receive_text(&self, content: String);
    // fn r(&self, error: tungstenite::Error);
}

pub struct WebSocket {
    write: Mutex<Write>,
    read: Mutex<Read>,
}

impl WebSocket {
    pub async fn new<U>(url: U) -> Result<Arc<Self>, tungstenite::Error>
    where U: tungstenite::client::IntoClientRequest + Unpin {
        let (stream, _response) = connect_async(url).await?;
        let (write, read) = stream.split();
        Ok(Arc::new(Self {
            read: Mutex::new(read),
            write: Mutex::new(write)
        }))
    }

    pub async fn text(&self, message: String) -> Result<(), tungstenite::Error> {
        self.write.lock().await.send(Message::Text(message)).await?;
        Ok(())
    }

    pub async fn json<T: Serialize>(&self, message: &T) -> Result<(), SendJsonError> {
        Ok(self.text(serde_json::to_string(message)?).await?)
    }

    pub async fn listen<L: Listener>(&self, listener: Arc<L>) {
        while let Some(result) = self.read.lock().await.next().await {
            match result {
                Ok(message) => match message {
                    Message::Text(content) => listener.receive_text(content),
                    _ => todo!()
                },
                _ => todo!()
            }
        }

    }

}