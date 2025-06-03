use std::ops::{Deref, DerefMut};

use ::rand::random;
use macroquad::{prelude::*, telemetry::frame};
use rayon::prelude::*;

use crate::errors::Nresult;

use super::damage::{Damageable, Effects};

pub mod enemymap;

#[derive(Clone)]
pub struct Enemy {
    pub id: u32,
    pub loc: Vec2,
    pub velocity: Vec2,
    pub animation: f32,
    pub health: f32,
    pub stun_timer: f32,
}

impl Damageable for Enemy {
    fn take_damage(&mut self, damage: super::damage::Damage, effects: Vec<Effects>) -> Nresult {
        self.take_damage_raw(damage.evaluate()?)
    }
    fn take_damage_raw(&mut self, damage: f32) -> Nresult {
        self.health -= damage;
        Ok(())
    }
}

pub struct HordeEnemies {
    pub enemies: Vec<Enemy>,
}

impl Deref for HordeEnemies {
    type Target = Vec<Enemy>;
    fn deref(&self) -> &Self::Target {
        &self.enemies
    }
}

impl DerefMut for HordeEnemies {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.enemies
    }
}

impl IntoIterator for HordeEnemies {
    type Item = Enemy;
    type IntoIter = std::vec::IntoIter<Enemy>;

    fn into_iter(self) -> Self::IntoIter {
        self.enemies.into_iter()
    }
}

impl<'a> IntoIterator for &'a HordeEnemies {
    type Item = &'a Enemy;
    type IntoIter = std::slice::Iter<'a, Enemy>;

    fn into_iter(self) -> Self::IntoIter {
        self.enemies.iter()
    }
}

impl<'a> IntoIterator for &'a mut HordeEnemies {
    type Item = &'a mut Enemy;
    type IntoIter = std::slice::IterMut<'a, Enemy>;

    fn into_iter(self) -> Self::IntoIter {
        self.enemies.iter_mut()
    }
}

impl IntoParallelIterator for HordeEnemies {
    type Item = Enemy;
    type Iter = rayon::vec::IntoIter<Enemy>;

    fn into_par_iter(self) -> Self::Iter {
        self.enemies.into_par_iter()
    }
}

impl<'a> IntoParallelIterator for &'a HordeEnemies {
    type Item = &'a Enemy;
    type Iter = rayon::slice::Iter<'a, Enemy>;

    fn into_par_iter(self) -> Self::Iter {
        self.enemies.par_iter()
    }
}

impl<'a> IntoParallelIterator for &'a mut HordeEnemies {
    type Item = &'a mut Enemy;
    type Iter = rayon::slice::IterMut<'a, Enemy>;

    fn into_par_iter(self) -> Self::Iter {
        self.enemies.par_iter_mut()
    }
}

impl HordeEnemies {
    pub fn new() -> Self {
        Self {
            enemies: Vec::new(),
        }
    }

    pub fn kill_touching(&mut self, loc: Vec2, distance: f32) {
        self.enemies.retain(|el| el.loc.distance(loc) >= distance);
    }

    pub fn get_ref(&self, idx: usize) -> Option<&Enemy> {
        self.enemies.get(idx)
    }

    pub fn get_ref_mut(&mut self, idx: usize) -> Option<&mut Enemy> {
        self.enemies.get_mut(idx)
    }

    pub fn sort_y(&mut self) {
        self.enemies.par_sort_unstable_by(|a, b| {
            a.loc
                .y
                .partial_cmp(&b.loc.y)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    pub fn sort(&mut self) {
        self.enemies.par_sort_unstable_by(|a, b| {
            match a
                .loc
                .y
                .partial_cmp(&b.loc.y)
                .unwrap_or(std::cmp::Ordering::Equal)
            {
                std::cmp::Ordering::Equal => a
                    .loc
                    .x
                    .partial_cmp(&b.loc.x)
                    .unwrap_or(std::cmp::Ordering::Equal),
                neq => neq,
            }
        });
    }

    pub fn move_all_enemies_towards(&mut self, player: Vec2) -> Nresult {
        let frametime = get_frame_time();
        self.enemies.par_iter_mut().for_each(|enemy| {
            if let Some(info) = enemymap::get_enemy_info(enemy.id) {
                enemy.loc = Self::move_pt_towards(enemy.loc, player, info.speed, frametime);
            }
        });
        Ok(())
    }

    pub fn spawn_around(&mut self, player: Vec2, map_size: Vec2, min_dist: f32, id: u32) {
        let mut loc = Vec2 {
            x: rand::gen_range(0., map_size.x),
            y: rand::gen_range(0., map_size.y),
        };
        while loc.distance(player) < min_dist {
            loc = Vec2 {
                x: random::<f32>() * map_size.x,
                y: random::<f32>() * map_size.y,
            };
        }
        self.append(id, loc);
    }

    pub fn append(&mut self, id: u32, loc: Vec2) {
        let enemy_info = enemymap::get_enemy_info(id).unwrap();
        self.enemies.push(Enemy {
            id,
            loc,
            velocity: Vec2::ZERO,
            animation: 0.0,
            health: enemy_info.health,
            stun_timer: 0.0,
        });
    }

    pub fn move_pt_towards(initial: Vec2, target: Vec2, distance: f32, time: f32) -> Vec2 {
        let dist_vec = target - initial;
        let norm = dist_vec.normalize();
        initial + norm * distance * time
    }
    pub fn update_anim_frames(&mut self) {
        let frame_time = get_frame_time();
        self.enemies
            .par_iter_mut()
            .for_each(|el| el.animation = (el.animation + frame_time) % 0.6);
    }
}

impl Default for HordeEnemies {
    fn default() -> Self {
        Self::new()
    }
}
