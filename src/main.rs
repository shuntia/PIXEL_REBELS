#![deny(clippy::all)]

use errors::{GameError, Nresult};
use macroquad::prelude::*;
use util::{DEBUG_TX, create_mpsc, set_hooks};

mod assets;
mod errors;
mod input;
mod model;
mod renderer;
mod util;

#[macroquad::main("PIXEL_REBELS")]
async fn main() -> Nresult {
    println!("Starting.");
    print!("Setting hooks...");
    set_hooks();
    println!(" OK");
    print!("Starting asset loads...");
    assets::init_all().expect("Asset load failed.");
    println!(" OK");
    print!("initializing model...");
    let mut model = model::GameModel::new();
    model.init()?;
    println!(" OK");
    print!("initializing debug systems...");
    let mut rx = create_mpsc().await?;
    let tx = util::DEBUG_TX.get().unwrap().clone();
    model
        .set_debug_tx(tx)
        .expect("Errored non non-erroring operation(???)");
    println!(" OK");
    miniquad::window::show_mouse(false);
    println!("ALL CLEAR");
    loop {
        if util::INTERRUPT.load(std::sync::atomic::Ordering::Relaxed) {
            continue;
        }
        let _ = DEBUG_TX
            .get()
            .unwrap()
            .unbounded_send(format!("FPS: {}", get_fps()));
        model.input.kbd.update();
        model.update();
        model.call_render().await;
        model.call_render_dbg(&mut rx)?;
        next_frame().await;
    }
}
