use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneshotMessage {
    pub id: Uuid,
    pub content: String,
    pub scheduled_at: DateTime<Utc>,
}
