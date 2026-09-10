//Crud routes for authentication
use crate::{AppState, handlers::auth};
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(auth::foo))
}
