mod internal_server_error;
pub use internal_server_error::InternalServerError;

use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "code", content = "context", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
    InternalServerError(InternalServerError),
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub trait ApiError {
    fn status_code() -> StatusCode;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EitherError<E> {
    Common(Error),
    Handler(E),
}
