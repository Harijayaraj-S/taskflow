//! Main

use anyhow::Result;
use tokio::net::TcpListener;

use crate::config::types::AppConfig;

mod app;
mod config;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let config = AppConfig::new()?;
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(addr).await?;

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app::build()).await?;

    Ok(())
}
