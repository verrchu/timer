use http::StatusCode;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct InternalServerError {
    pub reason: String,
}

impl From<InternalServerError> for super::Error {
    fn from(input: InternalServerError) -> Self {
        Self::InternalServerError(input)
    }
}

impl super::ApiError for InternalServerError {
    fn status_code() -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
