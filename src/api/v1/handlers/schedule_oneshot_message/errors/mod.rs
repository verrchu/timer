pub mod request_constraint_error;
pub use request_constraint_error::RequestConstraintError;

use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "code", content = "context", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
    RequestConstraintError(RequestConstraintError),
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::RequestConstraintError(_) => StatusCode::BAD_REQUEST,
        }
    }
}
