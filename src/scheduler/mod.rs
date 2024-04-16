use croner::Cron;

use crate::scheduler::job::Job;

pub mod job;
pub mod tokio_scheduler;

pub trait Scheduler: Default {
    fn start(&mut self);

    fn schedule_job(&mut self, cron: Cron, task: Job);

    fn stop(&mut self);
}
