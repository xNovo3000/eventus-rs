use std::error::Error;

pub mod api;
pub mod core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}