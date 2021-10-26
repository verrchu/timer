mod api;
mod dao;
mod domain;

use axum::{Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/api", api::router());

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
