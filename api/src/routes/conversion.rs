use crate::AppState;
use crate::handlers::analytics;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new().route(
        "/{click_ref}/{conversion_type}",
        get(analytics::enqueue_click_event),
    )
}
