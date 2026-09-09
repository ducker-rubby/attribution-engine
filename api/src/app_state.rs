use crate::services::redis::{RedirectCache, RedisWorkerQueue};

//TODO: move app state to appropriate module
#[derive(Clone)]
pub struct AppState {
    pub redirect_cache: RedirectCache,
    pub worker_queue: RedisWorkerQueue,
}

impl AppState {
    pub fn build(redirect_cache: RedirectCache, worker_queue: RedisWorkerQueue) -> Self {
        Self {
            redirect_cache,
            worker_queue,
        }
    }
}
