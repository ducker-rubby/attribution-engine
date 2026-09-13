use sqlx::postgres::PgPool;

use crate::models::Link;
use crate::models::db::CreateLink;

pub struct LinkRepository {
    pool: PgPool,
}

impl LinkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Link, sqlx::Error> {
        let link = sqlx::query_as!(
            Link,
            /* sql */
            "
            SELECT
              id,
              redirect_url
            FROM
              link
            WHERE
              id = $1
            ",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        println!("{:#?}", link);

        Ok(link)
    }

    pub async fn insert_link(&self, link: CreateLink) -> Result<(), sqlx::Error> {
        sqlx::query!(
            /* sql */
            "
            INSERT INTO
              link (id, redirect_url, link_group_id)
            VALUES
              ($1, $2, $3)
            ",
            link.name,
            link.redirect_url,
            link.group_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
