use crate::{config, APP_LOG_DIR};
use simple_log::LogConfigBuilder;

pub fn init_logging() -> Result<(), String> {
    let log_file_path = format!(
        "{}/{}",
        APP_LOG_DIR.get().unwrap().display(),
        config::LOG_FILE_NAME
    );

    let log_config = LogConfigBuilder::builder()
        .path(&log_file_path)
        .size(1 * 1024 * 1024)
        .roll_count(3)
        .time_format("%Y-%m-%d %H:%M:%S")
        .level("info")
        .output_file()
        .output_console()
        .build();

    simple_log::new(log_config)
        .map_err(|e| format!("Cannot initialize logging: {}", e))?;

    Ok(())
}
