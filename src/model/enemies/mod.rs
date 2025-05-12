use macroquad::prelude::*;

pub struct HoardeEnemies {
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

impl HoardeEnemies {
    pub fn new() -> HoardeEnemies {
        HoardeEnemies {
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
        indices.sort_unstable_by(|&i, &j| {
            self.loc[i]
                .y
                .partial_cmp(&self.loc[j].y)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        self.id = indices.iter().map(|&i| self.id[i]).collect();
        self.loc = indices.iter().map(|&i| self.loc[i]).collect();
        self.velocity = indices.iter().map(|&i| self.velocity[i]).collect();
        self.animation = indices.iter().map(|&i| self.animation[i]).collect();
        self.health = indices.iter().map(|&i| self.health[i]).collect();
        self.stun_timer = indices.iter().map(|&i| self.stun_timer[i]).collect();
    }
}

impl Default for HoardeEnemies {
    fn default() -> Self {
        HoardeEnemies::new()
    }
}
