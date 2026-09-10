use std::error::Error;

use crate::{
    models::Event,
    services::redis::{RedisConnectionManager, connection_manager},
};
use redis::{AsyncTypedCommands, ErrorKind};

#[derive(Clone, Debug)]
pub struct RedisWorkerQueue {
    consumer_group_name: String,
    stream_name: String,
    connection_manager: Option<RedisConnectionManager>,
    url: String,
}

impl RedisWorkerQueue {
    pub fn default() -> Self {
        RedisWorkerQueue {
            connection_manager: None,
            stream_name: "workerstream".into(),
            consumer_group_name: "workergroup".into(),
            url: "redis://127.0.0.1:6729".into(),
        }
    }

    pub fn with_group(
        mut self,
        stream_name: impl Into<String>,
        consumer_group_name: impl Into<String>,
    ) -> Self {
        self.stream_name = stream_name.into();
        self.consumer_group_name = consumer_group_name.into();
        self
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    pub fn connect(
        mut self,
        connection_manager: RedisConnectionManager,
    ) -> Result<Self, Box<dyn Error>> {
        // let connection_manager = RedisConnectionManager::build(&self.url)?;
        self.connection_manager = Some(connection_manager);

        Ok(self)
    }

    //TODO: make create_consumer_group an internal implementation of worker queue
    pub async fn create_consumer_group(&self) -> Result<(), Box<dyn Error>> {
        let mut conn = self
            .connection_manager
            .as_ref()
            .ok_or_else(|| "Connection manager missing".to_string())?
            .get_conn()
            .await?;

        let result = conn
            .xgroup_create_mkstream(&self.stream_name, &self.consumer_group_name, 0)
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
        let mut conn = self
            .connection_manager
            .as_ref()
            .ok_or_else(|| "Connection manager missing".to_string())?
            .get_conn()
            .await?;

        conn.xadd(&self.stream_name, "*", &event.get_metadata())
            .await?;

        Ok(())
    }

    //FIX: Change this from conn.get_int placeholder to an xread redis command
    pub async fn dequeue_event(&self) -> Result<(), Box<dyn Error>> {
        let mut conn = self
            .connection_manager
            .as_ref()
            .ok_or_else(|| "Connection manager missing".to_string())?
            .get_conn()
            .await?;

        let value = conn.get_int("test_key").await?;
        println!("{:?}", value);

        Ok(())
    }
}
