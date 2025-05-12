#![deny(clippy::all)]

use camera::{UICamera, WorldCamera};
use errors::Nresult;
use macroquad::prelude::*;

mod camera;
mod errors;
mod input;
mod model;
mod renderer;
mod weapons;

#[macroquad::main("PIXEL_REBELS")]
async fn main() -> Nresult {
    debug!("Starting.");
    set_panic_handler(|msg, backtrace| async move {
        native_dialog::DialogBuilder::message().set_text(format!("FATAL ERROR: {msg}\nbacktrace:\n{backtrace}\n\nPlease report this to the devs at https://github.com/shuntia/pixel_rebels")).set_level(native_dialog::MessageLevel::Error).set_title(":(").alert().show().unwrap_or_else(|_|panic!("The dialog failed. I don't know why.\nOriginal Message:{msg}\nOriginal Backtrace:\n{backtrace}"));
    });
    debug!("Panic hook set.");
    let world = WorldCamera::default();
    let mut model = model::GameModel::new();
    model.init()?;
    debug!("init done.");
    loop {
        UICamera::activate();
        renderer::render_ui(&model).await;
        if let Some(s) = &model.world {
            world.activate();
            renderer::render_world(s).await;
        }
        next_frame().await;
    }
}
