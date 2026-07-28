use slint::{slint, ComponentHandle};
use crate::launcher::player_manager::set_player;
use crate::logger::info;
use crate::Window;

pub async fn init() -> Result<(), Box<dyn std::error::Error>> {

    info("Loading main window").await;

    let ui = Window::new()?;
    let ui_weak = ui.as_weak();

    tokio::spawn(async move {

        set_player(ui_weak, "Polokalap").await;

    });

    info("Window loaded!").await;
    ui.run()?;

    Ok(())

}