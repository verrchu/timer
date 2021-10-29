use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Response {
    message_id: Uuid,
    scheduled_at: DateTime<Utc>,
}
