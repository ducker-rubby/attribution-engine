use crate::{handlers::analytics, services::redis::RedisWorkerQueue};
use axum::{Router, routing::get};

pub fn routes() -> Router<RedisWorkerQueue> {
    Router::new().route("/{id}", get(analytics::enqueue_click_event))
}
