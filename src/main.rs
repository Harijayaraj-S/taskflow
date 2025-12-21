//! Main

use anyhow::Result;
use axum::Extension;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt};

use crate::{config::types::AppConfig, state::types::AppState};

mod app;
mod config;
mod domain;
mod middleware;
mod repositories;
mod routes;
mod service;
mod state;

fn tracing_int() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_level(true)
        .compact()
        .init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_int()?;

    let config = AppConfig::new()?;
    let addr = config.get_addr();
    let listener = TcpListener::bind(addr).await?;
    let state = AppState::new(&config).await?;

    println!("listening on {}", listener.local_addr()?);
    tracing::debug!(
        "listening on {} in Env: {:?}",
        listener.local_addr()?,
        config.env
    );

    let app = app::build().layer(Extension(state));
    axum::serve(listener, app).await?;

    Ok(())
}
