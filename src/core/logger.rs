use std::{error::Error, fmt};

use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;

use super::config::{AppConfigurationLogger, AppConfigurationLoggerOutput};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidAppConfigurationLogger(String);

impl fmt::Display for InvalidAppConfigurationLogger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a valid logger level", &self.0)
    }
}

impl Error for InvalidAppConfigurationLogger {}

pub fn init(config: &AppConfigurationLogger) -> Result<WorkerGuard, Box<dyn Error>> {
    // Get logger level
    let logger_level = match config.level.as_str() {
        "error" => Level::ERROR,
        "warn" => Level::WARN,
        "info" => Level::INFO,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        value => return Err(Box::new(InvalidAppConfigurationLogger(value.to_string())))
    };
    // Generate writer and worker guard
    let (logger_writer, logger_worker_guard) = match &config.output {
        AppConfigurationLoggerOutput::Console => {
            tracing_appender::non_blocking(std::io::stdout())
        },
        AppConfigurationLoggerOutput::File { path, file_name } => {
            let file_appender = tracing_appender::rolling::daily(path, file_name);
            tracing_appender::non_blocking(file_appender)    
        },
    };
    // Generate subscriber
    tracing_subscriber::fmt()
        .with_writer(logger_writer)
        .with_max_level(logger_level)
        .try_init()
        .map_err(|error| error as Box<dyn Error>)?;
    // Return worker guard
    Ok(logger_worker_guard)
}