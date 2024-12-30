use abi::{
    config::Config,
    message::{tcp::TcpMessageListener, MessageListener},
    tokio::{self, sync::mpsc},
    tracing,
};

use crate::{manager::Manager, rpc::MsgRpcService, Result};

pub struct TcpServer;

impl TcpServer {
    pub async fn start(config: Config) -> Result<()> {
        let (tx, rx) = mpsc::channel(1024);
        let hub = Manager::new(&config, tx).await;

        let mut cloned_hub = hub.clone();

        tokio::spawn(async move {
            cloned_hub.run(rx).await;
        });

        let config_clone = config.clone();
        let cloned_hub = hub.clone();

        tokio::spawn(async move {
            // start rpc server
            MsgRpcService::start(cloned_hub, &config_clone)
                .await
                .unwrap();
        });

        let mut listener = TcpMessageListener::new(config.msg_server.url());
        listener.listen().await?;

        loop {
            let message = listener.accept().await?;

            let mut cloned_hub = hub.clone();

            tokio::spawn(async move {
                if let Err(e) = cloned_hub.handle_message(message).await {
                    tracing::error!("hub handle error: {}", e);
                };
            });
        }
    }
}
