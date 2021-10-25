mod v1;

use axum::{routing::BoxRoute, Router};

pub fn router() -> Router<BoxRoute> {
    Router::new()
        .route("/api/v1", v1::router())
        .boxed()
}
