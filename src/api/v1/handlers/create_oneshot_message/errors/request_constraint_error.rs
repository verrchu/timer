use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestConstraintError {
    EmptyMessageData(EmptyMessageData),
    InvalidMessageScheduleTime(InvalidMessageScheduleTime),
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct EmptyMessageData {
    #[builder(setter(into))]
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct InvalidMessageScheduleTime {
    value: DateTime<Utc>,
}

impl From<EmptyMessageData> for super::Error {
    fn from(input: EmptyMessageData) -> Self {
        Self::RequestConstraintError(RequestConstraintError::EmptyMessageData(input))
    }
}

impl From<InvalidMessageScheduleTime> for super::Error {
    fn from(input: InvalidMessageScheduleTime) -> Self {
        Self::RequestConstraintError(RequestConstraintError::InvalidMessageScheduleTime(input))
    }
}
