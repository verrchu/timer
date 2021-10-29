use serde::{Deserialize, Serialize};

use crate::domain::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub alias: String,
}

impl From<Request> for User {
    fn from(input: Request) -> Self {
        Self { alias: input.alias }
    }
}
