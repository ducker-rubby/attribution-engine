//Crud routes for dashboard
use crate::handlers::analytics;
use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/", get(analytics::foo))
}
