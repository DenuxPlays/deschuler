use std::sync::Arc;
use std::time::Duration;

use chrono::SubsecRound;
use croner::Cron;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::error;

use crate::scheduler::job::Job;
use crate::scheduler::tokio_scheduler::config::TokioSchedulerConfig;
use crate::scheduler::Scheduler;

pub mod config;

type JobHandle = Box<dyn FnOnce() -> JoinHandle<()> + Send + Sync>;

#[derive(Debug)]
pub struct TokioScheduler {
    config: TokioSchedulerConfig,
    sender: Sender<JobHandle>,
    worker_sender: watch::Sender<bool>,
}

impl Default for TokioScheduler {
    fn default() -> Self {
        let config = TokioSchedulerConfig::default();
        let (sender, receiver) = channel(config.channel_size);
        let (worker_sender, worker_receiver) = watch::channel(false);

        let scheduler = Self {
            config,
            sender,
            worker_sender,
        };

        Self::start_worker(worker_receiver, receiver);

        scheduler
    }
}

impl Scheduler for TokioScheduler {
    fn start(&mut self) {
        self.worker_sender.send(true).unwrap();
    }

    fn schedule_job(&mut self, cron: Cron, task: Job) {
        let config = self.config.clone();
        let sender = self.sender.clone();
        let builder_config = config.builder_config.clone();
        tokio::spawn(async move {
            let task = Arc::new(task.job);
            loop {
                let now = builder_config.get_now().round_subsecs(0);
                match cron.find_next_occurrence(&now, false) {
                    Ok(next) => {
                        let _now = now.to_rfc3339();
                        let _next = next.to_rfc3339();
                        let duration = next.signed_duration_since(now).num_seconds();
                        let duration = Duration::from_secs(duration as u64);
                        sleep(duration).await;
                        let task = task.clone();
                        let job_handle = Box::new(move || {
                            tokio::spawn(async move {
                                task(now).await;
                            })
                        });

                        if let Err(err) = sender.send(job_handle).await {
                            error!("Error while sending job: {:?}", err);
                        }
                    }
                    Err(err) => {
                        error!("Error while finding next occurrence: {:?}", err);
                        break;
                    }
                }
            }
        });
    }

    fn stop(&mut self) {
        self.worker_sender.send(false).unwrap();
    }
}

impl TokioScheduler {
    fn start_worker(worker_receiver: watch::Receiver<bool>, mut receiver: Receiver<JobHandle>) {
        tokio::spawn(async move {
            loop {
                match worker_receiver.has_changed() {
                    Ok(bool) => {
                        if bool {
                            match receiver.recv().await {
                                Some(job) => Self::handle_task(job),
                                None => break,
                            }
                        }
                    }
                    Err(err) => {
                        error!("The worker has experienced an unexpected error: {:?}", err);
                        break;
                    }
                }
            }
        });
    }

    fn handle_task(job: JobHandle) {
        tokio::spawn(async move { job().await });
    }
}
