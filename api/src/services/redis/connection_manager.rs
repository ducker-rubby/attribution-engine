use core::error;

use deadpool_redis::{Config, Runtime};

#[derive(Clone, Debug)]
pub struct RedisConnectionManager {
    pub pool: deadpool_redis::Pool,
}

impl RedisConnectionManager {
    pub fn build(url: &str) -> Result<Self, Box<dyn error::Error>> {
        let cfg = Config::from_url(url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;

        Ok(Self { pool })
    }

    pub async fn get_conn(&self) -> Result<deadpool_redis::Connection, Box<dyn error::Error>> {
        let conn = self.pool.get().await?;
        Ok(conn)
    }
}
