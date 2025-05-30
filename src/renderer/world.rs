use std::sync::atomic::AtomicBool;

use crate::{
    assets::{self, get_map},
    errors::{Nresult, Result},
    model::{World, player::Player, weapons::Weapon},
    util::{PlayerAnimation, get_mouse_angle, get_mouse_angle_centered},
};
use macroquad::{miniquad::window::screen_size, prelude::*};

pub async fn render_world(world: &World, player: &Player) {
    let tex = get_map(world.map).unwrap();
    let scale = get_world_scale() / 10.;
    let screen = vec2(screen_width(), screen_height());
    let half_screen = screen / 2.0;
    let map_size = vec2(tex.width() as f32, tex.height() as f32);

    let min_cam = half_screen / scale;
    let max_cam = map_size - min_cam;
    let camera_pos = world.player_pos.clamp(min_cam, max_cam);

    draw_map(tex, camera_pos, scale)
        .await
        .unwrap_or_else(|err| error!("{}", err));
    draw_weapon(&player.weapon, world.player_pos, camera_pos, scale, world);
    draw_player(world, camera_pos, scale);
    draw_enemies(world, camera_pos, scale);
}

async fn draw_map(tex: &Texture2D, camera_pos: Vec2, scale: f32) -> Nresult {
    let screen = vec2(screen_width(), screen_height());
    let draw_pos = -camera_pos * scale + screen / 2.0;

    draw_texture_ex(
        tex,
        draw_pos.x,
        draw_pos.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(scale * tex.width(), scale * tex.height())),
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

fn draw_player(world: &World, camera_pos: Vec2, scale: f32) {
    let sprite = &assets::SPRITES[PlayerAnimation::Idle as usize];
    let screen = vec2(screen_width(), screen_height());
    let half_screen = screen / 2.0;

    let player_screen_pos = (world.player_pos - camera_pos) * scale + half_screen;
    let size = sprite.size() * scale;

    draw_texture_ex(
        sprite,
        player_screen_pos.x - size.x / 2.,
        player_screen_pos.y - size.y / 2.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(size),
            flip_x: is_key_down(KeyCode::A),
            ..Default::default()
        },
    );
}

fn draw_weapon(
    weapon: &Weapon,
    player_pos: Vec2,
    camera_pos: Vec2,
    scale: f32,
    world: &World,
) -> Nresult {
    let center = (player_pos - camera_pos) * scale + vec2(screen_width(), screen_height()) / 2.0;
    let wscale = scale;

    match weapon.kind {
        crate::model::weapons::WeaponKind::Melee { range, angle } => {
            let mangle = get_mouse_angle_centered(world);

            draw_line(
                center.x,
                center.y,
                center.x + range * (mangle + angle / 2.0).cos() * wscale,
                center.y - range * (mangle + angle / 2.0).sin() * wscale,
                5.0,
                RED,
            );
            draw_line(
                center.x,
                center.y,
                center.x + (range * wscale + 5.0) * (mangle - angle / 2.0).cos(),
                center.y - (range * wscale + 5.0) * (mangle - angle / 2.0).sin(),
                5.0,
                RED,
            );

            if weapon.cooldown_counter > 0.0 {
                draw_line(
                    center.x,
                    center.y,
                    center.x + range * mangle.cos() * wscale,
                    center.y - range * mangle.sin() * wscale,
                    5.0,
                    if weapon.cooldown_counter == weapon.cooldown {
                        GREEN
                    } else if is_mouse_button_down(MouseButton::Left) {
                        BLUE
                    } else {
                        RED
                    },
                );
            }

            draw_arc(
                center.x,
                center.y,
                50,
                range * wscale,
                -(mangle + angle / 2.0).to_degrees(),
                5.0,
                angle.to_degrees(),
                RED,
            );
        }
        crate::model::weapons::WeaponKind::Projectile { .. } => {
            // Add logic here if you plan to draw projectiles.
        }
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

fn draw_enemies(world: &World, camera_pos: Vec2, scale: f32) {
    let screen = vec2(screen_width(), screen_height());
    let half_screen = screen / 2.0;

    for i in &world.horde {
        let tex = &assets::ENEMIES[i.id as usize];
        let enemy_screen_pos = (i.loc - camera_pos) * scale + half_screen;
        let tex_size = tex.size() * scale;
        let draw_pos = enemy_screen_pos - tex_size / 2.0;

        if draw_pos.cmple(screen - scale).all() && draw_pos.cmpge(Vec2::ONE * scale).all() {
            draw_texture_ex(
                tex,
                draw_pos.x,
                draw_pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(tex_size),
                    ..Default::default()
                },
            );
        }
    }
}

pub fn get_camera_pos(player_pos: Vec2, map_id: u32) -> Vec2 {
    let tex = get_map(map_id).expect("Invalid map id");
    let scale = get_world_scale() / 10.0;
    let screen = vec2(screen_width(), screen_height());
    let half_screen = screen / 2.0;

    let map_size = vec2(tex.width() as f32, tex.height() as f32);
    let min_cam = half_screen / scale;
    let max_cam = map_size - min_cam;

    player_pos.clamp(min_cam, max_cam)
}

pub fn get_player_screen_pos(player_pos: Vec2, map_id: u32) -> Vec2 {
    let cam_pos = get_camera_pos(player_pos, map_id);
    let scale = get_world_scale() / 10.0;
    let screen = vec2(screen_width(), screen_height());
    let half_screen = screen / 2.0;

    (player_pos - cam_pos) * scale + half_screen
}

pub fn player_on_scr(world: &World) -> Vec2 {
    let tex = get_map(world.map).unwrap();
    let scale = get_world_scale() / 10.;
    let screen = vec2(screen_width(), screen_height());
    let half_screen = screen / 2.0;
    let map_size = vec2(tex.width() as f32, tex.height() as f32);

    let min_cam = half_screen / scale;
    let max_cam = map_size - min_cam;
    let camera_pos = world.player_pos.clamp(min_cam, max_cam);

    (world.player_pos - camera_pos) * scale + vec2(screen_width(), screen_height()) / 2.0
}
