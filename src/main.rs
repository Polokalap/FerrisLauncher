use crate::logger::{error, info, warn, zip_latest};
use crate::mojang::player::{fetch_profile, fetch_raw, get_texture_value};
use crate::skin::head::get_head;

pub mod logger;
pub mod mojang;
pub mod skin;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let _ = zip_latest().await;

    info("Loading main window").await;

    let ui = Window::new()?;
    let ui_weak = ui.as_weak();

    tokio::spawn(async {

        if let Err(e) = get_head("Polokalap").await {
            error(&format!("Failed to fetch skin/head: {}", e)).await;
        }

    });

    info("Window loaded!").await;
    ui.run()?;

    Ok(())

}
