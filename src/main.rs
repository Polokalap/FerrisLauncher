use crate::launcher::ui_manager::init;
use crate::logger::{error, info, warn, zip_latest};

pub mod logger;
pub mod mojang;
pub mod skin;
pub mod launcher;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let _ = zip_latest().await;

    init().await?;

    Ok(())

}
