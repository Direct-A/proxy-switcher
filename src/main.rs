#![windows_subsystem = "windows"]

mod config;
mod logger;
mod network;
mod proxy;
mod state;
mod tray;

use anyhow::Result;
use msgbox::IconType;
use std::sync::Arc;
use tokio;

use config::AppConfig;
use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    let config = match config::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            msgbox::create("Load Configuration Error", &e.to_string(), IconType::Error)
                .unwrap_or_else(|_| eprintln!("Failed to load configuration: {}", e));
            return Err(e.into());
        }
    };

    match logger::setup_logger(&config) {
        Ok(()) => (),
        Err(e) => {
            msgbox::create("Logger Setup Error", &e.to_string(), IconType::Error)
                .unwrap_or_else(|_| eprintln!("Failed to setup logger: {}", e));
            return Err(e.into());
        }
    }

    let app_state = Arc::new(AppState::new(config));
    let _tray = tray::setup_tray(Arc::clone(&app_state))?;

    network::run_network_check_loop(app_state).await?;

    log::info!("Program exiting");
    Ok(())
}
