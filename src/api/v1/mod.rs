mod handlers;

use axum::{routing::BoxRoute, Router, handler::post};

pub fn router() -> Router<BoxRoute> {
    Router::new()
        .route("/createOneshotMessage", post(handlers::create_oneshot_message))
        .boxed()
}
