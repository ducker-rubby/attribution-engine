use sqlx::postgres::PgPool;

//TODO: implement link group repository
pub struct LinkGroupRepository {
    pool: PgPool,
}

impl LinkGroupRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn get_by_id() {
        unimplemented!()
    }

    pub fn insert_link_group() {
        unimplemented!()
    }

    pub fn update_link_group() {
        unimplemented!()
    }
}
