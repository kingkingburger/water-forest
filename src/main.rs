use std::sync::Arc;

use russh::server::{self, Server};

use crate::{chat_server::ChatServer, room::Room};

mod chat_server;
mod client;
mod room;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = server::Config {
        keys: vec![russh::keys::load_secret_key("ssh_host_ed25519_key", None)?],
        ..Default::default()
    };

    let room = Arc::new(Room::new());
    let mut server = ChatServer::new(room);

    server
        .run_on_address(Arc::new(config), ("127.0.0.1", 2222))
        .await?;

    Ok(())
}
