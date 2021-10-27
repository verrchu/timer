use serde::{Deserialize, Serialize};

use super::queuer;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub queuer: queuer::Config,
}
