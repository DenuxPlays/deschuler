use chrono::{FixedOffset, TimeZone};
use time::{Date, Month, OffsetDateTime, Time, UtcOffset};

use deschuler::cron_builder::config::BuilderConfigBuilder;

#[test]
pub fn test_time_to_chrono() {
    let date = Date::from_calendar_date(2024, Month::January, 1).unwrap();
    let now =
        OffsetDateTime::new_in_offset(date, Time::from_hms(0, 0, 0).unwrap(), UtcOffset::from_hms(4, 0, 0).unwrap());
    let (timezone, is_utc) = extract_tz(&now);

    let config = BuilderConfigBuilder::default().timezone(timezone).use_utc(is_utc).build().unwrap();

    assert!(!config.use_utc);
    assert_eq!(config.timezone.local_minus_utc(), 14400);
}

fn extract_tz(datetime: &OffsetDateTime) -> (FixedOffset, bool) {
    let offset_seconds = datetime.offset().whole_seconds();
    let offset = FixedOffset::east_opt(offset_seconds).unwrap();
    let fixed_offset = TimeZone::from_offset(&offset);

    let is_utc = offset_seconds == 0;

    (fixed_offset, is_utc)
}
