use crate::AppState;
use anyhow::Ok;
use anyhow::Result;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tap::TapFallible;
use winreg::enums::*;
use winreg::RegKey;

const HKCU: RegKey = RegKey::predef(HKEY_CURRENT_USER);
const REG_PATH: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings";

enum ProxyValue {
    Switcher(u32),
    Server(String),
}

pub fn is_proxy_enabled() -> Result<bool> {
    let key = HKCU
        .open_subkey(REG_PATH)
        .tap_err(|e| log::error!("Failed to open registry key: {:#}", e))?;
    let enabled: u32 = key
        .get_value("ProxyEnable")
        .tap_err(|e| log::error!("Failed to get ProxyEnable value: {:#}", e))?;
    Ok(enabled != 0)
}

fn proxy_switcher(name: &str, value: ProxyValue) -> Result<()> {
    let key = HKCU
        .open_subkey_with_flags(REG_PATH, KEY_ALL_ACCESS)
        .tap_err(|e| log::error!("Failed to open registry key with write access\n{:#}", e))?;

    match (name, value) {
        ("ProxyEnable", ProxyValue::Switcher(val)) => key
            .set_value("ProxyEnable", &val)
            .tap_err(|e| log::error!("Failed to set ProxyEnable value\n{:#}", e))?,
        ("ProxyServer", ProxyValue::Server(val)) => key
            .set_value("ProxyServer", &val)
            .tap_err(|e| log::error!("Failed to set ProxyServer value\n{:#}", e))?,
        _ => log::error!("Wrong value name or type: {}", name),
    };
    Ok(())
}

pub fn enable_proxy(host: &str, port: u16) -> Result<()> {
    proxy_switcher("ProxyEnable", ProxyValue::Switcher(1))?;
    proxy_switcher("ProxyServer", ProxyValue::Server(format!("{}:{}", host, port)))?;
    log::info!("Automatically enabled proxy");
    Ok(())
}

pub fn disable_proxy() -> Result<()> {
    proxy_switcher("ProxyEnable", ProxyValue::Switcher(0))?;
    log::info!("Automatically disabled proxy");
    Ok(())
}

pub fn enable_proxy_manually(app_state: &Arc<AppState>) -> Result<()> {
    proxy_switcher("ProxyEnable", ProxyValue::Switcher(1))?;
    log::info!("Manually enabled proxy");
    app_state.is_proxy_enabled.store(true, Ordering::Relaxed);
    app_state.manually_disabled.store(false, Ordering::Relaxed);
    Ok(())
}

pub fn disable_proxy_manually(app_state: &Arc<AppState>) -> Result<()> {
    proxy_switcher("ProxyEnable", ProxyValue::Switcher(0))?;
    log::info!("Manually disabled proxy");
    app_state.is_proxy_enabled.store(false, Ordering::Relaxed);
    app_state.manually_disabled.store(true, Ordering::Relaxed);
    Ok(())
}
