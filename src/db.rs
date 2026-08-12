use sqlx::{PgConnection, PgPool};
use uuid::Timestamp;

use crate::types;

type DbResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Database { pool }
    }

    pub async fn migrate(&self) -> DbResult {
        sqlx::migrate!("./migrations").run(&self.pool).await?;

        Ok(())
    }

    pub async fn insert_event(&self, event: types::TEvent) -> DbResult {
        sqlx::query("INSERT INTO tempors.events (id, func_id, payload) VALUES ($1, $2, $3)")
            .bind(event.id)
            .bind(event.func_id)
            .bind(event.payload)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn insert_function(&self, event_run: types::TEventRun) -> DbResult {
        sqlx::query("INSERT INTO tempors.event_runs (id, event_id, payload) VALUES ($1, $2, $3)")
            .bind(uuid::Uuid::now_v7().to_string())
            .bind(event_run.event_id)
            .bind(event_run.payload)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn batch_get_event_runs(&self) -> Result<Vec<types::TEvent>, Box<dyn std::error::Error + Send + Sync>> {
        let result = sqlx::query("SELECT * from tempors.event_runs er WHERE er.status = 'pending'").fetch_all::<types::TEventRun>(executor)
    }
}
