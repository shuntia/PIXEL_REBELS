use std::path::PathBuf;

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

const TYPES: usize = 2;

impl EnemyMap {
    async fn init() -> Result<Self> {
        let mut textures = vec![];

        for i in 0..TYPES {
            let mut file_futures = vec![];
            PathBuf::from("assets/enemies/")
                .read_dir()
                .unwrap()
                .filter(|el| {
                    el.as_ref()
                        .unwrap()
                        .file_name()
                        .to_str()
                        .unwrap()
                        .starts_with(&i.to_string())
                })
                .map(async |el| {
                    (
                        0.1,
                        Texture2D::from_file_with_format(
                            &load_file(el.unwrap().path().as_os_str().to_str().unwrap())
                                .await
                                .unwrap(),
                            None,
                        ),
                    )
                })
                .for_each(|el| file_futures.push(el));
            let mut tex = Vec::with_capacity(file_futures.len());

            for i in file_futures {
                tex.push(i.await);
            }
            textures.push(tex);
        }
        let mut contents = Vec::new();
        contents.push(EnemyKind {
            animation: textures.remove(0),
            cooldown: 0.5,
            attack: 1.,
            health: 20.,
            speed: 50.,
            stunnable: false,
        });
        contents.push(EnemyKind {
            animation: textures.remove(0),
            cooldown: 1.,
            attack: 2.,
            health: 30.,
            speed: 40.,
            stunnable: false,
        });
        Ok(EnemyMap { map: contents })
    }
    fn init_sync() -> Self {
        block_on(Self::init()).unwrap()
    }
}
