use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::UserId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneshotMessage {
    pub user_id: UserId,
    pub content: String,
    pub scheduled_at: DateTime<Utc>,
}
