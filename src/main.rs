use crate::logger::{error, info, warn, zip_latest};

pub mod logger;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {

    let _ = zip_latest().await;

    info("Wow this is so fast!").await;
    warn("I love Rust!").await;
    error("no I actually don't really like it").await;

    let ui = Window::new()?;
    let ui_weak = ui.as_weak();

    ui.run()

}
