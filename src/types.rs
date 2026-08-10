use serde_json::Value;
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};
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

#[derive(Clone, Debug)]
pub struct TEvent {
    pub id: String,
    pub func_id: String,
    pub payload: HashMap<String, Value>,
}
#[derive(Clone, Debug)]
pub struct TFunction {
    pub id: String,
    pub payload: HashMap<String, Value>,
}
#[derive(Clone, Debug)]
pub enum TEventRunStatus {
    Queued,
    Running,
    Failed,
    Completed,
    Canceled,
}

pub struct TEventRun {
    pub id: String,
    pub event_id: String,
    pub payload: HashMap<String, Value>,
    pub status: TEventRunStatus,
}
