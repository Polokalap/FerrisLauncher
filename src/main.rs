use crate::logger::{error, info, warn, zip_latest};

pub mod logger;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {

    zip_latest();

    info("meow");
    warn("meow");
    error("meow");

    let ui = Window::new()?;
    let ui_weak = ui.as_weak();

    ui.run()

}
