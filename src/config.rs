use color_eyre::Result;
use config::{Config, Environment};
use serde::Deserialize;
// use tracing_subscriber::{filter, prelude::*};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub database_url: String,
    pub listen: String,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self> {
        let cfg = Config::builder()
            .set_default(
                "database_url",
                "postgresql://postgres:postgres@localhost:5432/postgre",
            )?
            .set_default("listen", "127.0.0.1:8080")?
            .add_source(Environment::default())
            .build()?;
        Ok(cfg.try_deserialize()?)
    }

    pub fn init_log(&self) -> Result<()> {
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env())
            .init();
        Ok(())
    }
}
