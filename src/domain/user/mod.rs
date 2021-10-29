use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub alias: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl From<Uuid> for UserId {
    fn from(input: Uuid) -> Self {
        Self(input)
    }
}
