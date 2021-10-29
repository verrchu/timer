use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UserAliasAlreadyTaken {
    alias: String,
}

impl From<UserAliasAlreadyTaken> for super::Error {
    fn from(input: UserAliasAlreadyTaken) -> Self {
        Self::UserAliasAlreadyTaken(input)
    }
}
