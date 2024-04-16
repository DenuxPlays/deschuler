use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
use derive_builder::Builder;

use crate::util::date_time::get_now_naive;

#[derive(Debug, Clone, PartialEq, Eq, Builder)]
pub struct BuilderConfig {
    pub timezone: Tz,
    pub use_utc: bool,
}

impl BuilderConfig {
    pub fn get_now(&self) -> DateTime<Tz> {
        let now = get_now_naive(self.use_utc);

        let offset = self.timezone.offset_from_utc_datetime(&now);
        DateTime::from_naive_utc_and_offset(now, offset)
    }
}

impl Default for BuilderConfig {
    fn default() -> Self {
        let use_utc = false;
        Self {
            timezone: Tz::UTC,
            use_utc,
        }
    }
}
