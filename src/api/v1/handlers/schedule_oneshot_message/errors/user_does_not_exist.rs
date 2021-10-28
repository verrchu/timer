use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UserDoesNotExist {
    user_id: Uuid,
}

impl From<UserDoesNotExist> for super::Error {
    fn from(input: UserDoesNotExist) -> Self {
        Self::UserDoesNotExist(input)
    }
}
