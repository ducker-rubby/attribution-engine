use anyhow;
use attribution_engine::AppState;
use attribution_engine::router;
use attribution_engine::services::postgres::{PgPoolManager, link::LinkRepository};
use attribution_engine::services::redis::{
    RedirectCache, RedisConnectionManager, RedisWorkerQueue,
};

pub async fn run() -> anyhow::Result<()> {
    //TODO: add error handling

    // let worker_queue = RedisWorkerQueue::build("clickstream", "clickgroup")
    //     .expect("Failed to initialize Redis worker queue");

    //TODO: change connection string to env variable
    let connection_string = format!(
        "postgres://postgres:{}@localhost:5433/{}",
        "admin", "attributiondb"
    );

    let postgres_pool_manager = PgPoolManager::new(&connection_string).await.unwrap();

    let link_repo = LinkRepository::new(postgres_pool_manager.pool);

    // link_repo.get_by_id("test").await.unwrap();

    let connection_manager = RedisConnectionManager::build("redis://127.0.0.1:6379")
        .expect("Could not make redis connection manager");

    let worker_queue = RedisWorkerQueue::default()
        .with_group("clickstream", "clickgroup")
        .connect(connection_manager.clone())
        .expect("Could not create RedisWorkerQueue");

    worker_queue
        .create_consumer_group()
        .await
        .unwrap_or_else(|err| {
            eprintln!("Could not create consumer group, {err}");
            panic!()
        });

    let redirect_cache = RedirectCache::new(connection_manager.clone())
        .expect("Failed to initialze Redis redirect cache");

    let redirects: [(&str, &str); 3] = [
        ("cat", "https://www.google.com?q=cats"),
        ("dog", "https://www.google.com?q=dogs"),
        ("bat", "https://www.google.com?q=bats"),
    ];

    redirect_cache
        .add_redirects(&redirects)
        .await
        .expect("Could not add redirects");

    let redirect = redirect_cache.get_redirect("dog").await.unwrap();

    println!("{:?}", redirect);

    let app_state = AppState::build(redirect_cache, worker_queue);

    let app = router::build_axum_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
