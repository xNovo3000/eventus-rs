use core::fmt;
use std::error::Error;

use config::{Config, File, FileFormat};
use diesel_async::{pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager}, AsyncPgConnection};
use serde::Deserialize;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;

const CONFIGURATION_FILE_PATH: &str = "config/application";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AppConfiguration {
    pub datasource: Option<AppConfigurationDatasource>,
    pub logger: Option<AppConfigurationLogger>,
    pub profile: String
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AppConfigurationDatasource {
    pub postgres: AppConfigurationDatasourcePostgres
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AppConfigurationDatasourcePostgres {
    pub hostname: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub database: String
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AppConfigurationLogger {
    pub level: String,
    pub output: AppConfigurationLoggerOutput
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "mode")]
pub enum AppConfigurationLoggerOutput {
    Console,
    File {
        path: String,
        file_name: String
    }
}

pub fn load_config() -> Result<AppConfiguration, Box<dyn Error>> {
    // Create default config builder
    let mut config_builder = Config::builder()
        .add_source(File::new(CONFIGURATION_FILE_PATH, FileFormat::Yaml));
    // Build first configuration
    let mut configuration = config_builder
        .clone()
        .build()?
        .try_deserialize::<AppConfiguration>()?;
    // Create profile-specific configuration
    let profile_configuration_file_path = CONFIGURATION_FILE_PATH.to_owned() + "-" + &configuration.profile;
    config_builder = config_builder
        .add_source(File::new(&profile_configuration_file_path, FileFormat::Yaml));
    // Override witf profile-specific configuration (if exists)
    if let Ok(config) = config_builder.build() {
        configuration = config.try_deserialize::<AppConfiguration>()?;
    }
    // Return app configuration
    Ok(configuration)
}

pub type AsyncPgConnectionPool = Pool<AsyncPgConnection>;

pub async fn create_database_connection_pool(config: &AppConfigurationDatasourcePostgres) -> Result<AsyncPgConnectionPool, Box<dyn Error>> {
    // Create URL from configuration
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user,
        config.password,
        config.hostname,
        config.port,
        config.database
    );
    // Create config
    let pool_config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    // Create pool
    let pool = Pool::builder(pool_config)
        .max_size(5)
        .build()?;
    // Test connection
    let _ = pool.get().await?;
    // Ok
    Ok(pool)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidAppConfigurationLogger {
    value: String
}

impl fmt::Display for InvalidAppConfigurationLogger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a valid logger level", &self.value)
    }
}

impl Error for InvalidAppConfigurationLogger {}

pub fn init_logger(config: &AppConfigurationLogger) -> Result<WorkerGuard, Box<dyn Error>> {
    // Get logger level
    let logger_level = match config.level.as_str() {
        "error" => Level::ERROR,
        "warn" => Level::WARN,
        "info" => Level::INFO,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        value => return Err(Box::new(InvalidAppConfigurationLogger { value: value.to_string() }))
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