use std::{
    fs::{File, create_dir, exists},
    io::Read,
    panic::catch_unwind,
    path::PathBuf,
};

use futures::executor::block_on;
use macroquad::prelude::*;
use once_cell::sync::Lazy;

use crate::errors::{GameError, Nresult, Result};

pub static MAPS: Lazy<Vec<Texture2D>> = Lazy::new(|| block_on(init_map()));
pub static SPRITES: Lazy<Vec<Texture2D>> = Lazy::new(|| block_on(init_sprites()));

const ASSET_LOC: &str = "assets/";
const MAP_LOC: &str = "maps/";
const PLAYER_ANIM_LOC: &str = "player_anim";

pub fn init_all() -> Nresult {
    if let Err(e) = catch_unwind(init_all_inner) {
        error!("FAILED TO INIT!!! {:?}", e);
    }
    Ok(())
}

fn init_all_inner() {
    if !exists("save/").unwrap() {
        let _ = create_dir("save/");
    }
    MAPS.is_empty();
    debug!("MAPS: {:?}", MAPS);
    SPRITES.is_empty();
}

async fn init_map() -> Vec<Texture2D> {
    let mut map_loc = PathBuf::from(ASSET_LOC);
    map_loc.push(MAP_LOC);
    let mut handles = Vec::new();
    for i in map_loc.read_dir().unwrap() {
        handles.push(gen_loader(i.unwrap().path()));
    }
    futures::future::join_all(handles).await
}

async fn init_sprites() -> Vec<Texture2D> {
    let mut player_anim_loc = PathBuf::from(ASSET_LOC);
    player_anim_loc.push(PLAYER_ANIM_LOC);
    let mut handles = Vec::new();
    for i in player_anim_loc.read_dir().unwrap() {
        handles.push(gen_loader(i.unwrap().path()));
    }
    futures::future::join_all(handles).await
}

#[allow(non_snake_case)]
pub async fn load_texture2D(path: PathBuf) -> Result<Texture2D> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(Texture2D::from_file_with_format(buf.as_slice(), None))
}

#[allow(clippy::manual_async_fn)]
fn gen_loader(path: PathBuf) -> impl std::future::Future<Output = Texture2D> {
    async move {
        let tex = load_texture(path.to_str().unwrap()).await.unwrap();
        tex.set_filter(FilterMode::Nearest);
        tex
    }
}

pub fn get_map(id: u32) -> Result<&'static Texture2D> {
    MAPS.get(id as usize)
        .ok_or(GameError::AssetLoadFailure(format!("MAP {id} NOT FOUND")))
}

pub static CROSSHAIR_TEX: Lazy<Texture2D> = Lazy::new(|| block_on(load_crosshair()));

async fn load_crosshair() -> Texture2D {
    let mut asset_loc = ASSET_LOC.to_owned();
    asset_loc.push_str("crosshair.png");
    load_texture(&asset_loc).await.unwrap()
}
