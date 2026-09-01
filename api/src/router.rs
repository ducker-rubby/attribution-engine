use crate::routes;
use axum::Router;

pub fn build_axum_router() -> Router {
    Router::new()
        .merge(routes::redirect::routes())
        .nest("/dash", routes::dash::routes())
        .nest("/conversion", routes::conversion::routes())
}
