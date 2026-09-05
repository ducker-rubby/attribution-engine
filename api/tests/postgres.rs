use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};

#[tokio::test]
async fn test_postgres() {
    let postgres_instance = Postgres::default()
        .with_db_name("attributiondb")
        .with_user("postgres")
        .with_password("admin")
        .start()
        .await
        .unwrap();

    let host_ip = postgres_instance.get_host().await.unwrap();
    let host_port = postgres_instance.get_host_port_ipv4(5432).await.unwrap();

    let connection_string = format!("postgres://postgres:postgres@{host_ip}:{host_port}/postgres");
}
