use axum::extract::{Extension, Json};
use http::StatusCode;
use sqlx::PgPool;

mod request;
pub use request::Request;
mod response;
pub use response::Response;
mod errors;

use errors::{request_constraint_error, Error as HandlerError};

use crate::api::v1::errors::EitherError;
use crate::api::v1::errors::Error as CommonError;
use crate::api::v1::errors::InternalServerError;
use crate::dal::{
    self,
    oneshot_message::schedule::{ConstraintError, QueryError},
};
use crate::domain;

pub async fn schedule_oneshot_message(
    Json(request): Json<Request>,
    Extension(db_pool): Extension<PgPool>,
) -> Result<Json<Response>, (StatusCode, Json<EitherError<HandlerError>>)> {
    let mut db_conn = db_pool.acquire().await.map_err(|err| {
        let common_error: CommonError = InternalServerError::builder()
            .reason(format!("Failed to acquire DB connection: {:?}", err))
            .build()
            .into();

        (
            common_error.status_code(),
            Json(EitherError::Common(common_error)),
        )
    })?;

    let domain_object: domain::message::OneshotMessage = request.into();

    let message_id = dal::oneshot_message::schedule(&mut db_conn, &domain_object.clone().into())
        .await
        .map_err(|err| match err {
            QueryError::Generic(inner) => {
                let common_error: CommonError = InternalServerError::builder()
                    .reason(format!("Failed to perform DB query: {:?}", inner))
                    .build()
                    .into();

                (
                    common_error.status_code(),
                    Json(EitherError::Common(common_error)),
                )
            }
            QueryError::ConstraintError(inner) => {
                let handler_error: HandlerError = match inner {
                    ConstraintError::EmptyMessageData => {
                        request_constraint_error::EmptyMessageData::builder()
                            .value(domain_object.data)
                            .build()
                            .into()
                    }
                    ConstraintError::InvalidMessageScheduleTime => {
                        request_constraint_error::InvalidMessageScheduleTime::builder()
                            .value(domain_object.scheduled_at)
                            .build()
                            .into()
                    }
                };

                (
                    handler_error.status_code(),
                    Json(EitherError::Handler(handler_error)),
                )
            }
        })?;

    Ok(Json(Response::builder().message_id(message_id).build()))
}
