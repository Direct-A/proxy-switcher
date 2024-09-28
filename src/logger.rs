use anyhow::Result;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config as Log4rsConfig, Root};
use chrono::Local;
use msgbox::IconType;
use crate::AppConfig;

pub fn setup_logger(config: &AppConfig) -> Result<()> {
    let log_dir = std::env::current_exe()?.parent().unwrap().to_path_buf();
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let log_file = log_dir.join(format!("proxy_switcher_{}.log", timestamp));

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
        .build(log_file)?;

    let log_level = match config.log_level.to_lowercase().as_str() {
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    };

    let config = Log4rsConfig::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(log_level))?;

    if let Err(e) = log4rs::init_config(config) {
        let error_message = format!("Failed to initialize logger: {}", e);
        msgbox::create("Logger Initialization Error", &error_message, IconType::Error)
            .unwrap_or_else(|_| eprintln!("Failed to show error message box: {}", error_message));
        return Err(e.into());
    }

    log::info!("Logger initialized");
    Ok(())
}
