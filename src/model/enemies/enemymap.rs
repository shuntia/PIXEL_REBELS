use futures::executor::block_on;
use macroquad::prelude::*;
use once_cell::sync::Lazy;

use crate::errors::Result;

pub struct EnemyMap {
    map: Vec<EnemyKind>,
}

pub struct EnemyKind {
    pub animation: Vec<(f32, Texture2D)>,
    pub cooldown: f32,
    pub attack: f32,
    pub health: f32,
    pub speed: f32,
    pub stunnable: bool,
}

static ENEMYMAP: Lazy<EnemyMap> = Lazy::new(EnemyMap::init_sync);

pub fn get_enemy_info(id: u32) -> Option<&'static EnemyKind> {
    ENEMYMAP.map.get(id as usize)
}

impl EnemyMap {
    async fn init() -> Result<Self> {
        Ok(EnemyMap {
            map: vec![EnemyKind {
                animation: vec![(
                    1.,
                    Texture2D::from_image(&load_image("assets/enemies/placeholder.png").await?),
                )],
                cooldown: 3.,
                attack: 1.,
                health: 1.,
                speed: 1.,
                stunnable: false,
            }],
        })
    }
    fn init_sync() -> Self {
        block_on(Self::init()).unwrap()
    }
}
