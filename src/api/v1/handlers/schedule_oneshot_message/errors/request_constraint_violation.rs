use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "reason", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RequestConstraintViolation {
    EmptyMessageContent(EmptyMessageContent),
    InvalidMessageScheduleTime(InvalidMessageScheduleTime),
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct EmptyMessageContent {
    #[builder(setter(into))]
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct InvalidMessageScheduleTime {
    value: DateTime<Utc>,
}

impl From<EmptyMessageContent> for super::Error {
    fn from(input: EmptyMessageContent) -> Self {
        Self::RequestConstraintViolation(RequestConstraintViolation::EmptyMessageContent(input))
    }
}

impl From<InvalidMessageScheduleTime> for super::Error {
    fn from(input: InvalidMessageScheduleTime) -> Self {
        Self::RequestConstraintViolation(RequestConstraintViolation::InvalidMessageScheduleTime(
            input,
        ))
    }
}
