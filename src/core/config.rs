use std::error::Error;

use config::{Config, File, FileFormat};
use serde::Deserialize;

const CONFIGURATION_FILE_PATH: &str = "config/application";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AppConfiguration {
    pub datasource: AppConfigurationDatasource,
    pub logger: AppConfigurationLogger
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AppConfigurationDatasource {
    pub postgres: String
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

pub fn load() -> Result<AppConfiguration, Box<dyn Error>> {
    // Create config builder
    let config_builder = Config::builder()
        .add_source(File::new(CONFIGURATION_FILE_PATH, FileFormat::Yaml));
    // Build configuration
    let configuration = config_builder
        .build()?
        .try_deserialize::<AppConfiguration>()?;
    // Return app configuration
    Ok(configuration)
}