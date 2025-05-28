use futures::channel::mpsc::UnboundedReceiver;

use crate::{
    errors::Nresult,
    model::{Status, player::Player},
};

mod ui;
mod world;

pub struct Renderer {}
impl Renderer {
    pub async fn render_ui(&mut self, stat: &Status) {
        ui::render_ui(stat).await
    }

    pub async fn render_world(&mut self, world: &crate::model::World, player: &Player) {
        world::render_world(&world, &player).await
    }

    pub fn render_debug(&mut self, rx: &mut UnboundedReceiver<String>) -> Nresult {
        ui::render_dbg(rx)
    }

    pub fn new() -> Renderer {
        Renderer {}
    }
}
