use sqlx::postgres::{PgPool, PgPoolOptions};

pub mod link;

pub struct PgPoolManager {
    pub pool: PgPool,
}

impl PgPoolManager {
    pub async fn new(connection_string: impl Into<String>) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string.into().as_str())
            .await?;

        Ok(Self { pool })
    }
}
