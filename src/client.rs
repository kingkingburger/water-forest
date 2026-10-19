use std::sync::Arc;

use russh::server::{self, Auth};

use crate::room::Room;

pub struct Client {
    room: Arc<Room>,
    input_buffer: Vec<u8>,
}

impl Client {
    pub fn new(room: Arc<Room>) -> Self {
        Self {
            room,
            input_buffer: Vec::new(),
        }
    }
}

impl server::Handler for Client {
    type Error = russh::Error;

    // 사용자 데이터 보내주기, 데이터 수신될 때 마다 호출
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

        // 채팅 모으기, 발송
        for &byte in data {
            match byte {
                b'\r' => {
                    let mut message = std::mem::take(&mut self.input_buffer);
                    message.extend_from_slice(b"\r\n> ");

                    self.room.broadcast(message).await;
                }
                _ => {
                    // 자기 자신에게 보내기
                    session.data(channel, vec![byte])?;
                    self.input_buffer.push(byte);
                }
            }
        }

        Ok(())
    }

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
}
