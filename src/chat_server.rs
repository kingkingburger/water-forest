use std::net::SocketAddr;

use russh::server::{self, Auth};

pub struct ChatServer;
pub struct Client;

impl server::Server for ChatServer {
    type Handler = Client;

    fn new_client(&mut self, peer: Option<SocketAddr>) -> Client {
        println!("새 연결: {peer:?}");
        Client
    }
}

impl server::Handler for Client {
    type Error = russh::Error;

    async fn auth_none(&mut self, user: &str) -> Result<Auth, Self::Error> {
        println!("접속 허용: {user}");
        Ok(Auth::Accept)
    }

    async fn channel_open_session(
        &mut self,
        _channel: russh::Channel<server::Msg>,
        reply: server::ChannelOpenHandle,
        _session: &mut server::Session,
    ) -> Result<(), Self::Error> {
        reply.accept().await;
        Ok(())
    }
}
