use anyhow;
use attribution_engine::router;
use attribution_engine::services::redis::RedisWorkerQueue;

pub async fn run() -> anyhow::Result<()> {
    //TODO: add error handling
    let worker_queue = RedisWorkerQueue::build("clickstream", "clickgroup")
        .expect("Failed to initialize Redis worker queue");

    worker_queue.create_consumer_group().await.unwrap();

    let app = router::build_axum_router(worker_queue);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
