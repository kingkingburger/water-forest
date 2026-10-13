use std::sync::Arc;

use russh::server::{self, Server};

use crate::chat_server::ChatServer;

mod chat_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = server::Config {
        keys: vec![russh::keys::load_secret_key("ssh_host_ed25519_key", None)?],
        ..Default::default()
    };

    let mut server = ChatServer;

    server
        .run_on_address(Arc::new(config), ("127.0.0.1", 2222))
        .await?;

    Ok(())
}
