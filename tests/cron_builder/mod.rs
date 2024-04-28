use deschuler::cron_builder::utils::{every, last};
use deschuler::cron_builder::CronBuilder;

mod complex;
mod special_cases;

#[test]
fn default() {
    let cron_builder = get_cron_builder();
    let cron = cron_builder.build().unwrap();

    let actual = cron.pattern.to_string();
    let expected = "* * * * * *";

    assert_eq!(actual, expected);
}

#[test]
fn every_4_minutes() {
    let mut cron_builder = get_cron_builder();
    cron_builder.minute(every("4"));
    let cron = cron_builder.build().unwrap();

    let actual = cron.pattern.to_string();
    let expected = "* */4 * * * *";

    assert_eq!(actual, expected);
}

#[test]
fn every_3_days() {
    let mut cron_builder = get_cron_builder();
    cron_builder.day_of_week(every("2"));
    let cron = cron_builder.build().unwrap();

    let actual = cron.pattern.to_string();
    let expected = "* * * * * */2";

    assert_eq!(actual, expected);
}

#[test]
fn every_last_friday_of_the_month() {
    let mut cron_builder = get_cron_builder();
    cron_builder.day_of_week(last("5"));
    let cron = cron_builder.build().unwrap();

    let actual = cron.pattern.to_string();
    let expected = "* * * * * 5l";

    assert_eq!(actual, expected);
}

fn get_cron_builder() -> CronBuilder {
    CronBuilder::default()
}
