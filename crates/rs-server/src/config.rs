use config::{Config as Loader, Environment, File};
use serde::Deserialize;

pub const RS_MARKET_DEFAULT_CONFIG_FILE: &str = "rs-market.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        dotenvy::dotenv().ok();
        let builder = Loader::builder()
            .add_source(File::with_name(RS_MARKET_DEFAULT_CONFIG_FILE).required(false))
            .add_source(
                Environment::with_prefix("RS_MARKET")
                    .separator("__")
                    .try_parsing(true),
            );

        let cfg: Config = builder.build()?.try_deserialize()?;
        Ok(cfg)
    }
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default = "default_acquire_timeout_ms")]
    pub acquire_timeout_ms: u64,
    #[serde(default = "default_statement_timeout_ms")]
    pub statement_timeout_ms: u64,
}

const fn default_max_connections() -> u32 {
    5
}

const fn default_acquire_timeout_ms() -> u64 {
    3_000
}

const fn default_statement_timeout_ms() -> u64 {
    5_000
}
