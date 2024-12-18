use std::sync::Arc;
use abi::config::Config;
use cache::Cache;

#[derive(Clone)]
pub struct AppState {
   pub cache: Arc<dyn Cache>,
   pub config: Arc<Config>
}