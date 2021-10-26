mod v1;

use axum::{routing::BoxRoute, Router};

pub fn router() -> Router<BoxRoute> {
    Router::new().nest("/v1", v1::router()).boxed()
}
