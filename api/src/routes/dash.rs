//Crud routes for dashboard
use crate::{handlers::analytics, services::redis::RedisWorkerQueue};
use axum::{Router, routing::get};

pub fn routes() -> Router<RedisWorkerQueue> {
    Router::new().route("/", get(analytics::foo))
}
