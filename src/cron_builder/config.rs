use chrono::{DateTime, FixedOffset, Offset, TimeZone, Utc};
use chrono_tz::Tz;
use derive_builder::Builder;

use crate::util::date_time::get_now_naive;

#[derive(Debug, Clone, PartialEq, Eq, Builder)]
pub struct BuilderConfig {
    pub timezone: FixedOffset,
    pub use_utc: bool,
}

impl BuilderConfig {
    pub fn get_now(&self) -> DateTime<FixedOffset> {
        let now = get_now_naive(self.use_utc);

        DateTime::from_naive_utc_and_offset(now, self.timezone)
    }
}

impl Default for BuilderConfig {
    fn default() -> Self {
        let offset = Tz::UTC.offset_from_utc_datetime(&Utc::now().naive_utc()).fix();
        let use_utc = true;
        Self {
            timezone: offset,
            use_utc,
        }
    }
}
