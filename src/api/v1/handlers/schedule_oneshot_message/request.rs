use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::message::OneshotMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub schedule: Schedule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Schedule {
    At { at: DateTime<Utc> },
}

impl From<Request> for OneshotMessage {
    fn from(input: Request) -> Self {
        let scheduled_at = match input.schedule {
            Schedule::At { at } => at,
        };

        Self {
            message_id: input.message_id.into(),
            user_id: input.user_id.into(),
            content: input.content,
            scheduled_at,
        }
    }
}
