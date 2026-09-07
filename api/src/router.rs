use crate::{routes, services::redis::RedisWorkerQueue};
use axum::Router;

pub fn build_axum_router(queue: RedisWorkerQueue) -> Router {
    Router::new()
        .merge(routes::redirect::routes())
        .nest("/dash", routes::dash::routes())
        .nest("/conversion", routes::conversion::routes())
        .with_state(queue)
}
