use std::{collections::HashMap, fmt::format};

use serde_json::Value;

use crate::types::{self, TError, TEvent, TEventRun, TFunction};

struct DummyDB {
    pub events: Vec<TEvent>,
    pub functions: Vec<TFunction>,
    pub event_runs: Vec<TEventRun>,
}

impl DummyDB {
    pub fn new() -> Self {
        DummyDB {
            events: Vec::new(),
            functions: Vec::new(),
            event_runs: Vec::new(),
        }
    }
}

pub struct TemporsApi {
    db: DummyDB,
}

impl TemporsApi {
    pub fn new() -> Self {
        let mut db = DummyDB::new();
        TemporsApi { db }
    }
}

impl TemporsApi {
    pub fn register_event(&mut self, event: TEvent) -> Result<(), TError> {
        self.db.events.insert(0, event);
        Ok(())
    }

    pub fn fire_event(&mut self, data: FireEventPayload) -> Result<(), TError> {
        // lookup event

        let mut event: Option<TEvent> = None;

        for e in &self.db.events {
            if e.id == data.event_id {
                let b = event = Some(e.clone());
                break;
            }
        }

        match event {
            Some(_) => {}
            None => {
                return Err(TError::NotFound("Event not found".to_string()));
            }
        }

        // lookup associated func

        let mut func: Option<TFunction> = None;

        for f in &self.db.functions {
            if f.id == event.clone().unwrap().func_id {
                func = Some(f.clone())
            }
        }

        match func {
            Some(_) => {}
            None => {
                return Err(TError::NotFound("func not found".to_string()));
            }
        }

        let event_run = TEventRun {
            id: format!("event-run-{}", self.db.event_runs.len()),
            event_id: event.clone().unwrap().id,
            payload: event.clone().unwrap().payload,
            status: types::TEventRunStatus::Queued,
        };

        self.db.event_runs.insert(0, event_run);

        Ok(())
    }
}

//todo move or remove
pub struct FireEventPayload {
    pub event_id: String,
    pub payload: HashMap<String, Value>,
}

// 2 queues:
// EventsQueue - all created events are getting processed by worker
// EventRunsQueue - processing EventsQueue == creating new EventRuns
