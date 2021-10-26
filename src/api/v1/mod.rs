pub mod errors;
mod handlers;
pub use errors::Error;

use axum::{handler::post, routing::BoxRoute, Router};

pub fn router() -> Router<BoxRoute> {
    Router::new()
        .route(
            "/createOneshotMessage",
            post(handlers::create_oneshot_message),
        )
        .boxed()
}
