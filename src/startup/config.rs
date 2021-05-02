use color_eyre::eyre::{Result, WrapErr};
use dotenv::dotenv;
use serde::Deserialize;
use tracing::{info, instrument};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub discord_token: String,
    pub rust_log: String,
    pub bot_prefix: String,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        info!("Loading configuration");
        // Cargar las variables de entorno desde el archivo `.env`.
        // Mira el archivo `.env.example` para saber cómo configurarlo.
        dotenv().ok();
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::default())?;
        cfg.try_into()
            .context("Loading configuration from environment")
    }
}
