use axum::{
    extract::{Path, State},
    response::Redirect,
};

use crate::{AppState, models::ClickEvent};

//TODO: rething handler file convetion (analytics.rs has two handlers?)

pub async fn foo() {
    println!("FOO")
}

pub async fn enqueue_click_event(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Redirect {
    println!("Click uploaded");
    let click = ClickEvent::build(id.as_str());

    println!("{:?}", click);

    app_state.worker_queue.enqueue_event(click).await.unwrap();

    let result = app_state.redirect_cache.get_redirect(&id).await.unwrap();

    Redirect::to(&result.redirect_url)
}

pub async fn enqueue_conversion_event(
    Path(_click_ref): Path<String>,
    Path(_conversion_type): Path<String>,
) {
    unimplemented!()
}
