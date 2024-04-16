use deschuler::cron_builder::CronBuilder;

#[test]
fn hourly() {
    let cron = CronBuilder::hourly();

    let actual = cron.pattern.to_string();
    let expected = "0 * * * *".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn daily() {
    let cron = CronBuilder::daily();

    let actual = cron.pattern.to_string();
    let expected = "0 0 * * *".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn weekly() {
    let cron = CronBuilder::weekly();

    let actual = cron.pattern.to_string();
    let expected = "0 0 * * 0".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn monthly() {
    let cron = CronBuilder::monthly();

    let actual = cron.pattern.to_string();
    let expected = "0 0 1 * *".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn yearly() {
    let cron = CronBuilder::yearly();

    let actual = cron.pattern.to_string();
    let expected = "0 0 1 1 *".to_string();
    assert_eq!(actual, expected);
}
