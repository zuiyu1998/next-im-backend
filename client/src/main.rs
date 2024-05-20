use abi::{
    pb::message::{msg::Union, ChatMsg, ChatMsgType, Msg, Platfrom},
    stream::{tcp::TcpStream, MessageStream},
    tokio::{self, net::TcpSocket},
    tracing, tracing_subscriber,
};

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string())
        .parse()
        .unwrap();

    let socket = TcpSocket::new_v4()?;

    let stream = socket.connect(addr).await?;

    tracing::info!("server running on {}", addr);

    let stream = TcpStream::new_platform(stream, Platfrom::Windows);

    let chat_msg = ChatMsg {
        local_id: "0".to_string(),
        server_id: 0.to_string(),
        receiver_id: 0,
        sender_id: 0,
        create_at: 0,
        seq_id: 0,
        msg_type: ChatMsgType::Text as i32,
        content: vec![],
    };

    stream
        .send(&Msg {
            union: Some(Union::ChatMsg(chat_msg)),
        })
        .await?;

    loop {}
}
