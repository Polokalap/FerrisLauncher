use std::fmt::format;
use slint::{ComponentHandle, SharedString};
use crate::mojang::player::{fetch_raw, User};
use crate::skin::head::{get_head, set_head};
use crate::{AccountMenuState, Window};
use crate::logger::info;

pub async fn set_player(ui_weak: slint::Weak<Window>, name: &str) {

    info(format!("Setting player {}", name).as_str()).await;

    set_head(ui_weak.clone(), name).await;
    set_name(ui_weak.clone(), name).await;

}

pub async fn set_name(ui_weak: slint::Weak<Window>, name: &str) {

    info(format!("Setting name {}", name).as_str()).await;

    let user: User = fetch_raw(String::from(name)).await.unwrap();

    info(format!("User online: {}", user.online).as_str()).await;

    let _ = ui_weak.upgrade_in_event_loop(move |ui| {
        ui.global::<AccountMenuState>().set_player_name(SharedString::from(user.name));
    });

}