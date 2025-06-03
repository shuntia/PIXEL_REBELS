use futures::channel::mpsc::UnboundedReceiver;

use crate::{
    errors::Nresult,
    model::{GameModel, Status, player::Player},
};

pub mod ui;
pub mod world;

pub struct Renderer {}
impl Renderer {
    pub async fn render_ui(&mut self, stat: &Status, player: &Player) {
        ui::render_ui(stat, player).await
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
