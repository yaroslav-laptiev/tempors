use serde_json::{Map, Value};
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use std::fmt::Debug;
use thiserror::Error;
/*
    Error abstraction to be used accross the app
*/
#[derive(Debug, Error)]
pub enum TError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("internal error: {0}")]
    Internal(String),

    #[error("non retriable error: {0}")]
    NonRetraible(String),
}

#[derive(Clone, Copy, Debug, sqlx::Type)]
#[sqlx(type_name = "tempors.event_status", rename_all = "lowercase")]
pub enum TEventStatus {
    Pending,
    Processed,
}

#[derive(Clone, Copy, Debug, sqlx::Type)]
#[sqlx(type_name = "tempors.event_run_status", rename_all = "lowercase")]
pub enum TEventRunStatus {
    Queued,
    Running,
    Failed,
    Completed,
    Canceled,
}

// TODO: Add triggers, for now all EventHandlers act like they have event trigger
// trait TTrigger {}

// #[derive(Clone, Debug)]
// pub struct EventTrigger {
//     event_name: String
// }

// #[derive(Clone, Debug)]
// pub struct CronTrigger {
//     expression: String
// }

// impl EventTrigger {
//      pub fn new(event_name: String) -> Self {
//         EventTrigger { event_name }
//     }
// }

// impl TTrigger for EventTrigger {}

// impl CronTrigger {
//      pub fn new(expression: String) -> Self {
//         CronTrigger { expression }
//     }
// }

// impl TTrigger for CronTrigger {}

#[derive(Clone, Debug, FromRow)]
pub struct TEventHandler {
    pub id: String,
    pub name: String,
    pub event_id: uuid::Uuid,
}

#[derive(Clone, Debug, FromRow)]
pub struct TEvent {
    pub id: uuid::Uuid,
    pub func_id: String,
    pub payload: Json<Map<String, Value>>,
    pub status: TEventStatus,
}
#[derive(Clone, Debug, FromRow)]
pub struct TFunction {
    pub id: uuid::Uuid,
    pub payload: Json<Map<String, Value>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct TEventRun {
    pub id: uuid::Uuid,
    pub event_id: String,
    pub payload: Json<Map<String, Value>>,
    pub status: TEventRunStatus,
}
