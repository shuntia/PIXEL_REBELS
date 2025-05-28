#![deny(clippy::all)]

use errors::{GameError, Nresult};
use macroquad::prelude::*;
use util::{create_mpsc, set_hooks};

mod assets;
mod errors;
mod input;
mod model;
mod renderer;
mod util;

#[macroquad::main("PIXEL_REBELS")]
async fn main() -> Nresult {
    debug!("Starting.");
    set_hooks();
    debug!("Starting asset loads.");
    assets::init_all().expect("Asset load failed.");
    debug!("Got through the asset loads.");
    debug!("initializing model...");
    let mut model = model::GameModel::new();
    model.init()?;
    debug!("initializing debug systems...");
    let mut rx = create_mpsc().await?;
    let tx = util::DEBUG_TX.get().unwrap().clone();
    model
        .set_debug_tx(tx)
        .expect("Errored non non-erroring operation(???)");
    miniquad::window::show_mouse(false);
    debug!("init done.");
    loop {
        if util::INTERRUPT.load(std::sync::atomic::Ordering::Relaxed) {
            continue;
        }
        model.input.kbd.update();
        model.update();
        model.call_render().await;
        model.call_render_dbg(&mut rx)?;
        next_frame().await;
    }
}
