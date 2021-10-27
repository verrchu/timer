use serde::{Deserialize, Serialize};

use super::oneshot_message;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub oneshot_message: oneshot_message::Config,
}
