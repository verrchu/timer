pub mod user_alias_already_taken;
pub use user_alias_already_taken::UserAliasAlreadyTaken;

use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "code", content = "context", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
    UserAliasAlreadyTaken(UserAliasAlreadyTaken),
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        StatusCode::UNPROCESSABLE_ENTITY
    }
}
