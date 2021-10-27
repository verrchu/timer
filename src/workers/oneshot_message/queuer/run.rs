use sqlx::PgPool;
use tokio::time::sleep;
use tracing::{debug, info};

use super::Config;

pub async fn run(config: Config, db_pool: PgPool) -> eyre::Result<()> {
    loop {
        sleep(config.tick_interval).await;

        debug!("Scanning DB for unqueued oneshot messages");
    }
}
