use std::{f32::consts::PI, ops::Deref};

use crate::{
    errors::Nresult,
    util::{self, get_mouse_angle, get_mouse_angle_centered},
};

use super::{World, damage::Damageable};

use macroquad::prelude::*;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

pub mod bullets;
pub mod weaponmap;

pub struct Weapon {
    pub power: f32,
    pub crit_chance: f32,
    pub cooldown: f32,
    pub cooldown_counter: f32,
    pub knockback: f32,
    pub stun: f32,
    pub animation: f32,
    pub kind: WeaponKind,
}

impl Weapon {
    pub fn attack(&mut self, world: &mut World) -> Nresult {
        let mangle = get_mouse_angle_centered(world) + PI;
        match self.kind {
            WeaponKind::Melee { range, .. } => {
                if self.cooldown_counter > 0. {
                    return Ok(());
                } else {
                    self.cooldown_counter = self.cooldown;
                }
                let find_y_center = world.player_pos.y;
                let bottom = world
                    .horde
                    .deref()
                    .par_iter()
                    .enumerate()
                    .find_first(|(_, el)| el.loc.y >= find_y_center - range);
                let top_idx;
                if let Some((found, _)) = bottom {
                    if let Some((idx, _)) = world.horde[found..]
                        .par_iter()
                        .enumerate()
                        .find_first(|(_, el)| el.loc.y > find_y_center + range)
                    {
                        top_idx = found + idx;
                    } else {
                        top_idx = world.horde.len();
                    }
                    world.horde[found..top_idx]
                        .par_iter_mut()
                        .filter(|el| self.kind.collides(world.player_pos, el.loc, mangle))
                        .for_each(|el| {
                            let _ = el.take_damage_raw(self.power);
                        });
                    Ok(())
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
    pub fn adjust_cooldown(&mut self) {
        if self.cooldown_counter > 0. {
            self.cooldown_counter -= get_frame_time();
        }
    }
}

pub enum WeaponKind {
    Projectile { bullet: u32, speed: f32 },
    Melee { range: f32, angle: f32 },
}

impl WeaponKind {
    /// checks collision of weapon with origin `origin`, and target `target` both `Vec2`s. angle is
    /// in f32.

    pub fn collides(&self, origin: Vec2, target: Vec2, center_angle: f32) -> bool {
        match self {
            Self::Melee { range, angle } => {
                if origin.distance(target) > *range {
                    false
                } else {
                    let delta = target - origin;
                    let enemy_angle = PI - delta.to_angle();
                    let delta_angle = enemy_angle - center_angle;
                    delta_angle.abs() % (2. * PI) <= *angle / 2.
                        || (delta_angle + 2. * PI).abs() <= angle / 2.
                }
            }
            _ => false,
        }
    }
}
