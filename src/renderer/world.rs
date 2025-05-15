use crate::{
    assets::{self, get_map},
    errors::{Nresult, Result},
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
    let max_size = get_world_scale();
    let tex = get_map(world.map)?;
    draw_texture_ex(
        tex,
        -world.player_pos.x,
        -world.player_pos.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(max_size * tex.width(), max_size * tex.height())),
            source: None,
            ..Default::default()
        },
    );
    Ok(())
}

fn get_map_size(map: u32) -> Result<Vec2> {
    let tex = get_map(map)?;
    let max_size = get_world_scale();
    Ok(vec2(max_size * tex.width(), max_size * tex.height()))
}

fn draw_player() {
    draw_circle(screen_width() / 2., screen_height() / 2., 20., BLUE);
}

const SCALE_DIV: f32 = 500.;

pub fn get_world_scale() -> f32 {
    Vec2::from(screen_size()).max_element() / SCALE_DIV
}
pub fn get_min_world_scale() -> f32 {
    Vec2::from(screen_size()).min_element() / SCALE_DIV
}

fn draw_enemies(world: &World) {
    for loc in &world.horde.loc {
        //culling
        let actual_loc = get_actual_loc(*loc);
        if actual_loc
            .cmple(Vec2::from(screen_size()) - get_world_scale())
            .all()
            && actual_loc.cmpge(Vec2::ZERO + get_world_scale()).all()
        {
            //if in view THEN render
            draw_circle(actual_loc.x, actual_loc.y, get_world_scale(), RED);
        }
    }
}

fn get_actual_loc(loc: Vec2) -> Vec2 {
    loc * get_world_scale()
}
