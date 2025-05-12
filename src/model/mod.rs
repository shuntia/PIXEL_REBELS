use async_std::{path::PathBuf, stream::StreamExt};
use enemies::HoardeEnemies;
use macroquad::prelude::*;
use player::Player;

use crate::errors::{Nresult, Result};

mod damage;
mod enemies;
mod entity;
mod map;
mod player;

// CONSTANTS

const SAVE_LOC: &str = "save/";

pub enum GameMode {
    Menu,
    Play,
    Pause,
}

/// The GameModel is responsible for generating data that the
pub struct GameModel {
    pub mode: GameMode,
    pub world: Option<World>,
    pub player: Player,
}

impl GameModel {
    pub fn new() -> Self {
        GameModel {
            mode: GameMode::Menu,
            world: None,
            player: Player::default(),
        }
    }
    pub fn init(&mut self) -> Nresult {
        self.mode = GameMode::Menu;
        self.world = None;
        self.player = Player::default();
        Ok(())
    }
    pub async fn can_continue() -> bool {
        PathBuf::from(SAVE_LOC).read_dir().await.iter().len() != 0
    }
    pub async fn get_savefile_names() -> Result<Vec<String>> {
        let mut ret = Vec::new();
        PathBuf::from(SAVE_LOC)
            .read_dir()
            .await?
            .map(|p| p.expect("Failed to read save dir...").file_name())
            .for_each(|el| ret.push(el.into_string().expect("Thought that you used UTF-8. Maybe not use invalid characters in your filename.")))
            .await;
        Ok(ret)
    }
}

pub struct World {
    pub player_pos: Vec2,
    pub hoarde: enemies::HoardeEnemies,
    pub map: u32,
}

impl World {
    pub fn new() -> Self {
        World {
            player_pos: vec2(0.0, 0.0),
            hoarde: HoardeEnemies::new(),
            map: 0,
        }
    }
}

pub struct Stats {
    health: u32,
    lives: u32,
}
