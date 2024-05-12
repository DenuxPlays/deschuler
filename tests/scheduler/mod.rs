use std::sync::{Arc, Mutex};

use chrono::{SubsecRound, Utc};
use tokio::time::{sleep, Duration};

use deschuler::cron_builder::utils::every;
use deschuler::cron_builder::CronBuilder;
use deschuler::scheduler::job::Job;
use deschuler::scheduler::tokio_scheduler::TokioScheduler;
use deschuler::scheduler::Scheduler;

mod interrupted;
mod time;

#[tokio::test]
async fn schedule_task_every_second_async() {
    let cron = CronBuilder::default().second(every("1")).build().unwrap();
    assert_eq!(cron.pattern.to_string(), "*/1 * * * * *");

    let count = Arc::new(Mutex::new(0));
    let cloned_count = count.clone();
    let cron = cron.clone();

    let start = Utc::now().round_subsecs(0);
    println!("Start: {:?}", start);

    let job = Job::new_async(Box::new(move |now| {
        let count = cloned_count.clone();
        println!("Now: {}", now);
        Box::pin(async move {
            let mut count = count.lock().unwrap();
            *count += 1;
        })
    }));

    let mut scheduler = TokioScheduler::default();
    scheduler.schedule_job(cron, Arc::new(job));
    scheduler.start();

    sleep(Duration::from_millis(3010)).await;

    scheduler.stop();

    let end = Utc::now().round_subsecs(0);
    println!("End: {:?}", end);

    let actual = *count.lock().unwrap();
    let expected = 3;

    assert_eq!(actual, expected);
}

#[tokio::test]
async fn schedule_task_every_second_sync() {
    let cron = CronBuilder::default().second(every("1")).build().unwrap();
    assert_eq!(cron.pattern.to_string(), "*/1 * * * * *");

    let count = Arc::new(Mutex::new(0));
    let cloned_count = count.clone();
    let cron = cron.clone();

    let start = Utc::now().round_subsecs(0);
    println!("Start: {:?}", start);

    let job = Job::new_sync(Box::new(move |_now| {
        println!("Now: {}", _now);
        let count = cloned_count.clone();
        let mut count = count.lock().unwrap();
        *count += 1;
    }));

    let mut scheduler = TokioScheduler::default();
    scheduler.schedule_job(cron, Arc::new(job));
    scheduler.start();

    sleep(Duration::from_millis(3010)).await;

    scheduler.stop();

    let end = Utc::now().round_subsecs(0);
    println!("End: {:?}", end);

    let actual = *count.lock().unwrap();
    let expected = 3;

    assert_eq!(actual, expected);
}
