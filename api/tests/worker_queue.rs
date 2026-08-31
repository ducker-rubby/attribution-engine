use testcontainers::{
    GenericImage,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};

#[tokio::test]
async fn test_redis() {
    let _container = GenericImage::new("redis", "8.8.2")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connection"))
        .start()
        .await
        .unwrap();
}
