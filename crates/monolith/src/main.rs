use rs_server::{Config, db::PostgresRepository, services::GiftCardService};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::load().expect("failed to load config");
    let pool = PgPoolOptions::new()
        .max_connections(cfg.database.max_connections)
        .acquire_timeout(Duration::from_millis(cfg.database.acquire_timeout_ms))
        .connect(&cfg.database.url)
        .await?;
    let repository = PostgresRepository::new(pool);
    let _service = GiftCardService::new(repository);
    Ok(())
}
