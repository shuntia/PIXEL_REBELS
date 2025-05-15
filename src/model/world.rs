use super::*;
use crate::assets::get_map;
use macroquad::prelude::*;

pub struct World {
    pub player_pos: Vec2,
    pub horde: enemies::HordeEnemies,
    pub map: u32,
}

impl World {
    pub fn new() -> Self {
        World {
            player_pos: vec2(0.0, 0.0),
            horde: HordeEnemies::new(),
            map: 0,
        }
    }
    pub fn map_size(&self) -> Vec2 {
        get_map(self.map).unwrap().size()
    }
}
