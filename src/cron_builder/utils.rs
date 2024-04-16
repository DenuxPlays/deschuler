pub fn every(pattern: &str) -> String {
    format!("*/{}", pattern)
}

pub fn nth_occurrence(pattern: &str, occurrence: usize) -> String {
    format!("{}#{}", pattern, occurrence)
}

pub fn closest_week_day(pattern: &str) -> String {
    format!("{}W", pattern)
}

pub fn last(pattern: &str) -> String {
    format!("{}L", pattern)
}

pub fn always() -> String {
    "*".to_string()
}
