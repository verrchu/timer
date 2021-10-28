use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UserAlreadyRegistered {
    user_id: Uuid,
}

impl From<UserAlreadyRegistered> for super::Error {
    fn from(input: UserAlreadyRegistered) -> Self {
        Self::UserAlreadyRegistered(input)
    }
}
