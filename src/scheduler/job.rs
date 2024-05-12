use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, RwLock};

use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

pub type AsyncJob =
    Box<dyn Fn(DateTime<FixedOffset>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync>;
pub type SyncJob = Box<dyn Fn(DateTime<FixedOffset>) + Send + Sync>;

pub struct Job {
    pub id: Uuid,
    pub job: Arc<AsyncJob>,
    interrupted: RwLock<bool>,
}

impl Job {
    pub fn new_async(job: AsyncJob) -> Self {
        Self {
            id: Uuid::new_v4(),
            job: Arc::new(job),
            interrupted: RwLock::new(false),
        }
    }

    pub fn new_sync(job: SyncJob) -> Self {
        let job = Arc::new(job);
        let job = Box::new(move |now| {
            let job = Arc::clone(&job);
            Box::pin(async move {
                job(now);
            }) as Pin<Box<dyn Future<Output = ()> + Send>>
        });

        Self::new_async(job)
    }

    pub fn interrupt(&self) {
        let mut interrupted = self.interrupted.write().expect("Failed to lock interrupted");
        *interrupted = true;
    }

    pub fn is_interrupted(&self) -> bool {
        *self.interrupted.read().expect("Failed to lock interrupted")
    }

    pub fn get_job(&self) -> Arc<AsyncJob> {
        self.job.clone()
    }
}
