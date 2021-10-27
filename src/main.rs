mod api;
mod dal;
mod dao;
mod domain;
mod workers;

mod config;
use config::Config;
mod args;
use args::Args;

use axum::{AddExtensionLayer, Router, Server};
use sqlx::PgPool;
use structopt::StructOpt;
use tokio::task::{self, JoinHandle};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::from_args();
    let config = Config::load(args.config)?;

    info!("Starting with config: {:?}", config);

    let db_pool = PgPool::connect(&config.db.addr)
        .await
        .map_err(Into::<eyre::Report>::into)?;

    let _ = tokio::try_join!(
        server_task(config.clone(), db_pool.clone()),
        oneshot_message_queuer_task(config.clone(), db_pool.clone())
    );

    Ok(())
}

fn server_task(config: Config, db_pool: PgPool) -> JoinHandle<eyre::Result<()>> {
    task::spawn(async move {
        let app = Router::new().nest("/api", api::router()).layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(AddExtensionLayer::new(db_pool)),
        );

        Server::bind(&config.http.addr)
            .serve(app.into_make_service())
            .await
            .map_err(Into::<eyre::Report>::into)
    })
}

fn oneshot_message_queuer_task(config: Config, db_pool: PgPool) -> JoinHandle<eyre::Result<()>> {
    task::spawn(async move {
        let config = config.workers.oneshot_message.queuer;
        workers::oneshot_message::queuer::run(config, db_pool).await
    })
}
