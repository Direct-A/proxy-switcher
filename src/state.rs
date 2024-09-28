use std::sync::atomic::AtomicBool;
use crate::AppConfig;

pub struct AppState {
    pub is_proxy_enabled: AtomicBool,
    pub should_exit: AtomicBool,
    pub manually_disabled: AtomicBool,
    pub config: AppConfig,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            is_proxy_enabled: AtomicBool::new(false),
            should_exit: AtomicBool::new(false),
            manually_disabled: AtomicBool::new(false),
            config,
        }
    }
}
