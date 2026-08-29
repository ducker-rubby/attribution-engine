use axum::extract::Path;

use crate::models::Click;

pub async fn foo() {
    println!("FOO")
}

pub async fn enqueue_click_event(Path(id): Path<String>) {
    println!("Click uploaded");
    let click = Click::build(&id);

    println!("{:?}", click);
}
