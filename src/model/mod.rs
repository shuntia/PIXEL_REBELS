use std::{
    fs::DirEntry,
    sync::{Arc, RwLock},
};

use async_std::{path::PathBuf, stream::StreamExt};
use damage::Damageable;
use enemies::{HordeEnemies, enemymap::get_enemy_info};
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use macroquad::prelude::*;
use once_cell::sync::Lazy;
use player::Player;
use rayon::iter::IntoParallelRefIterator;

use crate::{
    errors::{Nresult, Result},
    input::InputMan,
    renderer::Renderer,
    util::{find_in_distance, get_mouse_angle},
};
pub use world::World;

mod damage;
pub mod enemies;
mod entity;
pub mod player;
pub mod weapons;
pub mod world;

// CONSTANTS

pub const SAVE_LOC: &str = "save/";
pub const DAMAGE_DIST: f32 = 10.;

#[derive(Clone, Copy)]
pub enum TitlePhase {
    Start,
    Menu(u32),
}

#[derive(Clone, Copy)]
pub enum GameMode {
    Title { phase: TitlePhase },
    Play,
    Pause,
}
pub static SAVE_PATHBUF_CACHE: Lazy<RwLock<Vec<std::path::PathBuf>>> = Lazy::new(|| {
    RwLock::new(
        std::path::PathBuf::from(SAVE_LOC)
            .read_dir()
            .unwrap()
            .map(|el| el.unwrap())
            .map(|el| DirEntry::path(&el))
            .collect(),
    )
});

/// The GameModel is responsible for generating data that the
pub struct GameModel {
    pub debug: Option<Arc<UnboundedSender<String>>>,
    pub status: Status,
    pub world: World,
    pub player: Player,
    pub renderer: Renderer,
    pub input: InputMan,
}

impl GameModel {
    pub fn new() -> Self {
        GameModel {
            debug: None,
            status: Status {
                health: 100,
                lives: 5,
                mode: GameMode::Title {
                    phase: TitlePhase::Start,
                },
            },
            world: World {
                player_pos: vec2(0.0, 0.0),
                horde: HordeEnemies::new(),
                map: 0,
            },
            player: Player::default(),
            renderer: Renderer::new(),
            input: InputMan::new(),
        }
    }
    pub fn init(&mut self) -> Nresult {
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
    pub async fn call_render(&mut self) {
        self.renderer.render_world(&self.world, &self.player).await;
        self.renderer.render_ui(&self.status).await;
    }
    pub fn update(&mut self) {
        match self.status.mode {
            GameMode::Title { .. } => self.update_title(),
            GameMode::Play => self.update_gameplay(),
            GameMode::Pause => self.update_pause(),
            _ => todo!("implement"),
        }
    }
    fn update_title(&mut self) {
        if let GameMode::Title { phase } = self.status.mode {
            match phase {
                TitlePhase::Start => {
                    if self.input.kbd.keypress(KeyCode::Enter) {
                        self.status.mode = GameMode::Title {
                            phase: TitlePhase::Menu(0),
                        }
                    }
                }
                TitlePhase::Menu(selection) => {
                    if self.input.kbd.keypress(KeyCode::Up) && selection != 0 {
                        self.status.mode = GameMode::Title {
                            phase: TitlePhase::Menu(selection - 1),
                        }
                    }
                    if self.input.kbd.keypress(KeyCode::Down)
                        && !SAVE_PATHBUF_CACHE.read().unwrap().is_empty()
                        && selection != SAVE_PATHBUF_CACHE.read().unwrap().len() as u32 - 1
                    {
                        self.status.mode = GameMode::Title {
                            phase: TitlePhase::Menu(selection - 1),
                        }
                    }
                    if self.input.kbd.keypress(KeyCode::Enter) {
                        self.status.mode = GameMode::Play;
                    }
                }
                _ => {}
            }
        }
    }

    fn update_gameplay(&mut self) {
        let _ = self.log(&format!("MOUSE ANGLE: {}", get_mouse_angle()));
        let (x, y) = mouse_position();
        let _ = self.log(&format!("MOUSE POS: {} {}", x, y));
        self.catch_pause();
        self.move_player();
        self.update_enemies().expect("Should work.");
        self.update_attack();
        self.update_map();
        self.update_damage();
    }
    fn update_damage(&mut self) -> Nresult {
        if let Ok(o) = find_in_distance(&mut self.world.horde, self.world.player_pos, DAMAGE_DIST) {
            o.iter().for_each(|el| {
                self.player
                    .take_damage_raw(get_enemy_info(el.id).unwrap().attack);
            });
        }
        if self.player.health <= 0. {
            panic!("You died!!!\nI haven't made the game over though:P")
        }
        self.log(&format!("health: {}", self.player.health));
        Ok(())
    }
    fn update_attack(&mut self) {
        self.player.weapon.adjust_cooldown();
        self.log(&format!(
            "cooldown: {}",
            self.player.weapon.cooldown_counter
        ));
        if is_mouse_button_down(MouseButton::Left) {
            let _ = self.player.weapon.attack(&mut self.world);
        }
    }
    fn update_map(&mut self) {
        clear_background(GRAY);
    }
    fn update_enemies(&mut self) -> Nresult {
        if self.input.kbd.keydown(KeyCode::Space) {
            self.world
                .horde
                .spawn_around(self.world.player_pos, self.world.map_size(), 100., 0);
        }
        self.world
            .horde
            .move_all_enemies_towards(self.world.player_pos)?;
        self.world.horde.sort();
        self.world.horde.retain(|el| el.health > 0.);
        Ok(())
    }
    fn set_y(&mut self, y: f32) {
        if y < 0. {
            self.world.player_pos.y = 0.;
        } else if y > self.world.map_size().y {
            self.world.player_pos.y = self.world.map_size().y;
        } else {
            self.world.player_pos.y = y;
        }
    }
    fn set_x(&mut self, x: f32) {
        if x < 0. {
            self.world.player_pos.x = 0.;
        } else if x > self.world.map_size().x {
            self.world.player_pos.x = self.world.map_size().x;
        } else {
            self.world.player_pos.x = x;
        }
    }
    fn move_x(&mut self, delta_x: f32) {
        self.set_x(self.world.player_pos.x + delta_x)
    }
    fn move_y(&mut self, delta_y: f32) {
        self.set_y(self.world.player_pos.y + delta_y)
    }
    fn move_player(&mut self) {
        //TODO implement hitboxes and out of bounds
        if self.input.kbd.keydown(KeyCode::W) {
            self.move_y(-get_frame_time() * self.player.speed);
        }
        if self.input.kbd.keydown(KeyCode::S) {
            self.move_y(get_frame_time() * self.player.speed);
        }
        if self.input.kbd.keydown(KeyCode::A) {
            self.move_x(-get_frame_time() * self.player.speed);
        }
        if self.input.kbd.keydown(KeyCode::D) {
            self.move_x(get_frame_time() * self.player.speed);
        }
    }
    fn catch_pause(&mut self) {
        if self.input.kbd.keypress(KeyCode::Escape) {
            self.status.mode = GameMode::Pause;
        }
    }
    fn update_pause(&mut self) {
        if self.input.kbd.keypress(KeyCode::Escape) {
            self.status.mode = GameMode::Play;
        }
    }
    pub fn set_debug_tx(&mut self, sender: Arc<UnboundedSender<String>>) -> Nresult {
        self.debug = Some(sender);
        Ok(())
    }
    pub fn call_render_dbg(&mut self, rx: &mut UnboundedReceiver<String>) -> Nresult {
        self.renderer.render_debug(rx)
    }
    pub fn log(&mut self, s: &str) -> Nresult {
        if let Some(tx) = &self.debug {
            tx.unbounded_send(s.into())
                .map_err(|el| crate::GameError::Misc(Box::new(el)))?;
        }
        Ok(())
    }
}

pub struct Status {
    pub mode: GameMode,
    pub health: u32,
    pub lives: u32,
}
