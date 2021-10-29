use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneshotMessage {
    pub user_id: Uuid,
    pub content: String,
    pub scheduled_at: DateTime<Utc>,
}

mod transform {
    use super::OneshotMessage;
    use crate::domain::message as domain;

    impl From<domain::OneshotMessage> for OneshotMessage {
        fn from(input: domain::OneshotMessage) -> Self {
            Self {
                user_id: input.user_id.0,
                content: input.content,
                scheduled_at: input.scheduled_at,
            }
        }
    }
}
