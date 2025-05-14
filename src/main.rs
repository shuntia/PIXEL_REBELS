#![deny(clippy::all)]

use errors::Nresult;
use macroquad::prelude::*;

mod assets;
mod errors;
mod input;
mod model;
mod renderer;
mod weapons;

#[macroquad::main("PIXEL_REBELS")]
async fn main() -> Nresult {
    debug!("Starting.");
    set_panic_handler(|msg, backtrace| async move {
        native_dialog::DialogBuilder::message()
            .set_text(
            format!("FATAL ERROR:\n{msg:#?}\nbacktrace:\n{backtrace}\n\nPlease report this to the devs at https://github.com/shuntia/pixel_rebels"))
            .set_level(native_dialog::MessageLevel::Error)
            .set_title(":(")
            .alert()
            .show()
            .unwrap_or_else(|_|panic!("The dialog failed. I don't know why.\nOriginal Message:{msg}\nOriginal Backtrace:\n{backtrace}"));
    });
    debug!("Panic hook set.");
    debug!("Starting asset loads.");
    assets::init_all().expect("Asset load failed.");
    debug!("Got through the asset loads. Thank god.");
    let mut model = model::GameModel::new();
    model.init()?;
    debug!("init done.");
    loop {
        model.input.kbd.update();
        model.update();
        model.call_render().await;
        next_frame().await;
    }
}
