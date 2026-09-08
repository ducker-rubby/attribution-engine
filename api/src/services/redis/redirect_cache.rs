use std::error;

use crate::services::redis::RedisConnectionManager;
use redis::AsyncTypedCommands;

#[derive(Debug)]
pub struct Redirect {
    pub link_id: String,
    pub redirect_url: String,
}

impl Redirect {
    pub fn build(link_id: &str, redirect_url: &str) -> Self {
        Redirect {
            link_id: link_id.into(),
            redirect_url: redirect_url.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RedirectCache {
    connection_manager: RedisConnectionManager,
}

impl RedirectCache {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let connection_manager = RedisConnectionManager::new()?;
        Ok(Self { connection_manager })
    }

    pub async fn add_redirects(
        &self,
        redirects: &[(&str, &str)],
    ) -> Result<(), Box<dyn error::Error>> {
        let mut conn = self.connection_manager.get_conn().await?;
        conn.mset(redirects).await?;

        Ok(())
    }

    //TODO: Change this to return a Option instead of erroring on None
    pub async fn get_redirect(&self, link_id: &str) -> Result<Redirect, Box<dyn error::Error>> {
        let mut conn = self.connection_manager.get_conn().await?;
        let redirect = conn
            .get(link_id)
            .await?
            .ok_or_else(|| format!("Redirect link '{}' not found", link_id))?;

        Ok(Redirect::build(link_id, &redirect))
    }
}
