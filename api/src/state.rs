use std::sync::Arc;
use cache::Cache;

#[derive(Clone)]
pub struct AppState {
   pub cache: Arc<dyn Cache>
}