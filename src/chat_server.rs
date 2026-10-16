use std::{net::SocketAddr, sync::Arc};

use russh::server::{self};

use crate::{client::Client, room::Room};

pub struct ChatServer {
    room: Arc<Room>,
}

// server::Server는 정해진 trait만 정할 수 있음 그래서 따로 impl 뺏다.
impl ChatServer {
    pub fn new(room: Arc<Room>) -> Self {
        Self { room }
    }
}

impl server::Server for ChatServer {
    type Handler = Client;

    fn new_client(&mut self, peer: Option<SocketAddr>) -> Client {
        println!("새 연결: {peer:?}");
        Client::new(Arc::clone(&self.room))
    }
}
