//! Access Scratch API with Rust
pub use api::Api;
pub use session::Session;
pub use entities::*; // TO BE ENABLED # DISABLE IS DEV ONLY
pub use language::Language;

pub mod api;
pub mod session;
pub mod entities;
mod utils;
mod cursor;
mod cookies;
mod headers;
mod rss;
mod language;
#[cfg(feature = "web_socket")] mod web_socket;
#[cfg(feature = "html")] mod html;