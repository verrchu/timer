mod api;
mod dao;
mod domain;

mod config;
use config::Config;
mod args;
use args::Args;

use axum::{Router, Server};
use structopt::StructOpt;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::from_args();
    let config = Config::load(args.config).unwrap();

    let app = Router::new()
        .nest("/api", api::router())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    Server::bind(&config.http.addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
