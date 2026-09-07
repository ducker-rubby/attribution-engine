use crate::{handlers::analytics, services::redis::RedisWorkerQueue};
use axum::{Router, routing::get};

pub fn routes() -> Router<RedisWorkerQueue> {
    Router::new().route(
        "/{click_ref}/{conversion_type}",
        get(analytics::enqueue_click_event),
    )
}
