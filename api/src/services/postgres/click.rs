use sqlx::postgres::PgPool;

pub struct ClickRepository {
    pool: PgPool,
}

//TODO: implement click repository
impl ClickRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn insert_click() {
        unimplemented!()
    }
}
