use macroquad::prelude::*;

use crate::errors::Result;

pub struct EnemyMap {
    map: Vec<EnemyKind>,
}

pub struct EnemyKind {
    animation: Vec<(f32, Texture2D)>,
    attack: f32,
    health: f32,
    speed: f32,
    stunnable: bool,
}

impl EnemyMap {
    async fn init() -> Result<Self> {
        Ok(EnemyMap {
            map: vec![EnemyKind {
                animation: vec![(
                    1.0,
                    Texture2D::from_image(&load_image("assets/enemies/placeholder.png").await?),
                )],
                attack: 1.0,
                health: 1.0,
                speed: 1.0,
                stunnable: false,
            }],
        })
    }
}
