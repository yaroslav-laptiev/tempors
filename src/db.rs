use std::result;

use sqlx::{PgConnection, PgPool};
use uuid::Timestamp;

use crate::types::{self, TEvent, TEventHandler, TEventRun, TEventStatus, TFunction};

type DbErr = Box<dyn std::error::Error + Send + Sync>;

type DbResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type InsertResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
type ReturningResult<T> = Result<T, DbErr>;
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

    pub async fn insert_event_handler(
        &self,
        event_handler: TEventHandler,
    ) -> InsertResult<TEventHandler> {
        let result = sqlx::query_as::<_, TEventHandler>(
            "INSERT INTO tempors.event_handlers (id, name) VALUES ($1, $2) RETURNING *",
        )
        .bind(event_handler.id)
        .bind(event_handler.name)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn insert_event(&self, event: TEvent) -> InsertResult<TEvent> {
        let result = sqlx::query_as::<_, TEvent>(
            "INSERT INTO tempors.events (id, func_id, payload) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(uuid::Uuid::now_v7().to_string())
        .bind(event.func_id)
        .bind(event.payload)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn insert_function(&self, event_run: types::TEventRun) -> InsertResult<TFunction> {
        let result = sqlx::query_as::<_, TFunction>("INSERT INTO tempors.event_runs (id, event_id, payload) VALUES ($1, $2, $3) RETURNING *")
            .bind(uuid::Uuid::now_v7().to_string())
            .bind(event_run.event_id)
            .bind(event_run.payload)
            .fetch_one(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn get_events(
        &self,
        opts: &dbopts::SelectEvents,
    ) -> Result<Vec<TEvent>, Box<dyn std::error::Error + Send + Sync>> {
        let res: Vec<TEvent> = sqlx::query_as::<_, TEvent>(
            "select * from tempors.events e WHERE e.status = $1 limit $2 offset $3;",
        )
        .bind(opts.status.unwrap_or(types::TEventStatus::Pending))
        .bind(opts.page_size as i16)
        .bind((opts.page_size * opts.page) as i16)
        .fetch_all(&self.pool)
        .await?;

        Ok(res)
    }

    pub async fn get_events_for_update(
        &self,
        opts: &dbopts::SelectEvents,
    ) -> Result<Vec<TEvent>, Box<dyn std::error::Error + Send + Sync>> {
        let res: Vec<TEvent> = sqlx::query_as::<_, TEvent>(
            "select * from tempors.events e WHERE e.status = $1 limit $2 FOR UPDATE",
        )
        .bind(opts.status.unwrap_or(types::TEventStatus::Pending))
        .bind(opts.page_size as i16)
        .fetch_all(&self.pool)
        .await?;

        Ok(res)
    }

    pub async fn claim_events(&self) -> ReturningResult<Vec<TEvent>> {
        let res: Vec<TEvent> = sqlx::query_as::<_, TEvent>("select * from tempors.events e WHERE e.status = 'pending' order by e.created_at for update skip locked limit 10")
        .fetch_all(&self.pool)
        .await?;

        Ok(res)
    }

    pub async fn get_event_runs(
        &self,
        opts: &dbopts::SelectEventRuns,
    ) -> Result<Vec<TEventRun>, Box<dyn std::error::Error + Send + Sync>> {
        let res: Vec<TEventRun> = sqlx::query_as::<_, TEventRun>(
            "select * from tempors.event_runs er WHERE er.status = $1 LIMIT $2 offset $3;",
        )
        .bind(opts.status.unwrap_or(types::TEventRunStatus::Queued))
        .bind(opts.page_size as i16)
        .bind(opts.page as i16)
        .fetch_all(&self.pool)
        .await?;
        Ok(res)
    }

    pub async fn update_event_status(&self, event: TEvent) -> InsertResult<TEvent> {
        let res = sqlx::query_as::<_, TEvent>(
            "UPDATE tempors.events SET status=$1 WHERE e.id = $2 RETURNING *;",
        )
        .bind(event.status)
        .bind(event.id)
        .fetch_one(&self.pool)
        .await?;
        Ok(res)
    }

    pub async fn update_event_run_status(&self, event: TEventRun) -> InsertResult<TEventRun> {
        let res = sqlx::query_as::<_, TEventRun>(
            "UPDATE tempors.event_runs SET status=$1 WHERE e.id = $2 RETURNING *;",
        )
        .bind(event.status)
        .bind(event.id)
        .fetch_one(&self.pool)
        .await?;
        Ok(res)
    }

    // pub async fn batch_get_event_runs(&self) -> Result<Vec<types::TEvent>, Box<dyn std::error::Error + Send + Sync>> {
    //     let result = sqlx::query("SELECT * from tempors.event_runs er WHERE er.status = 'pending'").fetch_all::<types::TEventRun>(executor);
    //     Ok(Vec::new());
    // }
}

pub mod dbopts {
    use crate::types::{TEventRunStatus, TEventStatus};

    pub struct SelectEvents {
        pub page: usize,
        pub page_size: usize,
        pub status: Option<TEventStatus>,
    }

    pub struct SelectEventRuns {
        pub page: usize,
        pub page_size: usize,
        pub status: Option<TEventRunStatus>,
    }
}
