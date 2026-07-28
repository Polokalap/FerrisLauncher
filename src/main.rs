use std::iter::Once;
use slint::Image;
use crate::launcher::player_manager::set_player;
use crate::launcher::ui_manager::init;
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

    init().await?;

    Ok(())

}
