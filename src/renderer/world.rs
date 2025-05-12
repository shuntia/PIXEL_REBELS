use crate::model::World;
use macroquad::prelude::*;

pub async fn render_world(world: &World) {
    clear_background(DARKGREEN);
    draw_player(world.player_pos);
    draw_enemies(&world);
}

fn draw_player(pos: Vec2) {
    draw_circle(pos.x, pos.y, 10.0, BLUE);
}

fn draw_enemies(world: &World) {
    for loc in &world.hoarde.loc {
        draw_circle(loc.x, loc.y, 8.0, RED);
    }
}
