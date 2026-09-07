use core::error;

use crate::models::Event;
use deadpool_redis::{Config, Runtime};
use redis::AsyncTypedCommands;

#[derive(Clone)]
pub struct RedisWorkerQueue {
    consumer_group_name: String,
    stream_name: String,
    pool: deadpool_redis::Pool,
}

impl RedisWorkerQueue {
    pub fn build(
        stream_name: &str,
        consumer_group_name: &str,
    ) -> Result<Self, Box<dyn error::Error>> {
        let cfg = Config::from_url("redis://127.0.0.1:6379/");
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;

        Ok(RedisWorkerQueue {
            pool,
            stream_name: stream_name.into(),
            consumer_group_name: consumer_group_name.into(),
        })
    }

    async fn get_conn(&self) -> Result<deadpool_redis::Connection, Box<dyn error::Error>> {
        let conn = self.pool.get().await?;
        Ok(conn)
    }

    pub async fn create_consumer_group(&self) -> Result<(), Box<dyn error::Error>> {
        let mut conn = self.get_conn().await?;
        conn.xgroup_create(&self.stream_name, &self.consumer_group_name, 0)
            .await?;
        Ok(())
    }

    pub async fn enqueue_event<'a>(&self, event: impl Event) -> Result<(), Box<dyn error::Error>> {
        let mut conn = self.get_conn().await?;
        conn.xadd(&self.stream_name, "*", &event.get_metadata())
            .await?;

        Ok(())
    }

    //FIX: Change this from conn.get_int placeholder to an xread redis command
    pub async fn dequeue_event(&self) -> Result<(), Box<dyn error::Error>> {
        let mut conn = self.get_conn().await?;
        let value = conn.get_int("test_key").await?;
        println!("{:?}", value);

        Ok(())
    }
}
