use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use config::{Config, File};


/*
TODO：首次启动自动创建 config.toml，并提供示例 config.toml
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub proxy_network: String,
    pub proxy_host: String,
    pub proxy_port: u16,
    pub check_interval: u64,
    pub log_level: String,
}

// 把 config.toml 里的数据反序列化成 AppConfig 结构体
pub fn load_config() -> Result<AppConfig> {
    let config = Config::builder()
        .add_source(File::with_name("config.toml").required(false))
        .build()?;

    config.try_deserialize().context("Failed to deserialize configuration")
}
