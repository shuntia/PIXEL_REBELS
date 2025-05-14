use crate::model::World;
use macroquad::prelude::*;

pub async fn render_world(world: &World) {
    clear_background(DARKGREEN);
    draw_map(world);
    draw_player(world.player_pos);
    draw_enemies(world);
}

fn draw_map(world: &World) {}

fn draw_player(pos: Vec2) {
    draw_circle(pos.x, pos.y, 10.0, BLUE);
}

fn draw_enemies(world: &World) {
    for loc in &world.horde.loc {
        draw_circle(loc.x, loc.y, 8.0, RED);
    }
}
