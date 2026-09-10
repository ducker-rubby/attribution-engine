use sqlx::postgres::PgPool;

use crate::models::Link;

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
}
