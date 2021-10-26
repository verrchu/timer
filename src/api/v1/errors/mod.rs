mod internal_server_error;
pub use internal_server_error::InternalServerError;

use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Error {
    InternalServerError(InternalServerError),
}

pub trait ApiError {
    fn status_code() -> StatusCode;
}
