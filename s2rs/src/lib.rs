//! Access Scratch API with Rust
pub use api::Api;
pub use session::Session;
pub use entities::*; // TO BE ENABLED # DISABLE IS DEV ONLY
pub use language::Language;
pub use cursor::Cursor;

pub mod api;
pub mod session;
pub mod entities;
pub mod cursor;
pub mod language;
mod utils;
mod cookies;
mod headers;
mod json;
#[cfg(feature = "web_socket")] mod web_socket;
#[cfg(feature = "html")] mod html;