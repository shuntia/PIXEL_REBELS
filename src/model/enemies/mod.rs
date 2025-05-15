use ::rand::random;
use macroquad::prelude::*;
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::errors::Nresult;

mod enemymap;

pub struct HordeEnemies {
    pub id: Vec<u32>,
    pub loc: Vec<Vec2>,
    pub velocity: Vec<Vec2>,
    pub animation: Vec<f32>,
    pub health: Vec<f32>,
    pub stun_timer: Vec<f32>,
}

pub struct EnemyRef<'a> {
    id: &'a u32,
    loc: &'a Vec2,
    velocity: &'a Vec2,
    animation: &'a f32,
    health: &'a f32,
    stun_timer: &'a f32,
}

pub struct EnemyRefMut<'a> {
    id: &'a mut u32,
    loc: &'a mut Vec2,
    velocity: &'a mut Vec2,
    animation: &'a mut f32,
    health: &'a mut f32,
    stun_timer: &'a mut f32,
}

impl HordeEnemies {
    pub fn new() -> HordeEnemies {
        HordeEnemies {
            id: Vec::new(),
            loc: Vec::new(),
            velocity: Vec::new(),
            animation: Vec::new(),
            health: Vec::new(),
            stun_timer: Vec::new(),
        }
    }

    pub fn get_ref(&self, idx: usize) -> Option<EnemyRef<'_>> {
        Some(EnemyRef {
            id: self.id.get(idx)?,
            loc: self.loc.get(idx)?,
            velocity: self.velocity.get(idx)?,
            animation: self.animation.get(idx)?,
            health: self.health.get(idx)?,
            stun_timer: self.stun_timer.get(idx)?,
        })
    }

    pub fn get_ref_mut(&mut self, idx: usize) -> Option<EnemyRefMut<'_>> {
        Some(EnemyRefMut {
            id: self.id.get_mut(idx)?,
            loc: self.loc.get_mut(idx)?,
            velocity: self.velocity.get_mut(idx)?,
            animation: self.animation.get_mut(idx)?,
            health: self.health.get_mut(idx)?,
            stun_timer: self.stun_timer.get_mut(idx)?,
        })
    }

    pub fn sort_y(&mut self) {
        let mut indices: Vec<usize> = (0..self.id.len()).collect();
        indices.par_sort_unstable_by(|&i, &j| {
            self.loc[i]
                .y
                .partial_cmp(&self.loc[j].y)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        self.id = indices.par_iter().map(|&i| self.id[i]).collect();
        self.loc = indices.par_iter().map(|&i| self.loc[i]).collect();
        self.velocity = indices.par_iter().map(|&i| self.velocity[i]).collect();
        self.animation = indices.par_iter().map(|&i| self.animation[i]).collect();
        self.health = indices.par_iter().map(|&i| self.health[i]).collect();
        self.stun_timer = indices.par_iter().map(|&i| self.stun_timer[i]).collect();
    }
    pub fn move_all_enemies_towards(&mut self, player: Vec2) -> Nresult {
        self.loc = self
            .loc
            .par_iter()
            .zip(self.id.par_iter())
            .map(|(loc, id)| {
                Self::move_pt_towards(
                    *loc,
                    player,
                    enemymap::get_enemy_info(*id)
                        .expect("Illegal entity!")
                        .speed,
                )
            })
            .collect();
        Ok(())
    }
    pub fn spawn_around(&mut self, player: Vec2, map_size: Vec2, min_dist: f32, id: u32) {
        let mut loc = Vec2 {
            x: random::<f32>() % map_size.x,
            y: random::<f32>() % map_size.y,
        };
        while loc.distance(player) < min_dist {
            loc = Vec2 {
                x: random::<f32>() % map_size.x,
                y: random::<f32>() % map_size.y,
            };
        }
        self.append(id, loc);
    }

    pub fn append(&mut self, id: u32, loc: Vec2) {
        let enemy = enemymap::get_enemy_info(id).unwrap();
        self.id.push(id);
        self.loc.push(loc);
        self.animation.push(0.);
        self.health.push(enemy.health);
        self.velocity.push(Vec2::ZERO);
        self.stun_timer.push(0.);
    }

    pub fn move_pt_towards(initial: Vec2, target: Vec2, distance: f32) -> Vec2 {
        let dist_vec = target - initial;
        let dist = dist_vec.length();
        let norm = dist_vec.normalize();
        if dist <= distance * get_frame_time() {
            target
        } else {
            initial + norm * distance * get_frame_time()
        }
    }
}

impl Default for HordeEnemies {
    fn default() -> Self {
        HordeEnemies::new()
    }
}
