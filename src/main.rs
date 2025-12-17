// Main

use anyhow::Result;

mod app;
mod config;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, taskflow!");

    Ok(())
}
