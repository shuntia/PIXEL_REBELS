use crate::{
    assets::{self, get_map},
    errors::{Nresult, Result},
    model::{World, player::Player, weapons::Weapon},
    util::get_mouse_angle,
};
use macroquad::{miniquad::window::screen_size, prelude::*};

pub async fn render_world(world: &World, player: &Player) {
    clear_background(GRAY);
    draw_map(world)
        .await
        .unwrap_or_else(|err| error!("{}", err));
    draw_player();
    draw_weapon(&player.weapon);
    draw_enemies(world);
}

async fn draw_map(world: &World) -> Nresult {
    let max_size = get_world_scale();
    let tex = get_map(world.map)?;
    let (w, h) = screen_size();
    draw_rectangle(
        -world.player_pos.x * max_size + w / 2.,
        -world.player_pos.y * max_size + h / 2.,
        tex.width() * max_size,
        tex.height() * max_size,
        GRAY,
    );
    draw_texture_ex(
        tex,
        -world.player_pos.x * max_size + w / 2.,
        -world.player_pos.y * max_size + h / 2.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(max_size * tex.width(), max_size * tex.height())),
            source: None,
            ..Default::default()
        },
    );
    Ok(())
}

pub fn get_map_size(map: u32) -> Result<Vec2> {
    let tex = get_map(map)?;
    let max_size = get_world_scale();
    Ok(vec2(max_size * tex.width(), max_size * tex.height()))
}

fn draw_player() {
    draw_circle(screen_width() / 2., screen_height() / 2., 20., BLUE);
}

fn draw_weapon(weapon: &Weapon) -> Nresult {
    match weapon.kind {
        crate::model::weapons::WeaponKind::Melee { range, angle } => {
            let mangle = get_mouse_angle();
            let center = Vec2::from(screen_size()) / 2.;
            let wscale = get_world_scale();
            draw_line(
                center.x,
                center.y,
                center.x + range * (mangle + angle / 2.).cos() * wscale,
                center.y - range * (mangle + angle / 2.).sin() * wscale,
                5.,
                RED,
            );
            draw_line(
                center.x,
                center.y,
                center.x + (range * wscale + 5.) * (mangle - angle / 2.).cos(),
                center.y - (range * wscale + 5.) * (mangle - angle / 2.).sin(),
                5.,
                RED,
            );
            if weapon.cooldown_counter > 0. {
                draw_line(
                    center.x,
                    center.y,
                    center.x + range * mangle.cos() * wscale,
                    center.y - range * mangle.sin() * wscale,
                    5.,
                    if weapon.cooldown_counter == weapon.cooldown {
                        GREEN
                    } else {
                        if is_mouse_button_down(MouseButton::Left) {
                            BLUE
                        } else {
                            RED
                        }
                    },
                );
            }

            draw_arc(
                screen_width() / 2.,
                screen_height() / 2.,
                50,
                range * get_world_scale(),
                -(mangle + angle / 2.).to_degrees(),
                5.,
                angle.to_degrees(),
                RED,
            );
        }
        crate::model::weapons::WeaponKind::Projectile { bullet, speed } => {}
    }
    Ok(())
}

const SCALE_DIV: f32 = 250.;

pub fn get_world_scale() -> f32 {
    Vec2::from(screen_size()).max_element() / SCALE_DIV
}
pub fn get_min_world_scale() -> f32 {
    Vec2::from(screen_size()).min_element() / SCALE_DIV
}

fn draw_enemies(world: &World) {
    for i in &world.horde {
        //culling
        let tex = &assets::SPRITES[i.id as usize];
        let mut actual_loc = get_actual_loc(i.loc, world.player_pos);
        let tex_size = tex.size();

        actual_loc -= tex_size / 2.;
        if actual_loc
            .cmple(Vec2::from(screen_size()) - get_world_scale())
            .all()
            && actual_loc.cmpge(Vec2::ZERO + get_world_scale()).all()
        {
            //if in view THEN render
            draw_texture_ex(
                &tex,
                actual_loc.x,
                actual_loc.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(tex.size() * get_world_scale() / 10.),
                    ..Default::default()
                },
            );
        }
    }
}

fn get_actual_loc(loc: Vec2, player_pos: Vec2) -> Vec2 {
    get_world_scale() * (loc - player_pos) + Vec2::from(screen_size()) / 2.
}
