use axum::extract::{Extension, Json};
use http::StatusCode;
use sqlx::PgPool;

mod request;
pub use request::Request;
mod response;
pub use response::Response;

use crate::api::v1 as api;
use crate::api::v1::errors::ApiError;
use crate::api::v1::errors::InternalServerError;
use crate::dal;
use crate::domain;

pub async fn create_oneshot_message(
    Json(request): Json<Request>,
    Extension(db_pool): Extension<PgPool>,
) -> Result<Json<Response>, (StatusCode, Json<api::Error>)> {
    let mut db_conn = db_pool.acquire().await.map_err(|err| {
        (
            InternalServerError::status_code(),
            Json(
                InternalServerError::builder()
                    .reason(format!("Failed to acquire DB connection: {:?}", err))
                    .build()
                    .into(),
            ),
        )
    })?;

    let domain_object: domain::message::OneshotMessage = request.into();

    let message_id = dal::oneshot_message::insert(&mut db_conn, &domain_object.into())
        .await
        .map_err(|err| {
            (
                InternalServerError::status_code(),
                Json(
                    InternalServerError::builder()
                        .reason(format!("Failed to perform DB query: {:?}", err))
                        .build()
                        .into(),
                ),
            )
        })?;

    Ok(Json(Response::builder().message_id(message_id).build()))
}
