//! Config Types

use anyhow::Result;
use config::Config;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub host: String,
    pub env: AppEnv,
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AppEnv {
    Live,
    Dev,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        println!("{:?}", std::env::var("APP_JWT_SECRET"));

        let config = Config::builder()
            .add_source(config::Environment::with_prefix("APP").try_parsing(true))
            .build()?;

        let app: AppConfig = config.try_deserialize()?;

        Ok(app)
    }

    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
