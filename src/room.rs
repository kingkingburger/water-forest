use russh::{ChannelId, server};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Member {
    handle: server::Handle,
    channel: ChannelId,
}

pub struct Room {
    members: Mutex<Vec<Member>>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            members: Mutex::new(Vec::new()),
        }
    }

    // 참가자 등록
    pub async fn join(&self, handle: server::Handle, channel: ChannelId) {
        let mut members = self.members.lock().await;

        members.push(Member { handle, channel });
    }

    pub async fn broadcast(&self, message: Vec<u8>) {
        // 전송 대상을 복사하고 잠금을 해제
        let members = {
            let members = self.members.lock().await;
            members.clone()
        };

        for member in members {
            if member
                .handle
                .data(member.channel, message.clone())
                .await
                .is_err()
            {
                eprintln!("참가자에게 메세지를 보내지 못했습니다");
            }
        }
    }
}
