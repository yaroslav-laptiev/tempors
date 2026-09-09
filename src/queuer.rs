use core::error;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use crate::common::{Queue, Stack};
use crate::db::{Database, dbopts};
use crate::types::{TEvent, TEventRun, TEventRunStatus, TEventStatus};

type QueuerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type VoidResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub struct Queuer {
    db: Arc<Database>,
    events: Queue<TEvent>,
    event_runs: Queue<TEventRun>,
    should_stop: Arc<AtomicBool>,
    error_stack: Arc<Mutex<Stack<Box<dyn std::error::Error>>>>,
}

impl Queuer {
    pub fn new(db: Database) -> Self {
        let db = Arc::new(db);
        let events: Queue<TEvent> = Queue::new();
        let event_runs: Queue<TEventRun> = Queue::new();
        let should_stop = Arc::new(AtomicBool::new(false));
        let error_stack: Arc<Mutex<Stack<Box<dyn std::error::Error>>>> =
            Arc::new(Mutex::new(Stack::new(Some(10))));
        Queuer {
            db,
            events,
            event_runs,
            should_stop,
            error_stack,
        }
    }

    pub fn stop(&self) {
        self.should_stop
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub async fn start(&self) -> QueuerResult {
        let db = self.db.clone();
        let should_stop = self.should_stop.clone();
        let errors = self.error_stack.clone();
        let events_jh = tokio::spawn(async move {
            let mut page = 0;
            while !should_stop.load(std::sync::atomic::Ordering::Relaxed) {
                let events_res = db
                    .claim_events()
                    .await;

                match events_res {
                    Ok(events) => {
                        if events.is_empty() {
                            tokio::time::sleep(Duration::from_secs(2));
                        }
                    }
                    Err(e) => {
                        // errors.lock().push(e);
                    }
                }
            }
        });

        let event_runs_jh = tokio::spawn(async move {
            // poll event runs
        });

        events_jh.await?;
        event_runs_jh.await?;

        Ok(())
    }

    // fn process_events(&self, events: Vec<TEvent>) -> {}
}
