use crate::model::{GameModel, World};

mod ui;
mod world;

pub async fn render_ui(game: &GameModel) {
    ui::render_ui(game).await;
}

pub async fn render_world(world: &World) {
    world::render_world(world).await;
}
