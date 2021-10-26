use std::time::Duration;

use axum::extract::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

pub async fn create_oneshot_message(Json(request): Json<Request>) {
    println!("{:?}", request);
}
