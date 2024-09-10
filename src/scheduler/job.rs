use chrono::{DateTime, FixedOffset};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use uuid::Uuid;

pub type AsyncJob =
Box<dyn Fn(DateTime<FixedOffset>) -> Pin<Box<dyn Future<Output=()> + Send + 'static>> + Send + Sync>;
pub type SyncJob = Box<dyn Fn(DateTime<FixedOffset>) + Send + Sync>;

pub struct Job {
    pub id: Uuid,
    pub job: Arc<AsyncJob>,
    interrupted: AtomicBool,
}

impl Job {
    pub fn new_async(job: AsyncJob) -> Self {
        Self {
            id: Uuid::new_v4(),
            job: Arc::new(job),
            interrupted: AtomicBool::new(false),
        }
    }

    pub fn new_sync(job: SyncJob) -> Self {
        let job = Arc::new(job);
        let job = Box::new(move |now| {
            let job = Arc::clone(&job);
            Box::pin(async move {
                job(now);
            }) as Pin<Box<dyn Future<Output=()> + Send>>
        });

        Self::new_async(job)
    }

    pub fn interrupt(&self) {
        self.interrupted.store(true, Ordering::Relaxed);
    }

    pub fn is_interrupted(&self) -> bool {
        self.interrupted.load(Ordering::Relaxed)
    }

    pub fn get_job(&self) -> Arc<AsyncJob> {
        self.job.clone()
    }
}
