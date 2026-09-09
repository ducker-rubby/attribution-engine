use sqlx::postgres::{PgPool, PgPoolOptions};

//TODO: implement postgres struct
pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub async fn build(connection_string: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await?;

        let query = /* sql */ "
            SELECT
              VERSION()
        ";

        let row: (String,) = sqlx::query_as(query).fetch_one(&pool).await?;
        println!("{:?}", row);

        Ok(Self { pool })
    }
}
