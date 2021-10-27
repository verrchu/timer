use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub tick_interval: Duration,
    pub message_limit: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tick_interval: Duration::from_secs(10),
            message_limit: 100,
        }
    }
}
