pub mod request_constraint_violation;
pub use request_constraint_violation::RequestConstraintViolation;

pub mod user_does_not_exist;
pub use user_does_not_exist::UserDoesNotExist;

pub mod message_already_scheduled;
pub use message_already_scheduled::MessageAlreadyScheduled;

use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "code", content = "context", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
    RequestConstraintViolation(RequestConstraintViolation),
    UserDoesNotExist(UserDoesNotExist),
    MessageAlreadyScheduled(MessageAlreadyScheduled),
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        StatusCode::UNPROCESSABLE_ENTITY
    }
}
