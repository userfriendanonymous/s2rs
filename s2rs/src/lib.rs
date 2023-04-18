//! Access Scratch API with Rust
pub use api::Api;
pub use session::Session;
// pub use entities::*; // TO BE ENABLED # DEV ONLY

pub mod api;
pub mod session;
pub mod entities;
mod utils;
mod cursor;
#[cfg(feature = "web_socket")] mod web_socket;
mod cookies;
mod headers;
mod rss;
#[cfg(feature = "html")] mod html;

#[cfg(test)]
mod tests {
    use crate::{session::Session, api::{Tokens, FeaturedLabel}};

    #[tokio::test]
    async fn main() -> Result<(), ()> {
        dotenv::dotenv().ok();
        let tokens = Tokens {
            csrf: "a".to_string(),
            session: std::env::var("SESSION_TOKEN").unwrap(),
            x: std::env::var("X_TOKEN").unwrap()
        };
        let session = Session::with_auth("", &tokens).unwrap();

        Ok(())
    }

    async fn _example1() -> Result<(), ()> {
        
        Ok(())
    }
}