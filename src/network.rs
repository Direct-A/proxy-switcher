use crate::proxy;
use crate::AppState;

use anyhow::{Context, Result};
use msgbox::IconType;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use sysinfo::System;
use tokio::time::{interval, Duration};

pub async fn run_network_check_loop(app_state: Arc<AppState>) -> Result<()> {
    let mut sys = System::new_all();
    let mut interval = interval(Duration::from_secs(app_state.config.check_interval));

    while !app_state.should_exit.load(Ordering::Relaxed) {
        interval.tick().await;
        if let Err(e) =
            process_network_state(&app_state.config.proxy_network, &mut sys, &app_state).await
        {
            log::error!("Error processing network state: {}", e);
        }
    }

    Ok(())
}

async fn process_network_state(
    proxy_network: &str,
    sys: &mut System,
    app_state: &Arc<AppState>,
) -> Result<()> {
    log::debug!("Checking network state");

    if app_state.manually_disabled.load(Ordering::Relaxed) {
        return Ok(());
    }

    let interfaces = match NetworkInterface::show() {
        Ok(interfaces) => interfaces,
        Err(e) => {
            msgbox::create(
                "Get NetworkInterface Error",
                &e.to_string(),
                IconType::Error,
            )
            .unwrap_or_else(|_| eprintln!("Failed to get network interfaces: {}", e));
            return Err(e.into());
        }
    };
    let is_proxy_net = interfaces.iter().any(|interface| {
        log::debug!("Testing interface: {}", interface.name);
        interface.name == proxy_network
    });

    sys.refresh_all();
    let proxy_enabled = match proxy::is_proxy_enabled() {
        Ok(enabled) => enabled,
        Err(e) => {
            msgbox::create("Check Proxy Status Error", &e.to_string(), IconType::Error)
                .unwrap_or_else(|_| eprintln!("Failed to check if proxy is enabled: {}", e));
            return Err(e.into());
        }
    };

    let current_proxy_state = app_state.is_proxy_enabled.load(Ordering::Relaxed);
    let should_enable = is_proxy_net && !proxy_enabled && !current_proxy_state;
    let should_disable = !is_proxy_net && proxy_enabled && current_proxy_state;

    if should_enable {
        proxy::enable_proxy(&app_state.config.proxy_host, app_state.config.proxy_port)?;
        app_state.is_proxy_enabled.store(true, Ordering::Relaxed);
        log::info!("Enabled proxy for {}", proxy_network);
    } else if should_disable {
        proxy::disable_proxy().context("Failed to disable proxy")?;
        app_state.is_proxy_enabled.store(false, Ordering::Relaxed);
        log::info!("Disabled proxy for other network");
    }

    Ok(())
}
