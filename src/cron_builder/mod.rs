use croner::errors::CronError;
use croner::Cron;

use crate::cron_builder::config::BuilderConfig;
use crate::cron_builder::utils::always;

pub mod config;
pub mod utils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CronBuilder {
    pub second: String,
    pub minute: String,
    pub hour: String,
    pub day_of_month: String,
    pub month: String,
    pub day_of_week: String,

    pub config: BuilderConfig,
}

impl CronBuilder {
    pub fn new() -> Self {
        Self::new_with_config(BuilderConfig::default())
    }

    pub fn new_with_config(config: BuilderConfig) -> Self {
        Self {
            second: always(),
            minute: always(),
            hour: always(),
            day_of_month: always(),
            month: always(),
            day_of_week: always(),
            config,
        }
    }

    pub fn second(&mut self, second: String) -> &mut Self {
        self.second = second;
        self
    }

    pub fn minute(&mut self, minute: String) -> &mut Self {
        self.minute = minute;
        self
    }

    pub fn hour(&mut self, hour: String) -> &mut Self {
        self.hour = hour;
        self
    }

    pub fn day_of_month(&mut self, day_of_month: String) -> &mut Self {
        self.day_of_month = day_of_month;
        self
    }

    pub fn month(&mut self, month: String) -> &mut Self {
        self.month = month;
        self
    }

    pub fn day_of_week(&mut self, day_of_week: String) -> &mut Self {
        self.day_of_week = day_of_week;
        self
    }

    pub fn always() -> Cron {
        Cron::new("* * * * * *").parse().unwrap()
    }

    pub fn hourly() -> Cron {
        Cron::new("@hourly").parse().unwrap()
    }

    pub fn daily() -> Cron {
        Cron::new("@daily").parse().unwrap()
    }

    pub fn weekly() -> Cron {
        Cron::new("@weekly").parse().unwrap()
    }

    pub fn monthly() -> Cron {
        Cron::new("@monthly").parse().unwrap()
    }

    pub fn yearly() -> Cron {
        Cron::new("@yearly").parse().unwrap()
    }

    pub fn build(&self) -> Result<Cron, CronError> {
        Self::get_cron(&self.get_pattern())
    }

    pub fn get_pattern(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.second, self.minute, self.hour, self.day_of_month, self.month, self.day_of_week
        )
    }

    fn get_cron(pattern: &str) -> Result<Cron, CronError> {
        Cron::new(pattern).with_seconds_required().with_dom_and_dow().parse()
    }
}

impl Default for CronBuilder {
    fn default() -> Self {
        Self::new()
    }
}
