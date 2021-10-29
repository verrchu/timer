use axum::extract::{Extension, Json};
use http::StatusCode;
use sqlx::PgPool;

mod request;
pub use request::Request;
mod response;
pub use response::Response;
mod errors;

use errors::{
    message_already_scheduled::MessageAlreadyScheduled, request_constraint_violation,
    user_does_not_exist::UserDoesNotExist, Error as HandlerError,
};

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

    let domain_object: domain::message::OneshotMessage = request.try_into().map_err(|err| {
        let common_error: CommonError = InternalServerError::builder()
            .reason(format!(
                "Failed to trnsform request to internal structure: {:?}",
                err
            ))
            .build()
            .into();

        (
            common_error.status_code(),
            Json(EitherError::Common(common_error)),
        )
    })?;

    let db_response = dal::oneshot_message::schedule(&mut db_conn, domain_object.clone().into())
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
                    ConstraintError::EmptyMessageContent => {
                        request_constraint_violation::EmptyMessageContent::builder()
                            .value(domain_object.content)
                            .build()
                            .into()
                    }
                    ConstraintError::InvalidMessageScheduleTime => {
                        request_constraint_violation::InvalidMessageScheduleTime::builder()
                            .value(domain_object.scheduled_at)
                            .build()
                            .into()
                    }
                    ConstraintError::UserDoesNotExist => UserDoesNotExist::builder()
                        .user_id(domain_object.user_id.0)
                        .build()
                        .into(),
                    ConstraintError::MessageAlreadyScheduled => MessageAlreadyScheduled::builder()
                        .user_id(domain_object.user_id.0)
                        .build()
                        .into(),
                };

                (
                    handler_error.status_code(),
                    Json(EitherError::Handler(handler_error)),
                )
            }
        })?;

    Ok(Json(
        Response::builder()
            .message_id(db_response.message_id)
            .scheduled_at(db_response.scheduled_at)
            .build(),
    ))
}
