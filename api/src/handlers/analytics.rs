use axum::{
    extract::{Path, State},
    response::Redirect,
};

use crate::models::ClickEvent;
use crate::services::redis::RedisWorkerQueue;

pub async fn foo() {
    println!("FOO")
}

pub async fn enqueue_click_event(
    State(queue): State<RedisWorkerQueue>,
    Path(id): Path<String>,
) -> Redirect {
    println!("Click uploaded");
    let click = ClickEvent::build(id.as_str());

    println!("{:?}", click);

    queue.enqueue_event(click).await.unwrap();

    Redirect::to("https://www.google.com")
}

pub async fn enqueue_conversion_event(
    Path(click_ref): Path<String>,
    Path(conversion_type): Path<String>,
) {
    unimplemented!()
}
