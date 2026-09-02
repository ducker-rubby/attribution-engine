use core::error;

use crate::models::Event;
use deadpool_redis::{Config, Runtime};
use redis::AsyncTypedCommands;

pub struct RedisWorkerQueue {
    pool: deadpool_redis::Pool,
}

impl RedisWorkerQueue {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let cfg = Config::from_url("redis://127.0.0.1:6379/");
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;

        Ok(RedisWorkerQueue { pool })
    }

    //TODO: Check if this method is necessary
    async fn connect(&self) -> Result<deadpool_redis::Connection, Box<dyn error::Error>> {
        let conn = self.pool.get().await?;
        Ok(conn)
    }

    pub async fn enqueue_event<'a>(
        &self,
        stream: &str,
        event: impl Event,
    ) -> Result<(), Box<dyn error::Error>> {
        let mut conn = self.connect().await?;
        conn.xadd(stream, "*", &event.get_metadata()).await?;

        Ok(())
    }

    //FIX: Change this from conn.get_int placeholder to an xread redis command
    pub async fn dequeue_event(&self) -> Result<(), Box<dyn error::Error>> {
        let mut conn = self.connect().await?;
        let value = conn.get_int("test_key").await?;
        println!("{:?}", value);

        Ok(())
    }
}

pub async fn connect_redis() -> redis::RedisResult<()> {
    let queue = RedisWorkerQueue::new().unwrap_or_else(|err| {
        eprintln!("Error connecting to redis server {}", err);
        std::process::exit(1)
    });

    // queue.enqueue_event().await.unwrap();
    // queue.dequeue_event().await.unwrap();

    Ok(())
}
