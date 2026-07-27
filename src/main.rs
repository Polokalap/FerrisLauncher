use slint::Image;
use crate::launcher::player_manager::set_player;
use crate::logger::{error, info, warn, zip_latest};
use crate::mojang::player::{fetch_profile, fetch_raw, get_texture_value};
use crate::skin::head::{get_head, set_head};

pub mod logger;
pub mod mojang;
pub mod skin;
pub mod launcher;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let _ = zip_latest().await;

    info("Loading main window").await;

    let ui = Window::new()?;
    let ui_weak = ui.as_weak();

    tokio::spawn(async move {

        // set_head(ui_weak, "Polokalap").await;

        set_player(ui_weak, "Polokalap").await;

    });

    info("Window loaded!").await;
    ui.run()?;

    Ok(())

}
