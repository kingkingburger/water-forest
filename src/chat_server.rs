use std::{net::SocketAddr, sync::Arc};

use russh::server::{self, Auth};

use crate::room::Room;

pub struct ChatServer {
    room: Arc<Room>,
}
pub struct Client {
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
        Client {
            room: Arc::clone(&self.room),
        }
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

    // 터미널 사용요청 수락
    async fn pty_request(
        &mut self,
        channel: russh::ChannelId,
        _term: &str,
        _col_width: u32,
        _row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _modes: &[(russh::Pty, u32)],
        session: &mut server::Session,
    ) -> Result<(), Self::Error> {
        session.channel_success(channel)?;
        Ok(())
    }

    // 채팅 시작화면 출력
    async fn shell_request(
        &mut self,
        channel: russh::ChannelId,
        session: &mut server::Session,
    ) -> Result<(), Self::Error> {
        session.channel_success(channel)?;
        self.room.join(session.handle(), channel).await;

        let welcome = concat!("\r\n", "===== 채팅방에 오신걸 환영합니다 =====", "\r\n");
        session.data(channel, welcome.as_bytes())?;
        Ok(())
    }

    // 사용자 데이터 보내주기
    async fn data(
        &mut self,
        channel: russh::ChannelId,
        data: &[u8],
        session: &mut server::Session,
    ) -> Result<(), Self::Error> {
        //Ctrl + c or Ctrl + D
        if data.contains(&3) || data.contains(&4) {
            session.data(channel, "\r\n 연결을 종료합니다".as_bytes())?;
            session.exit_status_request(channel, 0)?;
            session.eof(channel)?;
            session.close(channel)?;
            return Ok(());
        }

        let mut output = Vec::new();

        for &byte in data {
            match byte {
                b'\r' => output.extend_from_slice(b"\r\n> "),
                _ => output.push(byte),
            }
        }

        // session.data(channel, output)?;
        self.room.broadcast(output).await;
        Ok(())
    }
}
