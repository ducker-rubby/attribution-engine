use attribution_engine::router;

use attribution_engine::services::redis;

#[tokio::main]
async fn main() {
    println!("Hello world");

    redis::connect_redis().await.unwrap();

    let app = router::build_axum_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}
