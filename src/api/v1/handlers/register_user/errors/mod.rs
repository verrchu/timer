pub mod user_already_registered;
pub use user_already_registered::UserAlreadyRegistered;

use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "code", content = "context", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
    UserAlreadyRegistered(UserAlreadyRegistered),
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        StatusCode::UNPROCESSABLE_ENTITY
    }
}
