mod api;
mod dao;
mod domain;

use axum::{Router, Server};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::debug!("STARTING");

    let app = Router::new()
        .nest("/api", api::router())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
