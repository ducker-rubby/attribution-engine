use redis::Commands;
use testcontainers_modules::{
    redis::{REDIS_PORT, Redis},
    testcontainers::runners::AsyncRunner,
};

#[tokio::test]
async fn test_redis() {
    let redis_instance = Redis::default().start().await.unwrap();
    let host_ip = redis_instance.get_host().await.unwrap();
    let host_port = redis_instance.get_host_port_ipv4(REDIS_PORT).await.unwrap();

    let url = format!("redis://{host_ip}:{host_port}");
    let client = redis::Client::open(url.as_ref()).unwrap();
    let mut conn = client.get_connection().unwrap();

    conn.set::<_, _, ()>("testkey", 42).unwrap();
    let result: i64 = conn.get("testkey").unwrap();

    assert_eq!(result, 42);
}
