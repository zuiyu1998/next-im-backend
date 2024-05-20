use tonic::async_trait;

pub struct Config;

#[async_trait]
pub trait FromConfig {
    async fn from_conifg(&self, conifg: &Config) -> Self;
}
