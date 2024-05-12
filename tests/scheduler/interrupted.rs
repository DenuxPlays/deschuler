use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::{SubsecRound, Utc};
use tokio::time::sleep;

use deschuler::cron_builder::utils::every;
use deschuler::cron_builder::CronBuilder;
use deschuler::scheduler::job::{AsyncJob, Job};
use deschuler::scheduler::tokio_scheduler::TokioScheduler;
use deschuler::scheduler::Scheduler;

#[tokio::test]
pub async fn test_interruption() {
    let cron = CronBuilder::default().second(every("1")).build().unwrap();
    assert_eq!(cron.pattern.to_string(), "*/1 * * * * *");

    let count = Arc::new(Mutex::new(0));
    let cloned_count = count.clone();
    let cron = cron.clone();

    let start = Utc::now().round_subsecs(0);
    println!("Start: {:?}", start);

    let job1 = Job::new_async(get_job_blueprint(cloned_count.clone()));
    let job2 = Job::new_async(get_job_blueprint(cloned_count.clone()));
    let job3 = Job::new_async(get_job_blueprint(cloned_count.clone()));

    job2.interrupt();

    let mut scheduler = TokioScheduler::default();
    scheduler.schedule_job(cron.clone(), Arc::new(job1));
    scheduler.schedule_job(cron.clone(), Arc::new(job2));
    scheduler.schedule_job(cron, Arc::new(job3));
    scheduler.start();

    sleep(Duration::from_millis(1010)).await;

    scheduler.stop();

    let end = Utc::now().round_subsecs(0);
    println!("End: {:?}", end);

    let actual = *count.lock().unwrap();
    let expected = 2;

    assert_eq!(actual, expected);
}

fn get_job_blueprint(count: Arc<Mutex<i32>>) -> AsyncJob {
    Box::new(move |now| {
        let count = count.clone();
        println!("Now: {}", now);
        Box::pin(async move {
            let mut count = count.lock().unwrap();
            *count += 1;
        })
    })
}
