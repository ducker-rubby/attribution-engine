use anyhow;
use attribution_engine::AppState;
use attribution_engine::router;
use attribution_engine::services::redis::{RedirectCache, RedisWorkerQueue};

pub async fn run() -> anyhow::Result<()> {
    //TODO: add error handling
    // let worker_queue = RedisWorkerQueue::build("clickstream", "clickgroup")
    //     .expect("Failed to initialize Redis worker queue");

    let worker_queue = RedisWorkerQueue::default()
        .with_group("clickstream", "clickgroup")
        .connect()
        .expect("Could not create RedisWorkerQueue");

    worker_queue.create_consumer_group().await.unwrap();

    let redirect_cache = RedirectCache::new().expect("Failed to initialze Redis redirect cache");

    let redirects: [(&str, &str); 3] = [
        ("cat", "https://www.google.com?q=cats"),
        ("dog", "https://www.google.com?q=dogs"),
        ("bat", "https://www.google.com?q=bats"),
    ];

    redirect_cache.add_redirects(&redirects).await.unwrap();

    let redirect = redirect_cache.get_redirect("dog").await.unwrap();

    println!("{:?}", redirect);

    let app_state = AppState::build(redirect_cache, worker_queue);

    let app = router::build_axum_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
