//Crud routes for dashboard
use crate::{AppState, handlers::analytics};
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(analytics::foo))
}
