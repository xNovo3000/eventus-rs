use std::error::Error;

pub mod config;
pub mod database;
pub mod logger;

#[derive(Debug)]
pub struct AppState;

pub async fn init() -> Result<(), Box<dyn Error>> {
    Ok(())
}