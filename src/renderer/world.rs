use crate::{
    assets::{self, get_map},
    errors::Nresult,
    model::World,
};
use macroquad::{miniquad::window::screen_size, prelude::*};

pub async fn render_world(world: &World) {
    clear_background(GRAY);
    draw_map(world)
        .await
        .unwrap_or_else(|err| error!("{}", err));
    draw_player();
    draw_enemies(world);
}

async fn draw_map(world: &World) -> Nresult {
    let max_size = Vec2::from(screen_size()).max_element();
    let tex = get_map(world.map)?;
    draw_texture_ex(
        tex,
        -world.player_pos.x,
        -world.player_pos.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                max_size * tex.width() / 1000.,
                max_size * tex.height() / 1000.,
            )),
            source: None,
            ..Default::default()
        },
    );
    Ok(())
}

fn draw_player() {
    draw_circle(screen_width() / 2., screen_height() / 2., 20., BLUE);
}

fn draw_enemies(world: &World) {
    for loc in &world.horde.loc {
        draw_circle(loc.x, loc.y, 8.0, RED);
    }
}
