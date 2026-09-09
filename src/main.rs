use sqlx::{PgPool, Postgres};

use crate::{config::Config, db::Database};

mod api;
mod common;
mod config;
mod db;
mod queuer;
mod types;

#[tokio::main]
async fn main() {
    let config = Config::from_env().expect("[INIT]: Failed to read the config");

    let pool = PgPool::connect_lazy(&config.database_url).expect("[INIT]: Failed to connect to DB");

    let db = Database::new(pool);
}
