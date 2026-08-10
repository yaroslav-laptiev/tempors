use sqlx::{PgPool, PgConnection};

pub struct Database {
    pool: PgPool
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Database {
            pool
        }
    }
}


