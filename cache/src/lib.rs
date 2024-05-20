use std::sync::Arc;

pub struct TestCache;

impl Cache for TestCache {}

pub trait Cache: 'static + Send + Sync {}

pub fn get_cache() -> Arc<dyn Cache> {
    Arc::new(TestCache)
}
