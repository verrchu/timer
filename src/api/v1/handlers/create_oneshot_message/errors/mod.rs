pub mod request_constraint_error;
pub use request_constraint_error::RequestConstraintError;

use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
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
