use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneshotMessage {
    pub message_id: Uuid,
    pub data: String,
    pub planned_at: DateTime<Utc>,
}

mod transform {
    use super::OneshotMessage;
    use crate::domain::message::oneshot as domain;

    impl From<domain::OneshotMessage> for OneshotMessage {
        fn from(input: domain::OneshotMessage) -> Self {
            Self {
                message_id: input.id,
                data: input.data,
                planned_at: input.planned_at,
            }
        }
    }
}
