use axum::extract::Path;

use crate::models::ClickEvent;

pub async fn foo() {
    println!("FOO")
}

pub async fn enqueue_click_event(Path(id): Path<String>) {
    println!("Click uploaded");
    let click = ClickEvent::build(&id);

    println!("{:?}", click);
}

pub async fn enqueue_conversion_event(
    Path(click_ref): Path<String>,
    Path(conversion_type): Path<String>,
) {
}
