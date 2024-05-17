use abi::{
    pb::message::{Msg, Platfrom},
    tokio::time::timeout,
    tonic::async_trait,
};

use crate::{Kind, Result};

#[async_trait]
pub trait MessageStream: 'static + Send + Sync {
    fn get_platfrom(&self) -> Platfrom;

    async fn next(&self) -> Result<Option<Msg>>;

    async fn send(&self, msg: &Msg) -> Result<Option<Msg>>;

    async fn next_ms(&self, ms: u64) -> Result<Option<Msg>> {
        let duration = std::time::Duration::from_millis(ms);

        let res = timeout(duration, self.next())
            .await
            .map_err(|_| Kind::Timeout)?;

        res
    }
}
