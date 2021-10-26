mod api;
mod dao;
mod domain;

mod config;
use config::Config;
mod args;
use args::Args;

use axum::{AddExtensionLayer, Router, Server};
use sqlx::PgPool;
use structopt::StructOpt;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::from_args();
    let config = Config::load(args.config)?;

    let db_pool = PgPool::connect(&config.db.addr)
        .await
        .map_err(Into::<eyre::Report>::into)?;

    let app = Router::new().nest("/api", api::router()).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(AddExtensionLayer::new(db_pool)),
    );

    Server::bind(&config.http.addr)
        .serve(app.into_make_service())
        .await
        .map_err(Into::<eyre::Report>::into)
}
