use derive_builder::Builder;

use crate::cron_builder::config::BuilderConfig;

#[derive(Debug, Clone, PartialEq, Eq, Builder)]
pub struct TokioSchedulerConfig {
    pub channel_size: usize,
    pub builder_config: BuilderConfig,
}

impl Default for TokioSchedulerConfig {
    fn default() -> Self {
        Self {
            channel_size: 1024,
            builder_config: BuilderConfig::default(),
        }
    }
}
