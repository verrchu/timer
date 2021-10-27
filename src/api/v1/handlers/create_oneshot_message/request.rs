use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::message::OneshotMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub message_id: Uuid,
    pub data: String,
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
            id: input.message_id,
            data: input.data,
            scheduled_at,
        }
    }
}
