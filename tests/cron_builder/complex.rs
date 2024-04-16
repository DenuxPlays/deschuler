use crate::cron_builder::get_cron_builder;
use deschuler::cron_builder::utils::{every, nth_occurrence};

#[test]
fn every_second_week_on_mon_and_fri() {
    let mut cron_builder = get_cron_builder();
    cron_builder.day_of_week("1,5".to_string());
    let cron = cron_builder.build().unwrap();

    let actual = cron.pattern.to_string();
    let expected = "* * * * * 1,5";

    assert_eq!(actual, expected);
}

#[test]
fn every_3rd_month_on_the_10th() {
    let mut cron_builder = get_cron_builder();
    cron_builder.month(every("3"));
    cron_builder.day_of_month("10".to_string());
    let cron = cron_builder.build().unwrap();

    let actual = cron.pattern.to_string();
    let expected = "* * * 10 */3 *";

    assert_eq!(actual, expected);
}

#[test]
fn every_3rd_month_on_the_second_monday() {
    let mut cron_builder = get_cron_builder();
    cron_builder.month(every("3"));
    cron_builder.day_of_week(nth_occurrence("1", 2));
    let cron = cron_builder.build().unwrap();

    let actual = cron.pattern.to_string();
    let expected = "* * * * */3 1#2";

    assert_eq!(actual, expected);
}

#[test]
fn every_aug_jan() {
    let mut cron_builder = get_cron_builder();
    cron_builder.month("1,8".to_string());
    let cron = cron_builder.build().unwrap();

    let actual = cron.pattern.to_string();
    let expected = "* * * * 1,8 *";

    assert_eq!(actual, expected);
}

// #[test]
// fn every_second_year_in_jan_aug_on_first_monday() {
//     let mut cron_builder = get_cron_builder();
//     cron_builder.year(every("2"));
//     cron_builder.month("1,8".to_string());
//     cron_builder.day_of_week(nth_occurrence("1", 1));
//     let cron = cron_builder.build().unwrap();
//
//     let actual = cron.pattern.to_string();
//     let expected = "* * * 1,8/2 1#1";
//
//     assert_eq!(actual, expected);
// }
