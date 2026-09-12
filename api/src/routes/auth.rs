//Crud routes for authentication
use crate::{AppState, handlers::auth};
use axum::{Router, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth::login))
        .route("/create-account", post(auth::create_account))
}
