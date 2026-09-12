use crate::AppState;
use crate::routes;
use axum::Router;

//TODO: Are route files too sparse?
pub fn build_axum_router(app_state: AppState) -> Router {
    Router::new()
        .merge(routes::redirect::routes())
        .nest("/dash", routes::dash::routes())
        .nest("/conversion", routes::conversion::routes())
        .nest("/auth", routes::auth::routes())
        .with_state(app_state)
}
