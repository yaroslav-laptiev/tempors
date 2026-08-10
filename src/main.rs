use sqlx::{PgPool, Postgres};

use crate::{config::Config, db::Database};

mod api;
mod types;
mod db;
mod config;


#[tokio::main]
async fn main() {

    let config = Config::from_env().expect("[INIT]: Failed to read the config");

    let pool = PgPool::connect_lazy(&config.database_url).expect("[INIT]: Failed to connect to DB");

    let db = Database::new(pool);
}
