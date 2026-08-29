use crate::routes;
use axum::{Router, routing::get};

pub fn build_axum_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world" }))
        .merge(routes::redirect::routes())
        .nest("/dash", routes::dash::routes())
}
