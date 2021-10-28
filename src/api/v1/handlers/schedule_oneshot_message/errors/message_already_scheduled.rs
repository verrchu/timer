use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct MessageAlreadyScheduled {
    user_id: Uuid,
}

impl From<MessageAlreadyScheduled> for super::Error {
    fn from(input: MessageAlreadyScheduled) -> Self {
        Self::MessageAlreadyScheduled(input)
    }
}
