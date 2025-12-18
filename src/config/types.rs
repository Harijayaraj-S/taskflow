//! Config Types

use anyhow::Result;
use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub host: String,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let config = Config::builder()
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_"),
            )
            .build()?;

        let app: AppConfig = config.try_deserialize()?;

        Ok(app)
    }
}
