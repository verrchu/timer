use axum::extract::{Extension, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename(deserialize = "message_id"))]
    id: Uuid,
    data: String,
    schedule: Schedule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Schedule {
    At { at: DateTime<Utc> },
}

pub async fn create_oneshot_message(
    Json(request): Json<Request>,
    Extension(db_pool): Extension<PgPool>,
) {
    println!("{:?}", request);
}
