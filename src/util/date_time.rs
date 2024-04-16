use chrono::{Local, NaiveDateTime, Utc};

pub(crate) fn get_now_naive(use_utc: bool) -> NaiveDateTime {
    match use_utc {
        true => now_utc(),
        false => now_local(),
    }
}

pub(crate) fn now_local() -> NaiveDateTime {
    Local::now().naive_local()
}

pub(crate) fn now_utc() -> NaiveDateTime {
    Utc::now().naive_utc()
}
