use crate::handlers::analytics;
use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route(
        "/{click_ref}/{conversion_type}",
        get(analytics::enqueue_click_event),
    )
}
