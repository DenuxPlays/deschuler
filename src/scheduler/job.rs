use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::DateTime;
use chrono_tz::Tz;

pub type AsyncJob = Box<dyn Fn(DateTime<Tz>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync>;
pub type SyncJob = Box<dyn Fn(DateTime<Tz>) + Send + Sync>;

pub struct Job {
    pub job: AsyncJob,
}

impl Job {
    pub fn new_async(job: AsyncJob) -> Self {
        Self {
            job,
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
}
