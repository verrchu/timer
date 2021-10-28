use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub user_id: Uuid,
}

impl From<Request> for User {
    fn from(input: Request) -> Self {
        Self {
            user_id: input.user_id.into(),
        }
    }
}
