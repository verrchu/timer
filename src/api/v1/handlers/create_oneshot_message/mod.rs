use axum::extract::{Extension, Json};
use sqlx::PgPool;

mod request;
pub use request::Request;

mod response;
// pub use response::Response;

pub async fn create_oneshot_message(
    Json(request): Json<Request>,
    Extension(db_pool): Extension<PgPool>,
) {
    println!("{:?}", request);
}
