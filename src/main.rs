#![deny(clippy::all)]

use errors::Nresult;
use macroquad::prelude::*;
use signal_hook::low_level::exit;

mod assets;
mod errors;
mod input;
mod model;
mod renderer;

#[macroquad::main("PIXEL_REBELS")]
async fn main() -> Nresult {
    debug!("Starting.");
    set_panic_handler(|msg, backtrace| async move {
        native_dialog::DialogBuilder::message()
            .set_text(
            format!("FATAL ERROR:\n{msg}\nbacktrace:\n{backtrace}\n\nPlease report this to the devs at https://github.com/shuntia/pixel_rebels"))
            .set_level(native_dialog::MessageLevel::Error)
            .set_title(":(")
            .alert()
            .show()
            .unwrap_or_else(|_|panic!("The dialog failed. I don't know why.\nOriginal Message:{msg}\nOriginal Backtrace:\n{backtrace}"));
    });
    debug!("Panic hook set.");
    ctrlc::set_handler(|| {
        native_dialog::DialogBuilder::message()
            .set_text("FATAL: control-c\n\nProcess killed by user")
            .set_title("X(")
            .set_level(native_dialog::MessageLevel::Error)
            .alert()
            .show()
            .unwrap_or_else(|_| {
                panic!("The dialog failed.\nFATAL: control-c\n\nProcess killed by user")
            });
        exit(130);
    })
    .unwrap_or_else(|_| error!("Failed to set ctrl-c hook."));
    unsafe {
        if let Err(_) = signal_hook::low_level::register(15, || {
            native_dialog::DialogBuilder::message()
                .set_text("FATAL: SIGTERM\n\nProcess killed by user")
                .set_title("X(")
                .set_level(native_dialog::MessageLevel::Error)
                .alert()
                .show()
                .unwrap_or_else(|_| {
                    panic!("The dialog failed.\nFATAL: SIGTERM\n\nProcess killed by user")
                });
            exit(133);
        }) {
            error!("Failed to set SIGTERM hook.");
        };
        if let Err(_) = signal_hook::low_level::register(6, || {
            native_dialog::DialogBuilder::message()
                .set_text("FATAL: SIGABRT\n\nProcess aborted itself.")
                .set_title(":O")
                .set_level(native_dialog::MessageLevel::Error)
                .alert()
                .show()
                .unwrap_or_else(|_| {
                    panic!("The dialog failed.\nFATAL: SIGABRT\n\nProcess aborted itself.")
                });
            exit(134);
        }) {
            error!("Failed to set SIGABRT hook.");
        };
    };
    debug!("Starting asset loads.");
    assets::init_all().expect("Asset load failed.");
    debug!("Got through the asset loads. Thank god.");
    let mut model = model::GameModel::new();
    model.init()?;
    miniquad::window::show_mouse(false);
    debug!("init done.");
    loop {
        model.input.kbd.update();
        model.update();
        model.call_render().await;
        next_frame().await;
    }
}
