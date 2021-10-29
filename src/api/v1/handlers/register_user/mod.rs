use axum::extract::{Extension, Json};
use http::StatusCode;
use sqlx::PgPool;

mod request;
pub use request::Request;
mod response;
pub use response::Response;
mod errors;

use errors::{user_alias_already_taken::UserAliasAlreadyTaken, Error as HandlerError};

use crate::api::v1::errors::EitherError;
use crate::api::v1::errors::Error as CommonError;
use crate::api::v1::errors::InternalServerError;
use crate::dal::{
    self,
    user::register::{ConstraintError, QueryError},
};
use crate::domain;

pub async fn register_user(
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

    let domain_object: domain::user::User = request.into();

    let user_id = dal::user::register(&mut db_conn, domain_object.clone().into())
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
                    ConstraintError::UserAliasAlreadyTaken => UserAliasAlreadyTaken::builder()
                        .alias(domain_object.alias)
                        .build()
                        .into(),
                };

                (
                    handler_error.status_code(),
                    Json(EitherError::Handler(handler_error)),
                )
            }
        })?;

    Ok(Json(Response::builder().user_id(user_id).build()))
}
