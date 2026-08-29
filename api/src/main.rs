use attribution_engine::router;

#[tokio::main]
async fn main() {
    println!("Hello world");

    let app = router::build_axum_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
