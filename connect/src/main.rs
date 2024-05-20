use abi::{
    stream::{tcp::TcpStream, MessageStream},
    tokio::{self, net::TcpListener},
    tracing, tracing_subscriber,
};
use connect::manager::Manager;

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();

    let _manager = Manager::new().await;

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());

    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("server running on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        // Spawn our handler to be run asynchronously.
        tokio::spawn(async move {
            //首次连接登录
            tracing::debug!("accepted connection");

            let connect = TcpStream::new(stream);

            match connect.next_ms(10000).await {
                Ok(msg) => match msg {
                    None => {
                        tracing::error!("login msg error: not found");
                    }
                    Some(msg) => {
                        tracing::info!("login msg: {:?}", msg);
                    }
                },
                Err(e) => {
                    tracing::error!("login msg error: {}", e);
                }
            }
        });
    }
}
