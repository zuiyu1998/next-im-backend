use abi::{
    stream::tcp::TcpStream,
    tokio::{self, net::TcpListener},
    tracing::{self, Level},
    tracing_subscriber,
};
use connect::{manager::Manager, session::Session};

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let manager = Manager::new().await;

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());

    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("server running on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        let manager = manager.clone();

        // Spawn our handler to be run asynchronously.
        tokio::spawn(async move {
            //首次连接登录
            tracing::debug!("accepted connection");

            let stream = TcpStream::new(stream);

            let session = Session::new(stream);

            if let Err(e) = session.run(manager).await {
                tracing::error!("session error: {}", e);
            }
        });
    }
}
