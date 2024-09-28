use crate::proxy;
use crate::AppState;
use anyhow::Result;
use std::sync::Arc;
use tap::TapFallible;
use tray_item::{IconSource, TrayItem};

pub fn setup_tray(app_state: Arc<AppState>) -> Result<TrayItem> {
    log::info!("Setting up tray");

    let mut tray = TrayItem::new("Proxy Switcher", IconSource::Resource("tray-icon"))
        .tap_err(|e| log::error!("Failed to create TrayItem\n{:#}", e))?;

    let state_clone = Arc::clone(&app_state);
    tray.add_menu_item("Enable Proxy", move || {
        if let Err(e) = proxy::enable_proxy_manually(&state_clone) {
            log::error!("Failed to enable proxy: {}", e);
        }
    })?;

    let state_clone = Arc::clone(&app_state);
    tray.add_menu_item("Disable Proxy", move || {
        if let Err(e) = proxy::disable_proxy_manually(&state_clone) {
            log::error!("Failed to disable proxy: {}", e);
        }
    })?;

    let state_clone = Arc::clone(&app_state);
    tray.add_menu_item("Exit", move || {
        state_clone
            .should_exit
            .store(true, std::sync::atomic::Ordering::Relaxed);
    })?;

    log::info!("Tray setup completed");
    Ok(tray)
}
