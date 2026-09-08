use std::error::Error;

use crate::{models::Event, services::redis::RedisConnectionManager};
use redis::{AsyncTypedCommands, ErrorKind};

#[derive(Clone, Debug)]
pub struct RedisWorkerQueue {
    consumer_group_name: String,
    stream_name: String,
    connection_manager: RedisConnectionManager,
}

impl RedisWorkerQueue {
    pub fn build(stream_name: &str, consumer_group_name: &str) -> Result<Self, Box<dyn Error>> {
        let connection_manager = RedisConnectionManager::new()?;

        Ok(RedisWorkerQueue {
            connection_manager,
            stream_name: stream_name.into(),
            consumer_group_name: consumer_group_name.into(),
        })
    }

    pub async fn create_consumer_group(&self) -> Result<(), Box<dyn Error>> {
        let mut conn = self.connection_manager.get_conn().await?;
        let result = conn
            .xgroup_create(&self.stream_name, &self.consumer_group_name, 0)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                if err.kind() == ErrorKind::Extension && err.to_string().contains("BUSYGROUP") {
                    Ok(())
                } else {
                    Err(err.into())
                }
            }
        }
    }

    pub async fn enqueue_event(&self, event: impl Event) -> Result<(), Box<dyn Error>> {
        let mut conn = self.connection_manager.get_conn().await?;
        conn.xadd(&self.stream_name, "*", &event.get_metadata())
            .await?;

        Ok(())
    }

    //FIX: Change this from conn.get_int placeholder to an xread redis command
    pub async fn dequeue_event(&self) -> Result<(), Box<dyn Error>> {
        let mut conn = self.connection_manager.get_conn().await?;
        let value = conn.get_int("test_key").await?;
        println!("{:?}", value);

        Ok(())
    }
}
