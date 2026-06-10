//! resource-governor — Sorabbyngo OS dynamic resource allocation

mod classification;
mod cpu;
mod memory;
mod gpu;
mod io;
mod network;
mod power;

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("resource-governor starting");
    Ok(())
}
