use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::message::OneshotMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub user_id: Uuid,
    pub content: String,
    pub schedule: Schedule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Schedule {
    At(DateTime<Utc>),
    In(u64),
}

impl TryFrom<Request> for OneshotMessage {
    type Error = eyre::Report;

    fn try_from(input: Request) -> eyre::Result<Self> {
        let scheduled_at = match input.schedule {
            Schedule::At(timestamp) => timestamp,
            Schedule::In(seconds) => {
                let seconds = i64::try_from(seconds).map_err(Into::<eyre::Report>::into)?;
                Utc::now() + Duration::seconds(seconds)
            }
        };

        Ok(Self {
            user_id: input.user_id.into(),
            content: input.content,
            scheduled_at,
        })
    }
}
