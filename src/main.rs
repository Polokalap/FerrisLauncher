use crate::logger::{error, info, warn, zip_latest};

pub mod logger;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {

    let _ = zip_latest();

    info("This is very cool!").await;
    warn("wow yellow! scarryyy").await;
    error("red error").await;

    let ui = Window::new()?;
    let ui_weak = ui.as_weak();

    ui.run()

}
