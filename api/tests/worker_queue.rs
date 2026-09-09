use redis::streams::{StreamReadOptions, StreamReadReply};
use redis::{AsyncCommands, from_redis_value_ref};
use testcontainers_modules::{
    redis::{REDIS_PORT, Redis},
    testcontainers::runners::AsyncRunner,
};

use attribution_engine::{
    models::ClickEvent,
    services::redis::{RedisConnectionManager, RedisWorkerQueue},
};

//TODO: test that create consumer group successfully deals with BUSYGROUP error

//TODO: test RedirectCache

//TODO: change test to use RedisWorkerQueue::dequeue_event
#[tokio::test]
async fn test_redis_worker_queue() {
    let redis_instance = Redis::default()
        .start()
        .await
        .expect("Could not start Redis server");

    let host_ip = redis_instance
        .get_host()
        .await
        .expect("Could not get Redis host");
    let host_port = redis_instance
        .get_host_port_ipv4(REDIS_PORT)
        .await
        .expect("Could not get Redis port");

    let url = format!("redis://{}:{}", host_ip, host_port);

    let run_id = uuid::Uuid::now_v7();
    let stream_name = format!("teststream_{}", run_id);
    let consumer_group_name = format!("testgroup_{}", run_id);
    let test_link_id = "testlink";

    let queue = RedisWorkerQueue::default()
        .with_group(&stream_name, &consumer_group_name)
        .with_url(&url)
        .connect()
        .expect("Could not create RedisWorkerQueue");

    queue
        .create_consumer_group()
        .await
        .expect("Could not create consumer group");

    queue
        .enqueue_event(ClickEvent::build("testlink"))
        .await
        .expect("Could not enqueue event in Redis server");

    let connection_manager =
        RedisConnectionManager::build(&url).expect("Could not create connection manager");
    let mut conn = connection_manager
        .get_conn()
        .await
        .expect("Could not establish connection with Redis server");

    let opts = StreamReadOptions::default()
        .group(consumer_group_name, "consumer-1")
        .count(1);

    let results: StreamReadReply = conn
        .xread_options(&[stream_name], &[">"], &opts)
        .await
        .expect("Could not get StreamReadReply from Redis stream");

    let retrieved_link_id = results
        .keys
        .first()
        .and_then(|k| k.ids.first())
        .and_then(|i| i.map.get("link_id"))
        .map(|v| from_redis_value_ref::<String>(v).unwrap())
        .expect("Failed to locate link_id in stream payload");

    assert_eq!(test_link_id, retrieved_link_id);
}
